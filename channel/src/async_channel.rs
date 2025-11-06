use std::fmt::{Display, Formatter};
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::mpsc::UnboundedReceiver as TokioUnboundedReceiver;
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::mpsc::UnboundedSender as TokioUnboundedSender;
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::mpsc::unbounded_channel;
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::oneshot::{Receiver, Sender};

// #[cfg(not(target_arch = "wasm32"))]
// #[derive(Debug)]
#[derive(Debug)]
pub struct SendError(pub String);

impl Display for SendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 收取消息错误，原因只有一个：发送端提交关闭（且收取端为空)
#[derive(Debug)]
pub struct RecvError;

impl Display for RecvError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "send half canceled")
    }
}

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

impl<T> Clone for UnboundedSender<T> {
    fn clone(&self) -> Self {
        UnboundedSender {
            inner: self.inner.clone(),
        }
    }
}

impl<T> UnboundedSender<T> {
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub fn send(&self, data: T) -> Result<(), SendError> {
        self.inner.send(data)?;
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub fn send(&self, data: T) -> Result<(), SendError> {
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
    #[inline]
    pub async fn recv(&mut self) -> Option<T> {
        self.inner.recv().await
    }

    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub async fn recv(&mut self) -> Option<T> {
        let r = self.inner.recv().await;
        match r {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let (tx, rx) = unbounded_channel();
    (
        UnboundedSender { inner: tx },
        UnboundedReceiver { inner: rx },
    )
}

#[cfg(target_arch = "wasm32")]
pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let (tx, rx) = async_channel::unbounded();
    (
        UnboundedSender { inner: tx },
        UnboundedReceiver { inner: rx },
    )
}

pub struct OneshotSender<T> {
    #[cfg(not(target_arch = "wasm32"))]
    tokio_sender: Sender<T>,
    #[cfg(target_arch = "wasm32")]
    web_sender: async_channel::Sender<T>,
}

impl<T> OneshotSender<T> {
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub fn send(self, data: T) -> Result<(), T> {
        self.tokio_sender.send(data)
    }
    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub fn send(self, data: T) -> Result<(), T> {
        self.web_send(data)
    }

    // #[cfg(not(target_arch = "wasm32"))]
    // #[inline]
    // fn tokio_send(self, data: T) -> Result<(), T> {
    //     self.tokio_sender.send(data)
    // }

    #[cfg(target_arch = "wasm32")]
    #[inline]
    fn web_send(self, data: T) -> Result<(), T> {
        match self.web_sender.force_send(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.0),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn oneshot<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let (tokio_sender, tokio_receiver) = tokio::sync::oneshot::channel();
    (
        OneshotSender { tokio_sender },
        OneshotReceiver { tokio_receiver },
    )
}

#[cfg(target_arch = "wasm32")]
pub fn oneshot<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let (web_sender, web_receiver) = async_channel::unbounded();
    (
        OneshotSender { web_sender },
        OneshotReceiver { web_receiver },
    )
}

pub struct OneshotReceiver<T> {
    #[cfg(not(target_arch = "wasm32"))]
    tokio_receiver: Receiver<T>,
    #[cfg(target_arch = "wasm32")]
    web_receiver: async_channel::Receiver<T>,
}

impl<T> OneshotReceiver<T> {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn recv(self) -> Result<T, RecvError> {
        match self.tokio_receiver.await {
            Ok(r) => Ok(r),
            Err(_) => Err(RecvError),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn recv(self) -> Result<T, RecvError> {
        match self.web_receiver.recv().await {
            Ok(r) => Ok(r),
            Err(_) => Err(RecvError),
        }
    }
}
