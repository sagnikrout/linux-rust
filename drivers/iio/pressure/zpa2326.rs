//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/pressure/zpa2326.h
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
// Murata ZPA2326 pressure and temperature sensor IIO driver
//
// Copyright (c) 2016 Parrot S.A.
//
// Author: Gregor Boirie <gregor.boirie@parrot.com>
//
// Register map.

extern "C" {
    pub fn zpa2326_isreg_writeable(dev: *mut device, reg: c_uint) -> bool;
}
extern "C" {
    pub fn zpa2326_isreg_readable(dev: *mut device, reg: c_uint) -> bool;
}
extern "C" {
    pub fn zpa2326_isreg_precious(dev: *mut device, reg: c_uint) -> bool;
}
//
// zpa2326_probe() - Instantiate and register core ZPA2326 IIO device
// @parent: Hardware sampling device the created IIO device will be a child of.
// @name:   Arbitrary name to identify the device.
// @irq:    Interrupt line, negative if none.
// @hwid:   Expected device hardware id.
// @regmap: Registers map used to abstract underlying bus accesses.
//
// Return: Zero when successful, a negative error code otherwise.
//
// zpa2326_remove() - Unregister and destroy core ZPA2326 IIO device.
// @parent: Hardware sampling device the IIO device to remove is a child of.
//
extern "C" {
    pub fn zpa2326_remove(parent: *const device);
}

