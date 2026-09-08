//! `stream` feature: drain a paginated endpoint as a stream of items

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;

use crate::{Page, Result};

/// a response the stream can walk page by page; implemented by [`Page`]
pub trait Paginate {
    type Item;
    /// one item off the current page; pages are ~20 items, so a front
    /// removal is fine
    fn next_item(&mut self) -> Option<Self::Item>;
    fn has_next_page(&self) -> bool;
}

impl<T> Paginate for Page<T> {
    type Item = T;

    fn next_item(&mut self) -> Option<T> {
        if self.results.is_empty() {
            None
        } else {
            Some(self.results.remove(0))
        }
    }

    fn has_next_page(&self) -> bool {
        self.has_next()
    }
}

type Fetch<R> = Box<dyn FnMut(u32) -> Pin<Box<dyn Future<Output = Result<R>>>>>;

/// the stream returned by `into_stream` on paginated request builders
pub struct PageStream<R> {
    fetch: Fetch<R>,
    next_page: u32,
    pending: Option<Pin<Box<dyn Future<Output = Result<R>>>>>,
    page: Option<R>,
    done: bool,
}

pub(crate) fn page_stream<R, F, Fut>(mut fetch: F) -> PageStream<R>
where
    F: FnMut(u32) -> Fut + 'static,
    Fut: Future<Output = Result<R>> + 'static,
{
    PageStream {
        fetch: Box::new(move |page| Box::pin(fetch(page))),
        next_page: 1,
        pending: None,
        page: None,
        done: false,
    }
}

// everything pinned lives behind a Pin<Box>, so moving is safe
impl<R> Unpin for PageStream<R> {}

impl<R: Paginate> Stream for PageStream<R> {
    type Item = Result<R::Item>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            if let Some(item) = this.page.as_mut().and_then(Paginate::next_item) {
                return Poll::Ready(Some(Ok(item)));
            }
            if this.done {
                return Poll::Ready(None);
            }
            if this.pending.is_none() {
                this.pending = Some((this.fetch)(this.next_page));
            }
            // just set above, or on a previous poll
            let pending = this.pending.as_mut().expect("pending future");
            let page = match pending.as_mut().poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Err(error)) => {
                    this.done = true;
                    return Poll::Ready(Some(Err(error)));
                }
                Poll::Ready(Ok(page)) => page,
            };
            this.pending = None;
            this.done = !page.has_next_page();
            this.next_page += 1;
            this.page = Some(page);
        }
    }
}
