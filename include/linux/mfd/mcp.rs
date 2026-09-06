//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mcp.h
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
// linux/drivers/mfd/mcp.h
//
// Copyright (C) 2001 Russell King, All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp {
    pub owner: *mut module,
    pub ops: *mut mcp_ops,
    pub lock: spinlock_t,
    pub use_count: c_int,
    pub sclk_rate: c_uint,
    pub rw_timeout: c_uint,
    pub attached_device: device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_ops {
    pub int): *mut *mut *mut void (set_telecom_divisor)(struct mcp , unsigned,
    pub int): *mut *mut *mut void (set_audio_divisor)(struct mcp , unsigned,
    pub int): *mut *mut *mut void (reg_write)(struct mcp , unsigned int, unsigned,
    pub int): *mut *mut *mut unsigned int (reg_read)(struct mcp , unsigned,
    pub ): *mut *mut void (enable)(struct mcp,
    pub ): *mut *mut void (disable)(struct mcp,
}

extern "C" {
    pub fn mcp_set_telecom_divisor(: *mut mcp, int: unsigned);
}
extern "C" {
    pub fn mcp_set_audio_divisor(: *mut mcp, int: unsigned);
}
extern "C" {
    pub fn mcp_reg_write(: *mut mcp, int: unsigned, int: unsigned);
}
extern "C" {
    pub fn mcp_reg_read(: *mut mcp, int: unsigned) -> c_uint;
}
extern "C" {
    pub fn mcp_enable(: *mut mcp);
}
extern "C" {
    pub fn mcp_disable(: *mut mcp);
}

extern "C" {
    pub fn mcp_host_add(: *mut mcp, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn mcp_host_del(: *mut mcp);
}
extern "C" {
    pub fn mcp_host_free(: *mut mcp);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_driver {
    pub drv: device_driver,
    pub ): *mut *mut int (probe)(struct mcp,
    pub ): *mut *mut void (remove)(struct mcp,
}

extern "C" {
    pub fn mcp_driver_register(: *mut mcp_driver) -> c_int;
}
extern "C" {
    pub fn mcp_driver_unregister(: *mut mcp_driver);
}

