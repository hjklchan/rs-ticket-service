use std::net::SocketAddr;

use abi::ticket_server::TicketServer;
use tonic::{metadata::MetadataValue, Request, Status};

mod db;
mod model;
mod server;

#[tokio::main]
async fn main() {
    let pool = db::connect().await;

    let service = server::TicketService::new(pool);

    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 50061));

    tonic::transport::Server::builder()
        .add_service(TicketServer::with_interceptor(service, check_auth))
        .serve(socket_addr)
        .await
        .unwrap();
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(t) if t == "Bearer xxx" => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}
