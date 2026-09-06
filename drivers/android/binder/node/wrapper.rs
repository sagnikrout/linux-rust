
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

// Copyright (C) 2025 Google LLC.

use kernel::{list::ListArc, prelude::*, seq_file::SeqFile, seq_print, sync::UniqueArc};

use crate::{node::Node, thread::Thread, BinderReturnWriter, DArc, DLArc, DTRWrap, DeliverToRead};

use core::mem::MaybeUninit;

pub(crate) struct CritIncrWrapper {
    inner: UniqueArc<MaybeUninit<DTRWrap<NodeWrapper>>>,
}

impl CritIncrWrapper {
    pub(crate) fn new() -> Result<Self> {
        Ok(CritIncrWrapper {
            inner: UniqueArc::new_uninit(GFP_KERNEL)?,
        })
    }

    pub(super) fn init(self, node: DArc<Node>) -> DLArc<dyn DeliverToRead> {
        match self.inner.pin_init_with(DTRWrap::new(NodeWrapper { node })) {
            Ok(initialized) => ListArc::from(initialized) as DLArc<dyn DeliverToRead>,
            Err(err) => match err {},
        }
    }
}

struct NodeWrapper {
    node: DArc<Node>,
}

kernel::list::impl_list_arc_safe! {
    impl ListArcSafe<0> for NodeWrapper {
        untracked;
    }
}

impl DeliverToRead for NodeWrapper {
    fn do_work(
        self: DArc<Self>,
        _thread: &Thread,
        writer: &mut BinderReturnWriter<'_>,
    ) -> Result<bool> {
        let node = &self.node;
        let mut owner_inner = node.owner.inner.lock();
        let inner = node.inner.access_mut(&mut owner_inner);

        let ds = &mut inner.delivery_state;

        assert!(ds.has_pushed_wrapper);
        assert!(ds.has_strong_zero2one);
        ds.has_pushed_wrapper = false;
        ds.has_strong_zero2one = false;

        node.do_work_locked(writer, owner_inner)
    }

    fn cancel(self: DArc<Self>) {}

    fn should_sync_wakeup(&self) -> bool {
        false
    }

    #[inline(never)]
    fn debug_print(&self, m: &SeqFile, prefix: &str, _tprefix: &str) -> Result<()> {
        seq_print!(
            m,
            "{}node work {}: u{:016x} c{:016x}\n",
            prefix,
            self.node.debug_id,
            self.node.ptr,
            self.node.cookie,
        );
        Ok(())
    }
}
