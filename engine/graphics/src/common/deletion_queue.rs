use std::marker::PhantomData;

/// Anything that can be destroyed using a Destroyer type of object as dependency..
pub trait Destroyable<D> {
    unsafe fn destroy(self, destroyer: &D);
}

pub struct DeletionQueue<D, T: Destroyable<D>> {
    deletion_queue: Vec<T>,
    _phantom: PhantomData<D>,
}

impl<D, T: Destroyable<D>> DeletionQueue<D, T> {
    pub(crate) fn flush(&mut self, destroyer: &D) {
        unsafe {
            for deletable in self.deletion_queue.drain(..).rev() {
                deletable.destroy(destroyer);
            }
        }
    }

    pub(crate) fn enqueue(&mut self, item: T) {
        self.deletion_queue.push(item);
    }
}
