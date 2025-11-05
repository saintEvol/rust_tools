#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::mpsc::UnboundedReceiver as TokioUnboundedReceiver;
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::mpsc::UnboundedSender as TokioUnboundedSender;
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::mpsc::unbounded_channel;

// #[cfg(not(target_arch = "wasm32"))]
// #[derive(Debug)]
pub struct SendError(pub String);

// #[cfg(target_arch = "wasm32")]
// #[derive(Debug)]
// pub struct SendError<T>(async_channel::SendError<T>);


#[cfg(not(target_arch = "wasm32"))]
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for SendError {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        SendError(value.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
impl<T> From<async_channel::SendError<T>> for SendError {
    fn from(value: async_channel::SendError<T>) -> Self {
        SendError(value.to_string())
    }
}

pub struct UnboundedSender<T> {
    #[cfg(not(target_arch = "wasm32"))]
    inner: TokioUnboundedSender<T>,
    #[cfg(target_arch = "wasm32")]
    inner: async_channel::Sender<T>,
}

impl<T> UnboundedSender<T> {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn send(&mut self, data: T) -> Result<(), SendError> {
        self.inner.send(data)?;
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn send(&mut self, data: T) -> Result<(), SendError> {
        self.inner.force_send(data)?;
        Ok(())
    }
}

pub struct UnboundedReceiver<T> {
    #[cfg(not(target_arch = "wasm32"))]
    inner: TokioUnboundedReceiver<T>,
    #[cfg(target_arch = "wasm32")]
    inner: async_channel::Receiver<T>,
}

impl<T> UnboundedReceiver<T> {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn recv(&mut self) -> Option<T> {
         self.inner.recv().await
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn recv(&mut self) -> Option<T> {
        let r = self.inner.recv().await;
        match r {
            Ok(r) => {Some(r)}
            Err(_) => {None}
        }
    }

}

#[cfg(not(target_arch = "wasm32"))]
pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let (tx, rx) = unbounded_channel();
    (UnboundedSender { inner: tx }, UnboundedReceiver { inner: rx })
}

#[cfg(target_arch = "wasm32")]
pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let (tx, rx) = async_channel::unbounded();
    (UnboundedSender { inner: tx }, UnboundedReceiver { inner: rx })
}
