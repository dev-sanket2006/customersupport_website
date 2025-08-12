use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

use crate::{
    handlers::ticket_handler::{
        create_ticket,
        get_ticket_by_id,
        list_tickets,
        update_ticket,
        delete_ticket,
        admin_list_tickets,
        assign_ticket,
    },
    middleware::role_guard::require_roles,
    state::SharedState,
};

pub fn routes(state: SharedState) -> Router {
    let user_routes = Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route(
            "/tickets/{ticket_id}",
            get(get_ticket_by_id)
                .put(update_ticket)
                .delete(delete_ticket),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            |req, next| require_roles(req, next, &["user"]),
        ));

    let admin_routes = Router::new()
        .route("/admin/tickets", get(admin_list_tickets))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            |req, next| require_roles(req, next, &["admin"]),
        ));

    let assignment_routes = Router::new()
        .route("/tickets/assign/{ticket_id}/{agent_id}", put(assign_ticket))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            |req, next| require_roles(req, next, &["admin", "agent"]),
        ));

    Router::new()
        .merge(user_routes)
        .merge(admin_routes)
        .merge(assignment_routes)
        .with_state(state)
}
