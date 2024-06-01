use abi::TicketItem;
use sqlx::types::chrono;

#[derive(Debug, sqlx::FromRow)]
pub struct Ticket {
    pub id: u64,
    pub assignee_id: u64,
    pub title: String,
    pub description: String,
    pub body: String,
    // ? status should be u8 or else types?
    pub status: i32,
    pub created_at: Option<chrono::DateTime<chrono::Local>>,
    pub updated_at: Option<chrono::DateTime<chrono::Local>>,
}

/// implement From<Ticket> trait for TicketItem
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
