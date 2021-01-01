use my_cms::run;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("127.0.0.1:8000").expect("Can't bind to localhost:8000"))?.await
}
