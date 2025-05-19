use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion  {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}
