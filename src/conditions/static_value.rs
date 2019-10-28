use serde::{Deserialize, Serialize};

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
        Box::new(Self { value: conf.value })
    }
}

impl Condition for Static {
    fn check(&self, _: &Event) -> Result<bool, String> {
        return Ok(self.value);
    }
}

inventory::submit! {
    ConditionBuilder::new("static".to_owned(), | value | {
        match value.try_into() {
            Ok(c) => Ok(Static::new(&c)),
            Err(e) => Err(format!("{}", e)),
        }
    })
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
}