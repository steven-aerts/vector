use serde::{Deserialize,Serialize};
use toml::Value;

use crate::{Event, conditions::{ConditionImpl,Condition}};

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
        Box::new(Static {
            value: conf.value,
        })
    }

    pub fn new_from_value(value: Value) -> Box<dyn Condition> {
        let conf: StaticConfig = value.try_into().unwrap();
        Static::new(&conf)
    }
}

impl Condition for Static {
    fn check(&self, _: &Event) -> Result<bool, &'static str> {
        return Ok(self.value);
    }
}

inventory::submit! {
    ConditionImpl::new("static".to_owned(), Static::new_from_value)
}