use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::ticket::{TicketPriority, TicketStatus};

/// DTO for creating a ticket
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateTicketRequest {
    #[validate(length(min = 3, message = "Subject must be at least 3 characters"))]
    pub subject: String,

    #[validate(length(max = 500, message = "Description can be max 500 characters"))]
    pub description: Option<String>,

    pub priority: Option<TicketPriority>, // Uses enum

    #[validate(email)]
    pub customer_email: String,
}

/// DTO for updating a ticket
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTicketRequest {
    #[validate(length(min = 3, message = "Subject must be at least 3 characters"))]
    pub subject: Option<String>,

    #[validate(length(max = 500, message = "Description can be max 500 characters"))]
    pub description: Option<String>,

    pub status: Option<TicketStatus>,     // Enum: Open, InProgress, Closed
    pub priority: Option<TicketPriority>, // Enum: Low, Medium, High

    pub assigned_to: Option<Uuid>,
}
