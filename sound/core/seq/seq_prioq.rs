//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/seq_prioq.h
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
// ALSA sequencer Priority Queue
// Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
//

// === PRIOQ ===
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_prioq {
    pub /: *mut *mut *mut snd_seq_event_cell head; / pointer to head of prioq,
    pub /: *mut *mut *mut snd_seq_event_cell tail; / pointer to tail of prioq,
    pub cells: c_int,
    pub lock: spinlock_t,
}

// create new prioq (constructor)
// delete prioq (destructor)
extern "C" {
    pub fn snd_seq_prioq_delete(fifo: *mut snd_seq_prioq);
}
// enqueue cell to prioq
extern "C" {
    pub fn snd_seq_prioq_cell_in(f: *mut snd_seq_prioq, cell: *mut snd_seq_event_cell) -> c_int;
}
// dequeue cell from prioq
// return number of events available in prioq
extern "C" {
    pub fn snd_seq_prioq_avail(f: *mut snd_seq_prioq) -> c_int;
}
// client left queue
extern "C" {
    pub fn snd_seq_prioq_leave(f: *mut snd_seq_prioq, client: c_int, timestamp: c_int);
}
// Remove events
