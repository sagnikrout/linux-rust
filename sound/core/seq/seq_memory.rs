//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/seq_memory.h
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
// ALSA sequencer Memory Manager
// Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
//

// aliasing for legacy and UMP event packet handling
#[repr(C)]
#[derive(Copy, Clone)]
pub union __snd_seq_event {
    pub legacy: snd_seq_event,

    pub ump: snd_seq_ump_event,

    pub event: snd_seq_event,

    pub extra: u32,

    pub raw: } __packed,
}

// container for sequencer event (internal use)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_event_cell {
    pub event: snd_seq_event,
    pub ump: __snd_seq_event,
}

// design note: the pool is a contiguous block of memory, if we dynamicly
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_pool {
    pub /: *mut *mut *mut snd_seq_event_cell ptr; / pointer to first event chunk,
    pub /: *mut *mut *mut snd_seq_event_cell free; / pointer to the head of the free list,
    pub /: *mut *mut int total_elements; / pool size actually allocated,
    pub /: *mut *mut atomic_t counter; / cells free,
    pub /: *mut *mut int size; / pool size to be allocated,
    pub /: *mut *mut int room; / watermark for sleep/wakeup,
    pub closing: c_int,
// statistics
    pub max_used: c_int,
    pub event_alloc_nopool: c_int,
    pub event_alloc_failures: c_int,
    pub event_alloc_success: c_int,
// Write locking
    pub output_sleep: wait_queue_head_t,
// Pool lock
    pub lock: spinlock_t,
}

extern "C" {
    pub fn snd_seq_cell_free(cell: *mut snd_seq_event_cell);
}
// return number of unused (free) cells
// return total number of allocated cells
// init pool - allocate events
extern "C" {
    pub fn snd_seq_pool_init(pool: *mut snd_seq_pool) -> c_int;
}
// done pool - free events
extern "C" {
    pub fn snd_seq_pool_mark_closing(pool: *mut snd_seq_pool);
}
extern "C" {
    pub fn snd_seq_pool_done(pool: *mut snd_seq_pool) -> c_int;
}
// create pool
// remove pool
extern "C" {
    pub fn snd_seq_pool_delete(pool: *mut snd_seq_pool) -> c_int;
}
// polling
extern "C" {
    pub fn snd_seq_pool_poll_wait(pool: *mut snd_seq_pool, file: *mut file, wait: *mut poll_table) -> c_int;
}
