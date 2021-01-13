use std::task::{Context, Poll};
use std::time::Duration;

use anyhow::Result;
use async_io::Timer;
use futures_lite::Future;
use pin_project_lite::pin_project;
use std::pin::Pin;

pub trait Timeout: Future + Sized {
    fn timeout(self, time: Duration) -> Delay<Self> {
        Delay {
            task: self,
            timer: Timer::after(time),
        }
    }
}

impl<F: Future> Timeout for F {}

pin_project! {
    pub struct Delay<T>{
        #[pin]
        task: T,
        #[pin]
        timer: Timer,
    }
}

impl<T: Future> Future for Delay<T> {
    type Output = Result<T::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();

        // First, try polling the future
        if let Poll::Ready(v) = me.task.poll(cx) {
            return Poll::Ready(Ok(v));
        }

        // Now check the timer
        match me.timer.poll(cx) {
            Poll::Ready(_) => Poll::Ready(Err(anyhow::anyhow!("Timeout!"))),
            Poll::Pending => Poll::Pending,
        }
    }
}
