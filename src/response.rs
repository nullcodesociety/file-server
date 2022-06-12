use std::path::PathBuf;

enum Response {
    200(PathBuf),
    None
}

impl Response {
    pub fn from_foo() -> Self {
        Response::None
    }
}

/*
impl Response {
    pub fn new(file_path: PathBuf, status_code: StatusCode) -> Self {
        Self {
            file_path,
            status_code,
        }
    }

    pub fn from_request(config: &Config, _req: &Request<Body>) -> Response {
            match get_pathbuff(&config.dir, &_req) {
                Ok(p) => {
                    match p.starts_with(&config.dir) {
                        // ^--  convert 'starts with to named function for
                        //      semantic meaning
                        true => Response::new(p, StatusCode::OK),
                        false => Response::new(config.filepath_403.clone(), StatusCode::FORBIDDEN)
                    }
                }
                Err(_) => Response::new(config.filepath_404.clone(), StatusCode::NOT_FOUND)
            }
        }

    }
}
*/
/*
async fn serve_file_response(response: FileResponse) -> Result<Response<Body>, std::io::Error> {
    let request_path = response.file_path;
    let status_code = response.status_code;

    match File::open(&request_path).await {
        Ok(file) => {
            let content_type = get_content_type(&request_path);
            let stream = FramedRead::new(file, BytesCodec::new());
            let body = Body::wrap_stream(stream);
            let response = Response::builder()
                .status(status_code)
                .header(CONTENT_TYPE, content_type)
                .body(body)
                .unwrap();

            Ok(response)
        }
        Err(e) => Err(e),
    }
}
 */