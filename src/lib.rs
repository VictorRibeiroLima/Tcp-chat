pub mod types;
pub mod utils;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ClientAction {
    Join { chat_name: String },
    Leave { chat_name: String },
    Post { chat_name: String, message: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ServerAction {
    Message { chat_name: String, message: String },
    Error(String),
}

#[cfg(test)]
mod tests {
    use crate::{ClientAction, ServerAction};

    #[test]
    fn test_client_actions() {
        let action = ClientAction::Post {
            chat_name: String::from("Test"),
            message: String::from("Message for test on client"),
        };

        let json = serde_json::to_string(&action).expect("Expected client to be serialized");

        assert_eq!(
            json,
            r#"{"Post":{"chat_name":"Test","message":"Message for test on client"}}"#
        );

        assert_eq!(
            serde_json::from_str::<ClientAction>(&json).expect("Expected json to me deserialized"),
            action
        );
    }

    #[test]
    fn test_server_actions() {
        let action = ServerAction::Message {
            chat_name: String::from("Test"),
            message: String::from("Message for test on client"),
        };

        let json = serde_json::to_string(&action).expect("Expected client to be serialized");

        assert_eq!(
            json,
            r#"{"Message":{"chat_name":"Test","message":"Message for test on client"}}"#
        );

        assert_eq!(
            serde_json::from_str::<ServerAction>(&json).expect("Expected json to me deserialized"),
            action
        );
    }
}
