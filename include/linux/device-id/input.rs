//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/input.h
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

// Input
pub const INPUT_DEVICE_ID_EV_MAX: c_uint = 0x1f;
pub const INPUT_DEVICE_ID_KEY_MIN_INTERESTING: c_uint = 0x71;
pub const INPUT_DEVICE_ID_KEY_MAX: c_uint = 0x2ff;
pub const INPUT_DEVICE_ID_REL_MAX: c_uint = 0x0f;
pub const INPUT_DEVICE_ID_ABS_MAX: c_uint = 0x3f;
pub const INPUT_DEVICE_ID_MSC_MAX: c_uint = 0x07;
pub const INPUT_DEVICE_ID_LED_MAX: c_uint = 0x0f;
pub const INPUT_DEVICE_ID_SND_MAX: c_uint = 0x07;
pub const INPUT_DEVICE_ID_FF_MAX: c_uint = 0x7f;
pub const INPUT_DEVICE_ID_SW_MAX: c_uint = 0x11;
pub const INPUT_DEVICE_ID_PROP_MAX: c_uint = 0x1f;
pub const INPUT_DEVICE_ID_MATCH_BUS: c_int = 1;
pub const INPUT_DEVICE_ID_MATCH_VENDOR: c_int = 2;
pub const INPUT_DEVICE_ID_MATCH_PRODUCT: c_int = 4;
pub const INPUT_DEVICE_ID_MATCH_VERSION: c_int = 8;
pub const INPUT_DEVICE_ID_MATCH_EVBIT: c_uint = 0x0010;
pub const INPUT_DEVICE_ID_MATCH_KEYBIT: c_uint = 0x0020;
pub const INPUT_DEVICE_ID_MATCH_RELBIT: c_uint = 0x0040;
pub const INPUT_DEVICE_ID_MATCH_ABSBIT: c_uint = 0x0080;
pub const INPUT_DEVICE_ID_MATCH_MSCIT: c_uint = 0x0100;
pub const INPUT_DEVICE_ID_MATCH_LEDBIT: c_uint = 0x0200;
pub const INPUT_DEVICE_ID_MATCH_SNDBIT: c_uint = 0x0400;
pub const INPUT_DEVICE_ID_MATCH_FFBIT: c_uint = 0x0800;
pub const INPUT_DEVICE_ID_MATCH_SWBIT: c_uint = 0x1000;
pub const INPUT_DEVICE_ID_MATCH_PROPBIT: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_device_id {
    pub flags: kernel_ulong_t,
    pub bustype: __u16,
    pub vendor: __u16,
    pub product: __u16,
    pub version: __u16,
    pub 1]: kernel_ulong_t evbit[INPUT_DEVICE_ID_EV_MAX / BITS_PER_LONG +,
    pub 1]: kernel_ulong_t keybit[INPUT_DEVICE_ID_KEY_MAX / BITS_PER_LONG +,
    pub 1]: kernel_ulong_t relbit[INPUT_DEVICE_ID_REL_MAX / BITS_PER_LONG +,
    pub 1]: kernel_ulong_t absbit[INPUT_DEVICE_ID_ABS_MAX / BITS_PER_LONG +,
    pub 1]: kernel_ulong_t mscbit[INPUT_DEVICE_ID_MSC_MAX / BITS_PER_LONG +,
    pub 1]: kernel_ulong_t ledbit[INPUT_DEVICE_ID_LED_MAX / BITS_PER_LONG +,
    pub 1]: kernel_ulong_t sndbit[INPUT_DEVICE_ID_SND_MAX / BITS_PER_LONG +,
    pub 1]: kernel_ulong_t ffbit[INPUT_DEVICE_ID_FF_MAX / BITS_PER_LONG +,
    pub 1]: kernel_ulong_t swbit[INPUT_DEVICE_ID_SW_MAX / BITS_PER_LONG +,
    pub 1]: kernel_ulong_t propbit[INPUT_DEVICE_ID_PROP_MAX / BITS_PER_LONG +,
    pub driver_info: kernel_ulong_t,
}
