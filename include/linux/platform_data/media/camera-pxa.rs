//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/media/camera-pxa.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
pub const PXA_CAMERA_MASTER: c_int = 1;
pub const PXA_CAMERA_DATAWIDTH_4: c_int = 2;
pub const PXA_CAMERA_DATAWIDTH_5: c_int = 4;
pub const PXA_CAMERA_DATAWIDTH_8: c_int = 8;
pub const PXA_CAMERA_DATAWIDTH_9: c_uint = 0x10;
pub const PXA_CAMERA_DATAWIDTH_10: c_uint = 0x20;
pub const PXA_CAMERA_PCLK_EN: c_uint = 0x40;
pub const PXA_CAMERA_MCLK_EN: c_uint = 0x80;
pub const PXA_CAMERA_PCP: c_uint = 0x100;
pub const PXA_CAMERA_HSP: c_uint = 0x200;
pub const PXA_CAMERA_VSP: c_uint = 0x400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxacamera_platform_data {
    pub flags: c_ulong,
    pub mclk_10khz: c_ulong,
    pub sensor_i2c_adapter_id: c_int,
    pub sensor_i2c_address: c_int,
}

extern "C" {
    pub fn pxa_set_camera_info(: *mut pxacamera_platform_data);
}
