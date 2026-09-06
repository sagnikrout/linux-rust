//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/aspeed-p2a-ctrl.h
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
// Copyright 2019 Google Inc
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//
// Provides a simple driver to control the ASPEED P2A interface which allows
// the host to read and write to various regions of the BMC's memory.
//

pub const ASPEED_P2A_CTRL_READ_ONLY: c_int = 0;
pub const ASPEED_P2A_CTRL_READWRITE: c_int = 1;
//
// This driver provides a mechanism for enabling or disabling the read-write
// property of specific windows into the ASPEED BMC's memory.
//
// A user can map a region of the BMC's memory as read-only or read-write, with
// the caveat that once any region is mapped, all regions are unlocked for
// reading.
//
// Unlock a region of BMC physical memory for access from the host.
//
// Also used to read back the optional memory-region configuration for the
// driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_p2a_ctrl_mapping {
    pub addr: __u64,
    pub length: __u32,
    pub flags: __u32,
}

pub const __ASPEED_P2A_CTRL_IOCTL_MAGIC: c_uint = 0xb3;
//
// This IOCTL is meant to configure a region or regions of memory given a
// starting address and length to be readable by the host, or
// readable-writeable.
//

//
// This IOCTL is meant to read back to the user the base address and length of
// the memory-region specified to the driver for use with mmap.
//

