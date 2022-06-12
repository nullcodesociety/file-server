#[derive(Debug, Clone)]
struct FileResponse {
    file_path: PathBuf,
    status_code: StatusCode,
}

impl FileResponse {
    pub fn new(file_path: PathBuf, status_code: StatusCode) -> Self {
        Self {
            file_path,
            status_code,
        }
    }
}

fn response(config: &Config, _req: &Request<Body>) -> FileResponse {
    match get_pathbuff(&config.dir, &_req) {
        Ok(p) => {
            match p.starts_with(&config.dir) {
                // ^--  convert 'starts with to named function for
                //      semantic meaning
                true => FileResponse::new(p, StatusCode::OK),
                false => FileResponse::new(config.filepath_403.clone(), StatusCode::FORBIDDEN)
            }
        }
        Err(_) => FileResponse::new(config.filepath_404.clone(), StatusCode::NOT_FOUND)
    }
}
