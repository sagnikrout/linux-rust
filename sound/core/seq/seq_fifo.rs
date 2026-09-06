//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/seq_fifo.h
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
// ALSA sequencer FIFO
// Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
//

// === FIFO ===
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_fifo {
    pub /: *mut *mut *mut snd_seq_pool pool; / FIFO pool,
    pub /: *mut *mut *mut snd_seq_event_cell head; / pointer to head of fifo,
    pub /: *mut *mut *mut snd_seq_event_cell tail; / pointer to tail of fifo,
    pub cells: c_int,
    pub lock: spinlock_t,
    pub use_lock: snd_use_lock_t,
    pub input_sleep: wait_queue_head_t,
    pub overflow: core::sync::atomic::AtomicI32,
}

// create new fifo (constructor)
// delete fifo (destructor)
extern "C" {
    pub fn snd_seq_fifo_delete(f: *mut snd_seq_fifo);
}
// enqueue event to fifo
extern "C" {
    pub fn snd_seq_fifo_event_in(f: *mut snd_seq_fifo, event: *mut snd_seq_event) -> c_int;
}
// lock fifo from release

// get a cell from fifo - fifo should be locked
extern "C" {
    pub fn snd_seq_fifo_cell_out(f: *mut snd_seq_fifo, cellp: *mut snd_seq_event_cell, nonblock: c_int) -> c_int;
}
// free dequeued cell - fifo should be locked
extern "C" {
    pub fn snd_seq_fifo_cell_putback(f: *mut snd_seq_fifo, cell: *mut snd_seq_event_cell);
}
// clean up queue
extern "C" {
    pub fn snd_seq_fifo_clear(f: *mut snd_seq_fifo);
}
// polling
extern "C" {
    pub fn snd_seq_fifo_poll_wait(f: *mut snd_seq_fifo, file: *mut file, wait: *mut poll_table) -> c_int;
}
// resize pool in fifo
extern "C" {
    pub fn snd_seq_fifo_resize(f: *mut snd_seq_fifo, poolsize: c_int) -> c_int;
}
// get the number of unused cells safely
extern "C" {
    pub fn snd_seq_fifo_unused_cells(f: *mut snd_seq_fifo) -> c_int;
}
