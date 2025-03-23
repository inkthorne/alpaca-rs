use ollie_rs::message::OllamaMessage;
use ollie_rs::ollama::Ollama;
use ollie_rs::request::OllamaRequest;
use std::io::{self, Write};

async fn simple_chat_example() {
    // Create a default Ollama client (connects to 127.0.0.1:11434)
    let ollama = Ollama::default();

    // Create a message with the same question as in example1.rs
    let mut message = OllamaMessage::new();
    message.set_role("user").set_content("Why is the sky blue?");

    // Create and configure the request
    let mut request = OllamaRequest::new();
    request
        .set_model("gemma3:1b") // Use the model available on your Ollama server
        .push_message(&message);

    println!("\nQuestion: Why is the sky blue?\n");

    // Send the chat request and handle the response
    ollama
        .chat(&request, |response| {
            // Check if the response is an error
            if let Some(err) = response.error() {
                eprintln!("Error: {}", err);
                return;
            }

            // Extract the response content and print it
            response.content().map(|text| {
                print!("{}", text); // Print each chunk as it arrives
                io::stdout().flush().unwrap();
            });
        })
        .await
        .unwrap();

    println!("\n");
}

#[tokio::main]
async fn main() {
    simple_chat_example().await;
}
