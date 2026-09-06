//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lwq.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

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


// SPDX-License-Identifier: GPL-2.0-only
//
// Light-weight single-linked queue built from llist
//
// Entries can be enqueued from any context with no locking.
// Entries can be dequeued from process context with integrated locking.
//
// This is particularly suitable when work items are queued in
// BH or IRQ context, and where work items are handled one at a time
// by dedicated threads.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lwq_node {
    pub node: llist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lwq {
    pub lock: spinlock_t,
    pub /: *mut *mut *mut llist_node ready; / entries to be dequeued,
    pub /: *mut *mut llist_head new; / entries being enqueued,
}

//
// lwq_init - initialise a lwq
// @q:	the lwq object
//
// lwq_empty - test if lwq contains any entry
// @q:	the lwq object
//
// This empty test contains an acquire barrier so that if a wakeup
// is sent when lwq_dequeue returns true, it is safe to go to sleep after
// a test on lwq_empty().
//
// acquire ensures ordering wrt lwq_enqueue()
extern "C" {
    pub fn smp_load_acquire(llist_empty(&q->new: &q->ready) == NULL &&) -> return;
}
//
// lwq_dequeue - dequeue first (oldest) entry from lwq
// @q:		the queue to dequeue from
// @type:	the type of object to return
// @member:	them member in returned object which is an lwq_node.
//
// Remove a single object from the lwq and return it.  This will take
// a spinlock and so must always be called in the same context, typcially
// process contet.
//

//
// lwq_for_each_safe - iterate over detached queue allowing deletion
// @_n:		iterator variable
// @_t1:	temporary struct llist_node
// @_t2:	temporary struct llist_node
// @_l:		address of llist_node pointer from lwq_dequeue_all()
// @_member:	member in _n where lwq_node is found.
//
// Iterate over members in a dequeued list.  If the iterator variable
// is set to NULL, the iterator removes that entry from the queue.
//

// (_t1) ? (_n = container_of(*(_t1), typeof(*(_n)), _member.node),\
//
// lwq_enqueue - add a new item to the end of the queue
// @n	- the lwq_node embedded in the item to be added
// @q	- the lwq to append to.
//
// No locking is needed to append to the queue so this can
// be called from any context.
// Return %true is the list may have previously been empty.
//
// acquire enqures ordering wrt lwq_dequeue
//
// lwq_enqueue_batch - add a list of new items to the end of the queue
// @n	- the lwq_node embedded in the first item to be added
// @q	- the lwq to append to.
//
// No locking is needed to append to the queue so this can
// be called from any context.
// Return %true is the list may have previously been empty.
//
// acquire enqures ordering wrt lwq_dequeue
