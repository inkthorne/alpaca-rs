use alpaca_rs::action::AlpacaActions;
use ollie_rs::session::OllamaSession;
use std::io::{self, Write};

pub const SYS_PROMPT_2: &str = r#"
You are a helpful and friendly assistant. You excel at following instructions,
answering questions, and working step-by-step through problems.

You are proficient at using 'actions' like reading a file, listing the contents
of a directory, or fetching a web-page. These actions help you collect information
that is external to you, allowing you to answer questions
and complete tasks, where your own internal knowledge is not sufficient.

"#;

pub const SYS_PROMPT_3: &str = r#"
You are a helpful and friendly assistant. You excel at following instructions,
answering questions, and working step-by-step through problems.

You are proficient at using 'actions' like reading a file, listing the contents
of a directory, or fetching a web-page. These actions help you answer questions
and complete tasks, where your own internal knowledge is not sufficient.

Before using your first 'action', you will use the action 'list_actions' to get a
list of all the actions that are available remember them. You will only
use actions from the list that 'list_actions' returns.

An example of how to use the 'list_actions' action is below:

```json
{
    "action": "list_actions"
}
```

Before using an 'action' for the first time, you will use the 'desribe_action'
action to describe the action's purpose and see an example of how it needs
to be invoked.

An example of using the 'describe_action' action is below:

```json
{
    "action": "describe_action",
    "action_name": "list_actions"
}
```

When you have completed a task or finished answering a question, you will respond
with '** DONE **' at the end of your response. This indicates that you have no further
actions to perform and you am finished with the task.
"#;

pub const QUERY_1: &str = r#"
# Your Task

Find the files that end with ".lock" in the current directory and list the names of
those files. When you have the final answer, put it in JSON format like this:
```json
{
    "match_count": 3,
    "names": [
        "name1",
        "name2",
        "name3"
    ]
}
```

You will need to use actions to solve this task. Use the following to invoke the
`list_actions` action to get a list of all the actions that are available to you:
```json
{
    "action": "list_actions"
}
```
"#;

// ---

pub const QUERY_2: &str = r#"
# Your Task

Find the hidden subdirectories (subdirectories that start with '.') in the current
directory and list the names of those subdirectories. When you have the final answer,
output it in JSON format like this:
```json
{
    "match_count": 3,
    "names": [
        "name1",
        "name2",
        "name3"
    ]
}
```

Afterward, enumerate the steps you would use to reverse-check the answer. Use the following format:
```json
{
    "steps": [
        "step 1",
        "step 2",
        "step 3"
    ]
}
```

You will need to use 'actions' to solve this task. Use the following to invoke the
`list_actions` action to get a list of all the actions that are available to you:
```json
{
    "action": "list_actions"
}
```
"#;

#[tokio::main]
async fn main() {
    let actions = AlpacaActions::new();
    let model = "gemma3:4b";
    // let model = "gemma3:12b";
    // let model = "deepseek-r1:8b";
    // let model = "deepseek-r1:14b";
    let mut session = OllamaSession::local(model);
    // let mut session = OllamaSession::new(model);
    session.options().set_temperature(1.0);

    let prompt = SYS_PROMPT_2;
    println!("{}", prompt);
    session.system(prompt);
    let query = QUERY_2;
    println!("{}", query);
    session.user(query);

    for _ in 0..11 {
        println!("=== [[** ASSISTANT **]] ----------------------------\n");
        let mut accumulated_content = String::new();
        session
            .update(|content| {
                accumulated_content.push_str(content);
                print!("{}", content);
                io::stdout().flush().unwrap();
            })
            .await
            .unwrap();

        let mut action_count = 0;
        actions.invoke(&accumulated_content).map(|response| {
            println!("\n\n=== [[** USER **]] ---------------------------------");
            println!("{}", response);
            session.user(&response);
            action_count += 1;
        });

        if action_count == 0 {
            println!("\n=== [[** DONE **]] ---------------------------------\n");
            break;
        }
    }
}
