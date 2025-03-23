use serde_json::Value;

// ===
// AlpacaToolParameterType
// ===
/// Represents the possible data types for parameters in an Alpaca tool.
///
/// This enum defines the standard data types that can be used for tool parameters
/// when interacting with Alpaca language models.
pub enum AlpacaToolParameterType {
    /// String data type
    String,
    /// Integer number data type
    Integer,
    /// Floating-point number data type
    Float,
    /// Boolean (true/false) data type
    Boolean,
    /// Complex object data type (JSON object)
    Object,
    /// Array data type (JSON array)
    Array,
}

impl AlpacaToolParameterType {
    /// Converts the parameter type to its string representation.
    ///
    /// # Returns
    ///
    /// A string that represents the parameter type in JSON schema format.
    pub fn to_string(&self) -> String {
        match self {
            AlpacaToolParameterType::String => "string".to_string(),
            AlpacaToolParameterType::Integer => "integer".to_string(),
            AlpacaToolParameterType::Float => "float".to_string(),
            AlpacaToolParameterType::Boolean => "boolean".to_string(),
            AlpacaToolParameterType::Object => "object".to_string(),
            AlpacaToolParameterType::Array => "array".to_string(),
        }
    }
}

// ===
// AlpacaToolProto
// ===
const DESCRIPTION: &str = "description";
const FUNCTION: &str = "function";
const PARAMETERS: &str = "parameters";

/// Represents a tool prototype for Alpaca models.
///
/// This struct maintains a JSON representation of a tool that can be used
/// with Alpaca language models, including its function name and parameters.
pub struct AlpacaToolProto {
    object: Value,
}

impl AlpacaToolProto {
    /// Creates a new empty tool prototype.
    ///
    /// Returns a default tool prototype with an empty JSON object.
    pub fn new() -> AlpacaToolProto {
        AlpacaToolProto {
            object: Value::default(),
        }
    }

    /// Creates a tool prototype from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `json` - A string containing valid JSON that represents a tool prototype
    ///
    /// # Returns
    ///
    /// * `Ok(AlpacaToolProto)` if parsing was successful
    /// * `Err(())` if the string could not be parsed as valid JSON
    pub fn from_string(json: &str) -> Result<AlpacaToolProto, ()> {
        let object = serde_json::from_str(json);
        match object {
            Ok(object) => Ok(AlpacaToolProto { object }),
            Err(_) => Err(()),
        }
    }

    /// Serializes the tool prototype to a pretty-printed JSON string.
    ///
    /// # Returns
    ///
    /// A formatted JSON string representation of the tool prototype.
    pub fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.object).unwrap()
    }

    pub fn description(&self) -> Option<&str> {
        self.object[DESCRIPTION].as_str()
    }

    pub fn set_description(&mut self, function: &str) {
        self.object[DESCRIPTION] = Value::String(function.to_string());
    }

    /// Gets the function name of the tool prototype.
    ///
    /// # Returns
    ///
    /// An `Option` containing the function name as a string slice if it exists,
    /// or `None` if the function field doesn't exist or isn't a string.
    pub fn function(&self) -> Option<&str> {
        self.object[FUNCTION].as_str()
    }

    /// Sets the function name of the tool prototype.
    ///
    /// # Arguments
    ///
    /// * `function` - The name of the function to set
    pub fn set_function(&mut self, function: &str) {
        self.object[FUNCTION] = Value::String(function.to_string());
    }

    /// Adds a parameter to the tool prototype with the specified name and type.
    ///
    /// If the parameters field doesn't exist or isn't an object, it will be
    /// initialized as an empty object before adding the parameter.
    ///
    /// # Arguments
    ///
    /// * `param_name` - The name of the parameter
    /// * `param_type` - The type of the parameter as an `AlpacaToolParameterType`
    pub fn add_parameter(&mut self, param_name: &str, param_type: AlpacaToolParameterType) {
        if !self.object[PARAMETERS].is_object() {
            self.object[PARAMETERS] = Value::Object(Default::default());
        }
        self.object[PARAMETERS][param_name] = Value::String(param_type.to_string());
    }

    /// Gets the parameters of the tool prototype.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the parameters value if it exists,
    /// or `None` if the parameters field doesn't exist.
    pub fn parameters(&self) -> Option<&Value> {
        self.object.get(PARAMETERS)
    }
}

// ===
// AlpacaToolProto Tests
// ===
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Tests that creating a new `AlpacaToolProto` initializes it with a `Value::Null` object.
    ///
    /// This test verifies the default state of a newly created tool prototype.
    #[test]
    fn test_new() {
        let tool = AlpacaToolProto::new();
        assert_eq!(tool.object, Value::Null);
    }

    /// Tests creating an `AlpacaToolProto` from a valid JSON string.
    ///
    /// Verifies that the from_string method correctly parses valid JSON
    /// and initializes the tool prototype with the expected values.
    #[test]
    fn test_from_string_valid() {
        let json = r#"{"function":"test_func","parameters":{"param1":"string"}}"#;
        let tool = AlpacaToolProto::from_string(json).unwrap();
        assert_eq!(tool.function().unwrap(), "test_func");
    }

    /// Tests that creating an `AlpacaToolProto` from an invalid JSON string returns an error.
    ///
    /// Confirms that the from_string method properly handles and rejects malformed JSON input.
    #[test]
    fn test_from_string_invalid() {
        let json = r#"{"function":invalid_json"#;
        let result = AlpacaToolProto::from_string(json);
        assert!(result.is_err());
    }

    /// Tests the pretty-printing JSON serialization of an `AlpacaToolProto`.
    ///
    /// Verifies that the to_string_pretty method produces a properly formatted JSON string
    /// containing all the expected keys and values.
    #[test]
    fn test_to_string_pretty() {
        let mut tool_description =
            "this function returns information about files in a directory that match a search "
                .to_string();
        tool_description
            .push_str("pattern such as the file name, modification date, and file size.");

        let mut tool = AlpacaToolProto::new();
        tool.set_function("file_info");
        tool.set_description(&tool_description);
        tool.add_parameter("path", AlpacaToolParameterType::String);
        tool.add_parameter("search_pattern", AlpacaToolParameterType::String);

        let pretty_string = tool.to_string_pretty();
        println!("{}", pretty_string);
        assert!(pretty_string.contains("file_info"));
        assert!(pretty_string.contains("path"));
        assert!(pretty_string.contains("search_pattern"));
    }

    /// Tests the function getter method of `AlpacaToolProto`.
    ///
    /// Verifies that the function method correctly returns None for a new tool prototype
    /// and the expected function name after it has been set.
    #[test]
    fn test_function() {
        let mut tool = AlpacaToolProto::new();
        assert_eq!(tool.function(), None);

        tool.set_function("my_function");
        assert_eq!(tool.function().unwrap(), "my_function");
    }

    /// Tests the function setter method of `AlpacaToolProto`.
    ///
    /// Verifies that the set_function method correctly sets the function name
    /// and that it can be updated by calling the method again.
    #[test]
    fn test_set_function() {
        let mut tool = AlpacaToolProto::new();
        tool.set_function("func1");
        assert_eq!(tool.object[FUNCTION], json!("func1"));

        // Test changing the function name
        tool.set_function("func2");
        assert_eq!(tool.object[FUNCTION], json!("func2"));
    }

    /// Tests adding parameters to an `AlpacaToolProto`.
    ///
    /// Verifies that the add_parameter method correctly adds parameters of different types
    /// and that they can be accessed through the underlying JSON object.
    #[test]
    fn test_add_parameter() {
        let mut tool = AlpacaToolProto::new();

        // Add first parameter
        tool.add_parameter("param1", AlpacaToolParameterType::String);
        assert_eq!(tool.object[PARAMETERS]["param1"], json!("string"));

        // Add second parameter
        tool.add_parameter("param2", AlpacaToolParameterType::Integer);
        assert_eq!(tool.object[PARAMETERS]["param2"], json!("integer"));

        // Add third parameter of different type
        tool.add_parameter("param3", AlpacaToolParameterType::Boolean);
        assert_eq!(tool.object[PARAMETERS]["param3"], json!("boolean"));
    }

    /// Tests that adding a parameter initializes the parameters object if it doesn't exist.
    ///
    /// Verifies that the add_parameter method properly initializes the parameters field
    /// as an object when it is first used.
    #[test]
    fn test_add_parameter_initializes_parameters() {
        let mut tool = AlpacaToolProto::new();
        assert!(!tool.object[PARAMETERS].is_object());

        tool.add_parameter("param1", AlpacaToolParameterType::String);
        assert!(tool.object[PARAMETERS].is_object());
    }

    /// Tests the parameters getter method of `AlpacaToolProto`.
    ///
    /// Verifies that the parameters method returns None for a new tool prototype
    /// and the expected parameters object after a parameter has been added.
    #[test]
    fn test_parameters() {
        let mut tool = AlpacaToolProto::new();
        assert_eq!(tool.parameters(), None);

        tool.add_parameter("param1", AlpacaToolParameterType::String);
        let params = tool.parameters().unwrap();
        assert!(params.is_object());
        assert_eq!(params["param1"], json!("string"));
    }

    /// Tests creating a complete tool prototype with multiple parameters.
    ///
    /// Verifies that a tool prototype can be fully constructed with a function name
    /// and multiple parameters of different types, and that all values are correctly
    /// represented in both the object and the serialized JSON string.
    #[test]
    fn test_complete_tool_creation() {
        let mut tool = AlpacaToolProto::new();
        tool.set_function("calculate");
        tool.add_parameter("x", AlpacaToolParameterType::Float);
        tool.add_parameter("y", AlpacaToolParameterType::Float);
        tool.add_parameter("operation", AlpacaToolParameterType::String);

        assert_eq!(tool.function().unwrap(), "calculate");
        let params = tool.parameters().unwrap();
        assert_eq!(params["x"], json!("float"));
        assert_eq!(params["y"], json!("float"));
        assert_eq!(params["operation"], json!("string"));

        let json_str = tool.to_string_pretty();
        assert!(json_str.contains("calculate"));
        assert!(json_str.contains("x"));
        assert!(json_str.contains("y"));
        assert!(json_str.contains("operation"));
        assert!(json_str.contains("float"));
        assert!(json_str.contains("string"));
    }
}
