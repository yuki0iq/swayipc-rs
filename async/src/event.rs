use super::common::receive_from_stream;
use crate::{Event, Fallible};
use futures_lite::future::Boxed;
use futures_lite::ready;
use futures_lite::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::UnixStream;

pub struct EventStream(Boxed<(UnixStream, Fallible<Event>)>);

async fn receive(mut stream: UnixStream) -> (UnixStream, Fallible<Event>) {
    let data = receive_from_stream(&mut stream).await;
    (stream, data.and_then(Event::decode))
}

impl EventStream {
    pub(super) fn new(stream: UnixStream) -> Self {
        Self(Box::pin(receive(stream)))
    }
}

impl Stream for EventStream {
    type Item = Fallible<Event>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let (stream, item) = ready!(self.0.as_mut().poll(cx));
        self.0 = Box::pin(receive(stream));
        Poll::Ready(Some(item))
    }
}
