# rustylms - A LM-Studio API wrapper written in Rust

> ℹ️ If you are looking for an ollama api wrapper, consider looking at [ollama-rs](https://github.com/pepperoni21/ollama-rs)

> ⚠️ This project is still not finished! Bugs may occur

This library provides support for [**LM Studio Servers**](https://lmstudio.ai). All features are made according to the [official documentation](https://lmstudio.ai/docs/local-server).

## Feature List

-   Generating completions using chats
-   Retrieving all models from the server

## To-Do List

-   ~~Generating completions~~
    -   Supporting streams as responses
-   Creating embeddings

## Examples

### Retrieve models

```rust
use rustylms::lmsserver::LMSServer;

#[tokio::main]
async fn main() {
    let server = LMSServer::new("http://localhost:1234");
    let models = server.get_models().await.expect("Unable to retrieve models");

    println!("{:#?}", models);
}
```

### Generating a chat completion

```rust
use rustylms::{chat::Chat, lmsserver::LMSServer};

#[tokio::main]
async fn main() {
    let server = LMSServer::new("http://localhost:1234");
    let chat = Chat::new("model-name")
        .system_prompt(
            "You are a helpful assistant that gives information to any programming-related topic.",
        )
        .user_prompt("what is rust?");

    let completion = chat
        .get_completions(&server)
        .await
        .expect("could not get completions");
    let message = completion.get_message().unwrap();

    println!("The assistant answered: {}", message.content);
}
```
