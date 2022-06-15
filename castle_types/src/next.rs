use tokio::sync::{mpsc, oneshot};

use crate::{Value, Field};


pub struct Next {
    pub sender: mpsc::Sender<(oneshot::Sender<Result<Value, anyhow::Error>>, Field)>,
}

impl Next {
    async fn resolve(self, field: Field) -> Result<Value, anyhow::Error> {
        let (sender, receiver) = oneshot::channel();
        self.sender.send((sender, field)).await?;
        receiver.await?
    }
}
