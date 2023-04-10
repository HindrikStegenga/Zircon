#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AssetState {
    Loading = 0,
    Available = 1,
    Failed = 2,
}

use crate::AssetIdentifier;
use std::any::Any;
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::Arc;
use utils::{t_fatal, t_warn};

pub(super) struct AssetBuffer {
    asset_id: AssetIdentifier,
    state: AtomicU8,
    cell: UnsafeCell<Box<dyn Any>>,
}
impl AssetBuffer {
    pub(super) fn new(asset_id: AssetIdentifier) -> Arc<AssetBuffer> {
        Arc::new(AssetBuffer {
            asset_id,
            state: AtomicU8::new(AssetState::Loading as u8),
            cell: UnsafeCell::new(Box::new(())),
        })
    }
    pub(super) fn set_available<T: Sized + 'static>(&self, value: T, used: usize) {
        let state = self.state.load(Acquire);
        if state != (AssetState::Loading as u8) {
            t_fatal!("state != (Loading as u8)");
        }
        let item = unsafe { &mut *self.cell.get() };
        *item = Box::new(value);
        self.state.store(AssetState::Available as u8, Release);
    }
    pub(super) fn set_failed(&self) {
        self.state.store(AssetState::Failed as u8, Release);
    }

    pub(super) fn try_read<T: Sized + 'static>(&self) -> Option<&T> {
        let state = self.state.load(Acquire);
        if state != (AssetState::Available as u8) {
            return None;
        }
        unsafe {
            let item = &*self.cell.get();
            item.downcast_ref()
        }
    }
    pub(super) fn state(&self) -> AssetState {
        let state = self.state.load(Acquire);
        match state {
            0 => AssetState::Loading,
            1 => AssetState::Available,
            2 => AssetState::Failed,
            _ => unreachable!(),
        }
    }
}
