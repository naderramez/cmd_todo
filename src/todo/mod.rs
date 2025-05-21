pub use model::{Indexable, Todo};

use crate::storage;

mod model;

pub fn get_todos(namespace: &str) -> Vec<Todo> {
    storage::get_stored_data::<Todo>(namespace)
        .into_iter()
        .map(|todo| todo.clone())
        .collect()
}

fn get_todo_by_id(namespace: &str, id: &str) -> Option<Todo> {
    let todos = get_todos(namespace);
    todos.into_iter().find(|todo| todo.id() == id)
}

pub fn add_todo(namespace: &str, todo: Todo) {
    storage::add_item(namespace, todo);
}

pub fn remove_todo(namespace: &str, id: &str) {
    storage::remove_item::<Todo>(namespace, id);
}

pub fn toggle_done_status(namespace: &str, id: &str) {
    if let Some(todo) = get_todo_by_id(namespace, id) {
        let new_status = !todo.get_status();
        let updated_todo = todo.set_status(new_status);
        storage::modify_item::<Todo>(namespace, updated_todo);
    };
}
