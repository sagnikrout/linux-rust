//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/softing/softing_platform.h
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

// softing firmware directory prefix

#[repr(C)]
#[derive(Copy, Clone)]
pub struct softing_platform_data {
    pub manf: c_uint,
    pub prod: c_uint,
//
// generation
// 1st with NEC or SJA1000
// 8bit, exclusive interrupt, ...
// 2nd only SJA1000
// 16bit, shared interrupt
//
    pub generation: c_int,
    pub /: *mut *mut int nbus; / # buses on device,
    pub /: *mut *mut unsigned int freq; / operating frequency in Hz,
    pub max_brp: c_uint,
    pub max_sjw: c_uint,
    pub dpram_size: c_ulong,
    pub name: *const c_char,
    pub offs: c_ulong,
    pub addr: c_ulong,
    pub fw: *const c_char,
    pub app: } boot, load,,
//
// reset() function
// bring pdev in or out of reset, depending on value
//
    pub value): *mut *mut *mut int (reset)(struct platform_device pdev, int,
    pub value): *mut *mut *mut int (enable_irq)(struct platform_device pdev, int,
}
