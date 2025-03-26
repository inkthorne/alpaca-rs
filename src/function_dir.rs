use crate::function::AlpacaFunction;

const FUNCTION_DIR_INFO: &str = r#"
# `dir`

This function lists the files & directories in the current directory.

example call:
```tool_call
{
    "action": "invoke_function",
    "function": "dir",
    "arguments": {}
}
```

example response:
```tool_response
{
    "function": "dir",
    "files": [
        "file1.txt",
        "file2.txt"
    ],
    "directories": [
        "dir1",
        "dir2"
    ]
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
    fn execute(&self, _arguments: Option<&serde_json::Value>) -> String {
        let files = vec!["one.rs", "two.rs"];
        let directories = vec!["source", "target"];

        let json_output = serde_json::json!({
            "function": "dir",
            "output": {
                "files": files,
                "directories": directories
            }
        });

        let text_output = serde_json::to_string_pretty(&json_output).unwrap_or_default();
        let text_output = format!("```json\n{}\n```\n", text_output);

        text_output
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
