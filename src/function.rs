use serde_json::json;
use std::collections::HashMap;

const FUNCTIONS_INTRO: &str = r#"
You have access to 'functions' that will give you access to external data.
This allows you to obtain more information about a topic to assist your user.
If you feel you need more information, or are wanting to double-check that
your answer is accurate, using these 'functions' is a good idea.

To see a list of the 'functions' available to you, use the following function call:
```json
{
    "action": "invoke_function",
    "function": "list_functions"
} 
```
"#;

// ===
// AlpacaFunction
// ===
/// Trait defining the interface for all Alpaca functions
pub trait AlpacaFunction {
    /// Execute the function with the given parameters
    ///
    /// # Arguments
    ///
    /// * `arguments` - Optional JSON value containing the parameters for the function
    ///
    /// # Returns
    ///
    /// An Option<String> containing the result of the function execution, or None if execution failed
    fn execute(&self, arguments: Option<&serde_json::Value>) -> Option<String>;

    /// Return information about the function
    ///
    /// # Returns
    ///
    /// A static string containing detailed information about the function
    fn info(&self) -> &'static str;

    /// Return the name of the function
    ///
    /// # Returns
    ///
    /// A static string containing the name of the function
    fn name(&self) -> &'static str;

    /// Return the description of the function
    ///
    /// # Returns
    ///
    /// A static string containing a brief description of what the function does
    fn description(&self) -> &'static str;
}

// ===
// AlpacaFunctions
// ===
/// A collection of Alpaca functions that can be called by name
pub struct AlpacaFunctions {
    functions: HashMap<&'static str, Box<dyn AlpacaFunction>>,
}

impl AlpacaFunctions {
    /// Creates a new empty collection of Alpaca functions
    ///
    /// # Returns
    ///
    /// A new `AlpacaFunctions` instance with no functions registered
    pub fn new() -> Self {
        AlpacaFunctions {
            functions: HashMap::new(),
        }
    }

    /// Adds a function to the collection
    ///
    /// # Arguments
    ///
    /// * `function` - The function to add to the collection
    pub fn add_function(&mut self, function: Box<dyn AlpacaFunction>) {
        self.functions.insert(function.name(), function);
    }

    /// Lists all available functions in a formatted JSON string
    ///
    /// # Returns
    ///
    /// A string containing a markdown code block with JSON data about all available functions
    pub fn list_functions(&self) -> String {
        let descriptions = self
            .functions
            .iter()
            .map(|(name, function)| {
                serde_json::json!({
                    "function": name,
                    "description": function.description()
                })
            })
            .collect();

        let function_list = serde_json::Value::Array(descriptions);
        let json_output = json!({
            "function": "list_functions",
            "output": function_list
        });

        let text_output = format!(
            "```json\n{}\n```\n",
            serde_json::to_string_pretty(&json_output).unwrap_or_default()
        );

        text_output
    }

    /// Calls a function by name with the given arguments
    ///
    /// # Arguments
    ///
    /// * `function_name` - The name of the function to call
    /// * `arguments` - Optional JSON arguments to pass to the function
    ///
    /// # Returns
    ///
    /// * `Some(String)` - The result of the function call if the function exists
    /// * `None` - If the function does not exist
    pub fn call_function(
        &self,
        function_name: &str,
        arguments: Option<&serde_json::Value>,
    ) -> Option<String> {
        if let Some(function) = self.functions.get(function_name) {
            match function.execute(arguments) {
                Some(result) => Some(result),
                None => {
                    let usage_error = format!(
                        "Error: Incorrect usage of function '{}'. See usage below info below:\n{}",
                        function.name(),
                        function.info()
                    );
                    Some(usage_error)
                }
            }
        } else {
            let mut output_string = String::new();
            if function_name != "list_functions" {
                output_string.push_str(&format!(
                    "Function '{}' not found. Available functions are:\n",
                    function_name
                ));
            }

            output_string.push_str(&self.list_functions());
            Some(output_string)
        }
    }

    /// Returns the introductory text explaining how to use functions
    ///
    /// # Returns
    ///
    /// A static string with instructions for using functions
    pub fn intro(&self) -> &'static str {
        FUNCTIONS_INTRO
    }
}

// ===
// AlpacaFunctions Tests
// ===

#[cfg(test)]
mod tests {
    use super::*;

    // Mock function for testing
    struct MockFunction {
        name: &'static str,
        description: &'static str,
        return_value: &'static str,
    }

    impl MockFunction {
        fn new(name: &'static str, description: &'static str, return_value: &'static str) -> Self {
            MockFunction {
                name,
                description,
                return_value,
            }
        }
    }

    impl AlpacaFunction for MockFunction {
        fn execute(&self, _arguments: Option<&serde_json::Value>) -> Option<String> {
            Some(self.return_value.to_string())
        }

        fn info(&self) -> &'static str {
            "Mock function for testing"
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn description(&self) -> &'static str {
            self.description
        }
    }

    #[test]
    fn test_add_function() {
        let mut functions = AlpacaFunctions::new();
        let mock = MockFunction::new("test", "Test function", "test result");

        functions.add_function(Box::new(mock));

        assert_eq!(functions.functions.len(), 1);
        assert!(functions.functions.contains_key("test"));
    }

    #[test]
    fn test_list_functions() {
        let mut functions = AlpacaFunctions::new();
        functions.add_function(Box::new(MockFunction::new(
            "test1",
            "Test function 1",
            "result1",
        )));
        functions.add_function(Box::new(MockFunction::new(
            "test2",
            "Test function 2",
            "result2",
        )));

        let list = functions.list_functions();

        // Extract JSON from markdown code block
        let json_start = list.find("```json\n").unwrap() + "```json\n".len();
        let json_end = list.rfind("\n```").unwrap();
        let json_str = &list[json_start..json_end];

        let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();

        // Check the structure of the JSON response
        assert_eq!(parsed["function"], "list_functions");

        // Check the output array
        let output = &parsed["output"];
        assert!(output.is_array());
        let array = output.as_array().unwrap();
        assert_eq!(array.len(), 2);

        // Verify both functions are in the list
        let contains_test1 = array
            .iter()
            .any(|item| item["function"] == "test1" && item["description"] == "Test function 1");
        let contains_test2 = array
            .iter()
            .any(|item| item["function"] == "test2" && item["description"] == "Test function 2");

        assert!(contains_test1);
        assert!(contains_test2);
    }

    #[test]
    fn test_call_function() {
        let mut functions = AlpacaFunctions::new();
        functions.add_function(Box::new(MockFunction::new(
            "test",
            "Test function",
            "test result",
        )));

        // Test successful function call
        let result = functions.call_function("test", None);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "test result");
    }

    #[test]
    fn test_call_function_with_arguments() {
        let mut functions = AlpacaFunctions::new();
        functions.add_function(Box::new(MockFunction::new(
            "test",
            "Test function",
            "test result",
        )));

        // The MockFunction ignores arguments, but we should test that they are properly passed
        let args = serde_json::json!({"param": "value"});
        let result = functions.call_function("test", Some(&args));

        assert!(result.is_some());
        assert_eq!(result.unwrap(), "test result");
    }
}
