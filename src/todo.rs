use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::storage::{self, StoreableItem};

pub trait Indexable {
    fn id<'a>(&'a self) -> &'a str;
}

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

    pub fn get_completion_status_text(&self) -> &str {
        if self.done {
            return "Completed";
        } else {
            return "Not Completed";
        }
    }
}

pub fn get_todos(namespace: &str) -> Vec<Todo> {
    storage::get_stored_data::<Todo>(namespace)
        .into_iter()
        .map(|todo| todo.clone())
        .collect()
}

fn get_todo_by_id(namespace: &str, id: &str) -> Option<Todo> {
    let todos = get_todos(namespace);
    todos.into_iter().find(|todo| todo.id == id)
}

pub fn add_todo(namespace: &str, todo: Todo) {
    storage::add_item(namespace, todo);
}

pub fn remove_todo(namespace: &str, id: &str) {
    storage::remove_item::<Todo>(namespace, id);
}

pub fn toggle_done_status(namespace: &str, id: &str) {
    if let Some(todo) = get_todo_by_id(namespace, id) {
        let updated_todo = Todo {
            id: id.to_string(),
            content: todo.content,
            done: !todo.done,
        };
        storage::modify_item::<Todo>(namespace, updated_todo);
    };
}
