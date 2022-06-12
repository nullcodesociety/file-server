// Create entry point in the crate for modules
mod config;
mod server;
mod request;
mod response;
mod resource;

mod prelude {
    // Declare module components as usable in the prelude for inclusion
    // through `use prelude::*` for convenience.
    pub use crate::config::*;
    pub use crate::server::*;
    pub use crate::request::*;
    pub use crate::response::*;
    pub use crate::resource::*;


}

use prelude::*;
use std::env;

#[tokio::main]
async fn main() {

    match config::from(env::args()) {

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
