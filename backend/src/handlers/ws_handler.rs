use axum::{
    extract::{
        ws::{Message as AxumMessage, WebSocket, WebSocketUpgrade},
        Path, State, Query, // ✅ Added Query here
    },
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::TypedHeader;
use futures_util::{SinkExt, StreamExt};
use headers::{Header, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use uuid::Uuid;
use std::collections::HashMap; // ✅ Added HashMap here

use crate::{
    models::message::CreateMessageInput,
    services::collaboration_service::add_message_to_ticket,
    state::SharedState,
    utils::jwt::decode_token,
};

// ✅ Custom SecWebSocketProtocol header definition
#[derive(Debug, Clone)]
pub struct SecWebSocketProtocol(pub Vec<String>);

impl Header for SecWebSocketProtocol {
    fn name() -> &'static HeaderName {
        static NAME: HeaderName = HeaderName::from_static("sec-websocket-protocol");
        &NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let header_value = values
            .next()
            .ok_or_else(headers::Error::invalid)?
            .to_str()
            .map_err(|_| headers::Error::invalid())?;

        let protocols = header_value
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Ok(SecWebSocketProtocol(protocols))
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        if let Ok(value) = HeaderValue::from_str(&self.0.join(", ")) {
            values.extend(std::iter::once(value));
        }
    }
}

impl std::fmt::Display for SecWebSocketProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

#[derive(Debug, Deserialize)]
struct IncomingWsMessage {
    content: String,
    is_from_customer: bool,
}

#[derive(Debug, Serialize)]
struct OutgoingWsMessage {
    id: Uuid,
    ticket_id: Uuid,
    sender_id: Uuid,
    content: String,
    is_from_customer: bool,
    created_at: String,
}

// ✅ Original handler for subprotocol authentication
pub async fn handle_ws_upgrade(
    ws: WebSocketUpgrade,
    Path(ticket_id): Path<Uuid>,
    State(state): State<SharedState>,
    TypedHeader(protocol): TypedHeader<SecWebSocketProtocol>,
) -> impl IntoResponse {
    println!("🔐 Received Sec-WebSocket-Protocol: {:?}", protocol.0);

    let token_opt = protocol.0.first().cloned();

    match token_opt {
        Some(token) => match decode_token(&token, &state.config.jwt_secret) {
            Ok(token_data) => {
                let user_id = token_data.sub;
                println!("✅ JWT decoded for user: {:?}", user_id);

                ws.protocols([token.clone()])
                    .on_upgrade(move |socket| handle_socket(socket, ticket_id, user_id, state))
            }
            Err(err) => {
                eprintln!("❌ Invalid JWT: {:?}", err);
                StatusCode::UNAUTHORIZED.into_response()
            }
        },
        None => {
            eprintln!("❌ Missing Sec-WebSocket-Protocol header");
            StatusCode::BAD_REQUEST.into_response()
        }
    }
}

// ✅ New handler for query parameter authentication
// Enhanced version of your handle_ws_upgrade_query function with detailed logging

pub async fn handle_ws_upgrade_query(
    ws: WebSocketUpgrade,
    Path(ticket_id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    println!("🔐 WebSocket connection attempt for ticket: {}", ticket_id);
    println!("🗝️ Using JWT secret: '{}'", state.config.jwt_secret);
    println!("🎯 All query parameters: {:?}", params);

    // Try to get token from query parameter
    let token = match params.get("token") {
        Some(token) => {
            println!("✅ Token found in query params");
            println!("🔑 Token length: {}", token.len());
            println!("🔑 Token first 30 chars: {}", &token[..std::cmp::min(30, token.len())]);
            token.clone()
        }
        None => {
            eprintln!("❌ Missing token query parameter");
            eprintln!("📋 Available params: {:?}", params.keys().collect::<Vec<_>>());
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    println!("🔓 Attempting to decode JWT token...");
    match decode_token(&token, &state.config.jwt_secret) {
        Ok(token_data) => {
            let user_id = token_data.sub;
            println!("✅ JWT decoded successfully for user: {:?}", user_id);
            println!("📧 User email: {:?}", token_data.email);
            println!("👤 User role: {:?}", token_data.role);

            ws.on_upgrade(move |socket| handle_socket(socket, ticket_id, user_id, state))
        }
        Err(err) => {
            eprintln!("❌ JWT decoding failed: {:?}", err);
            eprintln!("🔑 Token that failed (first 50 chars): {}", &token[..std::cmp::min(50, token.len())]);
            eprintln!("🗝️ JWT secret being used: '{}'", state.config.jwt_secret);
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}
// ✅ Shared socket handling logic
async fn handle_socket(socket: WebSocket, ticket_id: Uuid, sender_id: Uuid, state: SharedState) {
    println!("🔗 WebSocket established: ticket = {ticket_id}, user = {sender_id}");

    let mut rx = {
        let mut channels = state.ws_channels.write().await;
        channels
            .entry(ticket_id)
            .or_insert_with(|| broadcast::channel::<String>(100).0)
            .subscribe()
    };

    let (mut sender, mut receiver) = socket.split();
    let db = state.db.clone();
    let state_clone = state.clone();

    let broadcast_tx = {
        let channels = state.ws_channels.read().await;
        channels.get(&ticket_id).unwrap().clone()
    };

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(AxumMessage::Text(msg.into())).await.is_err() {
                println!("🔌 Client disconnected while sending");
                break;
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(AxumMessage::Text(text))) = receiver.next().await {
            println!("📥 Incoming WS message: {:?}", text);

            if let Ok(incoming) = serde_json::from_str::<IncomingWsMessage>(&text) {
                let create_msg = CreateMessageInput {
                    content: incoming.content,
                    is_from_customer: incoming.is_from_customer,
                    channel: Some("web".to_string()),
                    in_reply_to: None,
                    subject: None,
                    attachment_ids: None,
                    message_id: None,
                    external_sender_email: None,
                    is_email: false,
                };

                match add_message_to_ticket(
                    &db,
                    ticket_id,
                    sender_id,
                    create_msg,
                    Some(state_clone.clone()),
                )
                .await
                {
                    Ok(saved) => {
                        let outgoing = OutgoingWsMessage {
                            id: saved.id,
                            ticket_id,
                            sender_id,
                            content: saved.content,
                            is_from_customer: saved.is_from_customer,
                            created_at: saved.created_at.to_rfc3339(),
                        };

                        if let Ok(json) = serde_json::to_string(&outgoing) {
                            let _ = broadcast_tx.send(json);
                        }
                    }
                    Err(err) => {
                        eprintln!("⚠️ Failed to save message: {:?}", err);
                    }
                }
            } else {
                eprintln!("⚠️ Invalid incoming WS message format");
            }
        }

        println!("📴 Client closed WebSocket");
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    println!("👋 WebSocket closed for ticket {}", ticket_id);
}