mod network;
mod utils;
use std::fs;
use std::thread;
use std::time::Duration;
use crate::network::stream::TcpWriter;

fn main() {
    let mut server = network::Server::new();

    server.run(|mut router| {

        router.get("/dream", |_req, res| {
            thread::sleep(Duration::from_secs(5));
            let contents = fs::read_to_string("sleep.html").unwrap();
            res.writer.send_200(contents);
        });

        router.get("/", |_req, res| {
            let contents = fs::read_to_string("index.html").unwrap();
            res.writer.send_200(contents);
        });


        router.hole(|_req, res| {
            res.writer.send_404();
        });

    });
}
