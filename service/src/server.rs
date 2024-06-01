use crate::model::Ticket;
use abi::{
    ticket_server::Ticket as TicketServicer, CreateTicketRep, CreateTicketReq, DeleteTicketReq,
    GetTicketRep, GetTicketReq, GetTicketsRep, GetTicketsReq, HasTicketRep, HasTicketReq,
    TicketItem, UpdateTicketReq,
};
use sqlx::{MySql, MySqlPool};
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
        let CreateTicketReq {
            assignee_id,
            title,
            description,
            body,
            status,
        } = request.into_inner();

        let sql = r#"
            INSERT INTO `tickets` (`assignee_id`, `title`, `description`, `body`, `status`, `created_at`, `updated_at`)
            VALUES (?, ?, ?, ?, ?, NOW(), NOW())
        "#;

        let last_insert_id = sqlx::query(sql)
            .bind(assignee_id)
            .bind(title)
            .bind(description)
            .bind(body)
            .bind(status)
            .execute(&*self.pool)
            .await
            .map(|result| result.last_insert_id()) // Return ()
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(CreateTicketRep {
            new_id: last_insert_id,
        }))
    }

    /// Delete a ticket
    async fn delete_ticket(
        &self,
        request: Request<DeleteTicketReq>,
    ) -> Result<Response<()>, Status> {
        let id = request.into_inner().id;

        // Delete record
        let _result = sqlx::query("DELETE FROM `tickets` WHERE `id` = ?")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map(|_unused_result| ())
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(()))
    }

    /// Update a ticket
    async fn update_ticket(
        &self,
        request: Request<UpdateTicketReq>,
    ) -> Result<Response<()>, Status> {
        let UpdateTicketReq {
            id: _,
            assignee_id,
            title,
            description,
            body,
            status,
        } = request.into_inner();

        let sql = r#"UPDATE `tickets` SET `assignee_id` = ?, `title` = ?, `description` = ?, `body` = ?, `status` = ?, `updated_at` = NOW() WHERE `id` = ?"#;
        sqlx::query(sql)
            .bind(assignee_id)
            .bind(title)
            .bind(description)
            .bind(body)
            .bind(status)
            .execute(&*self.pool)
            .await
            .map(|_unused_result| ())
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(()))
    }

    /// Get a ticket
    async fn get_ticket(
        &self,
        request: Request<GetTicketReq>,
    ) -> Result<Response<GetTicketRep>, Status> {
        let id = request.into_inner().id;

        let sql = r#"SELECT `id`, `assignee_id`, `title`, `description`, `body`, `status`, `created_at`, `updated_at` FROM `tickets` WHERE `id` = ? LIMIT 1"#;

        let ticket_option: Option<Ticket> = sqlx::query_as(sql)
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|err| {
                println!("{:?}", err);
                Status::internal(err.to_string())
            })?;

        match ticket_option {
            // If has record
            Some(data) => Ok(Response::new(GetTicketRep {
                id: data.id,
                assignee_id: data.assignee_id,
                title: data.title,
                description: data.description,
                body: data.body,
                status: data.status as i32,
            })),
            // If record does not exist?
            None => Err(Status::not_found("Ticket not found")),
        }
    }

    /// Get ticket list
    async fn get_tickets(
        &self,
        _request: Request<GetTicketsReq>,
    ) -> Result<Response<GetTicketsRep>, Status> {
        let GetTicketsReq { status } = _request.into_inner();

        let sql = "SELECT `id`, `assignee_id`, `title`, `description`, `body`, `status`, `created_at`, `updated_at` FROM `tickets` WHERE 1 = 1";

        let mut builder: sqlx::QueryBuilder<MySql> = sqlx::QueryBuilder::new(sql);
        if let Some(v) = status {
            builder.push(" AND `status` = ").push_bind(v);
        }

        let tickets = builder.build_query_as()
            .fetch_all(&*self.pool)
            .await
            .map(|records: Vec<Ticket>| {
                let mut ticket_items: Vec<TicketItem> = Vec::with_capacity(4);
                for record in records.into_iter() {
                    ticket_items.push(record.into());
                }
                ticket_items
            })
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(GetTicketsRep { tickets }))
    }

    async fn has_ticket(
        &self,
        request: Request<HasTicketReq>,
    ) -> Result<Response<HasTicketRep>, Status> {
        let id = request.into_inner().id;

        let sql = "SELECT 1 FROM `tickets` WHERE `id` = ? LIMIT 1";
        let result = sqlx::query(sql)
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(HasTicketRep {
            value: match result {
                Some(_) => true,
                None => false,
            },
        }))
    }
}
