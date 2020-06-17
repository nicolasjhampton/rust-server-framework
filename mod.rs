pub mod path;
pub use path::Path;
pub mod query;
pub use query::Query;
pub mod uri;
pub use uri::URI;


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;

    #[test]
    fn a_path_can_be_printed() {
        let mut path = Path::from("");
        path.push("this".to_string());
        path.push("is".to_string());
        path.push("the".to_string());
        path.push("way".to_string());
        assert_eq!(format!("{}", path), "/this/is/the/way")
    }

    #[test]
    fn a_path_can_be_created_from_a_string() {
        let mut path = Path::from("/this/is/the/way");
        assert_eq!(format!("{}", path), "/this/is/the/way")
    }

    #[test]
    fn a_query_can_be_printed() {
        let mut query = Query::from("");
        query.insert("query".to_string(), "what".into());
        assert_eq!(format!("{}", query), "?query=what")
    }

    #[test]
    fn a_query_can_be_made_from_a_string() {
        let query = Query::from("query=what&if=then");
        assert_eq!(*query.get("query").unwrap(), String::from("what"));
        assert_eq!(*query.get("if").unwrap(), String::from("then"));
    }

    #[test]
    fn a_uri_can_be_printed() {
        let mut uri_string = String::from("scheme://authority/path0/path1/path2?query=what#fragment");
        let uri = URI::from(uri_string);
        assert_eq!(format!("{}", uri), "scheme://authority/path0/path1/path2?query=what#fragment")
    }

    #[test]
    fn a_uri_can_be_made_from_a_string() {
        let mut uri_string = String::from("scheme://authority/path0/path1/path2?query=what#fragment");
        let uri = URI::from(uri_string);
        assert_eq!(uri.scheme(), "scheme");
        assert_eq!(uri.authority(), "authority");
        for (num, path) in uri.path.iter().enumerate() {
            assert_eq!(path, &(format!("path{}", num)));
        }
        assert_eq!(uri.query.get("query").unwrap(), "what");
        assert_eq!(uri.fragment(), "fragment");
    }

}
