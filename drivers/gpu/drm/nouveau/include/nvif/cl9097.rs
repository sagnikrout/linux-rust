//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/cl9097.h
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


// SPDX-License-Identifier: MIT
pub const FERMI_A_ZBC_COLOR: c_uint = 0x00;
pub const FERMI_A_ZBC_DEPTH: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fermi_a_zbc_color_v0 {
    pub version: __u8,
pub const FERMI_A_ZBC_COLOR_V0_FMT_ZERO: c_uint = 0x01;
pub const FERMI_A_ZBC_COLOR_V0_FMT_UNORM_ONE: c_uint = 0x02;
pub const FERMI_A_ZBC_COLOR_V0_FMT_RF32_GF32_BF32_AF32: c_uint = 0x04;
pub const FERMI_A_ZBC_COLOR_V0_FMT_R16_G16_B16_A16: c_uint = 0x08;
pub const FERMI_A_ZBC_COLOR_V0_FMT_RN16_GN16_BN16_AN16: c_uint = 0x0c;
pub const FERMI_A_ZBC_COLOR_V0_FMT_RS16_GS16_BS16_AS16: c_uint = 0x10;
pub const FERMI_A_ZBC_COLOR_V0_FMT_RU16_GU16_BU16_AU16: c_uint = 0x14;
pub const FERMI_A_ZBC_COLOR_V0_FMT_RF16_GF16_BF16_AF16: c_uint = 0x16;
pub const FERMI_A_ZBC_COLOR_V0_FMT_A8R8G8B8: c_uint = 0x18;
pub const FERMI_A_ZBC_COLOR_V0_FMT_A8RL8GL8BL8: c_uint = 0x1c;
pub const FERMI_A_ZBC_COLOR_V0_FMT_A2B10G10R10: c_uint = 0x20;
pub const FERMI_A_ZBC_COLOR_V0_FMT_AU2BU10GU10RU10: c_uint = 0x24;
pub const FERMI_A_ZBC_COLOR_V0_FMT_A8B8G8R8: c_uint = 0x28;
pub const FERMI_A_ZBC_COLOR_V0_FMT_A8BL8GL8RL8: c_uint = 0x2c;
pub const FERMI_A_ZBC_COLOR_V0_FMT_AN8BN8GN8RN8: c_uint = 0x30;
pub const FERMI_A_ZBC_COLOR_V0_FMT_AS8BS8GS8RS8: c_uint = 0x34;
pub const FERMI_A_ZBC_COLOR_V0_FMT_AU8BU8GU8RU8: c_uint = 0x38;
pub const FERMI_A_ZBC_COLOR_V0_FMT_A2R10G10B10: c_uint = 0x3c;
pub const FERMI_A_ZBC_COLOR_V0_FMT_BF10GF11RF11: c_uint = 0x40;
    pub format: __u8,
    pub index: __u8,
    pub pad03: [__u8; 5],
    pub ds: [__u32; 4],
    pub l2: [__u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fermi_a_zbc_depth_v0 {
    pub version: __u8,
pub const FERMI_A_ZBC_DEPTH_V0_FMT_FP32: c_uint = 0x01;
    pub format: __u8,
    pub index: __u8,
    pub pad03: [__u8; 5],
    pub ds: __u32,
    pub l2: __u32,
}
