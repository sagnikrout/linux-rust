//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gpio-pxa.h
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

// NOTE: some PXAs have fewer on-chip GPIOs (like PXA255, with 85).
// Those cases currently cause holes in the GPIO number space, the
// actual number of the last GPIO is recorded by 'pxa_last_gpio'.
//
extern "C" {
    pub fn pxa_irq_to_gpio(irq: c_int) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa_gpio_platform_data {
    pub irq_base: c_int,
    pub on): *mut *mut int (gpio_set_wake)(unsigned int gpio, unsigned int,
}
