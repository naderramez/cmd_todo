use std::{ffi::OsString, fmt::Debug, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

pub trait Indexable {
    fn id<'a>(&'a self) -> &'a str;
}

pub trait StoreableItem: Serialize + for<'de> Deserialize<'de> + Debug + Indexable {}

fn get_path(namespace: &str) -> PathBuf {
    let mut path = PathBuf::from("db");
    path.push(format!("{}.json", namespace));
    let file_path = path;
    file_path
}

pub fn get_stored_data<T: StoreableItem>(namespace: &str) -> Vec<T> {
    let file_path = get_path(namespace);
    if file_path.exists() == false {
        return Vec::new();
    }
    let file_content = fs::read_to_string(file_path).expect("Error with reading the file");
    let data_vec: Vec<T> =
        serde_json::from_str(&file_content).expect("JSON was not well-formatted");
    data_vec
}

pub fn persist_data<T: StoreableItem>(namespace: &str, data: Vec<T>) {
    let serialized_data = serde_json::to_string(&data).expect("Failed to serialize");
    let file_path = get_path(namespace);
    fs::write(file_path, serialized_data).expect("Failed to write file");
}

pub fn add_item<T: StoreableItem>(namespace: &str, item: T) {
    let mut data_vec = get_stored_data::<T>(namespace);
    data_vec.push(item);
    persist_data::<T>(namespace, data_vec);
}

pub fn remove_item<T: StoreableItem>(namespace: &str, id: &str) {
    let data = get_stored_data::<T>(namespace);
    if data.len() == 1 {
        let path = get_path(namespace);
        fs::remove_file(path).unwrap();
    } else {
        let mut data_vec = Vec::from(data);
        data_vec.retain(|item| item.id() != id);
        persist_data::<T>(namespace, data_vec);
    }
}

pub fn modify_item<T: StoreableItem>(namespace: &str, updated_item: T) {
    let data = get_stored_data::<T>(namespace);
    let mut data_vec = Vec::from(data);
    if let Some(item) = data_vec
        .iter_mut()
        .find(|item| item.id() == updated_item.id())
    {
        *item = updated_item;
    }
    persist_data::<T>(namespace, data_vec);
}

pub fn get_all_namespaces() -> impl Iterator<Item = OsString> {
    let path = PathBuf::from("db");
    let dir = fs::read_dir(path)
        .unwrap()
        .map(|result| result.unwrap().file_name());
    dir
}
