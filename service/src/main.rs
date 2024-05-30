use abi::ticket_server::TicketServer;
use server::TicketService;

mod db;
mod server;

#[tokio::main]
async fn main() {
    let pool = db::connect().await;

    let service = server::TicketService::new(pool);
    tonic::transport::Server::builder()
        .add_service(TicketServer::new(service))
        .serve("0.0.0.0:500561".parse().unwrap())
        .await
        .unwrap();

    println!("Hello, world!");
}
