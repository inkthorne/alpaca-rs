use ollie_rs::session::OllamaSession;
use std::io::{self, Write};

const SYSTEM_MESSAGE: &str = r#"
You have access to tools available to help you get more information about
a topic to assist your user. If a relevant tool is available for use, feel free to always use the tool_call
to gain more insight & context. In addition to using the tool_call, you can tell the user that you are gathering
additional context.

The following tools are available to you:
"#;

const USAGE_FETCH: &str = r#"
# `fetch` 
This tool fetches data from a URL and returns the data as a string.

## `fetch` parameters:
- `url`: String = the URL to fetch data from.

## `fetch` output:
- String

## `fetch` tool_call example:
```tool_call
{
    "tool": "fetch",
    "arguments": {
        "url": "something.com"
    }
}
```

## `fetch` tool_output example:
```tool_output
{
    "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
    "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
}
```
"#;

const USAGE_WORKSPACE_FILES: &str = r#"
# `workspace_files` 
This tool outputs a list of files in the user's current workspace.

## `workspace_files` parameters:
- `url`: String = the URL to fetch data from.

## `workspace_files` output:
- String

## `workspace_files` tool_call example:
```tool_call
{
    "tool": "workspace_files",
}
```

## `workspace_files` tool_output example:
```tool_output
{
    "files": [
        "hello.rs",
        "something.exe",
        "readme.txt"
    ]
}
```

"#;

#[tokio::main]
async fn main() {
    let system_message = format!(
        "{}\n{}\n{}",
        SYSTEM_MESSAGE, USAGE_FETCH, USAGE_WORKSPACE_FILES
    );
    let mut session = OllamaSession::new("gemma3:4b");
    session.system(&system_message);
    // session.user("can you tell how many files are in my workspace?");
    // session.user("what tools are available?");
    // session.user("can you tell me what example.com is about?");
    session.user("can you tell how many files are in my workspace? and can you tell me what example.com is about?");
    session
        .update(|response| {
            print!("{}", response);
            io::stdout().flush().unwrap();
        })
        .await
        .unwrap();
}
