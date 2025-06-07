use alpaca_rs::action::AlpacaActions;
use ollie_rs::{OllamaSession, XmlUtil};
use std::io::{self, Write};

pub const SYS_PROMPT_2: &str = r#"
You are a helpful and friendly assistant. You excel at following instructions,
answering questions, and working step-by-step through problems.

You are proficient at using 'actions' like reading a file, listing the contents
of a directory, or fetching a web-page. These 'actions' help you collect information
that is external to you, allowing you to answer questions and complete tasks more
quickly and more accurately than if you were to only use your internal knowledge alone.

## How to Use Actions

You will need to use 'actions' to successfully complete tasks. Use the following
JSON block to invoke the `list_actions` action to get a list of all the actions
that are available to you:
```json
{
    "action": "list_actions"
}
```

### Action Tips
- Don't assume the results of the action, end your turn and wait for the user to respond.
- Do not invoke more than one action per turn.
- The results of the 'action' will be returned by the user on the following turn.
- Don't forget to escape your backslashes in JSON strings. For example, use `\\` instead of `\`.
- Put your JSON in a code block using triple backticks (```json) to ensure it is parsed correctly. 
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

// ---

pub const QUERY_1: &str = r#"
# Your Task

## Objective
Find the files that end with ".lock" in the current directory and list the names of
those files.

## Instructions

Make a plan of enumerated steps you are going to take to solve the task. Then, perform
one step per-turn. After each step, output the result of the step and determine if the
step was a success or failure. If the step failed, make adjustments and retry the step.
Do not proceed to the next step until the current step is successful.

## Hints

You will need to use actions to solve this task. Use the following to invoke the
`list_actions` action to get a list of all the actions that are available to you:
```json
{
    "action": "list_actions"
}
```

Remember to escape your backslashes in JSON strings. For example, use `\\` instead of `\`.
This is especially important when using regex patterns in JSON strings.
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

## Hints

Use the `regex` action to ensure your filtering is correct.
"#;

pub const QUERY_3: &str = r#"
# Your Task

Find the filenames that end with ".lock" in the current directory and output the names
in a JSON array.

## Instructions

1. First, use the `list_actions` action to get a list of all the actions that are
available to you. Review the list and use it in step 2.

2. With the list of actions, make a plan of enumerated steps you are going to
take to solve the task.

3. Execute each step one at a time making sure to state which step you are
currently on. Only execute one step per turn. After each step, output the result of the
step and determine if the step was a success or failure. If the step failed,
make adjustments are retry the step.  Do not proceed to the next step until
the current step is successful.

4. When you have the final answer, output it in JSON format and end the turn
with the string '** DONE **'.

## Hints

Remember to escape your backslashes in JSON strings. For example, use `\\` instead of `\`.
This is especially important when using regex patterns in JSON strings.

When performing string matches or filtering, make sure you use an appropriate 'action' to 
double-check your results: for example, `regex` or `string_match`.
"#;

fn streaming_print(content: &str) {
    print!("{}", content);
    io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
    let actions = AlpacaActions::new();
    // let model = "devstral:24b"; // best coding agent so far
    // let model = "dolphin3:8b";
    // let model = "phi4";
    // let model = "llama3.1:8b";
    // let model = "qwen2.5:7b";
    // let model = "qwen2.5-coder:7b";
    // let model = "qwen2.5-coder:14b";
    // let model = "qwen3:8b"; // solves in 2 steps
    // let model = "gemma2:9b";
    // let model = "gemma3:4b";
    // let model = "gemma3:4b-it-qat";
    // let model = "gemma3:12b";
    // let model = "gemma3:12b-it-qat";
    // let model = "granite3.3:8b";
    // let model = "deepseek-r1:7b";
    let model = "deepseek-r1:8b";
    // let model = "deepseek-r1:14b";
    // let model = "deepseek-coder-v2:16b";
    let mut session = OllamaSession::local(model);
    // let mut session = OllamaSession::new(model);
    session.options().set_temperature(0.1);
    session.options().set_num_ctx(8192);
    // session.options().set_seed(9834);

    let prompt = SYS_PROMPT_3;
    println!("{}", prompt);
    session.system(prompt);
    let query = QUERY_3;
    println!("{}", query);
    session.user(query);

    let mut step_count = 0;
    for _ in 0..11 {
        println!("=== [[** ASSISTANT **]] ----------------------------\n");
        let response = session
            .update(|content| {
                streaming_print(content);
            })
            .await
            .unwrap();

        let content = response.text().unwrap();
        let cleaned = XmlUtil::remove_tag(&content, "think");
        let text = if cleaned.is_some() {
            &cleaned.unwrap()
        } else {
            content
        };

        println!("\n\n=== [[** ASSISTANT CLEANED **]] ---------------------------------");
        println!("{}", text);

        let mut action_count = 0;

        actions.invoke(text).map(|response| {
            println!("\n\n=== [[** USER **]] ---------------------------------");
            println!("{}", response);
            session.user(&response);
            action_count += 1;
        });

        if action_count == 0 {
            println!("\n=== [[** DONE **]] ---------------------------------\n");
            break;
        }

        step_count += 1;
    }

    println!("Total steps: {}", step_count);
}
