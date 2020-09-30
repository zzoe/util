use std::future::Future;
use std::iter::FromIterator;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures_core::Stream;

pub struct StreamVec<T: Unpin + Future>(Vec<T>);

impl<T: Unpin + Future> Stream for StreamVec<T> {
    type Item = T::Output;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let item = self
            .0
            .iter_mut()
            .enumerate()
            .find_map(|(i, f)| match Pin::new(f).poll(cx) {
                Poll::Pending => None,
                Poll::Ready(e) => Some((i, e)),
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

impl<T: Unpin + Future> FromIterator<T> for StreamVec<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}
