use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main] 
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port)).expect("Fialed to bind random port");
    println!("info: listening at {}", listener.local_addr().unwrap());
    run(listener)?.await
}


