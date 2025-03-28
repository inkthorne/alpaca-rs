use crate::function::AlpacaFunction;

const FUNCTION_READ_FILE_INFO: &str = r#"
# `read_file`

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
        // Initialize a default response
        let mut response = serde_json::json!({
            "function": "read_file",
            "output": "Error: Unable to read file"
        });

        // Extract file_name from arguments
        if let Some(args) = arguments {
            if let Some(file_name) = args.get("file_name").and_then(|v| v.as_str()) {
                // Attempt to read file contents
                match std::fs::read_to_string(file_name) {
                    Ok(content) => {
                        // Update the response with the file content
                        response["output"] = serde_json::Value::String(content);
                    }
                    Err(e) => {
                        // Update the response with the error message
                        response["output"] =
                            serde_json::Value::String(format!("Error reading file: {}", e));
                    }
                }
            } else {
                // Return info content when file_name argument is missing
                return None;
            }
        } else {
            // Return info content when no arguments are provided
            return None;
        }

        // Format the response as pretty JSON inside a code block
        let text_output = serde_json::to_string_pretty(&response).unwrap_or_default();
        Some(format!("```json\n{}\n```\n", text_output))
    }

    fn info(&self) -> &'static str {
        FUNCTION_READ_FILE_INFO
    }

    fn name(&self) -> &'static str {
        "read_file"
    }

    fn description(&self) -> &'static str {
        "Outputs the contents of the specified text file."
    }
}
