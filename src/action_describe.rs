use crate::action::AlpacaActionTrait;
use crate::action::AlpacaActions;
use serde_json::Value as JsonValue;

const DESCRIPTION: &str = r#"
The 'describe_action' action provides a detailed description of the specified action,
including its parameters and usage examples.

Here is an example of how to invoke it:
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
        let description = object["action_name"]
            .as_str()
            .map(|name| context.describe_action(name));

        if let Some(description) = description {
            let response = format!("## Success\n{}\n", &description);
            return response;
        }

        let error_text = format!(
            "## Error\n\n{}\n{}\n",
            "Request is missing the 'action_name' field.", DESCRIPTION
        );

        error_text
    }
}
