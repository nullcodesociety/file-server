use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TYPES: HashMap<&'static str, &'static str> = HashMap::from([
    ("mp4", "video/mp4"),
    ("mpeg", "video/mpeg"),
    ("ogv", "video/ogg"),
    ("webm", "video/webm"),
    ]);
}