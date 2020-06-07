use std::net::TcpListener;
use retry::retry_with_index;
use retry::delay::Fixed;

fn main() {
    let result = retry_with_index(Fixed::from_millis(100), |index| {
        match TcpListener::bind("127.0.0.1:7878") {
            Ok(sock) => Ok(sock),
            Err(e) => Err(format!("Failed {} times: {}", index, e))
        }
    });

    let listener = result.unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
    }
}
