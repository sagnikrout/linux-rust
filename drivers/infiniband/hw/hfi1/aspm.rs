//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/aspm.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2015-2017 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aspm_mode {
    ASPM_MODE_DISABLED = 0,	/* ASPM always disabled, performance mode */
    ASPM_MODE_ENABLED = 1,	/* ASPM always enabled, power saving mode */
    ASPM_MODE_DYNAMIC = 2,	/* ASPM enabled/disabled dynamically */
}

extern "C" {
    pub fn aspm_init(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn aspm_exit(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn aspm_hw_disable_l1(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn __aspm_ctx_disable(rcd: *mut hfi1_ctxtdata);
}
extern "C" {
    pub fn aspm_disable_all(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn aspm_enable_all(dd: *mut hfi1_devdata);
}
// Quickest exit for minimum impact
