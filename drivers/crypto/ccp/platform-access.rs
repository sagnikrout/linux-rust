//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccp/platform-access.h
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
// AMD Platform Security Processor (PSP) Platform Access interface
//
// Copyright (C) 2023 Advanced Micro Devices, Inc.
//
// Author: Mario Limonciello <mario.limonciello@amd.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psp_platform_access_device {
    pub dev: *mut device,
    pub psp: *mut psp_device,
    pub vdata: *mut platform_access_vdata,
    pub mailbox_mutex: mutex,
    pub doorbell_mutex: mutex,
    pub platform_access_data: *mut c_void,
}

extern "C" {
    pub fn platform_access_dev_destroy(psp: *mut psp_device);
}
extern "C" {
    pub fn platform_access_dev_init(psp: *mut psp_device) -> c_int;
}
