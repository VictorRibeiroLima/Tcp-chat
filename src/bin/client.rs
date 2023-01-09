use std::env::args;

use async_std::{io, net};
use async_std::{prelude::*, task};
use chat::types::ChatResult;
use chat::ClientAction;
use chat::{utils::*, ServerAction};

fn main() -> ChatResult<()> {
    let addr = args()
        .nth(1)
        .expect("Expected address to be given. addr:PORT");

    println!("Options:");
    println!("join CHAT");
    println!("leave CHAT");
    println!("post CHAT MESSAGE");
    println!("Type CTRL-D on Unix or CTRL-Z on Windows to close the connection");
    task::block_on(async {
        let connection = net::TcpStream::connect(addr).await?;
        let _ = connection.nodelay();
        let send_future = sender_loop(connection.clone());
        let receiver_future = receiver_loop(&connection);
        let _ = send_future.race(receiver_future).await?;
        Ok(())
    })
}

async fn sender_loop(mut connection: net::TcpStream) -> ChatResult<()> {
    let mut user_input = io::BufReader::new(io::stdin()).lines();
    while let Some(input_result) = user_input.next().await {
        let input = input_result?;
        let action = match get_action(&input) {
            Some(action) => action,
            None => {
                println!("Unrecognized input {}", input);
                continue;
            }
        };
        send_json(&mut connection, &action).await?;
        connection.flush().await?;
    }
    return Ok(());
}

async fn receiver_loop(connection: &net::TcpStream) -> ChatResult<()> {
    let buff = io::BufReader::new(connection);
    let mut stream = receive_json(buff);

    while let Some(msg) = stream.next().await {
        match msg? {
            ServerAction::Message { chat_name, message } => {
                println!("Chat message {}:", &chat_name);
                println!("  {}", &message);
            }
            ServerAction::Error(msg) => {
                println!("Error: {}", &msg)
            }
        }
    }
    Ok(())
}

fn get_action(input: &str) -> Option<ClientAction> {
    let (action, remainder) = parse_input(input)?;
    match action.to_uppercase().as_str() {
        "JOIN" => Some(ClientAction::Join {
            chat_name: String::from(remainder),
        }),
        "POST" => {
            let (chat_name, message) = parse_input(remainder)?;
            if message.is_empty() {
                return None;
            }
            Some(ClientAction::Post {
                chat_name: String::from(chat_name),
                message: String::from(message),
            })
        }
        "LEAVE" => Some(ClientAction::Leave {
            chat_name: String::from(remainder),
        }),
        _ => None,
    }
}

fn parse_input(input: &str) -> Option<(&str, &str)> {
    let trim_input = input.trim_start();
    if trim_input.is_empty() {
        return None;
    }

    match trim_input.find(char::is_whitespace) {
        Some(whitespace) => {
            let beginning = &input[0..whitespace];
            let end = &input[whitespace + 1..];
            return Some((beginning, end));
        }
        None => Some((input, "")),
    }
}
