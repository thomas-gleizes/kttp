use std::collections::HashMap;

pub struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (key, value) in &self.headers {
            result.push_str(&format!("{}: {}\r\n", key, value));
        }
        result
    }
}
