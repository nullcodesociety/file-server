use std::path;
use lazy_static::lazy_static;
use std::collections::HashMap;

// File types
pub mod audio;
pub mod compressed;
pub mod font;
pub mod image;
pub mod streaming;
pub mod video;
pub mod text;

// Directory/listing types
pub mod directory;

// Future optimizations include providing earlier matches for
// the most commonly requested files. Maybe that's splitting hairs
// though. Do a benchmark. :D

/// ```
/// use std::path;
/// use file_server::resource::{content_type, text};
///
/// assert_eq!(
///     content_type(&path::PathBuf::from("foo.html")),
///     "text/html; charset=utf-8"
/// );
/// assert_eq!(
///     content_type(&path::PathBuf::from("unknown.zzz")),
///     text::DEFAULT_CONTENT_TYPE
/// );
/// ```
pub fn content_type(request_path: &path::PathBuf) -> &str {
    let ext = extension(&request_path);
    if let Some(ct) = audio::TYPES.get(ext) {
        return ct;
    }
    if let Some(ct) = compressed::TYPES.get(ext) {
        return ct;
    }
    if let Some(ct) = font::TYPES.get(ext) {
        return ct;
    }
    if let Some(ct) = image::TYPES.get(ext) {
        return ct;
    }
    if let Some(ct) = streaming::TYPES.get(ext) {
        return ct;
    }
    if let Some(ct) = video::TYPES.get(ext) {
        return ct;
    }
    if let Some(ct) = text::TYPES.get(ext) {
        return ct;
    }
    text::DEFAULT_CONTENT_TYPE
}

/// ```
/// use std::path;
/// use file_server::resource::{extension, text};
///
/// assert_eq!(
///     extension(&path::PathBuf::from("foo.html")),
///     "html"
/// );
///
/// assert_eq!(
///     extension(&path::PathBuf::from("foo.unknown")),
///     "unknown"
/// );
///
/// assert_eq!(
///     extension(&path::PathBuf::from("missing-ext")),
///     text::DEFAULT_EXT
/// );
/// ```
pub fn extension(request_path: &path::PathBuf) -> &str {
    match request_path.extension() {
        Some(ext) => {
            match ext.to_str() {
                Some(e) => e,
                None => text::DEFAULT_EXT,
            }
        }
        None => text::DEFAULT_EXT,
    }
}