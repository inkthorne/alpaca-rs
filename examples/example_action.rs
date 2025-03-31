use alpaca_rs::action::AlpacaActions;
use ollie_rs::session::OllamaSession;
use std::io::{self, Write};

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
Find the number of files can with the 'lock' extension in
the current directory and list the names of those files.
"#;

#[tokio::main]
async fn main() {
    let actions = AlpacaActions::new();
    let model = "gemma3:4b";
    // let model = "deepseek-r1:8b";
    let mut session = OllamaSession::local(model);

    let prompt = SYS_PROMPT_3;
    println!("{}", prompt);
    session.system(prompt);
    let query = QUERY_1;
    println!("{}", query);
    session.user(query);

    for _ in 0..11 {
        println!("\n=== [[** ASSISTANT **]] ----------------------------\n");
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
            println!("\n=== [[** USER **]] ---------------------------------\n");
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
