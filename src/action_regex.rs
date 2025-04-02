use crate::action::AlpacaActionTrait;
use crate::action::AlpacaActions;
use regex::Regex;
use serde_json::Value as JsonValue;
use serde_json::json;

const NAME: &str = "regex";
const DESCRIPTION: &str = r#"
# `regex`

The 'regex' action allows you to perform regular expression operations on text.
You can search for patterns in text and extract matches.

Here is an example of how to invoke it with a single string:
```json
{
    "action": "regex",
    "pattern": "\\d+",
    "input": "The year is 2025 and the month is 4."
}
```

You can also provide an array of strings as the 'input' parameter:
```json
{
    "action": "regex",
    "pattern": "\\d+",
    "input": [
        "The year is 2025.",
        "The month is 4."
    ]
}
```

This will return all matches of the pattern in the provided text(s).
"#;

fn format_response(status: &str, response: &str) -> String {
    format!("## {}\n\n{}\n", status, response)
}

pub struct AlpacaActionRegex {}

impl AlpacaActionRegex {
    pub fn new() -> Self {
        Self {}
    }
}

impl AlpacaActionTrait for AlpacaActionRegex {
    fn name(&self) -> &str {
        NAME
    }

    fn description(&self) -> &str {
        DESCRIPTION
    }

    fn invoke(&self, object: &JsonValue, _context: &AlpacaActions) -> String {
        // Check if we have the required fields
        let pattern = object["pattern"].as_str();

        // Check if the 'pattern' argument is provided.
        if pattern.is_none() {
            return format_response(
                "Error",
                "Missing 'pattern' parameter; please use `describe_action` to see the correct parameters.",
            );
        }

        // Check if the 'input' argument is provided.
        let input = object.get("input");
        if input.is_none() {
            return format_response(
                "Error",
                "Missing 'input' parameter; please use `describe_action` to see the correct parameters.",
            );
        }

        let pattern = pattern.unwrap();

        // Compile the regex pattern
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(e) => {
                // Return error if regex pattern compilation fails
                let error = json!({
                    "error": format!("Invalid regex pattern: {}", e),
                });
                return format_response("Error", &AlpacaActions::blockify(&error));
            }
        };

        // Handle both cases: input as string or as array of strings
        if let Some(text) = object["input"].as_str() {
            // Case 1: 'input' is a single string
            let matches: Vec<String> = regex
                .find_iter(text)
                .map(|m| m.as_str().to_string())
                .collect();

            let response = json!({
                "matches": matches,
                // "count": matches.len(),
            });

            let regex_block = AlpacaActions::blockify(&response);
            let response_text = format!(
                "Regular expression results for pattern '{}' in the provided input:\n{}",
                pattern, &regex_block
            );

            format_response("Success", &response_text)
        } else if let Some(texts) = object["input"].as_array() {
            // Case 2: 'input' is an array of strings
            let mut all_results = Vec::new();
            let mut total_matches = 0;

            for (index, text_value) in texts.iter().enumerate() {
                if let Some(text) = text_value.as_str() {
                    let matches: Vec<String> = regex
                        .find_iter(text)
                        .map(|m| m.as_str().to_string())
                        .collect();

                    total_matches += matches.len();

                    all_results.push(json!({
                        // "index": index,
                        "input": text,
                        "matches": matches,
                        // "count": matches.len(),
                    }));
                } else {
                    // If an element in the array is not a string, include it as an error
                    all_results.push(json!({
                        "index": index,
                        "error": "Not a string value"
                    }));
                }
            }

            let response = json!({
                "results": all_results,
                "total_count": total_matches,
            });

            let regex_block = AlpacaActions::blockify(&response);
            let response_text = format!(
                "Regular expression results for pattern '{}' across {} text items:\n{}",
                pattern,
                texts.len(),
                &regex_block
            );

            format_response("Success", &response_text)
        } else {
            // Neither a string nor an array was provided
            format_response(
                "Error",
                "The 'text' parameter must be a string or an array of strings; please use `describe_action` to see the correct parameters.",
            )
        }
    }
}
