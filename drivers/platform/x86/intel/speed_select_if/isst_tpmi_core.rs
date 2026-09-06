//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/intel/speed_select_if/isst_tpmi_core.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Intel Speed Select Interface: Drivers Internal defines
// Copyright (c) 2023, Intel Corporation.
// All rights reserved.
//
extern "C" {
    pub fn tpmi_sst_init() -> c_int;
}
extern "C" {
    pub fn tpmi_sst_exit();
}
extern "C" {
    pub fn tpmi_sst_dev_add(auxdev: *mut auxiliary_device) -> c_int;
}
extern "C" {
    pub fn tpmi_sst_dev_remove(auxdev: *mut auxiliary_device);
}
extern "C" {
    pub fn tpmi_sst_dev_suspend(auxdev: *mut auxiliary_device);
}
extern "C" {
    pub fn tpmi_sst_dev_resume(auxdev: *mut auxiliary_device);
}
