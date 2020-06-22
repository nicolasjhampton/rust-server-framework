pub mod headers;
pub use headers::Headers;

pub mod protocol;
pub use protocol::Protocol;

pub mod request_mod;
pub use request_mod::{
    Request,
    route_mod::{
        Route, 
        Method,
        uri_mod::{
            URI,
            Path,
            Query
        }
    }
};

pub mod response_mod;
pub use response_mod::{
    Response,
    Status
};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_headers_print_in_set_order() {
        let mut headers = Headers::new();
        headers.insert("Referer".to_string(), "http://localhost/test.php".to_string());
        headers.insert("Connection".to_string(), "keep-alive".to_string());
        headers.insert("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string());
        headers.insert("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string());
        headers.insert("Host".to_string(), "localhost".to_string());
        assert_eq!(headers.to_string(), 
            "Host: localhost\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\nReferer: http://localhost/test.php\nContent-Type: application/x-www-form-urlencoded\nConnection: keep-alive\n"
        );
    }

    #[test]
    fn more_than_one_header_can_be_printed() {
        let mut headers = Headers::new();
        headers.insert("Host".to_string(), "localhost".to_string());
        headers.insert("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string());
        assert!(headers.to_string().contains("Host: localhost\n"));
        assert!(headers.to_string().contains("Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\n"));
        assert_eq!(headers.to_string().lines().collect::<Vec<&str>>().len(), 2);
    }

    #[test]
    fn header_can_be_printed() {
        let mut headers = Headers::new();
        headers.insert("Host".to_string(), "localhost".to_string());
        assert_eq!(headers.to_string(), "Host: localhost\n");
    }

    #[test]
    fn header_example_with_multiline() {
        let headers = Headers::from("
        Host: localhost
        User-Agent: Mozilla/5.0 (Windows; U; Windows NT 6.1; en-US; rv:1.9.1.5)
        Gecko/20091102 Firefox/3.5.5 (.NET CLR 3.5.30729)
        Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
        Accept-Language: en-us,en;q=0.5
        Accept-Encoding: gzip,deflate
        Accept-Charset: ISO-8859-1,utf-8;q=0.7,*;q=0.7
        Keep-Alive: 300
        Connection: keep-alive
        Referer: http://localhost/test.php
        Content-Type: application/x-www-form-urlencoded
        Content-Length: 43
        ");
        assert_eq!(headers.get("User-Agent").unwrap(), "Mozilla/5.0 (Windows; U; Windows NT 6.1; en-US; rv:1.9.1.5) Gecko/20091102 Firefox/3.5.5 (.NET CLR 3.5.30729)");
        assert_eq!(headers.get("Host").unwrap(), "localhost");
        assert_eq!(headers.get("Content-Length").unwrap(), "43");
        assert_eq!(headers.len(), 11);
    }

    #[test]
    fn header_example() {
        let headers = Headers::from(String::from("
        Host: localhost
        User-Agent: Mozilla/5.0 (Windows; U; Windows NT 6.1; en-US; rv:1.9.1.5) Gecko/20091102 Firefox/3.5.5 (.NET CLR 3.5.30729)
        Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
        Accept-Language: en-us,en;q=0.5
        Accept-Encoding: gzip,deflate
        Accept-Charset: ISO-8859-1,utf-8;q=0.7,*;q=0.7
        Keep-Alive: 300
        Connection: keep-alive
        Referer: http://localhost/test.php
        Content-Type: application/x-www-form-urlencoded
        Content-Length: 43
        "));
        assert_eq!(headers.get("Host").unwrap(), "localhost");
        assert_eq!(headers.get("Content-Length").unwrap(), "43");
        assert_eq!(headers.len(), 11);
    }

    #[test]
    fn multiline_headers() {
        let headers = Headers::from(String::from("Accept: gif\n png\n satelites\n\r\n"));
        assert_eq!(headers.get("Accept").unwrap(), "gif png satelites");
    }

    #[test]
    #[should_panic(expected = r#""#)]
    fn an_invalid_protocol_panics() {
        let _request = Protocol::from("HTTPWHATWHAT");
    }

    #[test]
    fn trailing_whitespace_not_included_in_headers() -> Result<(), std::io::Error> {
        let vec: &[u8] = b"GET / HTTP/1.1\r\nheader: my header\n\r\n";
        let request = Request::new(Box::new(vec));
        for (header, value) in request.headers.iter() {
            assert!(!header.contains("\r\n") && !value.contains("\r\n"));
        }
        Ok(())
    }

    #[test]
    fn header_can_be_retrieved_by_name() {
        let vec: &[u8] = b"GET / HTTP/1.1\r\nauth: noyabusiness\n\r\n";
        let request = Request::new(Box::new(vec));
        match request.headers.get("auth") {
            Some(value) => assert_eq!(value, "noyabusiness"),
            _ => panic!("auth header key not found!")
        };
    }

}
