//! ODK CLI - Unified Schema Compilation Tool
//!
//! Compile schemas from various sources to multiple targets:
//! - Input: TypeScript (.ts), WASM plugins (.wasm), JSON (.json)
//! - Output: TypeSpec emitters or native Rust emitters

use anyhow::{Context, Result, bail};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use odk_schema::CompiledSchema;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

mod emitters;
mod error_mapper;
mod extism_host;
mod loaders;
mod source_map;
mod ts_executor;
mod ts_schema_adapter;
mod typespec_bridge;

#[derive(Parser, Debug)]
#[command(name = "odk", version, about = "ODK - Omni Development Kit CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile a schema and emit to targets
    Compile {
        /// Input source: .ts, .yaml, .json, or .wasm file
        input: Utf8PathBuf,

        /// Output directory for generated files
        #[arg(short, long, default_value = "./generated")]
        out: Utf8PathBuf,

        /// Emitters to use (comma-separated or multiple -e flags)
        /// Native: json-schema, openapi, typescript
        /// TypeSpec: @typespec/json-schema, @typespec/openapi3, etc.
        /// Default: @typespec/json-schema, @typespec/openapi3
        #[arg(short, long, value_delimiter = ',')]
        emit: Option<Vec<String>>,

        /// Use native Rust emitters instead of TypeSpec (faster, no Node.js required)
        #[arg(long)]
        native: bool,

        /// Namespace for generated TypeSpec (overrides schema name)
        #[arg(short, long)]
        namespace: Option<String>,

        /// Keep temporary TypeSpec files (for debugging)
        #[arg(long)]
        keep_temp: bool,
    },

    /// Initialize a new ODK project
    Init,

    /// Check/validate a schema without emitting
    Check {
        /// Input source: .ts, .yaml, .json, or .wasm file
        input: Utf8PathBuf,
    },

    /// Generate TypeSpec source from a schema (without emitting)
    Typespec {
        /// Input source: .ts, .yaml, .json, or .wasm file
        input: Utf8PathBuf,

        /// Output .tsp file (stdout if not specified)
        #[arg(short, long)]
        out: Option<Utf8PathBuf>,

        /// Namespace for generated TypeSpec
        #[arg(short, long)]
        namespace: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile {
            input,
            out,
            emit,
            native,
            namespace,
            keep_temp,
        } => cmd_compile(input, out, emit, native, namespace, keep_temp),

        Commands::Init => cmd_init(),

        Commands::Check { input } => cmd_check(input),

        Commands::Typespec {
            input,
            out,
            namespace,
        } => cmd_typespec(input, out, namespace),
    }
}

/// Load a schema from the given input source (auto-detected by extension).
fn load_schema(input: &Utf8PathBuf) -> Result<CompiledSchema> {
    let ext = input.extension().unwrap_or("");

    match ext {
        "ts" => {
            // TypeScript schema via bun + odk-ts
            eprintln!("  Compiling TypeScript schema via bun...");
            let ts_schema = ts_executor::compile_ts_schema(input)?;
            Ok(ts_schema.to_compiled_schema())
        }
        "wasm" => {
            // WASM plugin via Extism
            extism_host::load_schema_from_plugin(input)
        }
        "json" => {
            // JSON schema file - try ODK YAML format first, then raw CompiledSchema
            let path = std::path::Path::new(input.as_str());
            match loaders::load_yaml_schema(path) {
                Ok(schema) => Ok(schema),
                Err(_) => {
                    // Fall back to raw CompiledSchema JSON
                    let content = fs::read_to_string(input)
                        .with_context(|| format!("Failed to read {}", input))?;
                    CompiledSchema::from_json(&content)
                        .with_context(|| format!("Failed to parse JSON schema from {}", input))
                }
            }
        }
        "yaml" | "yml" => {
            // YAML schema file (ODK format)
            let path = std::path::Path::new(input.as_str());
            loaders::load_yaml_schema(path)
                .map_err(|e| anyhow::anyhow!("Failed to load YAML schema: {}", e))
        }
        _ => bail!(
            "Unsupported input format: .{}. Use .ts, .wasm, .json, or .yaml",
            ext
        ),
    }
}

/// Compile command: load schema and emit to targets.
fn cmd_compile(
    input: Utf8PathBuf,
    out: Utf8PathBuf,
    emit: Option<Vec<String>>,
    native: bool,
    namespace: Option<String>,
    keep_temp: bool,
) -> Result<()> {
    // Load schema
    eprintln!("Loading schema from {}...", input);
    let schema = load_schema(&input)?;
    eprintln!(
        "  Loaded: {} ({} models, {} unions)",
        schema.name,
        schema.models.len(),
        schema.unions.len()
    );

    let emit = emit.unwrap_or_default();

    if native {
        // Use native Rust emitters
        compile_with_native(&schema, &emit, &out)
    } else {
        // Use TypeSpec emitters (default)
        compile_with_typespec(&schema, &emit, &out, namespace, keep_temp)
    }
}

/// Compile using native Rust emitters.
fn compile_with_native(
    schema: &CompiledSchema,
    targets: &[String],
    out_dir: &Utf8PathBuf,
) -> Result<()> {
    eprintln!("Using native emitters...");

    // Parse targets
    let emit_targets: Vec<emitters::NativeEmitTarget> = if targets.is_empty() {
        emitters::NativeEmitTarget::all()
    } else {
        targets
            .iter()
            .filter_map(|t| emitters::NativeEmitTarget::from_str(t))
            .collect()
    };

    if emit_targets.is_empty() {
        bail!(
            "No valid native emit targets. Valid targets: json-schema, openapi, typescript\n\
             For TypeSpec emitters (e.g., @typespec/openapi3), remove the --native flag."
        );
    }

    // Emit
    emitters::emit(schema, &emit_targets, out_dir)?;

    eprintln!("Generated output to {}", out_dir);
    Ok(())
}

/// Compile using TypeSpec emitters.
fn compile_with_typespec(
    schema: &CompiledSchema,
    emit: &[String],
    out: &Utf8PathBuf,
    namespace: Option<String>,
    keep_temp: bool,
) -> Result<()> {
    // Check that tsp is available
    if which::which("tsp").is_err() {
        bail!(
            "TypeSpec compiler (tsp) not found. Install it with:\n\
             npm install -g @typespec/compiler\n\n\
             Or use --native flag for Rust emitters (no Node.js required)."
        );
    }

    // Default emitters if none specified
    let emitters: Vec<String> = if emit.is_empty() {
        vec![
            "@typespec/json-schema".to_string(),
            "@typespec/openapi3".to_string(),
        ]
    } else {
        emit.to_vec()
    };

    eprintln!("Using TypeSpec emitters: {}", emitters.join(", "));

    // Generate TypeSpec source
    let ns = namespace.as_deref().unwrap_or(&schema.name);
    let tsp_source = typespec_bridge::compiled_to_typespec(schema, ns);

    // Create temp directory for TypeSpec project
    let temp_dir = tempfile::tempdir()?;
    let temp_path = Utf8PathBuf::try_from(temp_dir.path().to_path_buf())?;
    let tsp_file = temp_path.join("main.tsp");

    // Write TypeSpec source
    fs::write(&tsp_file, &tsp_source)?;

    // Create package.json
    let package_json = serde_json::json!({
        "name": "odk-temp-project",
        "type": "module",
        "dependencies": {
            "@typespec/compiler": "*",
            "@typespec/json-schema": "*"
        }
    });
    fs::write(temp_path.join("package.json"), package_json.to_string())?;

    // Find and symlink node_modules
    let search_start = Utf8PathBuf::try_from(std::env::current_dir()?)?;
    if let Some(node_modules) = find_node_modules(&search_start) {
        let temp_node_modules = temp_path.join("node_modules");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&node_modules, &temp_node_modules)
            .with_context(|| format!("Failed to symlink node_modules"))?;
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(&node_modules, &temp_node_modules)
            .with_context(|| format!("Failed to symlink node_modules"))?;
        eprintln!("  Using node_modules from {}", node_modules.display());
    } else {
        eprintln!("  Installing TypeSpec dependencies...");
        let npm_output = Command::new("npm")
            .args(["install", "--silent"])
            .current_dir(&temp_path)
            .output()
            .context("Failed to run npm install")?;
        if !npm_output.status.success() {
            bail!(
                "npm install failed: {}",
                String::from_utf8_lossy(&npm_output.stderr)
            );
        }
    }

    // Create tspconfig.yaml
    let absolute_out = if out.is_absolute() {
        out.clone()
    } else {
        Utf8PathBuf::try_from(std::env::current_dir()?)?.join(out)
    };

    let tsp_config = format!(
        "output-dir: \"{}\"\nemit:\n{}\noptions:\n{}",
        absolute_out,
        emitters
            .iter()
            .map(|e| format!("  - \"{}\"", e))
            .collect::<Vec<_>>()
            .join("\n"),
        emitters
            .iter()
            .map(|e| format!("  \"{}\":\n    emitter-output-dir: \"{{output-dir}}\"", e))
            .collect::<Vec<_>>()
            .join("\n")
    );
    fs::write(temp_path.join("tspconfig.yaml"), &tsp_config)?;

    // Ensure output directory exists
    fs::create_dir_all(&absolute_out)?;

    // Run TypeSpec compiler
    eprintln!("Running TypeSpec compiler...");
    let output = Command::new("tsp")
        .args(["compile", tsp_file.as_str()])
        .current_dir(&temp_path)
        .output()
        .context("Failed to run tsp compiler")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        eprintln!("TypeSpec stdout:\n{}", stdout);
        eprintln!("TypeSpec stderr:\n{}", stderr);
        bail!("TypeSpec compilation failed");
    }

    eprintln!("Generated output to {}", absolute_out);

    // Keep temp files if requested
    if keep_temp {
        let kept_path = temp_dir.keep();
        eprintln!("Kept temp files at: {}", kept_path.display());
    }

    Ok(())
}

/// Find node_modules directory by searching upward.
fn find_node_modules(start: &Utf8PathBuf) -> Option<PathBuf> {
    let mut current = start.as_std_path().to_path_buf();
    loop {
        let node_modules = current.join("node_modules");
        if node_modules.is_dir() && node_modules.join("@typespec").is_dir() {
            return Some(node_modules);
        }
        if !current.pop() {
            return None;
        }
    }
}

/// Init command: create a new ODK project.
fn cmd_init() -> Result<()> {
    eprintln!("Initializing ODK project...");

    let schema_content = r#"import { schema, model, field, union } from "@omnidotdev/odk";

// Define enums/unions
export const UserRole = union("UserRole", ["admin", "user", "guest"]);

// Define models
export const User = model("User", {
  id: field.uuid(),
  email: field.string(),
  name: field.string().optional(),
  role: field.ref(UserRole),
  createdAt: field.timestamp(),
});

// Export schema
export default schema("MyApp", [User, UserRole]);
"#;

    fs::write("schema.ts", schema_content)?;
    eprintln!("Created schema.ts");

    eprintln!(
        r#"
Next steps:
  1. Edit schema.ts to define your models
  2. Run: odk compile schema.ts

Options:
  TypeSpec emitters (default, recommended):
    odk compile schema.ts -e @typespec/openapi3,@typespec/json-schema

  Native emitters (faster, no Node.js):
    odk compile schema.ts -e json-schema,openapi,typescript --native
"#
    );

    Ok(())
}

/// Check command: validate a schema without emitting.
fn cmd_check(input: Utf8PathBuf) -> Result<()> {
    eprintln!("Checking {}...", input);

    let schema = load_schema(&input)?;

    println!("Schema: {}", schema.name);
    println!("Models: {}", schema.models.len());
    for model in &schema.models {
        println!("  - {} ({} fields)", model.name, model.fields.len());
    }
    println!("Unions: {}", schema.unions.len());
    for union in &schema.unions {
        println!("  - {} ({} variants)", union.name, union.variants.len());
    }

    if let Some(warnings) = &schema.warnings {
        if !warnings.is_empty() {
            println!("\nWarnings:");
            for w in warnings {
                println!("  - {}", w);
            }
        }
    }

    println!("\nSchema is valid!");
    Ok(())
}

/// Typespec command: generate TypeSpec source only.
fn cmd_typespec(
    input: Utf8PathBuf,
    out: Option<Utf8PathBuf>,
    namespace: Option<String>,
) -> Result<()> {
    let schema = load_schema(&input)?;
    let ns = namespace.as_deref().unwrap_or(&schema.name);
    let tsp_source = typespec_bridge::compiled_to_typespec(&schema, ns);

    match out {
        Some(path) => {
            fs::write(&path, &tsp_source)?;
            eprintln!("Wrote TypeSpec to {}", path);
        }
        None => {
            println!("{}", tsp_source);
        }
    }

    Ok(())
}
