use file_server::{config, resource, server};

#[tokio::main]
async fn main() {
    server::start(config::Config::default()).await;
}
