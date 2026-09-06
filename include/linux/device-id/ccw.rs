//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/ccw.h
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

pub const CCW_DEVICE_ID_MATCH_CU_TYPE: c_uint = 0x01;
pub const CCW_DEVICE_ID_MATCH_CU_MODEL: c_uint = 0x02;
pub const CCW_DEVICE_ID_MATCH_DEVICE_TYPE: c_uint = 0x04;
pub const CCW_DEVICE_ID_MATCH_DEVICE_MODEL: c_uint = 0x08;
// s390 CCW devices
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_device_id {
    pub /: *mut *mut __u16 match_flags; / which fields to match against,
    pub /: *mut *mut __u16 cu_type; / control unit type,
    pub /: *mut *mut __u16 dev_type; / device type,
    pub /: *mut *mut __u8 cu_model; / control unit model,
    pub /: *mut *mut __u8 dev_model; / device model,
    pub driver_info: kernel_ulong_t,
}
