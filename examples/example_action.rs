use alpaca_rs::action::AlpacaActions;
use ollie_rs::session::OllamaSession;
use std::io::{self, Write};

const PROMPT_1: &str = r#"
When you need to perform 'actions' like reading a file, listing the contents
of a directory, or fetching a web-page, you can ask me to perform an 'action'.
An example of the JSON format you need to request this in is as follows:

```json
{
    "action": "list_actions"
}
```

On my turn, I will then respond with the output of the 'action' in JSON format
for you to use on your next turn.
"#;

#[tokio::main]
async fn main() {
    let actions = AlpacaActions::new();
    let model = "gemma3:4b";
    let mut session = OllamaSession::local(model);

    let prompt = PROMPT_1;
    println!("{}", prompt);
    session.user(prompt);

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
