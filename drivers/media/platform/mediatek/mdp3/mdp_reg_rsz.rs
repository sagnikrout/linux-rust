//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mdp_reg_rsz.h
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//
pub const PRZ_ENABLE: c_uint = 0x000;
pub const PRZ_CONTROL_1: c_uint = 0x004;
pub const PRZ_CONTROL_2: c_uint = 0x008;
pub const PRZ_INPUT_IMAGE: c_uint = 0x010;
pub const PRZ_OUTPUT_IMAGE: c_uint = 0x014;
pub const PRZ_HORIZONTAL_COEFF_STEP: c_uint = 0x018;
pub const PRZ_VERTICAL_COEFF_STEP: c_uint = 0x01c;
pub const PRZ_LUMA_HORIZONTAL_INTEGER_OFFSET: c_uint = 0x020;
pub const PRZ_LUMA_HORIZONTAL_SUBPIXEL_OFFSET: c_uint = 0x024;
pub const PRZ_LUMA_VERTICAL_INTEGER_OFFSET: c_uint = 0x028;
pub const PRZ_LUMA_VERTICAL_SUBPIXEL_OFFSET: c_uint = 0x02c;
pub const PRZ_CHROMA_HORIZONTAL_INTEGER_OFFSET: c_uint = 0x030;
pub const PRZ_CHROMA_HORIZONTAL_SUBPIXEL_OFFSET: c_uint = 0x034;
pub const RSZ_ETC_CONTROL: c_uint = 0x22c;
// MASK
pub const PRZ_ENABLE_MASK: c_uint = 0x00010001;
pub const PRZ_CONTROL_1_MASK: c_uint = 0xfffffff3;
pub const PRZ_CONTROL_2_MASK: c_uint = 0x0ffffaff;
pub const PRZ_INPUT_IMAGE_MASK: c_uint = 0xffffffff;
pub const PRZ_OUTPUT_IMAGE_MASK: c_uint = 0xffffffff;
pub const PRZ_HORIZONTAL_COEFF_STEP_MASK: c_uint = 0x007fffff;
pub const PRZ_VERTICAL_COEFF_STEP_MASK: c_uint = 0x007fffff;
pub const PRZ_LUMA_HORIZONTAL_INTEGER_OFFSET_MASK: c_uint = 0x0000ffff;
pub const PRZ_LUMA_HORIZONTAL_SUBPIXEL_OFFSET_MASK: c_uint = 0x001fffff;
pub const PRZ_LUMA_VERTICAL_INTEGER_OFFSET_MASK: c_uint = 0x0000ffff;
pub const PRZ_LUMA_VERTICAL_SUBPIXEL_OFFSET_MASK: c_uint = 0x001fffff;
pub const PRZ_CHROMA_HORIZONTAL_INTEGER_OFFSET_MASK: c_uint = 0x0000ffff;
pub const PRZ_CHROMA_HORIZONTAL_SUBPIXEL_OFFSET_MASK: c_uint = 0x001fffff;
pub const RSZ_ETC_CONTROL_MASK: c_uint = 0xff770000;
