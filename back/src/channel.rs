use tokio::sync::broadcast;

use crate::message::SocketMessage;

#[derive(Debug, Clone)]
pub struct MessageTx(broadcast::Sender<SocketMessage>);
pub struct MessageRx(broadcast::Receiver<SocketMessage>);

impl MessageTx {
    pub fn new(tx: broadcast::Sender<SocketMessage>) -> Self {
        Self(tx)
    }

    pub fn broadcast(&self) -> MessageRx {
        MessageRx(self.0.subscribe())
    }

    pub fn send<I: Into<SocketMessage>>(
        &self,
        msg: I,
    ) -> Result<usize, broadcast::error::SendError<SocketMessage>> {
        let msg = msg.into();
        self.0.send(msg)
    }
}

impl MessageRx {
    pub async fn recv(&mut self) -> Result<SocketMessage, broadcast::error::RecvError> {
        self.0.recv().await
    }
}
