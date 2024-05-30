use abi::{
    ticket_server::Ticket as TicketServicer, CreateTicketRep, CreateTicketReq, DeleteTicketReq,
    GetTicketRep, GetTicketReq, GetTicketsRep, GetTicketsReq, HasTicketRep, HasTicketReq,
    UpdateTicketReq,
};
use sqlx::MySqlPool;
use std::{marker, sync::Arc};
use tonic::{Request, Response, Result, Status};
use uuid::Uuid;

pub struct TicketService {
    pool: MySqlPool,
}

impl TicketService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl TicketServicer for TicketService {
    /// Create a new ticket
    async fn create_ticket(
        &self,
        request: Request<CreateTicketReq>,
    ) -> Result<Response<CreateTicketRep>, Status> {
        let id = Uuid::new_v4().to_string();
        let CreateTicketReq {
            assignee_id,
            title,
            description,
            body,
            status,
        } = request.into_inner();

        let sql = r#"
            INSERT INTO `tickets` (`id`, `assignee_id`, `title`, `description`, `body`, `status`, `created_at`, `updated_at`)
            VALUES (?, ?, ?, ?, ?, ?, NOW(), NOW())
        "#;

        let _res = sqlx::query(sql)
            .bind(&id)
            .bind(assignee_id)
            .bind(title)
            .bind(description)
            .bind(body)
            .bind(status)
            .execute(&self.pool)
            .await
            .unwrap();

        Ok(Response::new(CreateTicketRep {
            new_id: id,
        }))
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
