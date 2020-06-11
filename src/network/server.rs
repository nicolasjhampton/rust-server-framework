use retry::delay::Fixed;
use retry::retry_with_index;
use std::net::TcpListener;
use crate::network::Router;
use crate::utils::ThreadPool;

pub struct Server {
    pool: ThreadPool,
    listener: TcpListener
}

impl Server {
    fn get_conn() -> Result<std::net::TcpListener, retry::Error<String>> {
        return retry_with_index(Fixed::from_millis(100), |index| {
            match TcpListener::bind("127.0.0.1:7878") {
                Ok(sock) => Ok(sock),
                Err(e) => Err(format!("Failed {} times: {}", index, e)),
            }
        });
    }

    pub fn new() -> Server {
        let pool = ThreadPool::new(4);
        let listener = Server::get_conn().unwrap();
        Server {
            pool: pool,
            listener: listener
        }
    }

    pub fn run<S: 'static>(&mut self, server_logic: S) 
    where
        S: FnOnce(Router) + Sync + Send + Copy
    {
        for stream in self.listener.incoming() {
            let tcp = stream.expect("Error binding");
            self.pool.execute(tcp, move |router| {
                server_logic(router);
            });
        }
    }
}
