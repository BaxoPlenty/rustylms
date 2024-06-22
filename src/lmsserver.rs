use anyhow::Result;
use reqwest::Client;

use crate::models::{
    ChatCompletionsRequest, ChatCompletionsResponse, GetModelsResponse, LoadedModel,
};

/// # LMSServer
///
/// This struct is the base-struct of all connections to LMS servers.
pub struct LMSServer {
    url: String,
    client: Client,
}

impl LMSServer {
    /// Creates a new instance of `LMSServer` using the provided url
    ///
    /// # Examples
    ///
    /// ```rust
    /// import rustylms::lmsserver::LMSServer;
    ///
    /// let server = LMSServer::new("http://localhost:1234");
    /// ```
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            url: url.into(),
            client: Client::new(),
        }
    }

    /// Gets all the loaded models from the server.
    ///
    /// # Examples
    ///
    /// ```rust
    /// import rustylms::lmsserver::LMSServer;
    ///
    /// let server = LMSServer::new("http://localhost:1234");
    /// let models = server.get_models().await.expect("Unable to retreive models");
    /// ```
    pub async fn get_models(&self) -> Result<Vec<LoadedModel>> {
        let request = reqwest::get(format!("{}/v1/models", self.url)).await?;
        let body = request.json::<GetModelsResponse>().await?;

        Ok(body.data)
    }

    /// Sends a chat completion request to the server.
    ///
    /// **Tip:** Use the `Chat` struct to use this easily.
    pub async fn get_chat_completion(
        &self,
        request: ChatCompletionsRequest,
    ) -> Result<ChatCompletionsResponse> {
        let request = self
            .client
            .post(format!("{}/v1/chat/completions", self.url))
            .json(&request)
            .send()
            .await?;
        let body = request.json::<ChatCompletionsResponse>().await?;

        Ok(body)
    }
}
