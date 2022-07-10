use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TYPES: HashMap<&'static str, &'static str> = HashMap::from([
        ("bmp", "image/bmp"),
        ("gif", "image/gif"),
        ("ico", "image/vnd.microsoft.icon"),
        ("jpeg", "image/jpeg"),
        ("jpg", "image/jpeg"),
        ("pdf", "application/pdf"),
        ("png", "image/png"),
        ("svg", "image/svg+xml"),
        ("tiff", "image/tiff"),
        ("webp", "image/webp"),
    ]);
}
