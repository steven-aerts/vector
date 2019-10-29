use crate::topology::config::component::{ComponentConfig, ComponentBuilder};
use crate::Event;
use inventory;

pub mod static_value;

type ConditionDefinition = ComponentBuilder<Box::<dyn Condition>>;
type ConditionConfig = ComponentConfig<Box::<dyn Condition>>;

pub trait Condition {
    fn check(&self, e: &Event) -> Result<bool, String>;
}

inventory::collect!(ConditionDefinition);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_bad_config_type() {
        assert_eq!(
            toml::from_str::<ConditionConfig>(
                r#"
      type = "not a real type"
      value = false
      "#
            )
            .err()
            .map(|e| format!("{}", e))
            .unwrap_or("".to_owned()),
            "unrecognized type 'not a real type'".to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_missing_type() {
        assert_eq!(
            toml::from_str::<ConditionConfig>(
                r#"
      nottype = "missing a type here"
      value = false
      "#
            )
            .err()
            .map(|e| format!("{}", e))
            .unwrap_or("".to_owned()),
            "missing type field".to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_extra_field() {
        assert_eq!(
            toml::from_str::<ConditionConfig>(
                r#"
      type = "static"
      value = false
      extra_field = "is unexpected"
      "#
            )
            .err()
            .map(|e| format!("{}", e))
            .unwrap_or("".to_owned()),
            "failed to parse type `static`: unknown field `extra_field`, expected `value`"
                .to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_missing_field() {
        assert_eq!(
            toml::from_str::<ConditionConfig>(
                r#"
      type = "static"
      "#
            )
            .err()
            .map(|e| format!("{}", e))
            .unwrap_or("".to_owned()),
            "failed to parse type `static`: missing field `value`".to_owned(),
        );
    }
}
