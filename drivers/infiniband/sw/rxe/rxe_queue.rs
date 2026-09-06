//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/rxe/rxe_queue.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
// Copyright (c) 2015 System Fabric Works, Inc. All rights reserved.
//
// Implements a simple circular buffer that is shared between user
// and the driver and can be resized. The requested element size is
// rounded up to a power of 2 and the number of elements in the buffer
// is also rounded up to a power of 2. Since the queue is empty when
// the producer and consumer indices match the maximum capacity of the
// queue is one less than the number of element slots.
//
// Notes:
// - The driver indices are always masked off to q->index_mask
// before storing so do not need to be checked on reads.
// - The user whether user space or kernel is generally
// not trusted so its parameters are masked to make sure
// they do not access the queue out of bounds on reads.
// - The driver indices for queues must not be written
// by user so a local copy is used and a shared copy is
// stored when the local copy is changed.
// - By passing the type in the parameter list separate from q
// the compiler can eliminate the switch statement when the
// actual queue type is known when the function is called at
// compile time.
// - These queues are lock free. The user and driver must protect
// changes to their end of the queues with locks if more than one
// CPU can be accessing it at the same time.
//
// enum queue_type - type of queue
// @QUEUE_TYPE_TO_CLIENT:	Queue is written by rxe driver and
// read by client which may be a user space
// application or a kernel ulp.
// Used by rxe internals only.
// @QUEUE_TYPE_FROM_CLIENT:	Queue is written by client and
// read by rxe driver.
// Used by rxe internals only.
// @QUEUE_TYPE_FROM_ULP:	Queue is written by kernel ulp and
// read by rxe driver.
// Used by kernel verbs APIs only on
// behalf of ulps.
// @QUEUE_TYPE_TO_ULP:		Queue is written by rxe driver and
// read by kernel ulp.
// Used by kernel verbs APIs only on
// behalf of ulps.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum queue_type {
    QUEUE_TYPE_TO_CLIENT,
    QUEUE_TYPE_FROM_CLIENT,
    QUEUE_TYPE_FROM_ULP,
    QUEUE_TYPE_TO_ULP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_queue {
    pub rxe: *mut rxe_dev,
    pub buf: *mut rxe_queue_buf,
    pub ip: *mut rxe_mmap_info,
    pub buf_size: usize,
    pub elem_size: usize,
    pub log2_elem_size: c_uint,
    pub index_mask: u32,
    pub type: queue_type,
// private copy of index for shared queues between
// driver and clients. Driver reads and writes
// this copy and then replicates to rxe_queue_buf
// for read access by clients.
//
    pub index: u32,
}

extern "C" {
    pub fn rxe_queue_reset(q: *mut rxe_queue);
}
extern "C" {
    pub fn rxe_queue_cleanup(queue: *mut rxe_queue);
}
// used by rxe, client owns the index
// used by rxe which owns the index
// used by ulp which owns the index
// used by ulp, rxe owns the index
// used by rxe which owns the index
// used by rxe, client owns the index
// used by ulp, rxe owns the index
// used by ulp which owns the index
// used by rxe, client owns the index
// used by rxe which owns the index
// release so client can read it safely
// used by ulp which owns the index
// release so rxe can read it safely
// used by ulp, rxe owns the index
// used by rxe which owns the index
// release so client can read it safely
// used by rxe, client owns the index
// used by ulp, rxe owns the index
// used by ulp which owns the index
// release so rxe can read it safely
extern "C" {
    pub fn queue_empty(_arg: q, queue_consumer_addr(q: type) ? NULL :, _arg: type) -> return;
}
