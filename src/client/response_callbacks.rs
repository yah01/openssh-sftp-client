use super::{Response, ThreadSafeWaker};

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use std::io;

use parking_lot::{Mutex, RwLock};
use thunderdome::Arena;

pub(crate) type Value = (ThreadSafeWaker, Mutex<Option<(Response, Vec<u8>)>>);

// TODO: Simplify this

#[derive(Debug, Default)]
pub(crate) struct ResponseCallbacks(RwLock<Arena<Value>>);

impl ResponseCallbacks {
    fn insert_impl(&self) -> u32 {
        let val = (ThreadSafeWaker::new(), Mutex::new(None));

        self.0.write().insert(val).slot()
    }

    async fn wait_impl(&self, slot: u32) {
        struct WaitFuture<'a>(Option<&'a RwLock<Arena<Value>>>, u32);

        impl Future for WaitFuture<'_> {
            type Output = ();

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let rwlock = if let Some(rwlock) = self.0.take() {
                    rwlock
                } else {
                    return Poll::Ready(());
                };

                let waker = cx.waker().clone();

                let guard = rwlock.read();
                let (_index, value) = guard.get_by_slot(self.1).expect("Invalid slot");

                if value.0.install_waker(waker) {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        }

        WaitFuture(Some(&self.0), slot).await;
    }

    pub fn insert(&self) -> SlotGuard {
        SlotGuard(self, Some(self.insert_impl()))
    }

    /// Prototype
    pub(crate) async fn do_callback(
        &self,
        slot: u32,
        response: Response,
        buffer: Vec<u8>,
    ) -> io::Result<()> {
        match self.0.read().get_by_slot(slot) {
            None => return Ok(()),
            Some((_index, value)) => {
                *value.1.lock() = Some((response, buffer));
                value.0.done();
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct SlotGuard<'a>(&'a ResponseCallbacks, Option<u32>);

impl SlotGuard<'_> {
    pub(crate) fn get_slot_id(&self) -> u32 {
        self.1.unwrap()
    }

    fn remove(&mut self, slot: u32) -> Value {
        self.0
             .0
            .write()
            .remove_by_slot(slot)
            .expect("Slot is removed before SlotGuard is dropped or waited")
            .1
    }

    pub(crate) async fn wait(mut self) -> (Response, Vec<u8>) {
        let slot = self.1.take().unwrap();
        self.0.wait_impl(slot).await;
        self.remove(slot).1.into_inner().unwrap()
    }
}

impl Drop for SlotGuard<'_> {
    fn drop(&mut self) {
        if let Some(slot) = self.1.take() {
            self.remove(slot);
        }
    }
}
