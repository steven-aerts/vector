use serde::{Deserialize, Serialize};
use toml::Value;

use crate::{conditions::{Condition, ConditionDefinition}, Event};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StaticConfig {
    pub value: bool,
}

pub struct Static {
    conf: StaticConfig,
}

impl Static {
    pub fn new(conf: StaticConfig) -> Box<dyn Condition> {
        Box::new(Self { conf: conf })
    }
}

impl Condition for Static {
    fn check(&self, _: &Event) -> Result<bool, String> {
        return Ok(self.conf.value);
    }
}

inventory::submit! {
    ConditionDefinition::new(
        "static".to_owned(),
        | value | value.try_into().map(|c| Static::new(c)).map_err(|e| format!("{}", e)),
        || Value::try_from(StaticConfig{
            value: false,
        }).map_err(|e| format!("{}", e))
    )
}

#[cfg(test)]
mod test {
    use crate::{Event, conditions::ConditionConfig};

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
                .condition
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
                .condition
                .check(&Event::from("foo bar baz".to_owned()))
        );
    }

    #[test]
    fn print_static_config() {
        let config_false: ConditionConfig = toml::from_str(
            r#"
      type = "static"
      value = false
      "#,
        )
        .unwrap();

        assert_eq!(
            r#"type = "static"
value = false
"#,
            toml::to_string(&config_false).unwrap()
        );

        let config_true: ConditionConfig = toml::from_str(
            r#"
      type = "static"
      value = true
      "#,
        )
        .unwrap();

        assert_eq!(
            r#"type = "static"
value = true
"#,
            toml::to_string(&config_true).unwrap()
        );
    }
}
