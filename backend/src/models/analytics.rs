use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TicketStats {
    pub total: i64,
    pub open: i64,
    pub in_progress: i64,
    pub resolved: i64,
    pub closed: i64,
}

#[derive(Debug, Serialize)]
pub struct AgentActivity {
    pub agent_id: String,
    pub agent_name: String,
    pub tickets_handled: i64,
    pub avg_response_time_minutes: Option<f64>,
}
