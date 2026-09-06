//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soundwire/intel_auxdevice.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2015-2022 Intel Corporation.
extern "C" {
    pub fn intel_link_startup(auxdev: *mut auxiliary_device) -> c_int;
}
extern "C" {
    pub fn intel_link_process_wakeen_event(auxdev: *mut auxiliary_device) -> c_int;
}
extern "C" {
    pub fn intel_resume_child_device(dev: *mut device, data: *mut c_void) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdw_intel_link_dev {
    pub auxdev: auxiliary_device,
    pub link_res: sdw_intel_link_res,
}

