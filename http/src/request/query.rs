use std::fmt;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;


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
            let key_value = pair.split("=").collect::<Vec<&str>>();
            query.insert(
                key_value.first().unwrap().to_string(), 
                key_value.last().unwrap().to_string()
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
