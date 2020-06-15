use std::fmt;


#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Protocol {
    HTTP1_1,
    HTTP2
}

impl Protocol {
    pub fn new(protocol: &str) -> Protocol {
        match protocol {
            "HTTP/1.1" => Protocol::HTTP1_1,
            "HTTP/2.0" => Protocol::HTTP2,
            _ => panic!("Invalid protocol: {}", protocol)
        }
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let protocol = match self {
            HTTP1_1 => "HTTP/1.1",
            HTTP2 => "HTTP/2.0"
        };
        write!(f, "{}", protocol)
    }
}
