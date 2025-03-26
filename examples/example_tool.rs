use alpaca_rs::function::AlpacaFunctions;
use alpaca_rs::function_dir::AlpacaFunctionDir;
use alpaca_rs::tool_dispatch::AlapacaToolDispatch;
use ollie_rs::session::OllamaSession;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let mut functions = AlpacaFunctions::new();
    functions.add_function(Box::new(AlpacaFunctionDir::new()));

    // let model = "gemma3:1b";
    let model = "gemma3:4b";
    // let model = "gemma3:12b";
    // let model = "deepseek-r1:7b";
    // let model = "deepseek-r1:14b";
    let mut session = OllamaSession::new(model);
    // session.system(&system_message);
    session.user(functions.intro());
    // println!("{}", functions.intro());

    // session.user("can you tell how many files are in my workspace?");
    session.user(
        "can you tell how many files are in my current directory? (hint: use the functions!)",
    );
    // session.user("what tools are available?");
    // session.user("can you tell me what example.com is about?");
    // session.user("can you tell how many files are in my workspace? and can you tell me what example.com is about?");

    for _ in 0..3 {
        let mut accumulated_content = String::new();
        session
            .update(|content| {
                accumulated_content.push_str(content);
                print!("{}", content);
                io::stdout().flush().unwrap();
            })
            .await
            .unwrap();

        let dispatch = AlapacaToolDispatch::new(&accumulated_content);
        let mut tool_output = String::new();
        for tool_call in dispatch.tool_calls() {
            tool_call.function().map(|name| {
                println!("\n\n (( tool_call: {} ))", name);
                functions
                    .call_function(name, tool_call.arguments())
                    .map(|output| {
                        tool_output.push_str(&output);
                    });
            });
        }

        println!(" (( tool_output: {} ))\n\n", tool_output);

        session.user(&tool_output);
    }
    /*
    session
        .update(|content| {
            accumulated_content.push_str(content);
            print!("{}", content);
            io::stdout().flush().unwrap();
        })
        .await
        .unwrap();

    stats.map(|r| {
        println!(
            "\n\n ** STATS: {} tokens used of {}\n",
            r.tokens_used(),
            session.context_window_size()
        );
    });
    */
}
