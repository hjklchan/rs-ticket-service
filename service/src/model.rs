use abi::TicketItem;
use sqlx::types::chrono;

#[derive(Debug, sqlx::FromRow)]
pub struct Ticket {
    pub id: String,
    pub assignee_id: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub status: u8,
    pub created_at: Option<chrono::DateTime<chrono::Local>>,
    pub updated_at: Option<chrono::DateTime<chrono::Local>>,
}

impl From<Ticket> for TicketItem {
    fn from(value: Ticket) -> Self {
        TicketItem {
            id: value.id,
            assignee_id: value.assignee_id,
            title: value.title,
            description: value.description,
            body: value.body,
            status: value.status.into(),
            created_at: None,
            updated_at: None,
        }
    }
}