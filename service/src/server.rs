use abi::{
    ticket_server::Ticket as TicketServicer, CreateTicketRep, CreateTicketReq, DeleteTicketReq, GetTicketRep, GetTicketReq, GetTicketsRep, GetTicketsReq, HasTicketRep, HasTicketReq, UpdateTicketReq
};
use sqlx::MySqlPool;
use std::sync::Arc;
use tonic::{Request, Response, Result, Status};

pub struct TicketService {
    pool: Arc<MySqlPool>,
}

impl TicketService {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
}

#[tonic::async_trait]
impl TicketServicer for TicketService {
    /// Create a new ticket
    async fn create_ticket(
        &self,
        request: Request<CreateTicketReq>,
    ) -> Result<Response<CreateTicketRep>, Status> {
        todo!()
    }

    /// Delete a ticket
    async fn delete_ticket(
        &self,
        request: Request<DeleteTicketReq>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    /// Update a ticket
    async fn update_ticket(
        &self,
        request: Request<UpdateTicketReq>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    /// Get a ticket
    async fn get_ticket(
        &self,
        request: Request<GetTicketReq>,
    ) -> Result<Response<GetTicketRep>, Status> {
        todo!()
    }

    /// Get ticket list
    async fn get_tickets(
        &self,
        request: Request<GetTicketsReq>,
    ) -> Result<Response<GetTicketsRep>, Status> {
        todo!()
    }

    async fn has_ticket(
        &self,
        request: Request<HasTicketReq>,
    ) -> Result<Response<HasTicketRep>, Status> {
        todo!()
    }
}
