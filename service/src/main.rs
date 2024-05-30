use abi::ticket_server::TicketServer;

mod db;
mod server;

#[tokio::main]
async fn main() {
    let pool = db::connect().await;

    let service = server::TicketService::new(pool);
    tonic::transport::Server::builder()
        .add_service(TicketServer::new(service))
        .serve("0.0.0.0:50061".parse().unwrap())
        .await
        .unwrap();

    println!("Hello, world!");
}
