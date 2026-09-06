//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/iavf/iavf_ptp.h
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
// Copyright(c) 2024 Intel Corporation.

// bit indicating whether a 40bit timestamp is valid

extern "C" {
    pub fn iavf_ptp_init(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_ptp_release(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_ptp_process_caps(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_ptp_cap_supported(adapter: *const iavf_adapter, cap: u32) -> bool;
}
extern "C" {
    pub fn iavf_virtchnl_send_ptp_cmd(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_ptp_extend_32b_timestamp(cached_phc_time: u64, in_tstamp: u32) -> u64;
}

