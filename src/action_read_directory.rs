use crate::action::AlpacaActionTrait;
use crate::action::AlpacaActions;
use serde_json::Value as JsonValue;

const NAME: &str = "read_directory";
const DESCRIPTION: &str = r#"
The 'read_directory' action provides the names of the files and subdirectories
in the current working directory.

Here is an example of how to invoke it:
```json
{
    "action": "read_directory",
}
```
"#;

// ---

const EXAMPLE_USAGE: &str = r#"
Here is an example of how to invoke it:

```json
{
    "action": "read_directory",
}
```
"#;

// ---

fn format_response(status: &str, response: &str) -> String {
    format!("## {}\n\n{}\n", status, response)
}

// ===
// AlpacaActionReadDirectory
// ===

pub struct AlpacaActionReadDirectory {}

impl AlpacaActionReadDirectory {
    pub fn new() -> Self {
        Self {}
    }
}

impl AlpacaActionTrait for AlpacaActionReadDirectory {
    fn name(&self) -> &str {
        NAME
    }

    fn description(&self) -> &str {
        DESCRIPTION
    }

    fn invoke(&self, object: &JsonValue, _context: &AlpacaActions) -> String {
        // Return an error if the action contains too many arguments
        if let Some(object) = object.as_object() {
            if object.len() > 1 {
                // return format!("{}{}", ERR_TOO_MANY_ARGS, DESCRIPTION);
                let response =
                    format_response("Error", "The action was invoked with incorrect arguments.");
                return format!("{}{}", response, EXAMPLE_USAGE);
            }
        }

        // Read the current directory
        let current_dir = std::env::current_dir().unwrap_or_default();
        let mut files = Vec::new();
        let mut directories = Vec::new();

        // Read directory entries
        if let Ok(entries) = std::fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_type.is_file() {
                            files.push(file_name);
                        } else if file_type.is_dir() {
                            directories.push(file_name);
                        }
                    }
                }
            }
        }

        // Sort lists for consistent output
        files.sort();
        directories.sort();

        let ok = serde_json::json!({
            "files": files,
            "directories": directories,
        });

        let directory_block = AlpacaActions::blockify(&ok);
        let response = format!(
            "Here are the files and directories in the current directory:\n{}",
            &directory_block
        );

        format_response("Success", &response)
    }
}
