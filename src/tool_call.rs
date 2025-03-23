use serde_json::Value;

// ===
// AlpacaToolCall
// ===
/// A structured representation of a tool call, containing a function name and arguments.
///
/// This struct wraps a JSON object that follows the tool call format with a function
/// name and an arguments object.
pub struct AlpacaToolCall {
    object: Value,
}

impl AlpacaToolCall {
    /// Creates a new empty tool call.
    ///
    /// # Returns
    ///
    /// A new `AlpacaToolCall` instance with default values.
    pub fn new() -> AlpacaToolCall {
        AlpacaToolCall {
            object: Value::default(),
        }
    }

    /// Creates an `AlpacaToolCall` from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `json` - A string containing valid JSON representing a tool call
    ///
    /// # Returns
    ///
    /// * `Ok(AlpacaToolCall)` - If the JSON was successfully parsed
    /// * `Err(())` - If the JSON parsing failed
    pub fn from_str(json: &str) -> Result<AlpacaToolCall, ()> {
        let object = serde_json::from_str(json);
        match object {
            Ok(object) => Ok(AlpacaToolCall { object }),
            Err(_) => Err(()),
        }
    }

    /// Converts the tool call to a formatted JSON string.
    ///
    /// # Returns
    ///
    /// A pretty-printed JSON string representation of the tool call.
    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.object).unwrap()
    }

    /// Gets the function name of this tool call.
    ///
    /// # Returns
    ///
    /// * `Some(&str)` - The function name if it exists
    /// * `None` - If the function name is not set or is not a string
    pub fn function(&self) -> Option<&str> {
        self.object["function"].as_str()
    }

    /// Sets the function name for this tool call.
    ///
    /// # Arguments
    ///
    /// * `function` - The function name to set
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn set_function(&mut self, function: &str) -> &mut Self {
        self.object["function"] = Value::String(function.to_string());
        self
    }

    /// Gets an argument value by name.
    ///
    /// # Arguments
    ///
    /// * `arg` - The name of the argument to retrieve
    ///
    /// # Returns
    ///
    /// * `Some(&Value)` - The argument value if it exists
    /// * `None` - If the arguments object doesn't exist or if the specified argument isn't found
    pub fn argument(&self, arg: &str) -> Option<&Value> {
        let args = "arguments";
        self.object.get(args).and_then(|args| args.get(arg))
    }

    /// Adds or updates an argument with a JSON value.
    ///
    /// # Arguments
    ///
    /// * `arg` - The name of the argument to add
    /// * `value` - The JSON value to set for the argument
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining
    pub fn add_argument(&mut self, key: &str, value: Value) -> &mut Self {
        let args = "arguments";
        if !self.object.get(args).is_some() {
            self.object[args] = serde_json::json!({});
        }
        self.object[args][key] = value;
        self
    }
}

// ===
// AlpacaToolCall Tests
// ===

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the creation of a new `AlpacaToolCall` instance.
    ///
    /// Verifies that a new instance has a default (null) object value
    /// and no function name.
    #[test]
    fn test_new() {
        let tool_call = AlpacaToolCall::new();
        assert_eq!(tool_call.object, Value::Null);
        assert_eq!(tool_call.function(), None);
    }

    /// Tests the creation of an `AlpacaToolCall` from a valid JSON string.
    ///
    /// Verifies that parsing succeeds and the resulting instance
    /// correctly contains the expected function name and argument.
    #[test]
    fn test_from_string_valid() {
        let json = r#"{"function":"get_weather","arguments":{"location":"New York"}}"#;
        let result = AlpacaToolCall::from_str(json);
        assert!(result.is_ok());

        let tool_call = result.unwrap();
        assert_eq!(tool_call.function(), Some("get_weather"));
        assert_eq!(
            tool_call.argument("location").and_then(|v| v.as_str()),
            Some("New York")
        );
    }

    /// Tests the creation of an `AlpacaToolCall` from an invalid JSON string.
    ///
    /// Verifies that parsing fails with invalid JSON input and returns an error.
    #[test]
    fn test_from_string_invalid() {
        let invalid_json = r#"{"function":"get_weather","arguments":{"location"}"#; // Missing value
        let result = AlpacaToolCall::from_str(invalid_json);
        assert!(result.is_err());
    }

    /// Tests the retrieval of a function name from an `AlpacaToolCall`.
    ///
    /// Verifies that function() returns None for a new instance and
    /// returns the correct value after a function name is set.
    #[test]
    fn test_function() {
        let mut tool_call = AlpacaToolCall::new();
        assert_eq!(tool_call.function(), None);

        tool_call.set_function("search");
        assert_eq!(tool_call.function(), Some("search"));
    }

    /// Tests setting a function name on an `AlpacaToolCall`.
    ///
    /// Verifies that the method returns self for chaining and
    /// that the function name is correctly stored.
    #[test]
    fn test_set_function() {
        let mut tool_call = AlpacaToolCall::new();
        let chained = tool_call.set_function("calculate");

        // Test method chaining returns self
        assert!(std::ptr::eq(chained, &mut tool_call));

        // Test function was set correctly
        assert_eq!(tool_call.function(), Some("calculate"));
    }

    /// Tests retrieving a non-existent argument from an `AlpacaToolCall`.
    ///
    /// Verifies that attempting to access an argument that doesn't exist
    /// returns None.
    #[test]
    fn test_argument_nonexistent() {
        let tool_call = AlpacaToolCall::new();
        assert_eq!(tool_call.argument("query"), None);
    }

    /// Tests adding an argument to an empty `AlpacaToolCall`.
    ///
    /// Verifies that an argument can be added when no arguments exist yet
    /// and that it can be correctly retrieved.
    #[test]
    fn test_add_argument_to_empty() {
        let mut tool_call = AlpacaToolCall::new();
        tool_call.add_argument("query", Value::String("rust programming".to_string()));

        let arg_value = tool_call.argument("query");
        assert!(arg_value.is_some());
        assert_eq!(arg_value.and_then(|v| v.as_str()), Some("rust programming"));
    }

    /// Tests overwriting an existing argument in an `AlpacaToolCall`.
    ///
    /// Verifies that calling add_argument() with the same argument name
    /// overwrites the previous value.
    #[test]
    fn test_add_argument_overwrite() {
        let mut tool_call = AlpacaToolCall::new();
        tool_call.add_argument("query", Value::String("initial value".to_string()));
        tool_call.add_argument("query", Value::String("updated value".to_string()));

        assert_eq!(
            tool_call.argument("query").and_then(|v| v.as_str()),
            Some("updated value")
        );
    }

    /// Tests method chaining when adding arguments to an `AlpacaToolCall`.
    ///
    /// Verifies that multiple method calls can be chained and that
    /// all values are correctly set.
    #[test]
    fn test_add_argument_chaining() {
        let mut tool_call = AlpacaToolCall::new();
        let result = tool_call
            .set_function("search")
            .add_argument("query", Value::String("rust".to_string()))
            .add_argument("max_results", Value::String("10".to_string()));

        // Test method chaining returns self
        assert!(std::ptr::eq(result, &mut tool_call));

        // Test all values were set correctly
        assert_eq!(tool_call.function(), Some("search"));
        assert_eq!(
            tool_call.argument("query").and_then(|v| v.as_str()),
            Some("rust")
        );
        assert_eq!(
            tool_call.argument("max_results").and_then(|v| v.as_str()),
            Some("10")
        );
    }

    /// Tests the to_string_pretty method of `AlpacaToolCall`.
    ///
    /// Verifies that the method produces a correctly formatted JSON string
    /// that accurately represents the tool call's content.
    #[test]
    fn test_to_string_pretty() {
        // Create a tool call with a function and arguments
        let mut tool_call = AlpacaToolCall::new();
        tool_call
            .set_function("search")
            .add_argument("query", Value::String("rust programming".to_string()))
            .add_argument("max_results", Value::Number(serde_json::Number::from(10)));

        // Get the pretty-printed string
        let pretty_json = tool_call.to_string_pretty();
        println!("{}", pretty_json);

        // Verify it's properly formatted JSON
        let parsed_value: Value = serde_json::from_str(&pretty_json).expect("Should be valid JSON");

        // Check that the original data is preserved
        assert_eq!(parsed_value["function"].as_str(), Some("search"));
        assert_eq!(
            parsed_value["arguments"]["query"].as_str(),
            Some("rust programming")
        );
        assert_eq!(parsed_value["arguments"]["max_results"].as_i64(), Some(10));

        // Verify the string contains formatting (newlines and indentation)
        assert!(pretty_json.contains("\n"));
        assert!(pretty_json.contains("  ")); // Check for indentation
    }

    /// Tests a complete workflow with `AlpacaToolCall`.
    ///
    /// Verifies that an instance can be created from JSON, modified with
    /// new function name and arguments, and that all changes are properly applied.
    #[test]
    fn test_complete_workflow() {
        // Create a tool call from JSON
        let json = r#"{"function":"get_weather","arguments":{"location":"Seattle"}}"#;
        let mut tool_call = AlpacaToolCall::from_str(json).unwrap();

        // Modify it
        tool_call
            .set_function("search_weather")
            .add_argument("location", Value::String("Portland".to_string()))
            .add_argument("days", Value::String("5".to_string()));

        // Verify changes
        assert_eq!(tool_call.function(), Some("search_weather"));
        assert_eq!(
            tool_call.argument("location").and_then(|v| v.as_str()),
            Some("Portland")
        );
        assert_eq!(
            tool_call.argument("days").and_then(|v| v.as_str()),
            Some("5")
        );
    }
}
