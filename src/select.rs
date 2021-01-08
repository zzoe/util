use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use smol::stream::Stream;

pub struct Select<Fut>(pub Vec<Fut>);

impl<Fut: Unpin + Future> Stream for Select<Fut> {
    type Item = Fut::Output;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let item = self.0.iter_mut().enumerate().find_map(|(i, f)| {
            match Pin::new(f).poll(cx) {
                Poll::Pending => None,
                Poll::Ready(e) => Some((i, e)),
            }
        });
        match item {
            Some((idx, res)) => {
                self.0.remove(idx);
                Poll::Ready(Some(res))
            }
            None if self.0.is_empty() => Poll::Ready(None),
            _ => Poll::Pending,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len(), Some(self.0.len()))
    }
}