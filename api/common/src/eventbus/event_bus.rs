use crate::eventbus::Message;
use futures::SinkExt;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct EventBus {
    receiver: Receiver<Message>,
    sender: Receiver<Sender<Message>>,
}

impl EventBus {
    pub async fn send(&mut self, message: Message) {
        // TODO  self.sender.send(message).await.unwrap();
    }
}
