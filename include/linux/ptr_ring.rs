//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ptr_ring.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Definitions for the 'struct ptr_ring' datastructure.
//
// Author:
// Michael S. Tsirkin <mst@redhat.com>
//
// Copyright (C) 2016 Red Hat, Inc.
//
// This is a limited-size FIFO maintaining pointers in FIFO order, with
// one CPU producing entries and another consuming entries from a FIFO.
//
// This implementation tries to minimize cache-contention when there is a
// single producer and a single consumer CPU.
//
pub const _LINUX_PTR_RING_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptr_ring {
    pub ____cacheline_aligned_in_smp: int producer,
    pub producer_lock: spinlock_t,
    pub /: *mut *mut int consumer_head ____cacheline_aligned_in_smp; / next valid entry,
    pub /: *mut *mut int consumer_tail; / next entry to invalidate,
    pub consumer_lock: spinlock_t,
// Shared consumer/producer data
// Read-only by both the producer and the consumer
    pub /: *mut *mut int size ____cacheline_aligned_in_smp; / max entries in queue,
    pub /: *mut *mut int batch; / number of entries to consume in a batch,
    pub queue: *mut c_void,
}

// Note: callers invoking this in a loop must use a compiler barrier,
// for example cpu_relax().
//
// NB: this is unlike __ptr_ring_empty in that callers must hold producer_lock:
// see e.g. ptr_ring_full.
//
extern "C" {
    pub fn data_race(_arg: r->queue[r->producer]) -> return;
}
// Report whether the next __ptr_ring_produce() has room for one entry:
// 0 means the single slot at r->queue[r->producer] is free, -ENOSPC means
// the ring is full, which is transient, and -EINVAL means r->size is 0,
// which is permanent. A caller that stops producing and waits for space
// must therefore do so only for -ENOSPC.
//
// Note: callers invoking this in a loop must use a compiler barrier,
// for example cpu_relax(). Callers must hold producer_lock.
//
// Note: callers invoking this in a loop must use a compiler barrier,
// for example cpu_relax(). Callers must hold producer_lock.
// Callers are responsible for making sure pointer that is being queued
// points to a valid data.
//
// Make sure the pointer we are storing points to a valid data.
// Pairs with the dependency ordering in __ptr_ring_consume.
//
// Note: resize (below) nests producer lock within consumer lock, so if you
// consume in interrupt or BH context, you must disable interrupts/BH when
// calling this.
//
extern "C" {
    pub fn READ_ONCE(_arg: r->queue[r->consumer_head]) -> return;
}
//
// Test ring empty status without taking any locks.
//
// NB: This is only safe to call if ring is never resized.
//
// However, if some other CPU consumes ring entries at the same time, the value
// returned is not guaranteed to be correct.
//
// In this case - to avoid incorrectly detecting the ring
// as empty - the CPU consuming the ring entries is responsible
// for either consuming all ring entries until the ring is empty,
// or synchronizing with some other CPU and causing it to
// re-test __ptr_ring_empty and/or consume the ring enteries
// after the synchronization point.
//
// Note: callers invoking this in a loop must use a compiler barrier,
// for example cpu_relax().
//
// Zero entries from tail to specified head.
// NB: if consumer_head can be >= r->size need to fixup tail later.
//
// Zero out entries in the reverse order: this way we touch the
// cache line that producer might currently be reading the last;
// producer won't make progress and touch other cache lines
// besides the first one until we write out all entries.
//
// Must only be called after __ptr_ring_peek returned !NULL
// Fundamentally, what we want to do is update consumer
// index and zero out the entry so producer can reuse it.
// Doing it naively at each consume would be as simple as:
// consumer = r->consumer;
// r->queue[consumer++] = NULL;
// if (unlikely(consumer >= r->size))
// consumer = 0;
// r->consumer = consumer;
// but that is suboptimal when the ring is full as producer is writing
// out new entries in the same cache line.  Defer these updates until a
// batch of entries has been consumed.
//
// Note: we must keep consumer_head valid at all times for __ptr_ring_empty
// to work correctly.
//
// Once we have processed enough entries invalidate them in
// the ring all at once so producer can reuse their space in the ring.
// We also do this when we reach end of the ring - not mandatory
// but helps keep the implementation simple.
//
// matching READ_ONCE in __ptr_ring_empty for lockless tests
// The READ_ONCE in __ptr_ring_peek guarantees that anyone
// accessing data through the pointer is up to date. Pairs
// with smp_wmb in __ptr_ring_produce.
//
// Note: resize (below) nests producer lock within consumer lock, so if you
// call this in interrupt or BH context, you must disable interrupts/BH when
// producing.
//
// Cast to structure type and call a function without discarding from FIFO.
// Function must return a value.
// Callers must take consumer_lock.
//

// Not all gfp_t flags (besides GFP_KERNEL) are allowed. See
// documentation for vmalloc for which of them are legal.
//
extern "C" {
    pub fn kvmalloc_array_noprof(_arg: size, ): *mut sizeof(void, __GFP_ZERO: gfp |) -> return;
}
// We need to set batch at least to 1 to make logic
// in __ptr_ring_discard_one work correctly.
// Batching too much (because ring is small) would cause a lot of
// burstiness. Needs tuning, for now disable batching.
//

//
// Return entries into ring. Destroy entries that don't fit.
//
// Note: this is expected to be a rare slow path operation.
//
// Note: producer lock is nested within consumer lock, so if you
// resize you must make sure all uses nest correctly.
// In particular if you consume ring in interrupt or BH context, you must
// disable interrupts/BH when doing so.
//
// Clean out buffered entries (for simplicity). This way following code
// can test entries for NULL and if not assume they are valid.
//
// Go over entries in batch, start moving head back and copy entries.
// Stop when we run into previously unconsumed entries.
//
// This batch entry will have to be destroyed.
// matching READ_ONCE in __ptr_ring_empty for lockless tests
// Destroy all entries left in the batch.
//
// Note: producer lock is nested within consumer lock, so if you
// resize you must make sure all uses nest correctly.
// In particular if you consume ring in interrupt or BH context, you must
// disable interrupts/BH when doing so.
//

//
// Note: producer lock is nested within consumer lock, so if you
// resize you must make sure all uses nest correctly.
// In particular if you consume ring in BH context, you must
// disable BH when doing so.
//

