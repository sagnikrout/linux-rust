//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/iaa/iaa_crypto_stats.h
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
// Copyright(c) 2021 Intel Corporation. All rights rsvd.

extern "C" {
    pub fn iaa_crypto_debugfs_init() -> c_int;
}
extern "C" {
    pub fn iaa_crypto_debugfs_cleanup();
}
extern "C" {
    pub fn update_total_comp_calls();
}
extern "C" {
    pub fn update_total_comp_bytes_out(n: c_int);
}
extern "C" {
    pub fn update_total_decomp_calls();
}
extern "C" {
    pub fn update_total_sw_comp_calls();
}
extern "C" {
    pub fn update_total_sw_decomp_calls();
}
extern "C" {
    pub fn update_total_decomp_bytes_in(n: c_int);
}
extern "C" {
    pub fn update_completion_einval_errs();
}
extern "C" {
    pub fn update_completion_timeout_errs();
}
extern "C" {
    pub fn update_completion_comp_buf_overflow_errs();
}
extern "C" {
    pub fn update_wq_comp_calls(idxd_wq: *mut idxd_wq);
}
extern "C" {
    pub fn update_wq_comp_bytes(idxd_wq: *mut idxd_wq, n: c_int);
}
extern "C" {
    pub fn update_wq_decomp_calls(idxd_wq: *mut idxd_wq);
}
extern "C" {
    pub fn update_wq_decomp_bytes(idxd_wq: *mut idxd_wq, n: c_int);
}

