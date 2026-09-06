//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched_clock.h
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
// sched_clock.h: support for extending counters to full 64-bit ns counter
//

// Macro flag: #define LINUX_SCHED_CLOCK

//
// struct clock_read_data - data required to read from sched_clock()
//
// @epoch_ns:		sched_clock() value at last update
// @epoch_cyc:		Clock cycle value at last update.
// @sched_clock_mask:   Bitmask for two's complement subtraction of non 64bit
// clocks.
// @read_sched_clock:	Current clock source (or dummy source when suspended).
// @mult:		Multiplier for scaled math conversion.
// @shift:		Shift value for scaled math conversion.
//
// Care must be taken when updating this structure; it is read by
// some very hot code paths. It occupies <=40 bytes and, when combined
// with the seqcount used to synchronize access, comfortably fits into
// a 64 byte cache line.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_read_data {
    pub epoch_ns: u64,
    pub epoch_cyc: u64,
    pub sched_clock_mask: u64,
    pub (*read_sched_clock)(void): *mut u64,
    pub mult: u32,
    pub shift: u32,
}

extern "C" {
    pub fn sched_clock_read_retry(seq: c_uint) -> c_int;
}
extern "C" {
    pub fn generic_sched_clock_init();
}

