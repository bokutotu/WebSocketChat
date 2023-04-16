use tokio::sync::broadcast;

use crate::message::WebSocketMessageType;

#[derive(Debug, Clone)]
pub struct MessageTx(broadcast::Sender<WebSocketMessageType>);
pub struct MessageRx(broadcast::Receiver<WebSocketMessageType>);

impl MessageTx {
    pub fn new(tx: broadcast::Sender<WebSocketMessageType>) -> Self {
        Self(tx)
    }

    pub fn subscribe(&self) -> MessageRx {
        MessageRx(self.0.subscribe())
    }

    pub fn send<I: Into<WebSocketMessageType>>(
        &self,
        msg: I,
    ) -> Result<usize, broadcast::error::SendError<WebSocketMessageType>> {
        let msg = msg.into();
        self.0.send(msg)
    }
}

impl MessageRx {
    pub async fn recv(&mut self) -> Result<WebSocketMessageType, broadcast::error::RecvError> {
        self.0.recv().await
    }
}

pub fn channel() -> (MessageTx, MessageRx) {
    let (tx, rx) = broadcast::channel(10);
    (MessageTx::new(tx), MessageRx(rx))
}
