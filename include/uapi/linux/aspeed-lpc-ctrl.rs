//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/aspeed-lpc-ctrl.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Copyright 2017 IBM Corp.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// Window types
pub const ASPEED_LPC_CTRL_WINDOW_FLASH: c_int = 1;
pub const ASPEED_LPC_CTRL_WINDOW_MEMORY: c_int = 2;
//
// This driver provides a window for the host to access a BMC resource
// across the BMC <-> Host LPC bus.
//
// window_type: The BMC resource that the host will access through the
// window. BMC flash and BMC RAM.
//
// window_id: For each window type there may be multiple windows,
// these are referenced by ID.
//
// flags: Reserved for future use, this field is expected to be
// zeroed.
//
// addr: Address on the host LPC bus that the specified window should
// be mapped. This address must be power of two aligned.
//
// offset: Offset into the BMC window that should be mapped to the
// host (at addr). This must be a multiple of size.
//
// size: The size of the mapping. The smallest possible size is 64K.
// This must be power of two aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_lpc_ctrl_mapping {
    pub window_type: __u8,
    pub window_id: __u8,
    pub flags: __u16,
    pub addr: __u32,
    pub offset: __u32,
    pub size: __u32,
}

pub const __ASPEED_LPC_CTRL_IOCTL_MAGIC: c_uint = 0xb2;

