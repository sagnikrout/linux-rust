//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/rtc/rtc-sa1100.h
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
pub struct sa1100_rtc {
    pub lock: spinlock_t,
    pub rcnr: *mut void __iomem,
    pub rtar: *mut void __iomem,
    pub rtsr: *mut void __iomem,
    pub rttr: *mut void __iomem,
    pub irq_1hz: c_int,
    pub irq_alarm: c_int,
    pub rtc: *mut rtc_device,
    pub clk: *mut clk,
}

extern "C" {
    pub fn sa1100_rtc_init(pdev: *mut platform_device, info: *mut sa1100_rtc) -> c_int;
}
