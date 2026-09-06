//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/raspberrypi/pisp_be/pisp_be_formats.h
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
//
// PiSP Back End driver image format definitions.
//
// Copyright (c) 2021-2024 Raspberry Pi Ltd
//

pub const PISPBE_MAX_PLANES: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pisp_be_format {
    pub fourcc: c_uint,
    pub align: c_uint,
    pub bit_depth: c_uint,
// 0P3 factor for plane sizing
    pub plane_factor: [c_uint; PISPBE_MAX_PLANES],
    pub num_planes: c_uint,
    pub colorspace_mask: c_uint,
    pub colorspace_default: v4l2_colorspace,
}

//
// All three colour spaces SRGB, SMPTE170M and REC709 are fundamentally sRGB
// underneath (as near as makes no difference to us), just with different YCbCr
// encodings. Therefore the ISP can generate sRGB on its main output and any of
// the others on its low resolution output. Applications should, when using both
// outputs, program the colour spaces on them to be the same, matching whatever
// is requested for the low resolution output, even if the main output is
// producing an RGB format. In turn this requires us to allow all these colour
// spaces for every YUV/RGB output format.
//

// Single plane YUV formats
// 128 alignment to ensure U/V planes are 64 byte aligned.
// Multiplane YUV formats
// RGB formats
// Bayer formats - 8-bit
// Bayer formats - 16-bit
// Bayer formats unpacked to 16bpp
// 10 bit
// 12 bit
// 14 bit
// Bayer formats - 16-bit PiSP Compressed
// Greyscale Formats
// Configuration buffer format.
