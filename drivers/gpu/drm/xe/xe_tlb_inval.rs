//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_tlb_inval.h
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
// Copyright © 2025 Intel Corporation
//

extern "C" {
    pub fn xe_gt_tlb_inval_init_early(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_tlb_inval_reset(tlb_inval: *mut xe_tlb_inval);
}
extern "C" {
    pub fn xe_tlb_inval_ggtt(tlb_inval: *mut xe_tlb_inval) -> c_int;
}
extern "C" {
    pub fn xe_tlb_inval_vm(tlb_inval: *mut xe_tlb_inval, vm: *mut xe_vm);
}
//
// xe_tlb_inval_fence_wait() - TLB invalidiation fence wait
// @fence: TLB invalidation fence to wait on
//
// Wait on a TLB invalidiation fence until it signals, non interruptible
//
extern "C" {
    pub fn xe_tlb_inval_done_handler(tlb_inval: *mut xe_tlb_inval, seqno: c_int);
}
extern "C" {
    pub fn xe_tlb_inval_idle(tlb_inval: *mut xe_tlb_inval) -> bool;
}
extern "C" {
    pub fn xe_tlb_inval_batch_wait(batch: *mut xe_tlb_inval_batch);
}
