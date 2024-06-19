use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetModelsResponse {
    pub data: Vec<LoadedModel>,
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadedModel {
    pub id: String,
    pub owned_by: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    Assistant,
    User,
}
