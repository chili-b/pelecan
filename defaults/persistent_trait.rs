use confy;
//use serde_derive::{Serialize, Deserialize};

pub trait Persistent: serde::ser::Serialize + serde::de::DeserializeOwned + std::marker::Sized + std::default::Default {
    fn load(name: &str) -> Self {
        if let Ok(stored_data) = confy::load(name) {
            return stored_data;
        }
        Self::default()
    }
    fn store(&self, name: &str) -> Result<(), confy::ConfyError> {
        confy::store(name, &self)
    }
}
