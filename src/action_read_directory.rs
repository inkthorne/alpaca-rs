use crate::action::AlpacaActionTrait;
use crate::action::AlpacaActions;
use serde_json::Value as JsonValue;

const NAME: &str = "read_directory";
const DESCRIPTION: &str = r#"
# `read_directory`

The 'read_directory' action provides the names of the files and subdirectories
in the current working directory. Here is an example of how to invoke it:

```json
{
    "action": "read_directory",
}
```
"#;

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

    fn invoke(&self, _object: &JsonValue, _context: &AlpacaActions) -> String {
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

        AlpacaActions::blockify(&ok)
    }
}
