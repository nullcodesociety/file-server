use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TYPES: HashMap<&'static str, &'static str> = HashMap::from([
        ("otf", "font/otf"),
        ("ttf", "font/ttf"),
        ("woff", "font/woff"),
        ("woff2", "font/woff2"),
    ]);
}
