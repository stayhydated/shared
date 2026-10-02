use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Owns the callback and its one outstanding browser frame request together.
pub(crate) struct AnimationFrameHandle<Callback, Cancel: Fn(i32) = fn(i32)> {
    pub(crate) running: Rc<Cell<bool>>,
    pub(crate) frame_callback: Rc<RefCell<Option<Callback>>>,
    pub(crate) pending_frame: Rc<Cell<Option<i32>>>,
    cancel_frame: Cancel,
}

impl<Callback, Cancel: Fn(i32)> AnimationFrameHandle<Callback, Cancel> {
    pub(crate) fn new(cancel_frame: Cancel) -> Self {
        Self {
            running: Rc::new(Cell::new(true)),
            frame_callback: Rc::new(RefCell::new(None)),
            pending_frame: Rc::new(Cell::new(None)),
            cancel_frame,
        }
    }
}

impl<Callback, Cancel: Fn(i32)> Drop for AnimationFrameHandle<Callback, Cancel> {
    fn drop(&mut self) {
        self.running.set(false);
        if let Some(frame) = self.pending_frame.take() {
            (self.cancel_frame)(frame);
        }
        // JS must no longer have a pending invocation when its Closure is invalidated.
        // Taking the callback also breaks the cycle created by the render loop.
        self.frame_callback.borrow_mut().take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct FrameQueue {
        next_id: Cell<i32>,
        pending: RefCell<BTreeMap<i32, Rc<Cell<bool>>>>,
    }

    impl FrameQueue {
        fn request(&self, callback: &Callback) -> i32 {
            let id = self.next_id.get();
            self.next_id.set(id + 1);
            self.pending
                .borrow_mut()
                .insert(id, Rc::clone(&callback.valid));
            id
        }

        fn cancel(&self, id: i32) {
            self.pending.borrow_mut().remove(&id);
        }

        fn advance(&self) -> Result<usize, &'static str> {
            let pending = self.pending.take();
            if pending.values().any(|valid| !valid.get()) {
                return Err("queued callback was invalidated");
            }
            Ok(pending.len())
        }
    }

    struct Callback {
        valid: Rc<Cell<bool>>,
    }

    impl Drop for Callback {
        fn drop(&mut self) {
            self.valid.set(false);
        }
    }

    #[test]
    fn dropping_handle_cancels_pending_callback() {
        let queue = FrameQueue::default();
        let handle = AnimationFrameHandle::new(|id| queue.cancel(id));
        let callback = Callback {
            valid: Rc::new(Cell::new(true)),
        };
        handle.pending_frame.set(Some(queue.request(&callback)));
        handle.frame_callback.replace(Some(callback));

        drop(handle);

        assert_eq!(queue.advance(), Ok(0));
    }

    #[test]
    fn dropping_handle_cancels_the_rescheduled_frame() {
        let queue = FrameQueue::default();
        let handle = AnimationFrameHandle::new(|id| queue.cancel(id));
        let callback = Callback {
            valid: Rc::new(Cell::new(true)),
        };
        handle.pending_frame.set(Some(queue.request(&callback)));
        assert_eq!(queue.advance(), Ok(1));
        handle.pending_frame.take();
        handle.pending_frame.set(Some(queue.request(&callback)));
        handle.frame_callback.replace(Some(callback));

        drop(handle);

        assert_eq!(queue.advance(), Ok(0));
    }

    #[test]
    fn dropping_before_initialization_stops_the_loop() {
        let queue = FrameQueue::default();
        let handle = AnimationFrameHandle::<Callback, _>::new(|id| queue.cancel(id));
        let running = Rc::clone(&handle.running);
        let callback = Rc::clone(&handle.frame_callback);

        drop(handle);

        assert!(!running.get());
        assert!(callback.borrow().is_none());
        assert_eq!(queue.advance(), Ok(0));
    }
}
