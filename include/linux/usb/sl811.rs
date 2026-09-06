//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/sl811.h
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
//
// board initialization should put one of these into dev->platform_data
// and place the sl811hs onto platform_bus named "sl811-hcd".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sl811_platform_data {
    pub can_wakeup:1: unsigned,
// given port_power, msec/2 after power on till power good
    pub potpg: u8,
// mA/2 power supplied on this port (max = default = 250)
    pub power: u8,
// sl811 relies on an external source of VBUS current
    pub is_on): *mut *mut *mut void (port_power)(struct device dev, int,
// pulse sl811 nRST (probably with a GPIO)
    pub dev): *mut *mut void (reset)(struct device,
// some boards need something like these:
// int		(*check_overcurrent)(struct device *dev);
// void		(*clock_enable)(struct device *dev, int is_on);
}
