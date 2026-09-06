//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/sc16is7xx.h
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
// SC16IS7xx SPI/I2C tty serial driver

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc16is7xx_devtype {
    pub name: [c_char; 10],
    pub nr_gpio: c_int,
    pub nr_uart: c_int,
}

extern "C" {
    pub fn sc16is7xx_regmap_port_mask(port_id: c_uint) -> c_uint;
}
extern "C" {
    pub fn sc16is7xx_remove(dev: *mut device);
}
