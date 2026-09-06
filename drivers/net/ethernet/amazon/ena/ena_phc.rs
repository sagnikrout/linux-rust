//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amazon/ena/ena_phc.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright 2015-2022 Amazon.com, Inc. or its affiliates. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_phc_info {
// PTP hardware capabilities
    pub clock_info: ptp_clock_info,
// Registered PTP clock device
    pub clock: *mut ptp_clock,
// Adapter specific private data structure
    pub adapter: *mut ena_adapter,
// PHC lock
    pub lock: spinlock_t,
// Enabled by kernel
    pub enabled: bool,
}

extern "C" {
    pub fn ena_phc_enable(adapter: *mut ena_adapter, enable: bool);
}
extern "C" {
    pub fn ena_phc_is_enabled(adapter: *mut ena_adapter) -> bool;
}
extern "C" {
    pub fn ena_phc_is_active(adapter: *mut ena_adapter) -> bool;
}
extern "C" {
    pub fn ena_phc_get_index(adapter: *mut ena_adapter) -> c_int;
}
extern "C" {
    pub fn ena_phc_init(adapter: *mut ena_adapter) -> c_int;
}
extern "C" {
    pub fn ena_phc_destroy(adapter: *mut ena_adapter);
}
extern "C" {
    pub fn ena_phc_alloc(adapter: *mut ena_adapter) -> c_int;
}
extern "C" {
    pub fn ena_phc_free(adapter: *mut ena_adapter);
}
