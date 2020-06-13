use network;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let mut server = network::Server::new();

    server.run(|mut router| {

        router.get("/dream", |_req, res| {
            thread::sleep(Duration::from_secs(5));
            let contents = fs::read_to_string("sleep.html").unwrap();
            res.send_200(contents);
        });

        router.get("/", |_req, res| {
            let contents = fs::read_to_string("index.html").unwrap();
            res.send_200(contents);
        });

        // router.get("/", Mid::auth( Mid::log( |_req, res| {
        //     let contents = fs::read_to_string("index.html").unwrap();
        //     res.writer.send_200(contents);
        //     None
        // })));

        // use crate::routes::users_router;
        // router.mid("/", vec![Mid::auth, Mid::log]);
        // router.get("/", |_req, res| {
        //     let contents = fs::read_to_string("index.html").unwrap();
        //     res.send_200(contents);
        // });
        // router.mount("/users", users_router);




        router.hole(|_req, res| {
            res.send_404();
        });

    });
}
