//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/cgbc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Congatec Board Controller driver definitions
//
// Copyright (C) 2024 Bootlin
// Author: Thomas Richard <thomas.richard@bootlin.com>
//
// struct cgbc_version - Board Controller device version structure
// @feature:	Board Controller feature number
// @major:	Board Controller major revision
// @minor:	Board Controller minor revision
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgbc_version {
    pub feature: c_uchar,
    pub major: c_uchar,
    pub minor: c_uchar,
}

//
// struct cgbc_device_data - Internal representation of the Board Controller device
// @io_session:		Pointer to the session IO memory
// @io_cmd:		Pointer to the command IO memory
// @session:		Session id returned by the Board Controller
// @dev:		Pointer to kernel device structure
// @version:		Board Controller version structure
// @lock:		Board Controller mutex
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgbc_device_data {
    pub io_session: *mut void __iomem,
    pub io_cmd: *mut void __iomem,
    pub session: u8,
    pub dev: *mut device,
    pub version: cgbc_version,
    pub lock: mutex,
}
