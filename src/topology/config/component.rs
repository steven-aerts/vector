use serde::de::{Error, MapAccess, Visitor};
use serde::{Serialize};
use toml::Value;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ConfigSwapOut {
    #[serde(rename = "type")]
    pub typestr: String,
    #[serde(flatten)]
    pub nested: Value,
}

impl ConfigSwapOut {
    pub fn new() -> ConfigSwapOut {
        ConfigSwapOut {
            typestr: "".to_owned(),
            nested: Value::Table(BTreeMap::new()),
        }
    }
}

impl<'de> Visitor<'de> for ConfigSwapOut {
    type Value = ConfigSwapOut;

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
            Ok(ConfigSwapOut {
                typestr: typestr,
                nested: Value::Table(nested),
            })
        }
    }
}
