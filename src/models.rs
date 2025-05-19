use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoModel {
    pub id: String,
    content: String,
    done: bool,
}
