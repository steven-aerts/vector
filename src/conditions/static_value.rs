use serde::{Deserialize, Serialize};
use toml::Value;

use crate::{
    conditions::{Condition, ConditionBuilder},
    Event,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StaticConfig {
    pub value: bool,
}

pub struct Static {
    value: bool,
}

impl Static {
    pub fn new(conf: &StaticConfig) -> Box<dyn Condition> {
        Box::new(Static { value: conf.value })
    }

    pub fn new_from_value(value: Value) -> Result<Box<dyn Condition>, String> {
        match value.try_into() {
            Ok(c) => Ok(Static::new(&c)),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

impl Condition for Static {
    fn check(&self, _: &Event) -> Result<bool, String> {
        return Ok(self.value);
    }
}

inventory::submit! {
    ConditionBuilder::new("static".to_owned(), Static::new_from_value)
}
