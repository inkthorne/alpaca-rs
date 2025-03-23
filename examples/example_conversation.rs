use ollie_rs::session::OllamaSession;
use std::io::{self, Write};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let mut session1 = OllamaSession::new("gemma3:4b");
    session1.options().set_num_ctx(20000);
    let mut session2 = OllamaSession::new("gemma3:4b");
    session2.options().set_num_ctx(20000);

    let mut response1 = String::new();
    let mut response2 = "Hello there! How are you doing today?".to_string();

    loop {
        println!("\n\n ----- BOB:\n");

        response1.clear();
        session1.user(&response2);
        session1
            .update(|response| {
                print!("{}", response);
                io::stdout().flush().unwrap();
                response1.push_str(&response);
            })
            .await
            .unwrap();

        // Sleep for 10 seconds
        // sleep(Duration::from_secs(10)).await;

        println!("\n\n ----- FRED:\n");

        response2.clear();
        session2.user(&response1);
        session2
            .update(|response| {
                print!("{}", response);
                io::stdout().flush().unwrap();
                response2.push_str(&response);
            })
            .await
            .unwrap();

        // Sleep for 10 seconds
        // sleep(Duration::from_secs(10)).await;
    }
}
