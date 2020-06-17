use std::fmt;
use super::Query;
use super::Path;


#[derive(Debug)]
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

impl From<&str> for URI {
    fn from(uri_string: &str) -> Self {
        let uri_string = String::from(uri_string);
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
        println!("{}", &rest);
        let (scheme, rest) = URI::unwind_string_rev(rest, ":");
        let (authority, path) = URI::unwind_path_and_authority(rest);
        let query = Query::from(raw_query);
        (scheme, authority, path, query, fragment)
    }

    fn unwind_path_and_authority(uri_string: String) -> (String, Path) {
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

    fn unwind_string_rev(uri_string: String, delimiter: &str) -> (String, String) {
        let parsed: Vec<String> = uri_string.rsplitn(2, delimiter).map(|x| x.to_string()).collect();
        match parsed.as_slice() {
            [rest, fragment] => (fragment.to_string(), rest.to_string()),
            [all] => ("".to_string(), all.to_string()),
            _ => panic!("URI parse failure"),
        }
    }

}

impl fmt::Display for URI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{scheme}{authority}{path}{query}{fragment}", 
            scheme = if !self.scheme.is_empty() { format!("{}:", self.scheme) } else { "".to_string() },
            authority = if !self.authority.is_empty() { format!("//{}", self.authority) } else { "".to_string() },
            path = self.path,
            query = self.query,
            fragment = if !self.fragment.is_empty() { format!("#{}", self.fragment) } else { "".to_string() }
        )
    }
}
