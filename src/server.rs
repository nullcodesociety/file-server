use crate::config::Config;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::convert::Infallible;
use std::future::Future;
use std::path::PathBuf;

use hyper::service::{make_service_fn, service_fn};

use file_request;

pub fn start(conf: Config) {
    let service = make_service_fn(|_| {

        // Clone the config so that it is only parsed once
        // but can be used in each handler safely
        let conf = conf.clone();

        async {
            Ok::<_, Infallible>(service_fn(move |raw_req| {
                handle_request(
                    conf,
                    raw_req
                )
            }))
        }
    });

    let addr = SocketAddr::new(conf.ip, conf.port);

    hyper::Server::bind(&addr).serve(service);
}


fn handle_request<T>(conf: Config, req: hyper::Request<T>) {

    // Get a file request from the config and requested data

    // If we can get the file and we can load it ok

    // Provide a server error

    let handled_response = response(&config, &_req);

    if let Ok(response) = serve_file_response( handled_response ).await {
        return Ok(response);
    }

    let fallback_response = server_error_response(&config);

    if let Ok(response) = serve_file_response( fallback_response ).await {
        return Ok(response)
    }

    Ok(error_response())
}

