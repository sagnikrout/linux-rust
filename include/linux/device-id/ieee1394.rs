//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/ieee1394.h
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

pub const IEEE1394_MATCH_VENDOR_ID: c_uint = 0x0001;
pub const IEEE1394_MATCH_MODEL_ID: c_uint = 0x0002;
pub const IEEE1394_MATCH_SPECIFIER_ID: c_uint = 0x0004;
pub const IEEE1394_MATCH_VERSION: c_uint = 0x0008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee1394_device_id {
    pub match_flags: __u32,
    pub vendor_id: __u32,
    pub model_id: __u32,
    pub specifier_id: __u32,
    pub version: __u32,
    pub driver_data: kernel_ulong_t,
    pub driver_data_ptr: *const c_void,
}
