use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::env;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio_util::codec::{BytesCodec, FramedRead};
use tokio::fs::File;


static FORBIDDEN: &[u8] = b"Forbidden";
static NOTFOUND: &[u8] = b"Not Found";
static INTERNAL_SERVER_ERROR: &[u8] = b"Internal server error";
static INDEX: &str = "index";
static HTML: &str = "html";
static STATIC_FILES: &str = "static-files";


// TODO:
//
// - TLS certificate
// - command line args
// - json file {port, static-file-dir, tls-certificate}
// - get content-type from extension
// - add to response header


fn err_response(status_code: &StatusCode, body: &'static[u8]) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(body.into())
        .unwrap()
}

fn adjust_path_and_create_pathbuff(first_dir: &str, request_path: &str) -> Result<PathBuf, io::Error> {
    let path = first_dir.to_string() + request_path;

    let mut request_path = PathBuf::from(path).canonicalize()?;
    if request_path.is_dir() {
        request_path.push(INDEX);
        request_path.set_extension(HTML);
    }

    Ok(request_path)
}

fn compare_pathbuffs(source_path: &PathBuf, request_path: &PathBuf) -> bool {
    request_path.starts_with(source_path)
}

async fn simple_file_send(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // paramaterized
    let source_path = match env::current_dir() {
        Ok(curr_dir) => curr_dir,
        Err(_) => return Ok(err_response(&StatusCode::INTERNAL_SERVER_ERROR, INTERNAL_SERVER_ERROR)),
    };

    let request_path = match adjust_path_and_create_pathbuff(STATIC_FILES, _req.uri().path()) {
        Ok(request_path) => request_path,
        Err(_) => return Ok(err_response(&StatusCode::NOT_FOUND, NOTFOUND)),
    };

    if !compare_pathbuffs(&source_path, &request_path) {
        return Ok(err_response(&StatusCode::FORBIDDEN, FORBIDDEN));
    }



    if let Ok(file) = File::open(request_path).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(err_response(&StatusCode::NOT_FOUND, NOTFOUND))
}


#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(|_req| async {
            simple_file_send(_req).await
        }))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
