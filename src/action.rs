use crate::action_describe::AlpacaActionDescribe;
use crate::action_list::AlpacaActionList;
use crate::action_read_directory::AlpacaActionReadDirectory;
use crate::action_read_file::AlpacaActionReadFile;
use regex::Regex;
use serde_json::Value as JsonValue;
use serde_json::json;
use std::collections::HashMap;

// ===
// AlpacaAction
// ===

pub struct AlpacaAction {
    pub name: String,
    pub description: String,
}

impl AlpacaAction {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        let string = json!({
            "name": self.name,
            "description": self.description,
        });

        serde_json::to_string_pretty(&string).unwrap()
    }

    pub fn invoke(&self, _json_block: &JsonValue, actions: &AlpacaActions) -> String {
        // Check if the action is "list_actions"
        if self.name == "list_actions" {
            return actions.list_actions();
        }

        let string = json!({
            "action": self.name,
            "ok": true,
        });

        serde_json::to_string_pretty(&string).unwrap()
    }
}

// ===
// AlpacaActionTrait
// ===

pub trait AlpacaActionTrait {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn invoke(&self, object: &JsonValue, context: &AlpacaActions) -> String;
}

// ===
// AlpacaActions
// ===

pub struct AlpacaActions {
    actions: HashMap<String, Box<dyn AlpacaActionTrait>>,
}

// ===
// AlpacaAction: Public Methods
// ===

impl AlpacaActions {
    pub fn new() -> Self {
        let mut actions = Self {
            actions: HashMap::new(),
        };

        actions.add_action(Box::new(AlpacaActionList::new()));
        actions.add_action(Box::new(AlpacaActionDescribe::new()));
        actions.add_action(Box::new(AlpacaActionReadDirectory::new()));
        actions.add_action(Box::new(AlpacaActionReadFile::new()));

        actions
    }

    pub fn blockify(object: &JsonValue) -> String {
        let string = serde_json::to_string_pretty(object).unwrap();
        format!("```json\n{}\n```", string)
    }

    pub fn add_action(&mut self, action: Box<dyn AlpacaActionTrait>) {
        self.actions.insert(action.name().to_string(), action);
    }

    pub fn invoke(&self, message: &str) -> Option<String> {
        // Check each JSON block for an action
        let json_blocks = self.parse(message);
        let responses: Vec<String> = json_blocks
            .iter()
            .filter_map(|block| {
                block["action"].as_str().map(|name| {
                    // Check if the action exists in the actions map
                    if let Some(action) = self.actions.get(name) {
                        // If the action exists, execute it and get the response
                        format!(
                            "Here is the response from action '{}':\n\n{}",
                            name,
                            action.invoke(block, self)
                        )
                    } else {
                        // If the action does not exist, return an error message
                        format!(
                            "There was a problem attempting to perform the action '{}':\n\n{}",
                            name,
                            Self::response_action_not_found(name, name)
                        )
                    }
                })
            })
            .collect();

        if responses.is_empty() {
            // If no action was found, return None
            return None;
        }

        let response = responses.join("\n");
        Some(response)
    }

    pub fn describe_action(&self, action_name: &str) -> String {
        if let Some(action) = self.actions.get(action_name) {
            let object = json!({
                "description": action.description(),
            });

            return Self::blockify(&object);
        }

        Self::response_action_not_found("describe_action", action_name)
    }

    pub fn list_actions(&self) -> String {
        let mut action_names: Vec<String> = self.actions.keys().cloned().collect();
        action_names.sort();

        let object = json!({
            "available_actions": action_names,
        });

        Self::blockify(&object)
    }
}

// ===
// AlpacaAction: Private Methods
// ===

impl AlpacaActions {
    fn parse(&self, message: &str) -> Vec<JsonValue> {
        let mut results = Vec::new();

        // Regular expression to find ```json blocks
        // This looks for the starting ```json and ending ``` with any content in between
        let re = Regex::new(r"```json\s*([\s\S]*?)\s*```").unwrap();

        // Find all matches in the message
        for cap in re.captures_iter(message) {
            if let Some(json_str) = cap.get(1) {
                // Try to parse the captured content as JSON
                if let Ok(json_value) = serde_json::from_str(json_str.as_str()) {
                    results.push(json_value);
                }
            }
        }

        // If no JSON blocks were found, try to parse the entire message as JSON
        if results.is_empty() {
            if let Ok(json_value) = serde_json::from_str(message) {
                results.push(json_value);
            }
        }

        results
    }

    fn response_action_not_found(_from: &str, action: &str) -> String {
        let object = json!({
            "error": format!("Action '{}' not found. Use the action 'list_actions' to list all available actions.", action),
        });

        Self::blockify(&object)
    }
}
