use anyhow::Result;

use crate::models::{GetModelsResponse, LoadedModel};

/// # LMSServer
///
/// This struct is the base-struct of all connections to LMS servers.
pub struct LMSServer {
    url: String,
}

impl LMSServer {
    /// Creates a new instance of `LMSServer` using the provided url
    ///
    /// # Examples
    ///
    /// ```rust
    /// import rustylms::lmsserver::LMSServer;
    ///
    /// let server = LMSServer::new("localhost:1234");
    /// ```
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self { url: url.into() }
    }

    /// Gets all the loaded models from the server.
    ///
    /// # Examples
    ///
    /// ```rust
    /// import rustylms::lmsserver::LMSServer;
    ///
    /// let server = LMSServer::new("localhost:1234");
    /// let models = server.get_models().await.expect("Unable to retreive models");
    /// ```
    pub async fn get_models(&self) -> Result<Vec<LoadedModel>> {
        let request = reqwest::get(format!("http://{}/v1/models", self.url)).await?;
        let body = request.json::<GetModelsResponse>().await?;

        Ok(body.data)
    }
}
