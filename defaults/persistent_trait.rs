use confy;
use std::path::Path;
//use serde_derive::{Serialize, Deserialize};

pub trait Persistent: serde::ser::Serialize + serde::de::DeserializeOwned + std::marker::Sized + std::default::Default {
    fn load<T>(path: T) -> Self 
    where T: AsRef<Path>{
        if let Ok(stored_data) = confy::load_path(path.as_ref()) {
            return stored_data;
        }
        Self::default()
    }

    fn store<T>(&self, path: T) -> Result<(), confy::ConfyError> 
    where T: AsRef<Path> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        return confy::store_path(path, &self)
    }
}
