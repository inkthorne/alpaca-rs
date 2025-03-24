use alpaca_rs::tool_dispatch::AlapacaToolDispatch;
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
- json object

## `fetch` tool_call example:
```tool_call
{
    "tool": "fetch",
    "arguments": {
        "url": "something.com"
    }
}
```

## `fetch` tool_response example:
```tool_response
{
    "tool": "fetch",
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
- json array of file names.

## `workspace_files` tool_call example:
```tool_call
{
    "tool": "workspace_files",
}
```

## `workspace_files` tool_response example:
```tool_response
{
    "tool": "workspace_files",
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

    let model = "gemma3:4b";
    // let model = "gemma3:12b";
    // let model = "deepseek-r1:7b";
    // let model = "deepseek-r1:14b";
    let mut session = OllamaSession::new(model);
    session.system(&system_message);
    // session.user("can you tell how many files are in my workspace?");
    // session.user("what tools are available?");
    // session.user("can you tell me what example.com is about?");
    session.user("can you tell how many files are in my workspace? and can you tell me what example.com is about?");

    let mut accumulated_response = String::new();
    let stats = session
        .update(|response| {
            accumulated_response.push_str(response);
            print!("{}", response);
            io::stdout().flush().unwrap();
        })
        .await
        .unwrap();

    let dispatch = AlapacaToolDispatch::new(&accumulated_response);
    for tool_call in dispatch.tool_calls() {
        println!("\n\nTool call text:\n{}", tool_call.to_string_pretty());
    }

    stats.map(|r| {
        println!("\n\nResponse:\n{}", r.as_string_pretty());
    });
}
