use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::storage::{self, StoreableItem};

pub use storage::Indexable;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    id: String,
    content: String,
    done: bool,
}

impl Indexable for Todo {
    fn id<'a>(&'a self) -> &'a str {
        &self.id
    }
}

impl StoreableItem for Todo {}

impl Todo {
    pub fn new(text: String) -> Todo {
        let id = Uuid::new_v4().to_string();
        let todo = Todo {
            id,
            content: text,
            done: false,
        };
        todo
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_status(&self) -> bool {
        self.done
    }

    pub fn set_status(mut self, new_status: bool) -> Self {
        self.done = new_status;
        self
    }

    pub fn get_completion_status_text(&self) -> &str {
        if self.done {
            return "Completed";
        } else {
            return "Not Completed";
        }
    }
}
