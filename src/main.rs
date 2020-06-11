mod network;
mod utils;
use std::fs;
use network::TcpStreamExt;
use std::thread;
use std::time::Duration;

fn main() {
    let mut server = network::Server::new();

    server.run(|mut router| {

        router.get("/dream", |stream| {
            thread::sleep(Duration::from_secs(5));
            let contents = fs::read_to_string("sleep.html").unwrap();
            stream.send_200(contents);
        });

        router.get("/", |stream| {
            let contents = fs::read_to_string("index.html").unwrap();
            stream.send_200(contents);
        });


        router.hole(|stream| {
            stream.send_404();
        });

    });
}
