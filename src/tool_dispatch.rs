use crate::tool_call::AlpacaToolCall;

// ===
// AlapaToolDispatch
// ===
pub struct AlapacaToolDispatch {
    tool_calls: Vec<AlpacaToolCall>,
}

// ---
// AlapaToolDispatch: Public Methods
// ---
impl AlapacaToolDispatch {
    pub fn new(message: &str) -> Self {
        let tool_calls = Self::create_tool_calls(message);

        AlapacaToolDispatch { tool_calls }
    }

    pub fn tool_calls(&self) -> &Vec<AlpacaToolCall> {
        &self.tool_calls
    }
}

// ---
// AlapaToolDispatch: Private Methods
// ---
impl AlapacaToolDispatch {
    fn create_tool_calls(message: &str) -> Vec<AlpacaToolCall> {
        Self::find_tool_calls(message)
            .iter()
            .filter_map(|tool_call_text| AlpacaToolCall::from_str(*tool_call_text).ok())
            .collect()
    }

    fn find_tool_calls<'a>(message: &'a str) -> Vec<&'a str> {
        const START_MARKER: &str = "```tool_call";
        const END_MARKER: &str = "```";

        let mut results = Vec::new();
        let mut search_start = 0;

        // Continue searching for tool calls until no more are found
        while let Some(start) = message[search_start..].find(START_MARKER) {
            // Adjust the start index to be relative to the entire message
            let abs_start = search_start + start;
            let content_start = abs_start + START_MARKER.len();

            // Find the end marker after the start marker
            if let Some(end) = message[content_start..].find(END_MARKER) {
                // Extract the tool call text
                let tool_call_text = message[content_start..content_start + end].trim();
                results.push(tool_call_text);

                // Update search position to continue after this tool call
                search_start = content_start + end + END_MARKER.len();
            } else {
                // No matching end marker found, exit loop
                break;
            }
        }

        results
    }
}
