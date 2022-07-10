use file_server::{
    config,
    server,
    resource
};

#[tokio::main]
async fn main() {

    server::start(
        config::Config::default()
    ).await;

}
