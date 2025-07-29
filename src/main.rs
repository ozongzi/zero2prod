use std::net::TcpListener;
use zero2prod::run;


#[tokio::main] 
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Fialed to bind random port");
    println!("info: listening at {}", listener.local_addr().unwrap());
    run(listener)?.await
}


