
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Store {
    data: HashMap<String, Entry>,
}

pub struct Entry {
    t: Option<Instant>,
    value: String,
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {

        // self.data.insert(key, value);
        let entry = Entry{ t: None, value};
        self.data.insert(key, entry);
    }

    pub fn set_with_expiry(&mut self, key: String, value: String, expire_ms: u64) {
        let entry = Entry {
            t: Some(Instant::now() + Duration::from_millis(expire_ms)),
            value,
        };
        self.data.insert(key, entry);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        match self.data.get(key.as_str()) {
            Some(entry) => {
                if let Some(t) = &entry.t {
                    if Instant::now() > t.clone() {
                        self.data.remove(key.as_str());
                        return None
                    }
                }

                Some(entry.value.clone())
            }
            None => None,
        }
    }
}