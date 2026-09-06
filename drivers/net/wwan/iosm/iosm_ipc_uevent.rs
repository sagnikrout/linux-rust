//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_uevent.h
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
// Copyright (C) 2020-21 Intel Corporation.
//
// Baseband event strings

// Maximum length of user events
pub const MAX_UEVENT_LEN: c_int = 64;
//
// struct ipc_uevent_info - Uevent information structure.
// @dev:	Pointer to device structure
// @uevent:	Uevent information
// @work:	Uevent work struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_uevent_info {
    pub dev: *mut device,
    pub uevent: [c_char; MAX_UEVENT_LEN],
    pub work: work_struct,
}

//
// ipc_uevent_send - Send modem event to user space.
// @dev:	Generic device pointer
// @uevent:	Uevent information
//
extern "C" {
    pub fn ipc_uevent_send(dev: *mut device, uevent: *mut c_char);
}
