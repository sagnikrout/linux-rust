//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/funnel-queue.h
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
// Copyright 2023 Red Hat
//

//
// A funnel queue is a simple (almost) lock-free queue that accepts entries from multiple threads
// (multi-producer) and delivers them to a single thread (single-consumer). "Funnel" is an attempt
// to evoke the image of requests from more than one producer being "funneled down" to a single
// consumer.
//
// This is an unsynchronized but thread-safe data structure when used as intended. There is no
// mechanism to ensure that only one thread is consuming from the queue. If more than one thread
// attempts to consume from the queue, the resulting behavior is undefined. Clients must not
// directly access or manipulate the internals of the queue, which are only exposed for the purpose
// of allowing the very simple enqueue operation to be inlined.
//
// The implementation requires that a funnel_queue_entry structure (a link pointer) is embedded in
// the queue entries, and pointers to those structures are used exclusively by the queue. No macros
// are defined to template the queue, so the offset of the funnel_queue_entry in the records placed
// in the queue must all be the same so the client can derive their structure pointer from the
// entry pointer returned by vdo_funnel_queue_poll().
//
// Callers are wholly responsible for allocating and freeing the entries. Entries may be freed as
// soon as they are returned since this queue is not susceptible to the "ABA problem" present in
// many lock-free data structures. The queue is dynamically allocated to ensure cache-line
// alignment, but no other dynamic allocation is used.
//
// The algorithm is not actually 100% lock-free. There is a single point in vdo_funnel_queue_put()
// at which a preempted producer will prevent the consumers from seeing items added to the queue by
// later producers, and only if the queue is short enough or the consumer fast enough for it to
// reach what was the end of the queue at the time of the preemption.
//
// The consumer function, vdo_funnel_queue_poll(), will return NULL when the queue is empty. To
// wait for data to consume, spin (if safe) or combine the queue with a struct event_count to
// signal the presence of new entries.
//
// This queue link structure must be embedded in client entries.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct funnel_queue_entry {
// The next (newer) entry in the queue.
    pub next: *mut funnel_queue_entry,
}

//
// The dynamically allocated queue structure, which is allocated on a cache line boundary so the
// producer and consumer fields in the structure will land on separate cache lines. This should be
// consider opaque but it is exposed here so vdo_funnel_queue_put() can be inlined.
//
// The producers' end of the queue, an atomically exchanged pointer that will never be
// NULL.
//
// The consumer's end of the queue, which is owned by the consumer and never NULL.
extern "C" {
    pub fn __aligned(_arg: L1_CACHE_BYTES) -> *mut funnel_queue_entry oldest;
}
// A dummy entry used to provide the non-NULL invariants above.
extern "C" {
    pub fn vdo_make_funnel_queue(queue_ptr: *mut funnel_queue) -> int __must_check;
}
extern "C" {
    pub fn vdo_free_funnel_queue(queue: *mut funnel_queue);
}
//
// Put an entry on the end of the queue.
//
// The entry pointer must be to the struct funnel_queue_entry embedded in the caller's data
// structure. The caller must be able to derive the address of the start of their data structure
// from the pointer that passed in here, so every entry in the queue must have the struct
// funnel_queue_entry at the same offset within the client's structure.
//
// Barrier requirements: All stores relating to the entry ("next" pointer, containing data
// structure fields) must happen before the previous->next store making it visible to the
// consumer. Also, the entry's "next" field initialization to NULL must happen before any
// other producer threads can see the entry (the xchg) and try to update the "next" field.
//
// xchg implements a full barrier.
//
// Preemptions between these two statements hide the rest of the queue from the consumer,
// preventing consumption until the following assignment runs.
//
extern "C" {
    pub fn vdo_funnel_queue_poll(queue: *mut funnel_queue) -> *mut funnel_queue_entry __must_check;
}
extern "C" {
    pub fn vdo_is_funnel_queue_empty(queue: *mut funnel_queue) -> bool __must_check;
}
extern "C" {
    pub fn vdo_is_funnel_queue_idle(queue: *mut funnel_queue) -> bool __must_check;
}
