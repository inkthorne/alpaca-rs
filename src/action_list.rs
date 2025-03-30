use crate::action::AlpacaActionTrait;
use crate::action::AlpacaActions;
use serde_json::Value as JsonValue;

const DESCRIPTION: &str = r#"
# `list_actions`

The 'list_actions' action reponds with a list of all of the available actions.
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
        context.list_actions()
    }
}
