use crate::Result;
use rocksdb::DB;
use std::{ops::Deref, sync::Mutex};

pub struct Storage {
    db: DB,
}

impl Deref for Storage {
    type Target = DB;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl Storage {
    pub fn new(path: &str) -> Self {
        Self {
            db: DB::open_default(path).unwrap(),
        }
    }

    pub fn get_item(&self, key: &str) -> Result<Option<String>> {
        match self.get(key)? {
            Some(value) => Ok(Some(String::from_utf8(value)?)),
            None => Ok(None),
        }
    }

    pub fn set_item(&self, key: &str, value: &str) -> Result<()> {
        Ok(self.db.put(key, value)?)
    }

    pub fn remove_item(&self, key: &str) -> Result<()> {
        Ok(self.db.delete(key)?)
    }
}

pub struct StorageState(Mutex<Storage>);

impl Deref for StorageState {
    type Target = Mutex<Storage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StorageState {
    pub fn new(path: &str) -> Self {
        Self(Mutex::new(Storage::new(path)))
    }
}
