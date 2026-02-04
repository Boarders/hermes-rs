use serde_json::{json, Value, Map};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::utils::semilattice::{JoinSemilattice};

// TODO: use a set for Union
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum JsonType {
    Never,
    Null,
    Bool,
    Number,
    String,
    Array(Box<JsonType>),
    Object(BTreeMap<String, JsonType>),
    Union(Vec<JsonType>),
}

impl JoinSemilattice for JsonType {
    fn bot() -> JsonType {
       JsonType::Never
    }
    fn join(lhs: &Self, rhs: &Self) -> Self {
	use JsonType::*;
	match (lhs, rhs) {
	    (Bool, Bool) => Bool.clone(),
	    (Number, Number) => Number.clone(),
	    (String, String) => String.clone(),
	    (Never, x) => x.clone(),
	    (x, Never) => x.clone(),
	    (Array(lhs), Array(rhs)) => Array(Box::new(Self::join(lhs, rhs))),
	    (Object(lhs), Object(rhs)) => {
		let mut union = BTreeMap::new();
		for (k, v) in lhs {
		    if let Some(val) = rhs.get(k) {
			let combine = JsonType::join(v, val);
			union.insert(k.to_string(), combine);
		    }
		    union.insert(k.to_string(), v.clone());
		}
		for (k, v) in rhs {
		    if !lhs.contains_key(k) {
			union.insert(k.clone(), v.clone());
		    }
		}
		Object(union)
	    }
	    (Union(lhs), Union(rhs)) => {
		let mut unions = lhs.clone();
		// Append and only keep uniques
		unions.append(&mut rhs.clone());
		unions.sort_unstable();
		unions.dedup();
		Union(unions)
	    }
	    (Union(lhs), rhs) => {
		let mut lhs = lhs.clone();
		lhs.push(rhs.clone());
		Union(lhs)
	    }
	    (lhs, Union(rhs)) => {
		let mut rhs = rhs.clone();
		rhs.push(lhs.clone());
		Union(rhs)
	    }
	    (lhs, Union(rhs)) => {
		let mut rhs = rhs.clone();
		rhs.push(lhs.clone());
		Union(rhs)
	    }
	    (lhs, rhs) => Union(vec![lhs.clone(), rhs.clone()])
	}
    }
}

impl JsonType {
    /// Infer schema from a serde_json::Value
    pub fn infer(value: &serde_json::Value) -> Self {
	use JsonType::*;
        match value {
            serde_json::Value::Null => JsonType::Null,
            serde_json::Value::Bool(_) => JsonType::Bool,
            serde_json::Value::Number(_) => JsonType::Number,
            serde_json::Value::String(_) => JsonType::String,
            serde_json::Value::Array(arr) => {
		let inferred_types: Vec<JsonType> = arr
		    .iter()
		    .map(|obj| Self::infer(obj))
		    .collect();
		let vals: JsonType = Self::unify(&inferred_types);
		JsonType::Array(Box::new(vals))
	    }
            serde_json::Value::Object(obj) => {
                let mut map = BTreeMap::new();
                for (key, val) in obj {
                    map.insert(key.clone(), JsonType::infer(val));
                }
                JsonType::Object(map)
            }
        }
    }

    pub fn unify(json_types: &Vec<JsonType>) -> Self {
	json_types
	    .into_iter()
	    .fold(JsonType::bot(), |acc, ty| JsonType::join(&acc, &ty))
    }


    /// convert to json for printing
    pub fn to_pretty_json(&self) -> serde_json::Value {
        match self {
	    JsonType::Null => { json!("null") },
	    JsonType::Never => { json!("never") },
	    JsonType::Bool => { json!("bool") },
	    JsonType::Number => { json!("number") },
	    JsonType::String => { json!("string") },
	    JsonType::Array(inner) => {
		json!([inner.to_pretty_json()])
	    },
	    JsonType::Object(objects) => {
		let mut object = Map::new();
		for (k,v) in objects {
		    object.insert(k.to_string(),v.to_pretty_json());
		}
		return Value::Object(object)
	    },
	    JsonType::Union(unions) => {
		let unions : Vec<Value> = unions.into_iter().map(|obj| obj.to_pretty_json()).collect::<Vec<_>>();
		json!({
		"union": unions
	    })},
        }

    }
    pub fn to_pretty(&self) -> String {
	let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
	let mut buf = Vec::new();
	let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
	self.serialize(&mut ser).unwrap();
	String::from_utf8(buf).unwrap()
    }
}
