use std::pin::Pin;
use std::time::Duration;

use smol::future::{Future, FutureExt, Or};

pub trait Timeout: FutureExt {
    fn timeout<'a>(self, time: Duration, default: Self::Output) -> Or<Self, Pin<Box<dyn Future<Output=Self::Output> + Send + 'a>>>
        where Self: Sized,
              Self::Output: 'a + Send,
    {
        self.or(async move {
            smol::Timer::after(time).await;
            default
        }.boxed())
    }
}

impl<F: FutureExt> Timeout for F {}
