use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Transform {
    SelectField(String),
}

impl Transform {
    pub fn apply(&self, value: &Value) -> Result<Value, String> {
        match self {
            Transform::SelectField(field) => {
                if let Value::Object(obj) = value {
                    obj.get(field)
                        .cloned()
                        .ok_or_else(|| format!("Field '{}' not found", field))
                } else {
                    Err("Cannot select field from non-object {value}".to_string())
                }
            }
        }
    }

    pub fn to_jq(&self) -> String {
        match self {
            Transform::SelectField(field) => format!(".{}", field),
        }
    }

    // TODO: Write a real parser and decide on concrete syntax
    pub fn from_jq(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();
        if let Some(field) = trimmed.strip_prefix('.') {
            if field.is_empty() {
                Err("Field name cannot be empty".to_string())
            } else {
                Ok(Transform::SelectField(field.to_string()))
            }
        } else {
            Err("Transform must start with '.'".to_string())
        }
    }
}

#[derive(Clone)]
pub struct TransformPipeline {
    pub transforms: Vec<Transform>,
}

impl TransformPipeline {
    pub fn new() -> Self {
        Self { transforms: vec![] }
    }

    pub fn add(&mut self, transform: Transform) {
        self.transforms.push(transform);
    }

    pub fn apply(&self, mut value: Value) -> Result<Value, String> {
        for transform in &self.transforms {
            value = transform.apply(&value)?;
        }
        Ok(value)
    }

    pub fn to_jq(&self) -> String {
        if self.transforms.is_empty() {
            ".".to_string()
        } else {
            self.transforms
                .iter()
                .map(|t| t.to_jq())
                .collect::<Vec<_>>()
                .join(" | ")
        }
    }
}
