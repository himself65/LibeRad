use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub text: String,
    pub user_id: i32,
    pub receiver_id: i32,
}