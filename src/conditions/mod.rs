use inventory;
use serde::de::{Deserializer, Error, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use toml::Value;

use crate::Event;

pub mod static_value;

pub struct ConditionConfig {
    pub cond: Box<dyn Condition>,
}

impl<'de> Deserialize<'de> for ConditionConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let swap_out = deserializer.deserialize_map(ConditionSwapOut::new())?;
        match inventory::iter::<ConditionBuilder>
            .into_iter()
            .find(|t| t.name == swap_out.typestr)
        {
            Some(b) => match (b.constructor)(swap_out.nested) {
                Ok(c) => Ok(ConditionConfig { cond: c }),
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

#[derive(Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ConditionSwapOut {
    #[serde(rename = "type")]
    pub typestr: String,
    #[serde(flatten)]
    pub nested: Value,
}

impl<'de> Visitor<'de> for ConditionSwapOut {
    type Value = ConditionSwapOut;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut typestr = "".to_owned();
        let mut nested: BTreeMap<String, Value> = BTreeMap::new();

        while let Some((key, value)) = access.next_entry::<String, toml::Value>()? {
            if key == "type" {
                typestr = value.as_str().unwrap_or("").to_owned();
            } else {
                nested.insert(key, value);
            }
        }

        if typestr.len() == 0 {
            Err(Error::custom("missing type field"))
        } else {
            Ok(ConditionSwapOut {
                typestr: typestr,
                nested: Value::Table(nested),
            })
        }
    }
}

impl ConditionSwapOut {
    pub fn new() -> ConditionSwapOut {
        ConditionSwapOut {
            typestr: "".to_owned(),
            nested: Value::Table(BTreeMap::new()),
        }
    }
}

pub trait Condition {
    fn check(&self, e: &Event) -> Result<bool, String>;
}

type ConditionCtor = fn(Value) -> Result<Box<dyn Condition>, String>;

pub struct ConditionBuilder {
    pub name: String,
    pub constructor: ConditionCtor,
}

impl ConditionBuilder {
    pub fn new(name: String, ctor: ConditionCtor) -> ConditionBuilder {
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
    use crate::Event;

    #[test]
    fn parse_static_config() {
        let config_false: ConditionConfig = toml::from_str(
            r#"
      type = "static"
      value = false
      "#,
        )
        .unwrap();

        assert_eq!(
            Ok(false),
            config_false
                .cond
                .check(&Event::from("foo bar baz".to_owned()))
        );

        let config_true: ConditionConfig = toml::from_str(
            r#"
      type = "static"
      value = true
      "#,
        )
        .unwrap();

        assert_eq!(
            Ok(true),
            config_true
                .cond
                .check(&Event::from("foo bar baz".to_owned()))
        );
    }

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
