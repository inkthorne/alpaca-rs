use crate::action::AlpacaActionTrait;
use crate::action::AlpacaActions;
use serde_json::Value as JsonValue;
use serde_json::json;

const NAME: &str = "read_file";
const DESCRIPTION: &str = r#"
# `read_file`

The 'read_file' action outputs the contents of the specified file. Only files
in the current directory can be read.  Here is an example of how to invoke it:

```json
{
    "action": "read_file",
    "file_name": "example.txt"
}
```
"#;

pub struct AlpacaActionReadFile {}

impl AlpacaActionReadFile {
    pub fn new() -> Self {
        Self {}
    }

    fn read_file(&self, file_name: &str) -> Result<String, String> {
        // Attempt to read file contents
        match std::fs::read_to_string(file_name) {
            Ok(content) => Ok(content),
            Err(e) => {
                // Return the error message if file reading fails
                let error = format!(
                    "Failed to read file '{}': {}.\nPlease ensure the file name is correct and try again.",
                    file_name,
                    e.to_string()
                );
                Err(error)
            }
        }
    }
}

impl AlpacaActionTrait for AlpacaActionReadFile {
    fn name(&self) -> &str {
        NAME
    }

    fn description(&self) -> &str {
        DESCRIPTION
    }

    fn invoke(&self, object: &JsonValue, _context: &AlpacaActions) -> String {
        // If we don't have a 'file_name' field, return an error
        let filename = object["file_name"].as_str();
        if filename.is_none() {
            // If 'file_name' is not provided, return an error
            let error = json!({
                "error": format!(
                    "The 'file_name' field is missing from the request. Please review the following description and try again.\n\n{}\n",
                    DESCRIPTION),
            });

            return AlpacaActions::blockify(&error);
        }

        // If 'file_name' is provided, read the file
        let filename = filename.unwrap();
        match self.read_file(filename) {
            Ok(content) => {
                // Create a JSON object with the file content
                let response = json!({
                    "content": content,
                });

                return AlpacaActions::blockify(&response);
            }
            Err(error) => {
                // Return the error message if file reading fails
                let output = json!({
                    "error": error,
                });

                return AlpacaActions::blockify(&output);
            }
        }
    }
}
