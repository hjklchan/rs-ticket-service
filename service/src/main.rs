use std::net::SocketAddr;

use abi::ticket_server::TicketServer;

mod db;
mod model;
mod server;

#[tokio::main]
async fn main() {
    let pool = db::connect().await;

    let service = server::TicketService::new(pool);

    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 50061));

    tonic::transport::Server::builder()
        .add_service(TicketServer::new(service))
        .serve(socket_addr)
        .await
        .unwrap();
}
