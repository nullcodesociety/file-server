use crate::prelude::*;

use std::future::Future;
use std::convert::Infallible;
use hyper::{Body, Request, Response, Server};
use hyper::server::conn::AddrIncoming;

use hyper::service::{make_service_fn, service_fn};

pub async fn start(conf: Config) -> Result<String, String> {

    // We have to clone to mke the conf avaiable within the make service closure
    // an then again per thread.
    let service_conf = conf.clone();

    let service = make_service_fn(move |_| {

        // Per thread clone here.
        let service_conf = service_conf.clone();
        async {
            Ok::<_, Infallible>(service_fn(move |raw_req| {
                handle_request(
                    service_conf.clone(), // fucking clone everything!!!
                    raw_req,
                )
            }))
        }
    });

    println!("Starting server at: {:?}", &conf.addr().to_string());

    let s = hyper::Server::bind(&conf.addr()).serve(service);

    match s.await {
        Ok(_) => Ok(String::from("")),
        Err(e) => Err(e.to_string())
    }
}

async fn handle_request(conf: Config, req: hyper::Request<hyper::Body>)
                        -> Result<hyper::Response<hyper::Body>, Infallible> {
    Ok(test_response())
}

fn test_response() -> hyper::Response<hyper::Body> {
    hyper::Response::new(hyper::Body::from("Hello World"))
}
