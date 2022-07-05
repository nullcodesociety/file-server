mod config;
mod server;
// mod response;

#[tokio::main]
async fn main() {

    let server = server::start(
        config::Config::default()
    );

    match server.await {
        Ok(_) => println!("Server started"),
        Err(e) => eprintln!("server error: {}", e)
    }

}
