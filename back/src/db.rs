use tokio::sync::broadcast;

use crate::channel::MessageTx;
use crate::message::{ChatMessage, SocketMessage};

pub struct MessageDb {
    messages: Vec<String>,
    tx: MessageTx,
}

impl MessageDb {
    pub fn new(tx: broadcast::Sender<SocketMessage>) -> Self {
        Self {
            messages: Vec::new(),
            tx: MessageTx::new(tx),
        }
    }

    pub fn add_message<T>(&mut self, msg: T)
    where
        T: TryInto<ChatMessage> + Clone + Into<SocketMessage>,
        <T as TryInto<ChatMessage>>::Error: std::fmt::Debug,
    {
        println!("this is add_message");
        let msg_chat: ChatMessage = msg.try_into().unwrap();
        let msg_str: String = msg_chat.into();
        self.messages.push(msg_str);
    }

    pub fn init_message(&self) -> Vec<String> {
        self.messages.clone()
    }

    pub fn tx(&self) -> MessageTx {
        self.tx.clone()
    }
}
