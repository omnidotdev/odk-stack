//! JSON Schema emitter.
//!
//! Generates JSON Schema files for all models and unions.

use anyhow::Result;
use camino::Utf8Path;
use odk_schema::{CompiledModel, CompiledSchema, CompiledType, CompiledUnion, ScalarType};
use serde_json::{Value, json};
use std::fs::{self, File};
use std::io::Write;

/// Emit JSON Schema files for all models and unions.
pub fn emit(schema: &CompiledSchema, out_dir: &Utf8Path) -> Result<()> {
    let schema_dir = out_dir.join("json-schema");
    fs::create_dir_all(&schema_dir)?;

    // Emit models
    for model in &schema.models {
        let json_schema = model_to_json_schema(model);
        let path = schema_dir.join(format!("{}.json", model.name));
        let mut file = File::create(&path)?;
        file.write_all(serde_json::to_string_pretty(&json_schema)?.as_bytes())?;
    }

    // Emit unions
    for union in &schema.unions {
        let json_schema = union_to_json_schema(union);
        let path = schema_dir.join(format!("{}.json", union.name));
        let mut file = File::create(&path)?;
        file.write_all(serde_json::to_string_pretty(&json_schema)?.as_bytes())?;
    }

    Ok(())
}

fn model_to_json_schema(model: &CompiledModel) -> Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for field in &model.fields {
        let mut field_schema = compiled_type_to_json_schema(&field.field_type);

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
            if let Some(multiple_of) = constraints.multiple_of {
                field_schema["multipleOf"] = json!(multiple_of);
            }
        }

        properties.insert(field.name.clone(), field_schema);

        if field.optional != Some(true) {
            required.push(field.name.clone());
        }
    }

    let mut schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": format!("{}.json", model.name),
        "title": model.name,
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

fn union_to_json_schema(union: &CompiledUnion) -> Value {
    let variants: Vec<String> = union.variants.iter().map(|v| v.name.clone()).collect();

    let mut schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": format!("{}.json", union.name),
        "title": union.name,
        "enum": variants,
    });

    if let Some(desc) = &union.description {
        schema["description"] = json!(desc);
    }

    schema
}

fn compiled_type_to_json_schema(compiled_type: &CompiledType) -> Value {
    match compiled_type {
        CompiledType::Scalar { name } => scalar_to_json_schema(name),
        CompiledType::Ref { name } => json!({ "$ref": format!("{}.json", name) }),
        CompiledType::Array { items } => json!({
            "type": "array",
            "items": compiled_type_to_json_schema(items)
        }),
        CompiledType::Literal { value } => match value {
            odk_schema::LiteralValue::String(s) => json!({ "const": s }),
            odk_schema::LiteralValue::Number(n) => json!({ "const": n }),
            odk_schema::LiteralValue::Bool(b) => json!({ "const": b }),
        },
        CompiledType::Union { variants } => {
            let schemas: Vec<Value> = variants.iter().map(compiled_type_to_json_schema).collect();
            json!({ "oneOf": schemas })
        }
        CompiledType::Object { fields } => {
            let mut properties = serde_json::Map::new();
            let mut required = Vec::new();

            for field in fields {
                properties.insert(
                    field.name.clone(),
                    compiled_type_to_json_schema(&field.field_type),
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

fn scalar_to_json_schema(scalar: &ScalarType) -> Value {
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
