//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/part_stat.h
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disk_stats {
    pub nsecs: [u64; NR_STAT_GROUPS],
    pub sectors: [c_ulong; NR_STAT_GROUPS],
    pub ios: [c_ulong; NR_STAT_GROUPS],
    pub merges: [c_ulong; NR_STAT_GROUPS],
    pub io_ticks: c_ulong,
    pub in_flight: [local_t; 2],
}

//
// Macros to operate on percpu disk statistics:
//
// part_stat_{add|sub|inc|dec}() modify the stat counters and should
// be called between part_stat_lock() and part_stat_unlock().
//
// part_stat_read() can be called at any time.
//

extern "C" {
    pub fn bdev_count_inflight(part: *mut block_device) -> c_uint;
}
