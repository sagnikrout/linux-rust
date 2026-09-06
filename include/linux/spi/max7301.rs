//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/max7301.h
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
// Some registers must be read back to modify.
// To save time we cache them here in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max7301 {
    pub lock: mutex,
    pub /: *mut *mut u8 port_config[8]; / field 0 is unused,
    pub /: *mut *mut u32 out_level; / cached output levels,
    pub input_pullup_active: u32,
    pub chip: gpio_chip,
    pub dev: *mut device,
    pub val): *mut *mut *mut int (write)(struct device dev, unsigned int reg, unsigned int,
    pub reg): *mut *mut *mut int (read)(struct device dev, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max7301_platform_data {
// number assigned to the first GPIO
    pub base: unsigned,
//
// bitmask controlling the pullup configuration,
//
// _note_ the 4 lowest bits are unused, because the first 4
// ports of the controller are not used, too.
//
    pub input_pullup_active: u32,
}

extern "C" {
    pub fn __max730x_remove(dev: *mut device);
}
extern "C" {
    pub fn __max730x_probe(ts: *mut max7301) -> c_int;
}
