use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models,
    storage::{Storage, StoreableItem},
};

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

    pub fn get(self) -> Self {
        self
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn modify_content(&mut self, text: String) -> &Self {
        self.content = text;
        self
    }

    pub fn mark_done(&mut self) -> &Self {
        self.done = true;
        self
    }
    pub fn mark_undone(&mut self) -> &Self {
        self.done = false;
        self
    }
}

pub struct TodosManager {
    todos: Vec<Todo>,
    storage_driver: Storage,
}

impl TodosManager {
    pub fn new(namespace: String) -> TodosManager {
        Self {
            todos: Vec::new(),
            storage_driver: Storage::new(namespace),
        }
    }

    pub fn todos(&mut self) -> &[Todo] {
        self.todos = self
            .storage_driver
            .get_stored_data::<Todo>()
            .into_iter()
            .map(|todo| todo.clone())
            .collect();
        &self.todos
    }

    pub fn add_todo(&mut self, todo: Todo) -> &[Todo] {
        self.storage_driver.add_item(todo.clone());
        self.todos()
    }

    pub fn remove_todo(&mut self, id: &str) -> &[Todo] {
        self.storage_driver.remove_item::<Todo>(id);
        self.todos()
    }

    pub fn mark_done(&mut self, mut todo: Todo) -> &[Todo] {
        todo.mark_done();
        self.storage_driver.modify_item::<Todo>(todo);
        self.todos()
    }
}
