use std::fmt;
use std::ops::{Deref, DerefMut};
use std::collections::{HashMap};
use std::collections::hash_map::Iter;

use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;


#[derive(Debug)]
pub struct Headers(HashMap<String, String>);

// constants will only last as long as
// the Header struct per 'a lifetime
impl<'a> Headers {
    const GENERAL: &'a[&'a str] = &[
        "Connection",
        "Upgrade",
        "Upgrade-Insecure-Requests",
        "Pragma",
        "Cache-Control",
    ];

    const REQUEST: &'a[&'a str] = &[
        "Host",
        "User-Agent",
        "Accept",
        "Accept-Language",
        "Accept-Encoding",
        "Accept-Charset",
        "DNT",
        "Authorization",
        "Proxy-Authorization",
        "Origin",
        "Referer",
        "Cookie",
        "If-Modified-Since",
        "If-None-Match",
        "If-Match",
        "If-Range",
        "If-Unmodified-Since",
        "TE",
        "Range",
        "Max-Forwards",
        "Expect",
        "From",
    ];

    const RESPONSE: &'a[&'a str] = &[
        "Location",
        "vary",
        "Server",
    ];

    const ENTITY: &'a[&'a str] = &[
        "Allow",
        "Content-Encoding",
        "Content-Language",
        "Content-Length",
        "Content-Type",
        "Content-Location",
        "Content-MD5",
        "Content-Range",
        "Expires",
        "Last-Modified",
    ];

    pub fn new() -> Headers {
        Headers(HashMap::new())
    }
}

impl Deref for Headers {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
       &self.0
    }
}

impl DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
       &mut self.0
    }
}

impl fmt::Display for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut found_keys = vec![];
        let mut header_string = String::new();

        // Known headers in order
        for group in [Headers::REQUEST, Headers::RESPONSE, Headers::ENTITY, Headers::GENERAL].iter() {
            for key in group.iter() {
                if let Some(value) = self.get(key.to_owned()) {
                    write!(header_string, "{}: {}\n", key, value)?;
                    found_keys.push(key.to_string());
                }
            }
        }

        for (key, value) in self.iter().filter(|(k,v)| !found_keys.contains(k)) {
            write!(header_string, "{}: {}\n", key, value)?
        }
        write!(f, "{}", header_string)
    }
}

impl From<String> for Headers {
    fn from(raw: String) -> Headers {
        if raw.is_empty() { return Headers(HashMap::new()) };
        let (inner_headers, _) = raw.lines().fold((HashMap::new(), String::new()), |(mut hash, mut last_key), raw_line| {
            let line = raw_line.splitn(2, ":").map(|x| x.trim()).collect::<Vec<&str>>();
            match line.as_slice() {
                [key, value] => {
                    last_key = key.to_string();
                    hash.insert(key.to_string(), value.to_string());
                },
                // multiline header support
                [value] if !value.is_empty() => {
                    hash.insert(
                        last_key.to_owned(),
                        format!("{} {}", 
                            hash.get(&last_key).unwrap().to_string(), 
                            value.to_string()
                        )
                    );
                },
                [value] if value.is_empty() => {},
                invalid_value => panic!("Unexpected parse: {:?}", invalid_value)
            }
            (hash, last_key)
        });
        Headers(inner_headers)
    }
}

impl From<&str> for Headers {
    fn from(raw: &str) -> Headers {
        let (inner_headers, _) = raw.lines().fold((HashMap::new(), String::new()), |(mut hash, mut last_key), raw_line| {
            let line = raw_line.splitn(2, ":").map(|x| x.trim()).collect::<Vec<&str>>();
            match line.as_slice() {
                [key, value] => {
                    last_key = key.to_string();
                    hash.insert(key.to_string(), value.to_string());
                },
                // multiline header support
                [value] if !value.is_empty() => {
                    hash.insert(
                        last_key.to_string(),
                        format!("{} {}", 
                            hash.get(&last_key).unwrap().to_string(), 
                            value.to_string()
                        )
                    );
                },
                [value] if value.is_empty() => {},
                invalid_value => panic!("Unexpected parse: {:?}", invalid_value)
            }
            (hash, last_key)
        });
        Headers(inner_headers)
    }
}

// impl From<&str> for Headers {
//     fn from(raw: &str) -> Headers {
//         let mut inner_headers = HashMap::new();
//         let mut key = String::new();
//         let mut value = String::new();
//         for line in raw.lines() {
//             let h: Vec<&str> = line.splitn(2, ":").map(|x| x.trim()).collect();
//             if h.len() == 2 {
//                 if !key.is_empty() && !value.is_empty() {
//                     inner_headers.insert(key, value);
//                 }
//                 key = h[0].to_string();
//                 value = h[1].to_string();
//             } else if !h[0].is_empty() {
//                 write!(value, "{} {}", value.to_owned(), h[0]);
//             }
//         }
//         if !key.is_empty() && !value.is_empty() {
//             inner_headers.insert(key, value);
//         }
//         Headers(inner_headers)
//     }
// }
