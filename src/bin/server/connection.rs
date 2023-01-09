use std::{collections::HashMap, sync::Arc};

use async_std::{
    io::{BufReader, WriteExt},
    net::TcpStream,
    stream::StreamExt,
    sync::Mutex,
};
use chat::{types::ChatResult, utils, ClientAction, ServerAction};

use crate::{chat_map::ChatMap, chat_room::ChatRoom};

pub async fn handle(connection: TcpStream, chat_map: Arc<ChatMap>) -> ChatResult<()> {
    let client = Arc::new(Client::new(connection.clone()));
    let buffer = BufReader::new(connection);
    let mut messages_buffers = utils::receive_json(buffer);
    while let Some(action_result) = messages_buffers.next().await {
        let action = action_result?;
        let result = match action {
            ClientAction::Join { chat_name } => {
                let chat = chat_map.find_or_create(&chat_name);
                if client
                    .1
                    .lock()
                    .unwrap()
                    .insert(chat_name, chat.clone())
                    .is_none()
                {
                    chat.join(client.clone());
                }

                Ok(())
            }
            ClientAction::Leave { chat_name } => {
                let mut chats = client.1.lock().unwrap();
                let chat_option = chats.get(&chat_name);
                match chat_option {
                    Some(_) => {
                        chats.remove(&chat_name);
                        Ok(())
                    }
                    None => Err(format!("You are not in {}", &chat_name)),
                }
            }
            ClientAction::Post { chat_name, message } => {
                let chats = client.1.lock().unwrap();
                let chat_option = chats.get(&chat_name);
                match chat_option {
                    Some(chat) => {
                        chat.post(message);
                        Ok(())
                    }
                    None => Err(format!("You are not in {}", &chat_name)),
                }
            }
        };
        if let Err(error) = result {
            let report = ServerAction::Error(error);
            client.send(&report).await?;
        }
    }
    client.1.lock().unwrap().clear();
    Ok(())
}

pub struct Client(
    Mutex<TcpStream>,
    std::sync::Mutex<HashMap<String, Arc<ChatRoom>>>,
);

impl Client {
    pub fn new(connection: TcpStream) -> Client {
        Client(
            Mutex::new(connection),
            std::sync::Mutex::new(HashMap::new()),
        )
    }

    pub async fn send(&self, action: &ServerAction) -> ChatResult<()> {
        let mut connection = self.0.lock().await;
        utils::send_json(&mut *connection, &action).await?;
        connection.flush().await?;
        Ok(())
    }
}
