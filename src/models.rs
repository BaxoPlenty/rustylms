use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetModelsResponse {
    pub data: Vec<LoadedModel>,
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageInformation {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionChoice {
    pub index: i32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionsResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: UsageInformation,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoadedModel {
    pub id: String,
    pub owned_by: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionsRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    Assistant,
    User,
}

impl Message {
    pub fn new<T>(role: Role, content: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            content: content.into(),
            role,
        }
    }

    pub fn system<T>(content: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(Role::System, content)
    }

    pub fn assistant<T>(content: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(Role::Assistant, content)
    }

    pub fn user<T>(content: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(Role::User, content)
    }
}

impl ChatCompletionsResponse {
    pub fn get_message(&self) -> Option<&Message> {
        match self.choices.first() {
            Some(choice) => Some(&choice.message),
            _ => None,
        }
    }

    pub fn usage(&self) -> &UsageInformation {
        &self.usage
    }
}
