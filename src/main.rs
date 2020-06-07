use std::net::TcpListener;
use retry::retry_with_index;
use retry::delay::Fixed;

fn get_conn() -> Result<std::net::TcpListener, retry::Error<String>> {
    return retry_with_index(Fixed::from_millis(100), |index| {
        match TcpListener::bind("127.0.0.1:7878") {
            Ok(sock) => Ok(sock),
            Err(e) => Err(format!("Failed {} times: {}", index, e))
        }
    });
}

fn main() {
    let listener = get_conn().unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
    }
}
