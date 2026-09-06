//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_mert.h
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


// SPDX-License-Identifier: MIT
//
// Copyright(c) 2025, Intel Corporation. All rights reserved.
//

//
// struct xe_mert - MERT related data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_mert {
// @lock: protects the TLB invalidation status
    pub lock: spinlock_t,
// @tlb_inv_triggered: indicates if TLB invalidation was triggered
    pub tlb_inv_triggered: bool,
// @tlb_inv_done: completion of TLB invalidation
    pub tlb_inv_done: completion,
}

extern "C" {
    pub fn xe_mert_init_early(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_mert_invalidate_lmtt(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_mert_irq_handler(xe: *mut xe_device, master_ctl: u32);
}

