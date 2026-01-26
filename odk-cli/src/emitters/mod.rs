//! Native emitter modules for generating output files.
//!
//! These are standalone Rust implementations that don't require TypeSpec.
//! They're faster but less feature-complete than TypeSpec emitters.

pub mod json_schema;
pub mod openapi;
pub mod typescript;

use anyhow::Result;
use camino::Utf8Path;
use odk_schema::CompiledSchema;

/// Supported native emit targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeEmitTarget {
    JsonSchema,
    OpenApi,
    TypeScript,
}

impl NativeEmitTarget {
    /// Parse a target from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json-schema" | "jsonschema" => Some(Self::JsonSchema),
            "openapi" | "openapi3" => Some(Self::OpenApi),
            "typescript" | "ts" => Some(Self::TypeScript),
            _ => None,
        }
    }

    /// Get all available targets.
    pub fn all() -> Vec<Self> {
        vec![Self::JsonSchema, Self::OpenApi, Self::TypeScript]
    }

    /// Get target name for display.
    pub fn name(&self) -> &'static str {
        match self {
            Self::JsonSchema => "json-schema",
            Self::OpenApi => "openapi",
            Self::TypeScript => "typescript",
        }
    }
}

/// Emit a compiled schema to the specified native targets.
pub fn emit(
    schema: &CompiledSchema,
    targets: &[NativeEmitTarget],
    out_dir: &Utf8Path,
) -> Result<()> {
    for target in targets {
        eprintln!("  Emitting {}...", target.name());
        match target {
            NativeEmitTarget::JsonSchema => json_schema::emit(schema, out_dir)?,
            NativeEmitTarget::OpenApi => openapi::emit(schema, out_dir)?,
            NativeEmitTarget::TypeScript => typescript::emit(schema, out_dir)?,
        }
    }
    Ok(())
}
