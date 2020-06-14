use std::fmt;

#[derive(Copy, Clone)]
pub enum Status {
    CODE1(u8),
    CODE2(u8),
    CODE3(u8),
    CODE4(u8),
    CODE5(u8),
    INFO,
    SUCCESS,
    REDIRECT,
    BAD,
    ERROR
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (code, message) = match self {
            Status::CODE1(code) => {
                match code {
                    01 => (101, "Switching Protocol"),
                    02 => (102, "Processing"),
                    03 => (103, "Early Hints"),
                    _ => (100, "Continue")
                }
            },
            Status::CODE2(code) => {
                match code {
                    01 => (201, "Created"),
                    02 => (202, "Accepted"),
                    03 => (203, "Non-Authoritative Information"),
                    04 => (204, "No Content"),
                    05 => (205, "Reset Content"),
                    06 => (206, "Partial Content"),
                    _ => (200, "OK")
                }
            },
            Status::CODE3(code) => {
                match code {
                    01 => (301, "Moved Permanently"),
                    02 => (302, "Found"),
                    03 => (303, "See Other"),
                    04 => (304, "Not Modified"),
                    07 => (307, "Temporary Redirect"),
                    08 => (308, "Permanent Redirect"),
                    _ => (300, "Multiple Choice")
                }
            },
            Status::CODE4(code) => {
                match code {
                    01 => (401, "Unauthorized"),
                    02 => (402, "Payment Required"),
                    03 => (403, "Forbidden"),
                    04 => (404, "Not Found"),
                    05 => (405, "Method Not Allowed"),
                    06 => (406, "Not Acceptable"),
                    07 => (407, "Proxy Authentication Required"),
                    08 => (408, "Request Timeout"),
                    09 => (409, "Conflict"),
                    10 => (410, "Gone"),
                    11 => (411, "Length Required"),
                    12 => (412, "Precondition Failed"),
                    13 => (413, "Payload Too Large"),
                    14 => (414, "URI Too Large"),
                    15 => (415, "Unsupported Media Type"),
                    16 => (416, "Range Not Satisfiable"),
                    17 => (417, "Expectation Failed"),
                    18 => (418, "I'm a teapot"),
                    21 => (421, "Misdirected Request"),
                    22 => (422, "Unprocessable Entity"),
                    23 => (423, "Locked"),
                    24 => (424, "Failed Dependency"),
                    25 => (425, "Too Early"),
                    26 => (426, "Upgrade Required"),
                    28 => (428, "Precondition Required"),
                    29 => (429, "Too Many Requests"),
                    31 => (431, "Request Header Fields Too Large"),
                    51 => (451, "Lawyers who needs um am I right"),
                    _ => (400, "Bad Request")
                }
            },
            Status::CODE5(code) => {
                match code {
                    01 => (501, "Not Implemented"),
                    02 => (502, "Bad Gateway"),
                    03 => (503, "Service Unavailable"),
                    04 => (504, "Gateway Timeout"),
                    05 => (505, "HTTP Version Not Supported"),
                    06 => (506, "Variant Also Negotiates"),
                    07 => (507, "Insufficient Storage"),
                    08 => (508, "Loop Detected"),
                    10 => (510, "Not Extended"),
                    11 => (511, "Network Authentication Required"),
                    _ => (500, "Internal Server Error")
                }
            },
            Status::INFO => (100, "Continue"),
            Status::SUCCESS => (200, "OK"),
            Status::REDIRECT => (300, "Multiple Choice"),
            Status::BAD => (400, "Bad Request"),
            Status::ERROR => (500, "Internal Server Error"),
        };
        write!(f, "{} {}", code, message)
    }
}
