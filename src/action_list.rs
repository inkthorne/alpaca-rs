use crate::action::AlpacaActionTrait;
use crate::action::AlpacaActions;
use serde_json::Value as JsonValue;
use serde_json::json;

const DESCRIPTION: &str = r#"
The 'list_actions' action responds with a list of all of the available actions.

Here is an example of how to invoke it:
```json
{
    "action": "list_actions"
}
```
"#;

pub struct AlpacaActionList {}

impl AlpacaActionList {
    pub fn new() -> Self {
        Self {}
    }
}

impl AlpacaActionTrait for AlpacaActionList {
    fn name(&self) -> &str {
        "list_actions"
    }

    fn description(&self) -> &str {
        DESCRIPTION
    }

    fn invoke(&self, _object: &JsonValue, context: &AlpacaActions) -> String {
        let action_names = context.action_names();
        let object = json!({
            "available_actions": action_names,
        });

        let actions_block = AlpacaActions::blockify(&object);
        let response = format!(
            "## Success\n\nHere is the list of available actions:\n{}\n",
            &actions_block
        );
        response
    }
}
