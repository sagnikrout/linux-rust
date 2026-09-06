//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/olpc-ec.h
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


// SPDX-License-Identifier: GPL-2.0

// XO-1 EC commands
pub const EC_FIRMWARE_REV: c_uint = 0x08;
pub const EC_WRITE_SCI_MASK: c_uint = 0x1b;
pub const EC_WAKE_UP_WLAN: c_uint = 0x24;
pub const EC_WLAN_LEAVE_RESET: c_uint = 0x25;
pub const EC_DCON_POWER_MODE: c_uint = 0x26;
pub const EC_READ_EB_MODE: c_uint = 0x2a;
pub const EC_SET_SCI_INHIBIT: c_uint = 0x32;
pub const EC_SET_SCI_INHIBIT_RELEASE: c_uint = 0x34;
pub const EC_WLAN_ENTER_RESET: c_uint = 0x35;
pub const EC_WRITE_EXT_SCI_MASK: c_uint = 0x38;
pub const EC_SCI_QUERY: c_uint = 0x84;
pub const EC_EXT_SCI_QUERY: c_uint = 0x85;
// SCI source values

#[repr(C)]
#[derive(Copy, Clone)]
pub struct olpc_ec_driver {
    pub ): *mut *mut int (suspend)(struct platform_device,
    pub ): *mut *mut int (resume)(struct platform_device,
    pub ): *mut *mut *mut *mut int (ec_cmd)(u8, u8 , size_t, u8 , size_t, void,
    pub wakeup_available: bool,
}

extern "C" {
    pub fn olpc_ec_driver_register(drv: *mut olpc_ec_driver, arg: *mut c_void);
}
extern "C" {
    pub fn olpc_ec_wakeup_set(value: u16);
}
extern "C" {
    pub fn olpc_ec_wakeup_clear(value: u16);
}
extern "C" {
    pub fn olpc_ec_mask_write(bits: u16) -> c_int;
}
extern "C" {
    pub fn olpc_ec_sci_query(sci_value: *mut u16) -> c_int;
}
extern "C" {
    pub fn olpc_ec_wakeup_available() -> bool;
}
extern "C" {
    pub fn xo1_do_sleep(sleep_state: u8) -> asmlinkage int;
}

