use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}

lazy_static! {
    pub static ref HASHMAP: Mutex<HashMap<String, Value>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}