use crate::action_describe::AlpacaActionDescribe;
use crate::action_list::AlpacaActionList;
use crate::action_read_directory::AlpacaActionReadDirectory;
use crate::action_read_file::AlpacaActionReadFile;
use crate::action_regex::AlpacaActionRegex;
use regex::Regex;
use serde_json::Value as JsonValue;
use serde_json::json;
use std::collections::{HashMap, HashSet};

// ---

pub const ACTION_NOT_FOUND: &str = r#"
Use the `list_actions` action to get a list of all of the available actions.
Here is an example of how to invoke it:
```json
{
    "action": "list_actions"
}
```
"#;

// ---

fn string_action_response(action: &str, response: &str) -> String {
    format!(
        r#"
# `{}` Action Response

{}
"#,
        action, response
    )
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
// AlpacaActions: Public Methods
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
        actions.add_action(Box::new(AlpacaActionRegex::new()));

        actions
    }

    pub fn blockify(object: &JsonValue) -> String {
        let string = serde_json::to_string_pretty(object).unwrap();
        format!("```json\n{}\n```\n", string)
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
                        string_action_response(name, &action.invoke(block, self))
                    } else {
                        // If the action does not exist, return an error message
                        let response = format!(
                            "## Error\n\nAction '{}' not found.\n\n{}",
                            name,
                            self.action_list()
                        );
                        string_action_response(name, &response)
                        /*
                         format!(
                             // "There was a problem attempting to perform the action '{}':\n\n{}",
                             "Here is the response from trying to perform action '{}':\n\n{}",
                             name,
                             Self::response_action_not_found(name, name)
                         )
                        */
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
            return action.description().to_string();
            /*
            let object = json!({
                "description": action.description(),
            });

            return Self::blockify(&object);
            */
        }

        Self::response_action_not_found("describe_action", action_name)
    }

    pub fn action_list(&self) -> String {
        let action_names = self.action_names();
        let json_value = json!({
            "actions": action_names
        });

        format!(
            "## Available Actions\n\n{}\n{}",
            "Here is the list of available actions:",
            Self::blockify(&json_value)
        )
    }

    pub fn action_names(&self) -> Vec<String> {
        let mut action_names: Vec<String> = self.actions.keys().cloned().collect();
        action_names.sort();
        action_names
    }
}

// ===
// AlpacaActions: Private Methods
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
                // Replace non-breaking spaces with regular spaces.
                let json_str = json_str.as_str().replace("<0xC2><0xA0>", " ");

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

        // Remove any duplicates from the results
        self.remove_duplicates(results)
    }

    /// Removes duplicate JsonValue objects from a vector.
    /// Two JsonValue objects are considered duplicates if they have the same key-value pairs.
    ///
    /// # Arguments
    ///
    /// * `values` - A vector of JsonValue objects that may contain duplicates
    ///
    /// # Returns
    ///
    /// A new vector of JsonValue objects with duplicates removed
    fn remove_duplicates(&self, values: Vec<JsonValue>) -> Vec<JsonValue> {
        let mut seen = HashSet::new();
        let mut result = Vec::new();

        for value in values {
            let value_str = serde_json::to_string(&value).unwrap_or_default();
            if seen.insert(value_str) {
                result.push(value);
            }
        }

        result
    }

    fn response_action_not_found(_from: &str, action: &str) -> String {
        let object = json!({
            "error": format!("Action '{}' not found. Use the action 'list_actions' to list all available actions.", action),
        });

        Self::blockify(&object)
    }
}
