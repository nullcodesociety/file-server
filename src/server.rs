use crate::config::{Config};

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

    // The result has to always be ok so we're going to move the
    // actually response generator to avoid redundant Result wrapping
    Ok(generate_response(resource_root, request).await)
}

async fn generate_response(
    resource_root: path::PathBuf,
    request: hyper::Request<hyper::Body>,
) -> Response<Body> {

    let request_path = path::PathBuf::from( request.uri().path() );
    println!("Request: {:?}", request_path);

    if let Ok(requested_resource_path) = crate::resource::path(
        resource_root.clone(),
        request_path.clone(),
    ) {
        if let Ok(desired_response) = file_response(requested_resource_path).await {
            println!("  OK  ");
            return desired_response;
        }
    }

    let error_path = error_path("404");
    if let Ok( error_resource_path )  = crate::resource::path(
        resource_root,
        error_path
    ) {
        if let Ok(error_response) = file_response(error_resource_path).await {
            println!("  HANDLED ERROR  ");
            return error_response;
        }
    }

    println!("\
        Unhandled error: \n\
         -> Dead End",
    );
    server_error_response()
}

async fn file_response(requested_resource_path: path::PathBuf) -> Result<Response<Body>, Error> {
    //@todo get the content type and body
    //@todo inject status code properly
    match file_response_body(requested_resource_path).await {
        Ok(response_body) => {
            let r = prepare_response(StatusCode::OK, "TXT", response_body).await;
            Ok(r)
        },
        Err(e) => Err(e)
    }
}


fn error_path(error: &str) -> path::PathBuf {
    let mut path = PathBuf::from("error");
    path.with_file_name(error);
    path.set_extension("html");
    path
}

async fn prepare_response(
    status_code: StatusCode,
    content_type: &str,
    body: Body,
) -> Response<Body> {
    hyper::Response::builder()
        .status(status_code)
        .header(CONTENT_TYPE, content_type)
        .body(body)
        .unwrap()
}

async fn file_response_body(request_path: path::PathBuf) -> Result<Body, Error>
{
    match File::open(&request_path).await {
        Ok(file) => {
            let content_type = crate::resource::content_type(&request_path);
            let stream = FramedRead::new(file, BytesCodec::new());
            Ok(Body::wrap_stream(stream))
        }
        Err(e) => {
            println!("{:?}", e);
            Err(e)
        }
    }
}


fn server_error_response() -> Response<Body> {
    Response::new(Body::from("Server Error"))
}
