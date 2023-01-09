use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::chat_room::ChatRoom;

pub struct ChatMap(Mutex<HashMap<String, Arc<ChatRoom>>>);

impl ChatMap {
    pub fn new() -> ChatMap {
        ChatMap(Mutex::new(HashMap::new()))
    }

    pub fn find_or_create(&self, name: &str) -> Arc<ChatRoom> {
        self.0
            .lock()
            .expect("Expected mutex to be available")
            .entry(String::from(name))
            .or_insert_with(|| Arc::new(ChatRoom::new(name)))
            .clone()
    }
}
