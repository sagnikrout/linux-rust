//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mfd/simple-mfd-i2c.h
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
// Simple MFD - I2C
//
// Author: Lee Jones <lee.jones@linaro.org>
//
// This driver creates a single register map with the intention for it to be
// shared by all sub-devices.  Children can use their parent's device structure
// (dev.parent) in order to reference it.
//
// This driver creates a single register map with the intention for it to be
// shared by all sub-devices.  Children can use their parent's device structure
// (dev.parent) in order to reference it.
//
// Once the register map has been successfully initialised, any sub-devices
// represented by child nodes in Device Tree or via the MFD cells in the
// associated C file will be subsequently registered.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_mfd_data {
    pub regmap_config: *const regmap_config,
    pub mfd_cell: *const mfd_cell,
    pub mfd_cell_size: usize,
}
