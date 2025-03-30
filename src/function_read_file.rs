use crate::function::{AlpacaFunction, AlpacaFunctions};

const READ_FILE_INFO: &str = r#"
# `read_file` usage

This function outputs the contents of the specified text file.

example call:
```json
{
    "action": "invoke_function",
    "function": "read_file",
    "arguments": {
        "file_name": "example.txt"
    }
}
```
"#;

// ===
// AlpacaFunctionReadFile
// ===

pub struct AlpacaFunctionReadFile;

impl AlpacaFunctionReadFile {
    pub fn new() -> Self {
        AlpacaFunctionReadFile
    }
}

// Implement the AlpacaFunction trait for AlpacaFunctionReadFile
impl AlpacaFunction for AlpacaFunctionReadFile {
    fn execute(&self, arguments: Option<&serde_json::Value>) -> Option<String> {
        // Extract file_name from arguments
        if let Some(args) = arguments {
            if let Some(file_name) = args.get("file_name").and_then(|v| v.as_str()) {
                // Attempt to read file contents
                match std::fs::read_to_string(file_name) {
                    Ok(content) => {
                        // Update the response with the file content
                        // response["output"] = serde_json::Value::String(content);
                        let output = serde_json::Value::String(content);
                        return Some(AlpacaFunctions::ok(self.name(), &output));
                    }
                    Err(e) => {
                        // Return the eror message if file reading fails
                        let error = format!(
                            "Failed to read file '{}': {}.\nPlease ensure the file name is correct and try again.",
                            file_name,
                            e.to_string()
                        );
                        return Some(AlpacaFunctions::error(self.name(), &error));
                    }
                }
            }

            // if the 'file_name' field is not provided, return an error
            let error = AlpacaFunctions::error(
                self.name(),
                "The 'file_name' field is missing from the request. Please review the usage and try again.",
            );
            let usage = self.info();
            let response = format!("{}{}\n", error, usage);
            return Some(response);
        }

        // If 'arguments' is not provided, return an error
        let error = AlpacaFunctions::error(
            self.name(),
            "The 'arguments' field is missing from the request. Please review the usage and try again.",
        );

        let usage = self.info();
        let response = format!("{}{}\n", error, usage);
        Some(response)
    }

    fn info(&self) -> &'static str {
        READ_FILE_INFO
    }

    fn name(&self) -> &'static str {
        "read_file"
    }

    fn description(&self) -> &'static str {
        "Outputs the contents of the specified text file."
    }
}

// ===
// AlpacaFunctionReadFile Tests
// ===
