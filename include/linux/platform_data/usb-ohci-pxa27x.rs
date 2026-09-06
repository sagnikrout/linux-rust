//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/usb-ohci-pxa27x.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxaohci_platform_data {
    pub ): *mut *mut int (init)(struct device,
    pub ): *mut *mut void (exit)(struct device,
    pub flags: c_ulong,

    pub ms: *mut *mut int power_on_delay; / Power On to Power Good time - in,
// HCD must wait for this duration before
// accessing a powered on port
//
    pub port_mode: c_int,
pub const PMM_NPS_MODE: c_int = 1;
pub const PMM_GLOBAL_MODE: c_int = 2;
pub const PMM_PERPORT_MODE: c_int = 3;
    pub power_budget: c_int,
}

extern "C" {
    pub fn pxa_set_ohci_info(info: *mut pxaohci_platform_data);
}
