use std::io::{BufWriter, Write};
use super::Status;

pub struct Response<W: Write> {
    pub writer: BufWriter<Box<W>>,
    status: Option<Status>
}

impl<W: Write> Response<W> {
    pub fn new(stream: Box<W>) -> Response<W> {
        let writer = BufWriter::new(stream);
        Response {
            writer,
            status: None
        }
    }

    fn get_status(&self) -> Status {
        match self.status {
            Some(status) => status,
            None => Status::SUCCESS
        }
    }

    pub fn send_response(&mut self, response: String) {
        write!(self.writer, "{}", response).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn send_200(&mut self, html: String) {
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}\r\n", html);
        println!("Response:\r\n\r\n{}", response);
        self.send_response(response);
    }

    pub fn send_404(&mut self) {
        let response = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
        println!("Response:\r\n\r\n{}", response);
        self.send_response(response);
    }
}

// impl fmt::Display for Response {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

//     }
// }
