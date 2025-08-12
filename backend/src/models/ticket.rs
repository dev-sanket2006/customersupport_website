use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

/// Matches PostgreSQL enum `ticket_status`
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "ticket_status")]
#[serde(rename_all = "PascalCase")]
pub enum TicketStatus {
    Open,
    InProgress,
    Closed,
}

/// Matches PostgreSQL enum `ticket_priority`
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "ticket_priority")]
#[serde(rename_all = "PascalCase")]
pub enum TicketPriority {
    Low,
    Medium,
    High,
}

/// Ticket table mapping
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub subject: String,
    pub description: String,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub assigned_to: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub customer_email: Option<String>,
    pub user_id: Option<Uuid>, // User who created the ticket
}

/// Create ticket DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketInput {
    pub subject: String,
    pub description: String,
    pub priority: TicketPriority,
    pub customer_email: String,
}

/// Update ticket status DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTicketInput {
    pub status: TicketStatus,
}
