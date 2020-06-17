use std::fmt;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};


#[derive(Debug)]
pub struct Path(VecDeque<String>);

impl From<&str> for Path {
    fn from(raw_path: &str) -> Self {
        let path = raw_path
            .split("/")
            .filter_map(|x| if x != "" { Some(x.to_string()) } else { None })
            .collect::<VecDeque<String>>();
        Path(path)
    }
}

impl From<String> for Path {
    fn from(raw_path: String) -> Self {
        let path = raw_path
            .split("/")
            .filter_map(|x| if x != "" { Some(x.to_string()) } else { None })
            .collect::<VecDeque<String>>();
        Path(path)
    }
}

impl Path {
    pub fn push(&mut self, segment: String) {
        self.push_back(segment);
    }

    pub fn shift(&mut self) -> Option<String> {
        self.pop_front()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut path = String::new();
        if self.is_empty() { 
            path = String::from("/");
        } else {
            for partial in self.iter() {
                path.push_str(&(format!("/{}", partial)));
            }
        }
        write!(f, "{}", path)
    }
}

impl Deref for Path {
    type Target = VecDeque<String>;

    fn deref(&self) -> &Self::Target {
       &self.0
    } 
}

impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
