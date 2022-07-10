use lazy_static::lazy_static;
use std::collections::HashMap;

pub const DEFAULT_EXT: &'static str = "txt";
pub const DEFAULT_CONTENT_TYPE: &'static str = "text/plain; charset=utf-8";

lazy_static! {
    pub static ref TYPES: HashMap<&'static str, &'static str> = HashMap::from([
        ("css", "text/css"),
        ("csv", "text/csv"),
        ("html", "text/html; charset=utf-8"),
        ("js", "text/javascript"),
        ("json", "application/json"),
        (DEFAULT_EXT, DEFAULT_CONTENT_TYPE),
        ("xml", "application/xml"),
    ]);
}
