//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/ssam.h
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

pub type kernel_ulong_t = c_ulong;

// Surface System Aggregator Module
pub const SSAM_MATCH_TARGET: c_uint = 0x1;
pub const SSAM_MATCH_INSTANCE: c_uint = 0x2;
pub const SSAM_MATCH_FUNCTION: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_device_id {
    pub match_flags: __u8,
    pub domain: __u8,
    pub category: __u8,
    pub target: __u8,
    pub instance: __u8,
    pub function: __u8,
    pub driver_data: kernel_ulong_t,
}
