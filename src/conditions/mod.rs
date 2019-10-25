use inventory;
use toml::Value;
use serde::{Deserialize,Serialize};

use crate::Event;

pub mod static_value;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ConditionConfig {
    #[serde(rename = "type")]
    pub typestr: String,
    #[serde(flatten)]
    pub nested: Value,
}

pub trait Condition {
    fn check(&self, e: &Event) -> Result<bool, &'static str>;
}

pub struct ConditionImpl {
    pub name: String,
    pub constructor: fn(Value) -> Box<dyn Condition>,
}

impl ConditionImpl {
    pub fn new(name: String, ctor: fn(Value) -> Box<dyn Condition>) -> ConditionImpl {
        ConditionImpl{
            name: name,
            constructor: ctor,
        }
    }
}

inventory::collect!(ConditionImpl);