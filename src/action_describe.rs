use crate::action::AlpacaActionTrait;
use crate::action::AlpacaActions;
use serde_json::Value as JsonValue;
use serde_json::json;

const DESCRIPTION: &str = r#"
# `describe_action`

The 'describe_action' action provides a detailed description of the specified action,
including its parameters and usage examples. Here is an example of
how to invoke it:

```json
{
    "action": "describe_action",
    "action_name": "list_actions"
}
```
"#;

pub struct AlpacaActionDescribe {}

impl AlpacaActionDescribe {
    pub fn new() -> Self {
        Self {}
    }
}

impl AlpacaActionTrait for AlpacaActionDescribe {
    fn name(&self) -> &str {
        "describe_action"
    }

    fn description(&self) -> &str {
        DESCRIPTION
    }

    fn invoke(&self, object: &JsonValue, context: &AlpacaActions) -> String {
        let response = object["action_name"]
            .as_str()
            .map(|name| context.describe_action(name));

        if response.is_some() {
            return response.unwrap();
        }

        let error_text = format!(
            "{}\n{}\n",
            "Request is missing the 'action_name' field.", DESCRIPTION
        );

        let error = json!({
            "error": error_text
        });

        AlpacaActions::blockify(&error)
    }
}
