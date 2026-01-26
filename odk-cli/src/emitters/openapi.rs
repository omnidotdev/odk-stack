//! OpenAPI 3.1 emitter.
//!
//! Generates an OpenAPI 3.1 specification with all models as component schemas.

use anyhow::Result;
use camino::Utf8Path;
use odk_schema::{CompiledModel, CompiledSchema, CompiledType, ScalarType};
use serde_json::{Value, json};
use std::fs::{self, File};
use std::io::Write;

/// Emit an OpenAPI 3.1 specification.
pub fn emit(schema: &CompiledSchema, out_dir: &Utf8Path) -> Result<()> {
    let openapi_dir = out_dir.join("openapi");
    fs::create_dir_all(&openapi_dir)?;

    let mut components = serde_json::Map::new();

    // Add models as schemas
    for model in &schema.models {
        components.insert(model.name.clone(), model_to_schema(model));
    }

    // Add unions as enum schemas
    for union in &schema.unions {
        let variants: Vec<String> = union.variants.iter().map(|v| v.name.clone()).collect();
        let mut union_schema = json!({ "type": "string", "enum": variants });
        if let Some(desc) = &union.description {
            union_schema["description"] = json!(desc);
        }
        components.insert(union.name.clone(), union_schema);
    }

    let spec = json!({
        "openapi": "3.1.0",
        "info": {
            "title": schema.name,
            "version": "1.0.0",
        },
        "paths": {},
        "components": {
            "schemas": components
        }
    });

    let path = openapi_dir.join("openapi.json");
    let mut file = File::create(&path)?;
    file.write_all(serde_json::to_string_pretty(&spec)?.as_bytes())?;

    Ok(())
}

fn model_to_schema(model: &CompiledModel) -> Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for field in &model.fields {
        let mut field_schema = compiled_type_to_schema(&field.field_type);

        // Add constraints if present
        if let Some(constraints) = &field.constraints {
            if let Some(min_length) = constraints.min_length {
                field_schema["minLength"] = json!(min_length);
            }
            if let Some(max_length) = constraints.max_length {
                field_schema["maxLength"] = json!(max_length);
            }
            if let Some(min_value) = constraints.min_value {
                field_schema["minimum"] = json!(min_value);
            }
            if let Some(max_value) = constraints.max_value {
                field_schema["maximum"] = json!(max_value);
            }
            if let Some(pattern) = &constraints.pattern {
                field_schema["pattern"] = json!(pattern);
            }
            if let Some(format) = &constraints.format {
                field_schema["format"] = json!(format);
            }
        }

        if let Some(desc) = &field.description {
            field_schema["description"] = json!(desc);
        }

        properties.insert(field.name.clone(), field_schema);

        if field.optional != Some(true) {
            required.push(field.name.clone());
        }
    }

    let mut schema = json!({
        "type": "object",
        "properties": properties,
    });

    if !required.is_empty() {
        schema["required"] = json!(required);
    }

    if let Some(desc) = &model.description {
        schema["description"] = json!(desc);
    }

    schema
}

fn compiled_type_to_schema(compiled_type: &CompiledType) -> Value {
    match compiled_type {
        CompiledType::Scalar { name } => scalar_to_schema(name),
        CompiledType::Ref { name } => json!({ "$ref": format!("#/components/schemas/{}", name) }),
        CompiledType::Array { items } => json!({
            "type": "array",
            "items": compiled_type_to_schema(items)
        }),
        CompiledType::Literal { value } => match value {
            odk_schema::LiteralValue::String(s) => json!({ "const": s }),
            odk_schema::LiteralValue::Number(n) => json!({ "const": n }),
            odk_schema::LiteralValue::Bool(b) => json!({ "const": b }),
        },
        CompiledType::Union { variants } => {
            let schemas: Vec<Value> = variants.iter().map(compiled_type_to_schema).collect();
            json!({ "oneOf": schemas })
        }
        CompiledType::Object { fields } => {
            let mut properties = serde_json::Map::new();
            let mut required = Vec::new();

            for field in fields {
                properties.insert(
                    field.name.clone(),
                    compiled_type_to_schema(&field.field_type),
                );
                if field.optional != Some(true) {
                    required.push(field.name.clone());
                }
            }

            let mut obj = json!({
                "type": "object",
                "properties": properties,
            });
            if !required.is_empty() {
                obj["required"] = json!(required);
            }
            obj
        }
    }
}

fn scalar_to_schema(scalar: &ScalarType) -> Value {
    match scalar {
        ScalarType::String => json!({ "type": "string" }),
        ScalarType::Int32 | ScalarType::Int64 => json!({ "type": "integer" }),
        ScalarType::Float32 | ScalarType::Float64 => json!({ "type": "number" }),
        ScalarType::Boolean => json!({ "type": "boolean" }),
        ScalarType::Date | ScalarType::PlainDate => json!({ "type": "string", "format": "date" }),
        ScalarType::Datetime | ScalarType::UtcDateTime | ScalarType::OffsetDateTime => {
            json!({ "type": "string", "format": "date-time" })
        }
        ScalarType::PlainTime => json!({ "type": "string", "format": "time" }),
        ScalarType::Duration => json!({ "type": "string", "format": "duration" }),
        ScalarType::Bytes => json!({ "type": "string", "contentEncoding": "base64" }),
        ScalarType::Url => json!({ "type": "string", "format": "uri" }),
        ScalarType::Unknown => json!({ "type": "string" }),
    }
}
