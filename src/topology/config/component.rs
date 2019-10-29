use inventory;
use serde::de::{Deserializer, Error, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use toml::Value;

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

pub struct ComponentConfig<T: 'static>
where
    T: Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    pub condition: T,
}

impl<'de, T: 'static> Deserialize<'de> for ComponentConfig<T>
where
    T: Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let swap_out = deserializer.deserialize_map(ConfigSwapOut::new())?;
        match inventory::iter::<ComponentBuilder<T>>
            .into_iter()
            .find(|t| t.name == swap_out.typestr)
        {
            Some(b) => match (b.from_value)(swap_out.nested) {
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

type ComponentFromValue<T> = fn(Value) -> Result<T, String>;
type ComponentToValue = fn() -> Result<Value, String>;

pub struct ComponentBuilder<T: Sized> {
    pub name: String,
    pub from_value: ComponentFromValue<T>,
    pub default_value: ComponentToValue,
}

impl<T: Sized> ComponentBuilder<T> {
    pub fn new(
        name: String,
        from_value: ComponentFromValue<T>,
        default_value: ComponentToValue,
    ) -> Self {
        ComponentBuilder {
            name: name,
            from_value: from_value,
            default_value: default_value,
        }
    }
}
