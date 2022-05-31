use tokio::sync::{mpsc, oneshot};

use crate::Value;


pub struct Next {
    pub sender: mpsc::Sender<oneshot::Sender<Result<Value, anyhow::Error>>>,
}

impl Next {
    async fn resolve(self) -> Result<Value, anyhow::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send(sender).await?;
        receiver.await?
    }
}
