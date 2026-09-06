//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/nsc_gpio.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsc_gpio_ops {
    pub owner: *mut *mut module,
    pub bits): *mut *mut u32 (gpio_config) (unsigned iminor, u32 mask, u32,
    pub iminor): *mut *mut *mut void (gpio_dump) (struct nsc_gpio_ops amp, unsigned,
    pub iminor): *mut *mut int (gpio_get) (unsigned,
    pub state): *mut *mut void (gpio_set) (unsigned iminor, int,
    pub iminor): *mut *mut void (gpio_change) (unsigned,
    pub iminor): *mut *mut int (gpio_current) (unsigned,
    pub /: *mut *mut *mut device dev; / for dev_dbg() support, set in init,
}

extern "C" {
    pub fn nsc_gpio_dump(amp: *mut nsc_gpio_ops, index: unsigned);
}
