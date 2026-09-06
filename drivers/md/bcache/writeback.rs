//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/bcache/writeback.h
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
pub const CUTOFF_WRITEBACK: c_int = 40;
pub const CUTOFF_WRITEBACK_SYNC: c_int = 70;
pub const CUTOFF_WRITEBACK_MAX: c_int = 70;
pub const CUTOFF_WRITEBACK_SYNC_MAX: c_int = 90;
pub const MAX_WRITEBACKS_IN_PASS: c_int = 5;

pub const WRITEBACK_RATE_UPDATE_SECS_MAX: c_int = 60;
pub const WRITEBACK_RATE_UPDATE_SECS_DEFAULT: c_int = 5;
pub const BCH_AUTO_GC_DIRTY_THRESHOLD: c_int = 50;
pub const BCH_WRITEBACK_FRAGMENT_THRESHOLD_LOW: c_int = 50;
pub const BCH_WRITEBACK_FRAGMENT_THRESHOLD_MID: c_int = 57;
pub const BCH_WRITEBACK_FRAGMENT_THRESHOLD_HIGH: c_int = 64;
pub const BCH_DIRTY_INIT_THRD_MAX: c_int = 12;
//
// 14 (16384ths) is chosen here as something that each backing device
// should be a reasonable fraction of the share, and not to blow up
// until individual backing devices are a petabyte.
//
pub const WRITEBACK_SHARE_SHIFT: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dirty_init_thrd_info {
    pub state: *mut bch_dirty_init_state,
    pub thread: *mut task_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bch_dirty_init_state {
    pub c: *mut cache_set,
    pub d: *mut bcache_device,
    pub total_threads: c_int,
    pub key_idx: c_int,
    pub idx_lock: spinlock_t,
    pub started: core::sync::atomic::AtomicI32,
    pub enough: core::sync::atomic::AtomicI32,
    pub wait: wait_queue_head_t,
    pub infos: [dirty_init_thrd_info; BCH_DIRTY_INIT_THRD_MAX],
}

// d->nr_stripes is in range [1, INT_MAX]
//
// Here offset is definitly smaller than INT_MAX,
// return it as int will never overflow.
//
// XXX: should do this synchronously
extern "C" {
    pub fn bch_sectors_dirty_init(d: *mut bcache_device);
}
extern "C" {
    pub fn bch_cached_dev_writeback_init(dc: *mut cached_dev);
}
extern "C" {
    pub fn bch_cached_dev_writeback_start(dc: *mut cached_dev) -> c_int;
}
