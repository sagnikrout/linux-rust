//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/wait-queue.h
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
// A vdo_wait_queue is a circular singly linked list of entries waiting to be notified
// of a change in a condition. Keeping a circular list allows the vdo_wait_queue
// structure to simply be a pointer to the tail (newest) entry, supporting
// constant-time enqueue and dequeue operations. A null pointer is an empty waitq.
//
// An empty waitq:
// waitq0.last_waiter -> NULL
//
// A singleton waitq:
// waitq1.last_waiter -> entry1 -> entry1 -> [...]
//
// A three-element waitq:
// waitq2.last_waiter -> entry3 -> entry1 -> entry2 -> entry3 -> [...]
//
// linux/wait.h's wait_queue_head is _not_ used because vdo_wait_queue's
// interface is much less complex (doesn't need locking, priorities or timers).
// Made possible by vdo's thread-based resource allocation and locking; and
// the polling nature of vdo_wait_queue consumers.
//
// FIXME: could be made to use a linux/list.h's list_head but its extra barriers
// really aren't needed. Nor is a doubly linked list, but vdo_wait_queue could
// make use of __list_del_clearprev() -- but that would compromise the ability
// to make full use of linux's list interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_wait_queue {
// The tail of the queue, the last (most recently added) entry
    pub last_waiter: *mut vdo_waiter,
// The number of waiters currently in the queue
    pub length: usize,
}

//
// vdo_waiter_callback_fn - Callback type that will be called to resume processing
// of a waiter after it has been removed from its wait queue.
//
extern "C" {
    pub fn void(waiter: *mut *mut vdo_waiter_callback_fn)(struct vdo_waiter, context: *mut c_void) -> typedef;
}
//
// vdo_waiter_match_fn - Method type for waiter matching methods.
//
// Returns false if the waiter does not match.
//
extern "C" {
    pub fn bool(waiter: *mut *mut vdo_waiter_match_fn)(struct vdo_waiter, context: *mut c_void) -> typedef;
}
// The structure for entries in a vdo_wait_queue.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdo_waiter {
//
// The next waiter in the waitq. If this entry is the last waiter, then this
// is actually a pointer back to the head of the waitq.
//
    pub next_waiter: *mut vdo_waiter,
// Optional waiter-specific callback to invoke when dequeuing this waiter.
    pub callback: vdo_waiter_callback_fn,
}

//
// vdo_waiter_is_waiting() - Check whether a waiter is waiting.
// @waiter: The waiter to check.
//
// Return: true if the waiter is on some vdo_wait_queue.
//
// vdo_waitq_init() - Initialize a vdo_wait_queue.
// @waitq: The vdo_wait_queue to initialize.
//
// waitq = (struct vdo_wait_queue) {
//
// vdo_waitq_has_waiters() - Check whether a vdo_wait_queue has any entries waiting.
// @waitq: The vdo_wait_queue to query.
//
// Return: true if there are any waiters in the waitq.
//
// vdo_waitq_num_waiters() - Return the number of waiters in a vdo_wait_queue.
// @waitq: The vdo_wait_queue to query.
//
// Return: The number of waiters in the waitq.
//
