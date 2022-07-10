use lazy_static::lazy_static;
use std::path;

lazy_static! {
    pub static ref INDEX: path::PathBuf = {
        let mut i = path::PathBuf::from("index");
        i.set_extension("html");
        i
    };
}
