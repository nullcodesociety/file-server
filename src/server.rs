use crate::config::{Config};
use crate::resource;

use std::future::Future;
use std::convert::Infallible;
use std::io;
use std::io::Error;
use std::path;
use std::path::PathBuf;

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::header::CONTENT_TYPE;
use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};


#[derive(Debug)]
enum ServerError {
    ResourcePath(io::Error),
    FileOpen(path::PathBuf),
    Response,
}


pub async fn start(config: Config) -> Result<String, String> {

    // We have to clone to make the conf available within the
    // make service closure and then again per thread.
    let service_conf = config.clone();

    let service = make_service_fn(move |_| {

        // Per thread clone here.
        let service_conf = service_conf.clone();
        async {
            Ok::<_, Infallible>(service_fn(move |raw_req| {
                handle_request(
                    service_conf.resource_root(),
                    raw_req,
                )
            }))
        }
    });

    println!("Starting server at: {:?}", &config.addr().to_string());
    println!("Serving resources from : {:?}", &config.resource_root());

    let s = hyper::Server::bind(&config.addr())
        .serve(service);

    match s.await {
        Ok(_) => Ok(String::from("")),
        Err(e) => Err(e.to_string())
    }
}


async fn handle_request(
    resource_root: path::PathBuf,
    request: hyper::Request<hyper::Body>,
) -> Result<hyper::Response<hyper::Body>, Infallible> {
    let request_path = path::PathBuf::from(request.uri().path());
    println!("Request: {:?}", request_path);

    Ok(generate_response(resource_root, request_path).await)
}


async fn generate_response(
    resource_root: path::PathBuf,
    request_path: path::PathBuf,
) -> Response<Body> {

    match file_response(
        resource_root.clone(),
        request_path,
        StatusCode::OK
    ).await {

        Ok(r) => {
            println!(" ↪ OK");
            r
        }
        Err(e) => {
            println!(" | Error");

            match file_response(
                resource_root.clone(),
                error_path(),
                StatusCode::NOT_FOUND
            ).await {

                Ok(r) => {
                    println!(" ↪ Handled");
                    r
                }

                Err(e) => {
                    println!(" ↪ Unhandled");
                    failure_response()
                }

            }
        }
    }
}


async fn file_response(
    resource_root: path::PathBuf,
    request_path: path::PathBuf,
    status_code: StatusCode
) -> Result<Response<Body>, ServerError> {
    let resource_path = match resource::path(
        resource_root,
        request_path,
    ) {
        Err(e) => return Err(ServerError::ResourcePath(e)),
        Ok(r) => r
    };

    match file_response_body(resource_path.clone()).await {
        Err(e) => return Err(e),
        Ok(body) => {
            let content_type = resource::content_type(&resource_path);
            let response = hyper::Response::builder()
                .status(status_code)
                .header(CONTENT_TYPE, content_type)
                .body(body);
            match response {
                Ok(body) => Ok(body),
                Err(_) => Err(ServerError::Response)
            }
        }
    }
}


async fn file_response_body(request_path: path::PathBuf) -> Result<Body, ServerError>
{
    match File::open(&request_path).await {
        Ok(file) => {
            let stream = FramedRead::new(file, BytesCodec::new());
            Ok(Body::wrap_stream(stream))
        }
        Err(e) => Err(ServerError::FileOpen(request_path))
    }
}


fn failure_response() -> Response<Body> {
    hyper::Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Server Error"))
        .unwrap()
    // Unwrap is ok here because this should be ok and is not dynamic
}


/// ```
/// use std::path;
/// use file_server::server::error_path;
/// assert_eq!(error_path(), path::PathBuf::from("/404.html"));
/// ```
pub fn error_path() -> path::PathBuf {
    let mut path = PathBuf::from("/");
    path.push("404");
    path.set_extension("html");
    path
}
