use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TYPES: HashMap<&'static str, &'static str> = HashMap::from([
    ("aac", "audio/aac"),
    ("midi", "audio/midi"),
    ("flac", "audio/flac"),
    ("mp3", "audio/mpeg"),
    ("oga", "audio/ogg"),
    ("wav", "audio/wav"),
    ("weba", "audio/webm"),
    ]);
}