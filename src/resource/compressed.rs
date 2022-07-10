use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TYPES: HashMap<&'static str, &'static str> =
        HashMap::from([("gz", "application/gzip"), ("zip", "application/zip"),]);
}
