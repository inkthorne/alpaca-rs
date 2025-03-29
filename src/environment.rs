use serde_json::{Value, json};
use std::path::PathBuf;

// ===
// AlpacaEnvironment
// ===

/// `AlpacaEnvironment` provides access to the execution environment
/// information for Alpaca functions.
pub struct AlpacaEnvironment {
    /// The current working directory path
    current_dir: PathBuf,
}

impl AlpacaEnvironment {
    /// Creates a new environment instance with the current directory
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_default();

        AlpacaEnvironment { current_dir }
    }

    /// Process a request containing a function name and arguments
    ///
    /// # Arguments
    ///
    /// * `request` - A JSON Value with 'function' and optional 'arguments' fields
    ///
    /// # Returns
    ///
    /// * `Value` - A JSON response from the executed function or an error if the function is not supported
    pub fn process_invocation(&mut self, request: &Value) -> Value {
        // Extract function name from request
        let function_name = match request.get("function").and_then(|v| v.as_str()) {
            Some(name) => name,
            None => {
                return json!({
                    "error": "Missing required field 'function' in request."
                });
            }
        };

        // Create a standalone empty JSON object to use as default
        let empty_args = json!({});

        // Extract arguments from request, default to empty object if not present
        let arguments = request.get("arguments").unwrap_or(&empty_args);

        // Match function name and call appropriate method
        match function_name {
            "get_current_directory" => self.invoke_get_current_directory(),
            "list_directory" => self.invoke_list_directory(),
            "change_directory" => match self.invoke_change_directory(arguments) {
                Ok(result) => result,
                Err(error) => error,
            },
            _ => {
                json!({
                    "error": format!("Unsupported function: '{}'.", function_name)
                })
            }
        }
    }
}

// ===
// AlpacaEnvironment: LLM Invoked Methods
// ===

impl AlpacaEnvironment {
    /// Changes the current directory to one of its subdirectories
    ///
    /// # Arguments
    ///
    /// * `arguments` - A JSON Value containing a `subdir_name` field with the name of the directory to change to
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - A JSON object with current directory information if successful
    /// * `Err(Value)` - A JSON object with error details if the directory change failed
    fn invoke_change_directory(&mut self, arguments: &Value) -> Result<Value, Value> {
        let mut output = json!({
            "function": "change_directory",
        });

        // Check if subdir_name exists in arguments
        if !arguments.get("subdir_name").is_some() {
            output["error"] = json!("Missing required argument 'subdir_name'.");
            return Err(output);
        }

        let subdir_name = arguments
            .get("subdir_name")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Create the new path by joining the current directory with the subdirectory name
        let new_path = self.current_dir.join(subdir_name);

        // Check if the path exists and is a directory
        if !new_path.exists() {
            output["error"] = json!(format!("Subdirectory '{}' does not exist.", subdir_name));
            return Err(output);
        }

        if !new_path.is_dir() {
            output["error"] = json!(format!("'{}' is not a directory.", subdir_name));
            return Err(output);
        }

        // Update the current directory
        // Canonicalize the path to resolve ".." segments
        match new_path.canonicalize() {
            Ok(canonical_path) => {
                self.current_dir = canonical_path;
            }
            Err(err) => {
                output["error"] = json!(format!("Failed to resolve path: '{}'.", err));
                return Err(output);
            }
        }

        // Return as JSON object with current directory included
        output["ok"] = json!({
            "current_dir": self.current_dir.to_string_lossy(),
        });

        Ok(output)
    }

    /// Returns the current working directory as a JSON Value
    ///
    /// # Returns
    ///
    /// * `Value` - A JSON object containing the current directory path
    fn invoke_get_current_directory(&self) -> Value {
        json!({
            "function": "get_current_directory",
            "ok": {
                "current_dir": self.current_dir.to_string_lossy(),
            }
        })
    }

    /// Sets a new current directory path
    fn set_current_dir(&mut self, path: PathBuf) {
        self.current_dir = path;
    }

    /// Lists files and directories in the current directory
    ///
    /// # Returns
    ///
    /// * `Value` - A JSON object containing sorted lists of files and directories and the current directory path
    fn invoke_list_directory(&self) -> Value {
        let mut files = Vec::new();
        let mut directories = Vec::new();

        // Read directory entries
        if let Ok(entries) = std::fs::read_dir(&self.current_dir) {
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

        // Return as JSON object with current directory included
        json!({
            "function": "list_directory",
            "ok": {
                "current_dir": self.current_dir.to_string_lossy(),
                "files": files,
                "directories": directories
            }
        })
    }
}

// ===
// AlpacaEnvironment Tests
// ===

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_new() {
        let env = AlpacaEnvironment::new();
        let current_dir = std::env::current_dir().unwrap_or_default();
        let result = env.invoke_get_current_directory();
        assert_eq!(
            result,
            json!({
                "function": "get_current_directory",
                "ok": {
                    "current_dir": current_dir.to_string_lossy(),
                }
            })
        );
    }

    #[test]
    fn test_set_current_dir() {
        let mut env = AlpacaEnvironment::new();
        let test_path = PathBuf::from("/tmp");
        env.set_current_dir(test_path.clone());

        let result = env.invoke_get_current_directory();
        assert_eq!(
            result,
            json!({
                "function": "get_current_directory",
                "ok": {
                    "current_dir": test_path.to_string_lossy(),
                }
            })
        );
    }

    #[test]
    fn test_list_dir() {
        // Setup a temporary directory structure
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        let subdir_path = temp_dir.path().join("subdir");
        fs::write(&file_path, "test content").unwrap();
        fs::create_dir(&subdir_path).unwrap();

        let mut env = AlpacaEnvironment::new();
        env.set_current_dir(temp_dir.path().to_path_buf());

        let result = env.invoke_list_directory();
        assert_eq!(
            result,
            json!({
                "function": "list_directory",
                "ok": {
                    "current_dir": temp_dir.path().to_string_lossy(),
                    "files": vec!["file.txt"],
                    "directories": vec!["subdir"]
                }
            })
        );
    }

    #[test]
    fn test_current_dir() {
        let env = AlpacaEnvironment::new();
        let current_dir = std::env::current_dir().unwrap_or_default();
        let result = env.invoke_get_current_directory();
        assert_eq!(
            result,
            json!({
                "function": "get_current_directory",
                "ok": {
                    "current_dir": current_dir.to_string_lossy(),
                }
            })
        );
    }

    #[test]
    fn test_change_dir_success() {
        // Setup a temporary directory structure
        let temp_dir = tempfile::tempdir().unwrap();
        let subdir_path = temp_dir.path().join("subdir");
        fs::create_dir(&subdir_path).unwrap();

        // Create environment with temp_dir as current
        let mut env = AlpacaEnvironment::new();
        let temp_dir_canonical = temp_dir.path().canonicalize().unwrap();
        env.set_current_dir(temp_dir_canonical.clone());

        // Test changing to the subdir
        let subdir_args = json!({"subdir_name": "subdir"});
        let result = env.invoke_change_directory(&subdir_args);
        assert!(result.is_ok());

        // Check JSON structure and values
        let output = result.unwrap();
        assert_eq!(output["function"], "change_directory");
        assert!(output["ok"].is_object());
        let current_dir_value = output["ok"]["current_dir"].as_str().unwrap();

        // Create a canonical path and convert to string for comparison
        let canonical_subdir = subdir_path.canonicalize().unwrap();
        let expected_dir = canonical_subdir.to_string_lossy();
        assert_eq!(current_dir_value, expected_dir);

        // Verify the current directory was actually changed
        let current_dir_json = env.invoke_get_current_directory();
        assert_eq!(
            current_dir_json["ok"]["current_dir"].as_str().unwrap(),
            canonical_subdir.to_string_lossy()
        );

        // Test returning to the parent directory with ".."
        let parent_args = json!({"subdir_name": ".."});
        let result = env.invoke_change_directory(&parent_args);
        assert!(result.is_ok());

        // Check JSON structure and values
        let output = result.unwrap();
        assert_eq!(output["function"], "change_directory");
        assert!(output["ok"].is_object());
        let current_dir_value = output["ok"]["current_dir"].as_str().unwrap();
        let expected_dir = temp_dir_canonical.to_string_lossy();
        assert_eq!(current_dir_value, expected_dir);

        // Verify current directory was changed back
        let current_dir_json = env.invoke_get_current_directory();
        assert_eq!(
            current_dir_json["ok"]["current_dir"].as_str().unwrap(),
            temp_dir_canonical.to_string_lossy()
        );
    }

    #[test]
    fn test_change_dir_nonexistent() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut env = AlpacaEnvironment::new();
        env.set_current_dir(temp_dir.path().to_path_buf());

        let nonexistent_args = json!({"subdir_name": "nonexistent_dir"});
        let result = env.invoke_change_directory(&nonexistent_args);
        assert!(result.is_err());

        // Check JSON structure and error message
        let error_output = result.unwrap_err();
        assert_eq!(error_output["function"], "change_directory");
        assert!(error_output["error"].is_string());
        assert!(
            error_output["error"]
                .as_str()
                .unwrap()
                .contains("does not exist")
        );

        // Current directory should remain unchanged
        let current_dir_json = env.invoke_get_current_directory();
        assert_eq!(
            current_dir_json["ok"]["current_dir"].as_str().unwrap(),
            temp_dir.path().to_string_lossy()
        );
    }

    #[test]
    fn test_change_dir_not_a_directory() {
        // Create a temporary directory and file
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        fs::write(&file_path, "test content").unwrap();

        let mut env = AlpacaEnvironment::new();
        env.set_current_dir(temp_dir.path().to_path_buf());

        let file_args = json!({"subdir_name": "file.txt"});
        let result = env.invoke_change_directory(&file_args);
        assert!(result.is_err());

        // Check JSON structure and error message
        let error_output = result.unwrap_err();
        assert_eq!(error_output["function"], "change_directory");
        assert!(error_output["error"].is_string());
        assert!(
            error_output["error"]
                .as_str()
                .unwrap()
                .contains("is not a directory")
        );

        // Current directory should remain unchanged
        let current_dir_json = env.invoke_get_current_directory();
        assert_eq!(
            current_dir_json["ok"]["current_dir"].as_str().unwrap(),
            temp_dir.path().to_string_lossy()
        );
    }

    #[test]
    fn test_change_dir_missing_argument() {
        let mut env = AlpacaEnvironment::new();

        // Test with empty JSON object - missing subdir_name
        let empty_args = json!({});
        let result = env.invoke_change_directory(&empty_args);
        assert!(result.is_err());

        // Check JSON structure and error message
        let error_output = result.unwrap_err();
        assert_eq!(error_output["function"], "change_directory");
        assert!(error_output["error"].is_string());
        assert!(
            error_output["error"]
                .as_str()
                .unwrap()
                .contains("Missing required argument")
        );

        // Current directory should remain unchanged
        let original_dir = env.invoke_get_current_directory();
        let current_dir = std::env::current_dir().unwrap_or_default();
        assert_eq!(
            original_dir,
            json!({
                "function": "get_current_directory",
                "ok": {
                    "current_dir": current_dir.to_string_lossy(),
                }
            })
        );
    }

    #[test]
    fn test_change_dir_empty_string() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut env = AlpacaEnvironment::new();
        env.set_current_dir(temp_dir.path().to_path_buf());

        // Test with empty string as subdir_name
        let empty_string_args = json!({"subdir_name": ""});
        let result = env.invoke_change_directory(&empty_string_args);

        // This should be a success since "" resolves to the current directory
        assert!(result.is_ok());

        // Verify current directory remains the same
        let current_dir_json = env.invoke_get_current_directory();
        assert_eq!(
            current_dir_json["ok"]["current_dir"].as_str().unwrap(),
            temp_dir.path().canonicalize().unwrap().to_string_lossy()
        );
    }

    #[test]
    fn test_change_dir_null_value() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut env = AlpacaEnvironment::new();
        env.set_current_dir(temp_dir.path().to_path_buf());

        // Test with null value as subdir_name
        let null_args = json!({"subdir_name": null});
        let result = env.invoke_change_directory(&null_args);

        // Empty string from null should behave like the empty string test
        let original_dir = temp_dir.path().canonicalize().unwrap();

        // This should be a success since unwrap_or("") makes it behave like empty string
        assert!(result.is_ok());

        // Verify current directory remains the same
        let current_dir_json = env.invoke_get_current_directory();
        assert_eq!(
            current_dir_json["ok"]["current_dir"].as_str().unwrap(),
            original_dir.to_string_lossy()
        );
    }

    #[test]
    fn test_process_request_current_dir() {
        let mut env = AlpacaEnvironment::new();
        let current_dir = std::env::current_dir().unwrap_or_default();

        let request = json!({
            "function": "get_current_directory"
        });

        let result = env.process_invocation(&request);
        assert_eq!(
            result,
            json!({
                "function": "get_current_directory",
                "ok": {
                    "current_dir": current_dir.to_string_lossy(),
                }
            })
        );
    }

    #[test]
    fn test_process_request_list_dir() {
        // Setup a temporary directory structure
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        let subdir_path = temp_dir.path().join("subdir");
        fs::write(&file_path, "test content").unwrap();
        fs::create_dir(&subdir_path).unwrap();

        let mut env = AlpacaEnvironment::new();
        env.set_current_dir(temp_dir.path().to_path_buf());

        let request = json!({
            "function": "list_directory"
        });

        let result = env.process_invocation(&request);
        assert_eq!(
            result,
            json!({
                "function": "list_directory",
                "ok": {
                    "current_dir": temp_dir.path().to_string_lossy(),
                    "files": vec!["file.txt"],
                    "directories": vec!["subdir"]
                }
            })
        );
    }

    #[test]
    fn test_process_request_change_dir() {
        // Setup a temporary directory structure
        let temp_dir = tempfile::tempdir().unwrap();
        let subdir_path = temp_dir.path().join("subdir");
        fs::create_dir(&subdir_path).unwrap();

        let mut env = AlpacaEnvironment::new();
        let temp_dir_canonical = temp_dir.path().canonicalize().unwrap();
        env.set_current_dir(temp_dir_canonical.clone());

        let request = json!({
            "function": "change_directory",
            "arguments": {
                "subdir_name": "subdir"
            }
        });

        let result = env.process_invocation(&request);

        // Get the canonical path of the subdirectory for comparison
        let canonical_subdir = subdir_path.canonicalize().unwrap();

        assert_eq!(result["function"], "change_directory");
        assert!(result["ok"].is_object());
        assert_eq!(
            result["ok"]["current_dir"].as_str().unwrap(),
            canonical_subdir.to_string_lossy()
        );
    }

    #[test]
    fn test_process_request_change_dir_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut env = AlpacaEnvironment::new();
        env.set_current_dir(temp_dir.path().to_path_buf());

        let request = json!({
            "function": "change_directory",
            "arguments": {
                "subdir_name": "nonexistent_dir"
            }
        });

        let result = env.process_invocation(&request);
        /*
        println!(
            "request:\n{}\n\nresponse:\n{}\n",
            serde_json::to_string_pretty(&request).unwrap(),
            serde_json::to_string_pretty(&result).unwrap()
        ); // For debugging
        */

        assert_eq!(result["function"], "change_directory");
        assert!(result["error"].is_string());
        assert!(result["error"].as_str().unwrap().contains("does not exist"));
    }

    #[test]
    fn test_process_request_invalid_function() {
        let mut env = AlpacaEnvironment::new();

        let request = json!({
            "function": "invalid_function"
        });

        let result = env.process_invocation(&request);
        // println!("{}", serde_json::to_string_pretty(&result).unwrap()); // For debugging
        assert!(result["error"].is_string());
        assert!(
            result["error"]
                .as_str()
                .unwrap()
                .contains("Unsupported function")
        );
    }

    #[test]
    fn test_process_request_missing_function() {
        let mut env = AlpacaEnvironment::new();

        let request = json!({
            "arguments": {
                "some_arg": "value"
            }
        });

        let result = env.process_invocation(&request);
        /*
        println!(
            "request:\n{}\n\nresponse:\n{}\n",
            serde_json::to_string_pretty(&request).unwrap(),
            serde_json::to_string_pretty(&result).unwrap()
        ); // For debugging
        */
        assert!(result["error"].is_string());
        assert!(
            result["error"]
                .as_str()
                .unwrap()
                .contains("Missing required field 'function'")
        );
    }
}
