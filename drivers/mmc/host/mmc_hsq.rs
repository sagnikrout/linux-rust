//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/mmc_hsq.h
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
pub const HSQ_NUM_SLOTS: c_int = 64;

//
// For MMC host software queue, we only allow 2 requests in
// flight to avoid a long latency.
//
pub const HSQ_NORMAL_DEPTH: c_int = 2;
//
// For 4k random writes, we allow hsq_depth to increase to 5
// for better performance.
//
pub const HSQ_PERFORMANCE_DEPTH: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsq_slot {
    pub mrq: *mut mmc_request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_hsq {
    pub mmc: *mut mmc_host,
    pub mrq: *mut mmc_request,
    pub wait_queue: wait_queue_head_t,
    pub slot: *mut hsq_slot,
    pub lock: spinlock_t,
    pub retry_work: work_struct,
    pub next_tag: c_int,
    pub num_slots: c_int,
    pub qcnt: c_int,
    pub tail_tag: c_int,
    pub tag_slot: [c_int; HSQ_NUM_SLOTS],
    pub enabled: bool,
    pub waiting_for_idle: bool,
    pub recovery_halt: bool,
}

extern "C" {
    pub fn mmc_hsq_init(hsq: *mut mmc_hsq, mmc: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_hsq_suspend(mmc: *mut mmc_host);
}
extern "C" {
    pub fn mmc_hsq_resume(mmc: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_hsq_finalize_request(mmc: *mut mmc_host, mrq: *mut mmc_request) -> bool;
}
