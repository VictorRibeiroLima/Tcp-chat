use std::{env::args, sync::Arc};

use async_std::prelude::*;
use async_std::{net, task};

use chat::types::ChatResult;
use chat_map::ChatMap;
use connection::handle;

mod chat_map;
mod chat_room;
mod connection;

fn main() -> ChatResult<()> {
    let addr = args()
        .nth(1)
        .expect("Expected address to be given. addr:PORT");

    let chat_map = Arc::new(ChatMap::new());

    task::block_on(async {
        let socket = net::TcpListener::bind(addr).await?;
        let mut client_connections = socket.incoming();
        while let Some(connection_result) = client_connections.next().await {
            let connection = connection_result?;
            let chats = chat_map.clone();
            task::spawn(async { log_error(handle(connection, chats).await) });
        }
        Ok(())
    })
}

fn log_error(result: ChatResult<()>) {
    if let Err(err) = result {
        println!("Error 2: {}", err)
    }
}
