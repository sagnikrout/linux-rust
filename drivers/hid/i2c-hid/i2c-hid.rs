//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/i2c-hid/i2c-hid.h
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


// SPDX-License-Identifier: GPL-2.0+

extern "C" {
    pub fn i2c_hid_get_dmi_quirks(vendor: u16, product: u16) -> u32;
}

// i2c_hid_get_dmi_i2c_hid_desc_override(uint8_t *i2c_name)

//
// struct i2chid_ops - Ops provided to the core.
//
// @power_up: do sequencing to power up the device.
// @power_down: do sequencing to power down the device.
// @shutdown_tail: called at the end of shutdown.
// @restore_sequence: hibernation restore sequence.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2chid_ops {
    pub ops): *mut *mut int (power_up)(struct i2chid_ops,
    pub ops): *mut *mut void (power_down)(struct i2chid_ops,
    pub ops): *mut *mut void (shutdown_tail)(struct i2chid_ops,
    pub ops): *mut *mut void (restore_sequence)(struct i2chid_ops,
}

extern "C" {
    pub fn i2c_hid_core_remove(client: *mut i2c_client);
}
extern "C" {
    pub fn i2c_hid_core_shutdown(client: *mut i2c_client);
}
