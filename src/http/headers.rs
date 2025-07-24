/// This module defines the HTTP headers used in the Orion project.
/// src/http/headers.rs
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpHeaders {
    headers: HashMap<String, String>,
}

impl HttpHeaders {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.headers.insert(key.to_ascii_lowercase(), value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_ascii_lowercase())
    }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.headers.iter()
    }
}

impl Default for HttpHeaders {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<(String, String)> for HttpHeaders {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut headers = HttpHeaders::new();
        for (k, v) in iter {
            headers.insert(k, v);
        }
        headers
    }
}
