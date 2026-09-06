//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/txx9/ndfmc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// (C) Copyright TOSHIBA CORPORATION 2007
//
pub const NDFMC_PLAT_FLAG_USE_BSPRT: c_uint = 0x01;
pub const NDFMC_PLAT_FLAG_NO_RSTR: c_uint = 0x02;
pub const NDFMC_PLAT_FLAG_HOLDADD: c_uint = 0x04;
pub const NDFMC_PLAT_FLAG_DUMMYWRITE: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txx9ndfmc_platform_data {
    pub shift: c_uint,
    pub gbus_clock: c_uint,
    pub /: *mut *mut unsigned int hold; / hold time in nanosecond,
    pub /: *mut *mut unsigned int spw; / strobe pulse width in nanosecond,
    pub flags: c_uint,
    pub /: *mut *mut unsigned char ch_mask; / available channel bitmask,
    pub /: *mut *mut unsigned char wp_mask; / write-protect bitmask,
    pub /: *mut *mut unsigned char wide_mask; / 16bit-nand bitmask,
}
