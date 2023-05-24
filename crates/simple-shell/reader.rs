use core::{future::Future, pin::Pin, task::Poll};

pub(crate) struct Reader {
    read: fn() -> Option<u8>,
}

impl Reader {
    pub fn new(read: fn() -> Option<u8>) -> Self {
        Self { read }
    }
}

impl Future for Reader {
    type Output = u8;

    fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> Poll<Self::Output> {
        match (self.read)() {
            Some(c) => Poll::Ready(c),
            None => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}
