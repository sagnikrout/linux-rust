
// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------

// SPDX-License-Identifier: GPL-2.0

//! This module provides the `TagSet` struct to wrap the C `struct blk_mq_tag_set`.
//!
//! C header: [`include/linux/blk-mq.h`](srctree/include/linux/blk-mq.h)

use core::pin::Pin;

use crate::{
    bindings,
    block::mq::{operations::OperationsVTable, request::RequestDataWrapper, Operations},
    error::{self, Result},
    prelude::try_pin_init,
    types::Opaque,
};
use core::{convert::TryInto, marker::PhantomData};
use pin_init::{pin_data, pinned_drop, PinInit};

/// A wrapper for the C `struct blk_mq_tag_set`.
///
/// `struct blk_mq_tag_set` contains a `struct list_head` and so must be pinned.
///
/// # Invariants
///
/// - `inner` is initialized and valid.
#[pin_data(PinnedDrop)]
#[repr(transparent)]
pub struct TagSet<T: Operations> {
    #[pin]
    inner: Opaque<bindings::blk_mq_tag_set>,
    _p: PhantomData<T>,
}

impl<T: Operations> TagSet<T> {
    /// Try to create a new tag set
    pub fn new(
        nr_hw_queues: u32,
        num_tags: u32,
        num_maps: u32,
    ) -> impl PinInit<Self, error::Error> {
        let tag_set: bindings::blk_mq_tag_set = pin_init::zeroed();
        let tag_set: Result<_> = core::mem::size_of::<RequestDataWrapper>()
            .try_into()
            .map(|cmd_size| {
                bindings::blk_mq_tag_set {
                    ops: OperationsVTable::<T>::build(),
                    nr_hw_queues,
                    timeout: 0, // 0 means default which is 30Hz in C
                    numa_node: bindings::NUMA_NO_NODE,
                    queue_depth: num_tags,
                    cmd_size,
                    flags: 0,
                    driver_data: core::ptr::null_mut::<crate::ffi::c_void>(),
                    nr_maps: num_maps,
                    ..tag_set
                }
            })
            .map(Opaque::new)
            .map_err(|e| e.into());

        try_pin_init!(TagSet {
            inner <- tag_set.pin_chain(|tag_set| {
                // SAFETY: we do not move out of `tag_set`.
                let tag_set: &mut Opaque<_> = unsafe { Pin::get_unchecked_mut(tag_set) };
                // SAFETY: `tag_set` is a reference to an initialized `blk_mq_tag_set`.
                error::to_result( unsafe { bindings::blk_mq_alloc_tag_set(tag_set.get())})
            }),
            _p: PhantomData,
        })
    }

    /// Return the pointer to the wrapped `struct blk_mq_tag_set`
    pub(crate) fn raw_tag_set(&self) -> *mut bindings::blk_mq_tag_set {
        self.inner.get()
    }
}

#[pinned_drop]
impl<T: Operations> PinnedDrop for TagSet<T> {
    fn drop(self: Pin<&mut Self>) {
        // SAFETY: By type invariant `inner` is valid and has been properly
        // initialized during construction.
        unsafe { bindings::blk_mq_free_tag_set(self.inner.get()) };
    }
}
