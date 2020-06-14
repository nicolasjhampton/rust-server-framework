use std::collections::{HashMap};
use std::collections::hash_map::Iter;


#[derive(Debug)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn iter(&self) -> Iter<String, String> {
        self.0.iter()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    } 
}

impl From<Vec<String>> for Headers {
    fn from(raw: Vec<String>) -> Headers {
        let mut headers = HashMap::new();
        for header in raw.iter() {
            let h: Vec<&str> = header.split(": ").collect();
            headers.insert(
                h[0].trim().to_string(), 
                h[1].trim().to_string()
            );
        }
        Headers(headers)
    }
}
