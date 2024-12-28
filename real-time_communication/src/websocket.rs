use actix::prelude::*;
use tokio_tungstenite::tungstenite::protocol::Message as WebSocketMessage;
use std::sync::{Arc, Mutex};

pub struct WsMessage(pub WebSocketMessage);

impl Message for WsMessage {
    type Result = ();
}

pub struct WsManager {
    clients: Arc<Mutex<Vec<Recipient<WsMessage>>>>,
}

impl WsManager {
    pub fn new() -> Self {
        WsManager {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_client(&self, client: Recipient<WsMessage>) {
        let mut clients = self.clients.lock().unwrap();
        clients.push(client);
    }

    pub fn broadcast_message(&self, message: &str) {
        let clients = self.clients.lock().unwrap();
        for client in clients.iter() {
            let _ = client.do_send(WsMessage(WebSocketMessage::Text(message.to_string())));
        }
    }
}