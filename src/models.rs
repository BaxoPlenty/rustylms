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
