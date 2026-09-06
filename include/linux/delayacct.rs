//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/delayacct.h
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
// delayacct.h - per-task delay accounting
//
// Copyright (C) Shailabh Nagar, IBM Corp. 2006
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_delay_info {
    pub lock: raw_spinlock_t,
// For each stat XXX, add following, aligned appropriately
//
// struct timespec XXX_start, XXX_end;
// u64 XXX_delay;
// u32 XXX_count;
//
// Atomicity of updates to XXX_delay, XXX_count protected by
// single lock above (split into XXX_lock if contention is an issue).
//
// XXX_count is incremented on every XXX operation, the delay
// associated with the operation is added to XXX_delay.
// XXX_delay contains the accumulated delay time in nanoseconds.
//
    pub blkio_start: u64,
    pub blkio_delay_max: u64,
    pub blkio_delay_min: u64,
    pub /: *mut *mut u64 blkio_delay; / wait for sync block io completion,
    pub swapin_start: u64,
    pub swapin_delay_max: u64,
    pub swapin_delay_min: u64,
    pub /: *mut *mut u64 swapin_delay; / wait for swapin,
    pub /: *mut *mut u32 blkio_count; / total count of the number of sync block,
// io operations performed
    pub /: *mut *mut u32 swapin_count; / total count of swapin,
    pub freepages_start: u64,
    pub freepages_delay_max: u64,
    pub freepages_delay_min: u64,
    pub /: *mut *mut u64 freepages_delay; / wait for memory reclaim,
    pub thrashing_start: u64,
    pub thrashing_delay_max: u64,
    pub thrashing_delay_min: u64,
    pub /: *mut *mut u64 thrashing_delay; / wait for thrashing page,
    pub compact_start: u64,
    pub compact_delay_max: u64,
    pub compact_delay_min: u64,
    pub /: *mut *mut u64 compact_delay; / wait for memory compact,
    pub wpcopy_start: u64,
    pub wpcopy_delay_max: u64,
    pub wpcopy_delay_min: u64,
    pub /: *mut *mut u64 wpcopy_delay; / wait for write-protect copy,
    pub irq_delay_max: u64,
    pub irq_delay_min: u64,
    pub /: *mut *mut u64 irq_delay; / wait for IRQ/SOFTIRQ,
    pub /: *mut *mut u32 freepages_count; / total count of memory reclaim,
    pub /: *mut *mut u32 thrashing_count; / total count of thrash waits,
    pub /: *mut *mut u32 compact_count; / total count of memory compact,
    pub /: *mut *mut u32 wpcopy_count; / total count of write-protect copy,
    pub /: *mut *mut u32 irq_count; / total count of IRQ/SOFTIRQ,
    pub blkio_delay_max_ts: timespec64,
    pub swapin_delay_max_ts: timespec64,
    pub freepages_delay_max_ts: timespec64,
    pub thrashing_delay_max_ts: timespec64,
    pub compact_delay_max_ts: timespec64,
    pub wpcopy_delay_max_ts: timespec64,
    pub irq_delay_max_ts: timespec64,
}

extern "C" {
    pub fn delayacct_init();
}
extern "C" {
    pub fn __delayacct_tsk_init(: *mut task_struct);
}
extern "C" {
    pub fn __delayacct_tsk_exit(: *mut task_struct);
}
extern "C" {
    pub fn __delayacct_blkio_start();
}
extern "C" {
    pub fn __delayacct_blkio_end(: *mut task_struct);
}
extern "C" {
    pub fn delayacct_add_tsk(: *mut taskstats, : *mut task_struct) -> c_int;
}
extern "C" {
    pub fn __delayacct_blkio_ticks(: *mut task_struct) -> __u64;
}
extern "C" {
    pub fn __delayacct_freepages_start();
}
extern "C" {
    pub fn __delayacct_freepages_end();
}
extern "C" {
    pub fn __delayacct_thrashing_start(in_thrashing: *mut bool);
}
extern "C" {
    pub fn __delayacct_thrashing_end(in_thrashing: *mut bool);
}
extern "C" {
    pub fn __delayacct_swapin_start();
}
extern "C" {
    pub fn __delayacct_swapin_end();
}
extern "C" {
    pub fn __delayacct_compact_start();
}
extern "C" {
    pub fn __delayacct_compact_end();
}
extern "C" {
    pub fn __delayacct_wpcopy_start();
}
extern "C" {
    pub fn __delayacct_wpcopy_end();
}
extern "C" {
    pub fn __delayacct_irq(task: *mut task_struct, delta: u32);
}
// reinitialize in case parent's non-null pointer was dup'ed
// Free tsk->delays. Called from bad fork and __put_task_struct
// where there's no risk of tsk->delays being accessed elsewhere
//
extern "C" {
    pub fn __delayacct_blkio_ticks(_arg: tsk) -> return;
}

