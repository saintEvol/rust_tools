use std::pin::Pin;
use std::task::{Context, Poll};
use types::Instant;

#[pin_project::pin_project]
pub struct Sleep {
    #[cfg(not(target_arch = "wasm32"))]
    #[pin]
    tokio_inner: tokio::time::Sleep,
    #[cfg(target_arch = "wasm32")]
    #[pin]
    gloo_inner: gloo::timers::future::TimeoutFuture,
}

impl Future for Sleep {
    type Output = ();

    #[cfg(not(target_arch = "wasm32"))]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.tokio_inner.poll(cx)
    }

    #[cfg(target_arch = "wasm32")]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.gloo_inner.poll(cx)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn sleep_until(deadline: Instant) -> Sleep {
    gloo_sleep_until(deadline)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn sleep_until(deadline: Instant) -> Sleep {
    tokio_sleep_until(deadline)
}

#[cfg(not(target_arch = "wasm32"))]
fn tokio_sleep_until(deadline: Instant) -> Sleep {
    let s = tokio::time::sleep_until(deadline.into());
    Sleep { tokio_inner: s }
}

#[cfg(target_arch = "wasm32")]
fn gloo_sleep_until(deadline: Instant) -> Sleep {
    let now = Instant::now();
    let diff = deadline.saturating_duration_since(now);
    let s = gloo::timers::future::sleep(diff);
    Sleep { gloo_inner: s }
}
