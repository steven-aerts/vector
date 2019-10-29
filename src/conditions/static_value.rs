use serde::{Deserialize, Serialize};

use crate::{conditions::Condition, topology::config::component::ComponentBuilder, Event};

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
        Box::new(Self { value: conf.value })
    }
}

impl Condition for Static {
    fn check(&self, _: &Event) -> Result<bool, String> {
        return Ok(self.value);
    }
}

inventory::submit! {
    ComponentBuilder::<Box::<dyn Condition>>::new(
        "static".to_owned(),
        | value | {
            match value.try_into() {
                Ok(c) => Ok(Static::new(&c)),
                Err(e) => Err(format!("{}", e)),
            }
        },
        | _s | Err("not implemented".to_owned()),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{topology::config::component::ComponentConfig, Event};

    #[test]
    fn parse_static_config() {
        let config_false: ComponentConfig<Box<dyn Condition>> = toml::from_str(
            r#"
      type = "static"
      value = false
      "#,
        )
        .unwrap();

        assert_eq!(
            Ok(false),
            config_false
                .condition
                .check(&Event::from("foo bar baz".to_owned()))
        );

        let config_true: ComponentConfig<Box<dyn Condition>> = toml::from_str(
            r#"
      type = "static"
      value = true
      "#,
        )
        .unwrap();

        assert_eq!(
            Ok(true),
            config_true
                .condition
                .check(&Event::from("foo bar baz".to_owned()))
        );
    }
}
