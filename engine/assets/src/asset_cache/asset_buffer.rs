#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AssetState {
    Loading = 0,
    Available = 1,
    Failed = 2,
}
use crate::AssetIdentifier;
use crossbeam::queue::ArrayQueue;
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::Arc;
use utils::{t_fatal, t_warn};

pub enum AssetBufferState {
    Empty,
    LoadedBlob(Box<[u8]>),
    Deserialized(),
}

pub(super) struct AssetBuffer {
    asset_id: AssetIdentifier,
    buffers: Arc<ArrayQueue<Vec<u8>>>,
    state: AtomicU8,
    cell: UnsafeCell<(usize, Vec<u8>)>,
}
impl AssetBuffer {
    pub(super) fn new(
        asset_id: AssetIdentifier,
        buffers: Arc<ArrayQueue<Vec<u8>>>,
    ) -> Arc<AssetBuffer> {
        Arc::new(AssetBuffer {
            asset_id,
            buffers,
            state: AtomicU8::new(AssetState::Loading as u8),
            cell: UnsafeCell::new((0, vec![])),
        })
    }
    pub(super) fn set_available(&self, buffer: Vec<u8>, used: usize) {
        let state = self.state.load(Acquire);
        if state != (AssetState::Loading as u8) {
            t_fatal!("state != (Loading as u8)");
        }
        let (size, buf) = unsafe { &mut *self.cell.get() };
        *size = used;
        *buf = buffer;
        self.state.store(AssetState::Available as u8, Release);
    }
    pub(super) fn set_failed(&self) {
        self.state.store(AssetState::Failed as u8, Release);
    }
    pub(super) fn try_read(&self) -> Option<&[u8]> {
        let state = self.state.load(Acquire);
        if state != (AssetState::Available as u8) {
            return None;
        }
        unsafe {
            let (size, buf) = &*self.cell.get();
            Some(&buf.as_slice()[0..*size])
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

impl Drop for AssetBuffer {
    fn drop(&mut self) {
        match self.state() {
            AssetState::Loading | AssetState::Failed => return,
            AssetState::Available => {
                // By definition no more live refs
                let (_, buf) = unsafe { &mut *self.cell.get() };
                buf.clear();
                let mut empty = vec![];
                std::mem::swap(buf, &mut empty);
                match self.buffers.push(empty) {
                    Ok(_) => {}
                    Err(buf) => {
                        t_warn!(
                            "Could not recycle buffer. Buffer is lost: {}",
                            buf.capacity()
                        );
                    }
                }
            }
        }
    }
}

unsafe impl Send for AssetBuffer {}
unsafe impl Sync for AssetBuffer {}
