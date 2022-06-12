mod server;
mod request;
mod response;
mod status_code;

use std::env;

#[tokio::main]
async fn main() {

    match config::from_env_args(env::args()) {

        Err(e) => {
            println!("config error: {:?}", e.message);
            return;
        }

        Ok(conf) => {
            let s = server::start(conf);
            if let Err(e) = s.await {
                eprintln!("server error: {}", e);
            }
        }

    }

}
