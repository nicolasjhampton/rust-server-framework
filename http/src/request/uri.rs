use std::cell::RefCell;
use std::fmt;
use super::Query;
use std::collections::HashMap;


pub struct URI {
    authority: String,
    path: Vec<String>,
    query: Query,
    fragment: String,
    scheme: String,
}


impl From<String> for URI {
    fn from(uri_string: String) -> Self {
        let (scheme, authority, path, query, fragment) = URI::unwind_uri(uri_string);
        URI {
            fragment,
            query,
            path,
            authority,
            scheme
        }
    }
}

// scheme:[//authority]path[?query][#fragment]

impl URI {
    pub fn fragment(&self) -> &str {
        &self.fragment
    }

    fn unwind_uri(uri_string: String) -> (String, String, Vec<String>, Query, String) {
        let (rest, fragment) = URI::unwind_string(uri_string, "#");
        let (rest, raw_query) = URI::unwind_string(rest, "?");
        let (scheme, mut rest) = URI::unwind_string(rest, ":");
        let (mut authority, mut path) = if rest.starts_with("//") {
            rest = rest.trim_start_matches("//").to_string();
            let mut path: Vec<&str> = rest.split("/").collect();
            let authority: String = path.iter().next().unwrap().to_string();
            (authority, path)
        } else {
            let mut path: Vec<&str> = rest.split("/").collect();
            (String::from(""), path)
        };
        let query = Query::from(raw_query);
        let path = path.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        (scheme, authority, path, query, fragment)
    }

    fn unwind_string(uri_string: String, delimiter: &str) -> (String, String) {
        let parsed: Vec<String> = uri_string.splitn(2, delimiter).map(|x| x.to_string()).collect();
        match parsed.as_slice() {
            [rest, fragment] => (rest.to_string(), fragment.to_string()),
            [all] => (all.to_string(), "".to_string()),
            _ => panic!("URI parse failure"),
        }
    }

}