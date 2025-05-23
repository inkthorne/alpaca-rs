use crate::function::{AlpacaFunction, AlpacaFunctions};

const FUNCTION_DIR_INFO: &str = r#"
# `dir`

This function lists the files & directories in the current directory.

example call:
```json
{
    "action": "invoke_function",
    "function": "dir",
    "arguments": {}
}
```

example response:
```json
{
    "function": "dir",
    "output": {
        "files": [
            "file1.txt",
            "file2.txt"
        ],
        "directories": [
            "dir1",
            "dir2"
        ]
    }
}
"#;

// ===
// AlpacaFunctionDir
// ===
pub struct AlpacaFunctionDir;

impl AlpacaFunctionDir {
    pub fn new() -> Self {
        AlpacaFunctionDir
    }
}

// Implement the AlpacaFunction trait for AlpacaFunctionDir
impl AlpacaFunction for AlpacaFunctionDir {
    fn execute(&self, _arguments: Option<&serde_json::Value>) -> Option<String> {
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

        Some(AlpacaFunctions::ok(self.name(), &ok))
    }

    fn info(&self) -> &'static str {
        FUNCTION_DIR_INFO
    }

    fn name(&self) -> &'static str {
        "dir"
    }

    fn description(&self) -> &'static str {
        "Lists the files & directories in the current directory."
    }
}
