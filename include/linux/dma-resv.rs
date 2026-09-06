//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma-resv.h
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


//
// Header file for reservations for dma-buf and ttm
//
// Copyright(C) 2011 Linaro Limited. All rights reserved.
// Copyright (C) 2012-2013 Canonical Ltd
// Copyright (C) 2012 Texas Instruments
//
// Authors:
// Rob Clark <robdclark@gmail.com>
// Maarten Lankhorst <maarten.lankhorst@canonical.com>
// Thomas Hellstrom <thellstrom-at-vmware-dot-com>
//
// Based on bo.c which bears the following copyright notice,
// but is dual licensed:
//
// Copyright (c) 2006-2009 VMware, Inc., Palo Alto, CA., USA
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//

//
// enum dma_resv_usage - how the fences from a dma_resv obj are used
//
// This enum describes the different use cases for a dma_resv object and
// controls which fences are returned when queried.
//
// An important fact is that there is the order KERNEL<WRITE<READ<BOOKKEEP and
// when the dma_resv object is asked for fences for one use case the fences
// for the lower use case are returned as well.
//
// For example when asking for WRITE fences then the KERNEL fences are returned
// as well. Similar when asked for READ fences then both WRITE and KERNEL
// fences are returned as well.
//
// Already used fences can be promoted in the sense that a fence with
// DMA_RESV_USAGE_BOOKKEEP could become DMA_RESV_USAGE_READ by adding it again
// with this usage. But fences can never be degraded in the sense that a fence
// with DMA_RESV_USAGE_WRITE could become DMA_RESV_USAGE_READ.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_resv_usage {
//
// @DMA_RESV_USAGE_KERNEL: For in kernel memory management only.
//
// This should only be used for things like copying or clearing memory
// with a DMA hardware engine for the purpose of kernel memory
// management.
//
// Drivers *always* must wait for those fences before accessing the
// resource protected by the dma_resv object. The only exception for
// that is when the resource is known to be locked down in place by
// pinning it previously.
//
    DMA_RESV_USAGE_KERNEL,

//
// @DMA_RESV_USAGE_WRITE: Implicit write synchronization.
//
// This should only be used for userspace command submissions which add
// an implicit write dependency.
//
    DMA_RESV_USAGE_WRITE,

//
// @DMA_RESV_USAGE_READ: Implicit read synchronization.
//
// This should only be used for userspace command submissions which add
// an implicit read dependency.
//
    DMA_RESV_USAGE_READ,

//
// @DMA_RESV_USAGE_BOOKKEEP: No implicit sync.
//
// This should be used by submissions which don't want to participate in
// any implicit synchronization.
//
// The most common cases are preemption fences, page table updates, TLB
// flushes as well as explicitly synced user submissions.
//
// Explicitly synced user submissions can be promoted to
// DMA_RESV_USAGE_READ or DMA_RESV_USAGE_WRITE as needed using
// dma_buf_import_sync_file() when implicit synchronization should
// become necessary after initial adding of the fence.
//
    DMA_RESV_USAGE_BOOKKEEP
}

//
// dma_resv_usage_rw - helper for implicit sync
// @write: true if we create a new implicit sync write
//
// This returns the implicit synchronization usage for write or read accesses,
// see enum dma_resv_usage and &dma_buf.resv.
//
// This looks confusing at first sight, but is indeed correct.
//
// The rational is that new write operations needs to wait for the
// existing read and write operations to finish.
// But a new read operation only needs to wait for the existing write
// operations to finish.
//
// struct dma_resv - a reservation object manages fences for a buffer
//
// This is a container for dma_fence objects which needs to handle multiple use
// cases.
//
// One use is to synchronize cross-driver access to a struct dma_buf, either for
// dynamic buffer management or just to handle implicit synchronization between
// different users of the buffer in userspace. See &dma_buf.resv for a more
// in-depth discussion.
//
// The other major use is to manage access and locking within a driver in a
// buffer based memory manager. struct ttm_buffer_object is the canonical
// example here, since this is where reservation objects originated from. But
// use in drivers is spreading and some drivers also manage struct
// drm_gem_object with the same scheme.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_resv {
//
// @lock:
//
// Update side lock. Don't use directly, instead use the wrapper
// functions like dma_resv_lock() and dma_resv_unlock().
//
// Drivers which use the reservation object to manage memory dynamically
// also use this lock to protect buffer object state like placement,
// allocation policies or throughout command submission.
//
    pub lock: ww_mutex,
//
// @fences:
//
// Array of fences which where added to the dma_resv object
//
// A new fence is added by calling dma_resv_add_fence(). Since this
// often needs to be done past the point of no return in command
// submission it cannot fail, and therefore sufficient slots need to be
// reserved by calling dma_resv_reserve_fences().
//
    pub fences: *mut dma_resv_list __rcu,
}

//
// struct dma_resv_iter - current position into the dma_resv fences
//
// Don't touch this directly in the driver, use the accessor function instead.
//
// IMPORTANT
//
// When using the lockless iterators like dma_resv_iter_next_unlocked() or
// dma_resv_for_each_fence_unlocked() beware that the iterator can be restarted.
// Code which accumulates statistics or similar needs to check for this with
// dma_resv_iter_is_restarted().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_resv_iter {
// @obj: The dma_resv object we iterate over
    pub obj: *mut dma_resv,
// @usage: Return fences with this usage or lower.
    pub usage: dma_resv_usage,
// @fence: the currently handled fence
    pub fence: *mut dma_fence,
// @fence_usage: the usage of the current fence
    pub fence_usage: dma_resv_usage,
// @index: index into the shared fences
    pub index: c_uint,
// @fences: the shared fences; private, *MUST* not dereference
    pub fences: *mut dma_resv_list,
// @num_fences: number of fences
    pub num_fences: c_uint,
// @is_restarted: true if this is the first returned fence
    pub is_restarted: bool,
}

//
// dma_resv_iter_begin - initialize a dma_resv_iter object
// @cursor: The dma_resv_iter object to initialize
// @obj: The dma_resv object which we want to iterate over
// @usage: controls which fences to include, see enum dma_resv_usage.
//
// dma_resv_iter_end - cleanup a dma_resv_iter object
// @cursor: the dma_resv_iter object which should be cleaned up
//
// Make sure that the reference to the fence in the cursor is properly
// dropped.
//
// dma_resv_iter_usage - Return the usage of the current fence
// @cursor: the cursor of the current position
//
// Returns the usage of the currently processed fence.
//
// dma_resv_iter_is_restarted - test if this is the first fence after a restart
// @cursor: the cursor with the current position
//
// Return true if this is the first fence in an iteration after a restart.
//
// dma_resv_for_each_fence_unlocked - unlocked fence iterator
// @cursor: a struct dma_resv_iter pointer
// @fence: the current fence
//
// Iterate over the fences in a struct dma_resv object without holding the
// &dma_resv.lock and using RCU instead. The cursor needs to be initialized
// with dma_resv_iter_begin() and cleaned up with dma_resv_iter_end(). Inside
// the iterator a reference to the dma_fence is held and the RCU lock dropped.
//
// Beware that the iterator can be restarted when the struct dma_resv for
// @cursor is modified. Code which accumulates statistics or similar needs to
// check for this with dma_resv_iter_is_restarted(). For this reason prefer the
// lock iterator dma_resv_for_each_fence() whenever possible.
//

//
// dma_resv_for_each_fence - fence iterator
// @cursor: a struct dma_resv_iter pointer
// @obj: a dma_resv object pointer
// @usage: controls which fences to return
// @fence: the current fence
//
// Iterate over the fences in a struct dma_resv object while holding the
// &dma_resv.lock. @all_fences controls if the shared fences are returned as
// well. The cursor initialisation is part of the iterator and the fence stays
// valid as long as the lock is held and so no extra reference to the fence is
// taken.
//

extern "C" {
    pub fn dma_resv_reset_max_fences(obj: *mut dma_resv);
}

//
// dma_resv_lock - lock the reservation object
// @obj: the reservation object
// @ctx: the locking context
//
// Locks the reservation object for exclusive access and modification. Note,
// that the lock is only against other writers, readers will run concurrently
// with a writer under RCU. The seqlock is used to notify readers if they
// overlap with a writer.
//
// As the reservation object may be locked by multiple parties in an
// undefined order, a #ww_acquire_ctx is passed to unwind if a cycle
// is detected. See ww_mutex_lock() and ww_acquire_init(). A reservation
// object may be locked by itself by passing NULL as @ctx.
//
// When a die situation is indicated by returning -EDEADLK all locks held by
// @ctx must be unlocked and then dma_resv_lock_slow() called on @obj.
//
// Unlocked by calling dma_resv_unlock().
//
// See also dma_resv_lock_interruptible() for the interruptible variant.
//
extern "C" {
    pub fn ww_mutex_lock(_arg: &obj->lock, _arg: ctx) -> return;
}
//
// dma_resv_lock_interruptible - lock the reservation object
// @obj: the reservation object
// @ctx: the locking context
//
// Locks the reservation object interruptible for exclusive access and
// modification. Note, that the lock is only against other writers, readers
// will run concurrently with a writer under RCU. The seqlock is used to
// notify readers if they overlap with a writer.
//
// As the reservation object may be locked by multiple parties in an
// undefined order, a #ww_acquire_ctx is passed to unwind if a cycle
// is detected. See ww_mutex_lock() and ww_acquire_init(). A reservation
// object may be locked by itself by passing NULL as @ctx.
//
// When a die situation is indicated by returning -EDEADLK all locks held by
// @ctx must be unlocked and then dma_resv_lock_slow_interruptible() called on
// @obj.
//
// Unlocked by calling dma_resv_unlock().
//
extern "C" {
    pub fn ww_mutex_lock_interruptible(_arg: &obj->lock, _arg: ctx) -> return;
}
//
// dma_resv_lock_slow - slowpath lock the reservation object
// @obj: the reservation object
// @ctx: the locking context
//
// Acquires the reservation object after a die case. This function
// will sleep until the lock becomes available. See dma_resv_lock() as
// well.
//
// See also dma_resv_lock_slow_interruptible() for the interruptible variant.
//
// dma_resv_lock_slow_interruptible - slowpath lock the reservation
// object, interruptible
// @obj: the reservation object
// @ctx: the locking context
//
// Acquires the reservation object interruptible after a die case. This function
// will sleep until the lock becomes available. See
// dma_resv_lock_interruptible() as well.
//
extern "C" {
    pub fn ww_mutex_lock_slow_interruptible(_arg: &obj->lock, _arg: ctx) -> return;
}
//
// dma_resv_trylock - trylock the reservation object
// @obj: the reservation object
//
// Tries to lock the reservation object for exclusive access and modification.
// Note, that the lock is only against other writers, readers will run
// concurrently with a writer under RCU. The seqlock is used to notify readers
// if they overlap with a writer.
//
// Also note that since no context is provided, no deadlock protection is
// possible, which is also not needed for a trylock.
//
// Returns true if the lock was acquired, false otherwise.
//
extern "C" {
    pub fn ww_mutex_trylock(_arg: &obj->lock, _arg: NULL) -> return;
}
//
// dma_resv_is_locked - is the reservation object locked
// @obj: the reservation object
//
// Returns true if the mutex is locked, false if unlocked.
//
extern "C" {
    pub fn ww_mutex_is_locked(_arg: &obj->lock) -> return;
}
//
// dma_resv_locking_ctx - returns the context used to lock the object
// @obj: the reservation object
//
// Returns the context used to lock a reservation object or NULL if no context
// was used or the object is not locked at all.
//
// WARNING: This interface is pretty horrible, but TTM needs it because it
// doesn't pass the struct ww_acquire_ctx around in some very long callchains.
// Everyone else just uses it to check whether they're holding a reservation or
// not.
//
extern "C" {
    pub fn READ_ONCE(_arg: obj->lock.ctx) -> return;
}
//
// dma_resv_unlock - unlock the reservation object
// @obj: the reservation object
//
// Unlocks the reservation object following exclusive access.
//
extern "C" {
    pub fn dma_resv_init(obj: *mut dma_resv);
}
extern "C" {
    pub fn dma_resv_fini(obj: *mut dma_resv);
}
extern "C" {
    pub fn dma_resv_reserve_fences(obj: *mut dma_resv, num_fences: c_uint) -> c_int;
}
extern "C" {
    pub fn dma_resv_copy_fences(dst: *mut dma_resv, src: *mut dma_resv) -> c_int;
}
extern "C" {
    pub fn dma_resv_test_signaled(obj: *mut dma_resv, usage: dma_resv_usage) -> bool;
}
extern "C" {
    pub fn dma_resv_describe(obj: *mut dma_resv, seq: *mut seq_file);
}
