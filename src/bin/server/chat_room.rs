use std::sync::Arc;

use async_std::task::{self, JoinHandle};
use chat::ServerAction;
use tokio::sync::broadcast::{self, error::RecvError, Receiver};

use crate::connection::Client;

pub struct ChatRoom {
    name: Arc<String>,
    publisher: broadcast::Sender<String>,
}

impl ChatRoom {
    pub fn new(name: &str) -> ChatRoom {
        let (publisher, _) = broadcast::channel(1000);
        ChatRoom {
            name: Arc::new(String::from(name)),
            publisher,
        }
    }

    pub fn join(&self, client: Arc<Client>) -> JoinHandle<()> {
        let receiver = self.publisher.subscribe();
        let join_handler = task::spawn(subscribe(self.name.clone(), receiver, client));
        join_handler
    }

    pub fn post(&self, message: String) {
        let _ = self.publisher.send(message);
    }
}

async fn subscribe(chat_name: Arc<String>, mut receiver: Receiver<String>, client: Arc<Client>) {
    loop {
        let server_action = receiver.recv().await;
        if !client.is_in_room(&chat_name) {
            break;
        }
        let body = match server_action {
            Ok(message) => ServerAction::Message {
                chat_name: chat_name.clone().to_string(),
                message,
            },
            Err(RecvError::Lagged(n)) => {
                ServerAction::Error(format!("Dropped {} messages from {}.", n, &chat_name))
            }
            Err(RecvError::Closed) => break,
        };

        let client_result = client.send(&body).await;
        if client_result.is_err() {
            break;
        }
    }
}
