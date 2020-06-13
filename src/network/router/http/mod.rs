pub mod request;
pub use request::Request;
pub mod response;
pub use response::Response;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}