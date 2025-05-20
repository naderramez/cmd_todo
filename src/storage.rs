use std::{fmt::Debug, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::todo::Indexable;

pub struct Storage {
    namespace: String,
}

pub trait StoreableItem: Serialize + for<'de> Deserialize<'de> + Debug + Indexable {}

impl Storage {
    pub fn new(namespace: String) -> Storage {
        Self { namespace }
    }

    fn get_path(&self) -> PathBuf {
        let mut path = PathBuf::from("db");
        path.push(format!("{}.json", &self.namespace));
        let file_path = path;
        file_path
    }

    pub fn get_stored_data<T: StoreableItem>(&mut self) -> Vec<T> {
        let file_path = self.get_path();
        if file_path.exists() == false {
            return Vec::new();
        }
        let file_content = fs::read_to_string(file_path).expect("Error with reading the file");
        let data_vec: Vec<T> =
            serde_json::from_str(&file_content).expect("JSON was not well-formatted");
        data_vec
    }

    pub fn persist_data<T: StoreableItem>(&mut self, data: Vec<T>) {
        let serialized_data = serde_json::to_string(&data).expect("Failed to serialize");
        let file_path = self.get_path();
        fs::write(file_path, serialized_data).expect("Failed to write file");
    }

    pub fn add_item<T: StoreableItem>(&mut self, item: T) {
        let mut data_vec = self.get_stored_data::<T>();
        data_vec.push(item);
        self.persist_data::<T>(data_vec);
    }

    pub fn remove_item<T: StoreableItem>(&mut self, id: &str) {
        let data = self.get_stored_data::<T>();
        if data.len() == 1 {
            let path = self.get_path();
            fs::remove_file(path).unwrap();
        } else {
            let mut data_vec = Vec::from(data);
            data_vec.retain(|item| item.id() != id);
            self.persist_data::<T>(data_vec);
        }
    }

    pub fn modify_item<T: StoreableItem>(&mut self, updated_item: T) {
        let data = self.get_stored_data::<T>();
        let mut data_vec = Vec::from(data);
        if let Some(item) = data_vec
            .iter_mut()
            .find(|item| item.id() == updated_item.id())
        {
            *item = updated_item;
        }
        self.persist_data::<T>(data_vec);
    }
}
