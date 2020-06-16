use std::cell::RefCell;
use std::fmt;
use super::Query;
use super::Path;
use std::collections::HashMap;


pub struct URI {
    authority: String,
    pub path: Path,
    pub query: Query,
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

impl URI {
    pub fn fragment(&self) -> &str {
        &self.fragment
    }

    pub fn authority(&self) -> &str {
        &self.authority
    }

    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    fn unwind_uri(uri_string: String) -> (String, String, Path, Query, String) {
        let (rest, fragment) = URI::unwind_string(uri_string, "#");
        let (rest, raw_query) = URI::unwind_string(rest, "?");
        let (scheme, mut rest) = URI::unwind_string(rest, ":");
        let (authority, path) = URI::unwind_path_and_authority(rest);
        let query = Query::from(raw_query);
        (scheme, authority, path, query, fragment)
    }

    fn unwind_path_and_authority(mut uri_string: String) -> (String, Path) {
        let has_authority = uri_string.starts_with("//");
        let mut path = Path::from(uri_string);
        let authority = if has_authority {
            path.shift().unwrap()
        } else {
            String::from("")
        };
        (authority, path)
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

impl fmt::Display for URI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{scheme}{authority}{path}{query}{fragment}", 
            scheme = format!("{}:", self.scheme),
            authority = format!("//{}", self.authority),
            path = self.path,
            query = self.query,
            fragment = format!("#{}", self.fragment)
        )
    }
}