mod network;
mod utils;
use std::fs;
use network::TcpStreamExt;

fn main() {
    let mut server = network::Server::new();

    server.run(|mut router| {
        // let mut router = Router::new(tcp); //create router wrapper around stream to store route and success

        router.get("/dream", |stream| {
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


