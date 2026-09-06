//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/i3c.h
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

// i3c
pub const I3C_MATCH_DCR: c_uint = 0x1;
pub const I3C_MATCH_MANUF: c_uint = 0x2;
pub const I3C_MATCH_PART: c_uint = 0x4;
pub const I3C_MATCH_EXTRA_INFO: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_device_id {
    pub match_flags: __u8,
    pub dcr: __u8,
    pub manuf_id: __u16,
    pub part_id: __u16,
    pub extra_info: __u16,
    pub data: *const c_void,
}
