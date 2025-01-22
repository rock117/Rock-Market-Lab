use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
pub enum NumOrString {
    Int(usize),
    Double(f64),
    String(String),
}

impl NumOrString {
    pub fn into_string(self) -> Option<String> {
        match self {
            NumOrString::String(v) => Some(v),
            _ => None,
        }
    }
    pub fn into_double(self) -> Option<f64> {
        match self {
            NumOrString::Double(e) => Some(e),
            _ => None,
        }
    }
    pub fn into_int(self) -> Option<usize> {
        match self {
            NumOrString::Int(e) => Some(e),
            _ => None,
        }
    }
}

impl From<NumOrString> for String {
    fn from(value: NumOrString) -> Self {
        value.to_string()
    }
}