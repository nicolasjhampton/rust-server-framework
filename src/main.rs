mod network;
use std::fs;
use network::TcpStreamExt;

fn main() {
    let mut server = network::Server::new();

    server.run(|router| {
        let mut found = false;
        found = found || router.get("/", |stream| {
            let contents = fs::read_to_string("index.html").unwrap();
            stream.send_response(contents);
        });

        found = found || router.get("/dream", |stream| {
            let contents = fs::read_to_string("sleep.html").unwrap();
            stream.send_response(contents);
        });

        if !found {
            router.hole(|stream| {
                stream.send_404();
            });
        }
    });
}


