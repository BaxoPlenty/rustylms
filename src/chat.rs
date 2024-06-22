use anyhow::Result;

use crate::{
    lmsserver::LMSServer,
    models::{ChatCompletionsRequest, ChatCompletionsResponse, Message},
};

#[derive(Debug)]
pub struct Chat {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: i32,
}

impl Chat {
    /// Creates a new `Chat` with the selected model.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustylms::{
    ///     chat::Chat,
    ///     lmsserver::LMSServer
    /// };
    ///
    /// let server = LMSServer::new("http://localhost:1234");
    /// let chat = Chat::new("model-name").system_prompt("You are a helpful assistant.").user_prompt("Why does iron rust?");
    /// let completion = chat.get_completions(&server).await.expect("Could not get completions");
    ///
    /// println!("From assistant: {}", completion.get_message().unwrap().content);
    /// ```
    pub fn new<T>(model: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            model: model.into(),
            messages: vec![],
            temperature: 0.7,
            max_tokens: -1,
        }
    }

    /// Sets the temperature of the model. The default value for this is `0.7`.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;

        self
    }

    /// Sets the maximum tokens a completion can generate. The default value is `-1` meaning no limit.
    pub fn max_tokens(mut self, max_tokens: i32) -> Self {
        self.max_tokens = max_tokens;

        self
    }

    /// This function adds a system prompt to the messages.
    ///
    /// **NOTE:** This function doesn't clear the messages array before adding the system prompt!
    pub fn system_prompt<T>(mut self, system_prompt: T) -> Self
    where
        T: Into<String>,
    {
        self.add_system_message(system_prompt);

        self
    }

    /// This function adds a user prompt to the messages.
    ///
    /// **NOTE:** This function doesn't clear the messages array before adding the user prompt!
    pub fn user_prompt<T>(mut self, user_prompt: T) -> Self
    where
        T: Into<String>,
    {
        self.add_user_message(user_prompt);

        self
    }

    /// Clears all messages in the chat **including system prompts**.
    pub fn clear_messages(&mut self) {
        self.messages.clear()
    }

    /// Adds the provided `Message` to the chat.
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message)
    }

    /// Adds the provided message content as a system message.
    pub fn add_system_message<T>(&mut self, message: T)
    where
        T: Into<String>,
    {
        self.add_message(Message::system(message))
    }

    /// Adds the provided message content as a message from the assistant.
    pub fn add_assistant_message<T>(&mut self, message: T)
    where
        T: Into<String>,
    {
        self.add_message(Message::assistant(message))
    }

    /// Adds the provided message content as a message from the user.
    pub fn add_user_message<T>(&mut self, message: T)
    where
        T: Into<String>,
    {
        self.add_message(Message::user(message))
    }

    /// Gets the completion from the server by sending the current `Chat` struct.
    pub async fn get_completions(&self, server: &LMSServer) -> Result<ChatCompletionsResponse> {
        let request = ChatCompletionsRequest {
            max_tokens: self.max_tokens,
            messages: self.messages.clone(),
            model: self.model.clone(),
            temperature: self.temperature,
        };
        let response = server.get_chat_completion(request).await?;

        Ok(response)
    }
}
