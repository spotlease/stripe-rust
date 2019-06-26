use serde_json;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct Spec {
  pub schemas: BTreeMap<String, Schema>,
  // pub resources: BTreeMap<String, Resource>
}

#[derive(Debug)]
pub struct Schema {
  pub is_resource: bool,
  pub data: SchemaData,
}

#[derive(Debug)]
pub enum SchemaData {
  Properties(BTreeMap<String, SchemaProperty>),
  Polymorphic(Vec<String>),
}

#[derive(Debug)]
pub enum SchemaProperty {
  Reference(String),
  Expandable(Box<SchemaProperty>),
  Value {
    value_type: SchemaPropertyValueType,
    description: Option<String>,
    is_nullable: bool,
    is_expandable: bool,
  },
}

#[derive(Debug)]
pub enum SchemaPropertyValueType {
  String_,
  Integer,
  Number,
  Boolean,
  Metadata,
  Object(Schema),
  StringEnum(BTreeSet<String>),
  Array(Box<SchemaProperty>),
  AnyOf(Vec<SchemaProperty>),
}

// pub struct Resource {

// }

pub fn parse() -> Result<Spec, ()> {
  let raw = std::fs::read_to_string("openapi/spec3.json").unwrap();
  let raw_spec: serde_json::Value = serde_json::from_str(&raw).unwrap();

  let mut schemas = BTreeMap::new();

  for (key, raw_schema) in raw_spec["components"]["schemas"].as_object().unwrap() {

    let schema = parse_schema(raw_schema);

    schemas.insert(key.to_string(), schema);
  }

  let spec = Spec { schemas };

  Ok(spec)
}

fn parse_schema(raw_schema: &serde_json::Value) -> Schema {
  let raw_properties = match raw_schema["properties"].as_object() {
    Some(some) => some,
    None => panic!("no properties in schema {:?}", raw_schema),
  };

  let data = if let Some(raw_properties) = raw_schema.get("properties").as_object() {

    let required_fields: BTreeSet<String> =
      raw_schema["required"]
        .as_array()
        .map_or(BTreeSet::new(), |arr| {
          arr
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect()
        });
    let expandable_fields: BTreeSet<String> =
      raw_schema["x-expandableFields"]
        .as_array()
        .map_or(BTreeSet::new(), |arr| {
          arr
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect()
        });

    let mut properties = BTreeMap::new();

    for (name, ty) in raw_properties {
      let is_required = required_fields.contains(name);
      let is_expandable = expandable_fields.contains(name);
      properties.insert(
        name.to_string(),
        parse_schema_property(ty, is_required, is_expandable),
      );
    }
  } else if let Some(any_of) = raw_schema.get("anyOf").as_array() {
    
  }

  Schema {
    is_resource: raw_schema["x-resourceId"].is_string(),
    data,
  }
}

fn parse_schema_property(
  ty: &serde_json::Value,
  is_required: bool,
  is_expandable: bool,
) -> SchemaProperty {
  if let Some(path) = ty["$ref"].as_str() {
    return SchemaProperty::Reference(path.to_string());
  }

  let description = ty["description"].as_str().map(|s| s.to_string());
  let is_nullable = if let Some(is_nullable) = ty["nullable"].as_bool() {
    if is_nullable == is_required {
      panic!("nullabity mismatch");
    }
    is_nullable
  } else {
    !is_required
  };

  let value_type = if let Some(raw_type) = ty["type"].as_str() {
    match raw_type {
      "string" => {
        if let Some(enum_values) = ty["enum"].as_array() {
          SchemaPropertyValueType::StringEnum(
            enum_values
              .iter()
              .map(|v| v.as_str().unwrap().to_string())
              .collect(),
          )
        } else {
          SchemaPropertyValueType::String_
        }
      }
      "integer" => SchemaPropertyValueType::Integer,
      "number" => SchemaPropertyValueType::Number,
      "boolean" => SchemaPropertyValueType::Boolean,
      "object" => {
        if ty.get("additionalProperties").is_some() {
          SchemaPropertyValueType::Metadata
        } else {
          let schema = parse_schema(ty);
          SchemaPropertyValueType::Object(schema)
        }
      }
      "array" => {
        SchemaPropertyValueType::Array(Box::new(parse_schema_property(&ty["items"], false, false)))
      }
      _ => panic!("value type unknown {}", raw_type),
    }
  } else if let Some(any_of) = ty["anyOf"].as_array() {
    let mut variants = Vec::new();
    for ty in any_of {
      variants.push(parse_schema_property(ty, false, false));
    }
    SchemaPropertyValueType::AnyOf(variants)
  } else {
    panic!("unknown schema property type");
  };

  SchemaProperty::Value {
    value_type,
    description,
    is_nullable,
    is_expandable,
  }
}
