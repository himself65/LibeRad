use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub id: i32,
    pub text: String,
    pub user_id: i32,
    pub receiver_id: i32,
}