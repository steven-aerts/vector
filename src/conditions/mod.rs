use inventory;
use serde::de::{Deserializer, Error};
use serde::Deserialize;
use crate::topology::config::component::ConfigSwapOut;
use toml::Value;

use crate::Event;

pub mod static_value;

pub struct ConditionConfig {
    pub condition: Box<dyn Condition>,
}

impl<'de> Deserialize<'de> for ConditionConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let swap_out = deserializer.deserialize_map(ConfigSwapOut::new())?;
        match inventory::iter::<ConditionBuilder>
            .into_iter()
            .find(|t| t.name == swap_out.typestr)
        {
            Some(b) => match (b.constructor)(swap_out.nested) {
                Ok(c) => Ok(Self { condition: c }),
                Err(e) => Err(Error::custom(format!(
                    "failed to parse type `{}`: {}",
                    swap_out.typestr, e
                ))),
            },
            None => Err(Error::custom(format!(
                "unrecognized type '{}'",
                swap_out.typestr
            ))),
        }
    }
}

pub trait Condition {
    fn check(&self, e: &Event) -> Result<bool, String>;
}

type ComponentCtor<T> = fn(Value) -> Result<T, String>;

pub struct ConditionBuilder {
    pub name: String,
    pub constructor: ComponentCtor<Box<dyn Condition>>,
}

impl ConditionBuilder {
    pub fn new(name: String, ctor: ComponentCtor<Box<dyn Condition>>) -> ConditionBuilder {
        ConditionBuilder {
            name: name,
            constructor: ctor,
        }
    }
}

inventory::collect!(ConditionBuilder);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_bad_config_type() {
        assert_eq!(toml::from_str::<ConditionConfig>(
            r#"
      type = "not a real type"
      value = false
      "#).err().map(|e| format!("{}", e)).unwrap_or("".to_owned()),
            "unrecognized type 'not a real type'".to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_missing_type() {
        assert_eq!(toml::from_str::<ConditionConfig>(
            r#"
      nottype = "missing a type here"
      value = false
      "#).err().map(|e| format!("{}", e)).unwrap_or("".to_owned()),
            "missing type field".to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_extra_field() {
        assert_eq!(toml::from_str::<ConditionConfig>(
            r#"
      type = "static"
      value = false
      extra_field = "is unexpected"
      "#).err().map(|e| format!("{}", e)).unwrap_or("".to_owned()),
            "failed to parse type `static`: unknown field `extra_field`, expected `value`".to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_missing_field() {
        assert_eq!(toml::from_str::<ConditionConfig>(
            r#"
      type = "static"
      "#).err().map(|e| format!("{}", e)).unwrap_or("".to_owned()),
            "failed to parse type `static`: missing field `value`".to_owned(),
        );
    }
}
