use std::fmt;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;


#[derive(Debug)]
pub struct Query(HashMap<String, String>);

impl Deref for Query {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
       &self.0
    } 
}

impl DerefMut for Query {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for Query {
    fn from(query_string: String) -> Query {
        let mut query = HashMap::new();
        for pair in query_string.split("&") {
            if pair == "" {
                continue
            }
            let mut key_value = pair.split("=");
            query.insert(
                key_value.next().unwrap().to_string(), 
                key_value.next().unwrap().to_string()
            );
        }
        Query(query)
    } 
}

impl From<&str> for Query {
    fn from(query_string: &str) -> Query {
        let query_string = query_string.to_string();
        let mut query = HashMap::new();
        for pair in query_string.split("&") {
            if pair == "" {
                continue
            }
            let mut key_value = pair.split("=");
            query.insert(
                key_value.next().unwrap().to_string(), 
                key_value.next().unwrap().to_string()
            );
        }
        Query(query)
    } 
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut query_string = String::new();
        for (key, value) in self.iter() {
            query_string.push_str(format!("{}={}&", key, value).as_str());
        }
        query_string.pop();
        write!(f, "?{}", query_string)
    }
}
