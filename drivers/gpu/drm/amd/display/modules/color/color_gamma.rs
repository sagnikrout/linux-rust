//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/modules/color/color_gamma.h
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


//
// Copyright 2016 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

// For SetRegamma ADL interface support
// Must match escape type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union regamma_flags {
    pub raw: c_uint,
    pub use: unsigned int gammaRampArray :1; // RegammaRamp is in,
    pub use: unsigned int gammaFromEdid :1; //gamma from edid is in,
    pub 1.2: unsigned int gammaFromEdidEx :1; //gamma from edid is in use , but only for Display Id,
    pub used: unsigned int gammaFromUser :1; //user custom gamma is,
    pub use: unsigned int coeffFromUser :1; //coeff. A0-A3 from user is in,
    pub use: unsigned int coeffFromEdid :1; //coeff. A0-A3 from edid is in,
    pub driver: unsigned int applyDegamma :1; //flag for additional degamma correction in,
    pub gamma: unsigned int gammaPredefinedSRGB :1; //flag for SRGB,
    pub gamma: unsigned int gammaPredefinedPQ :1; //flag for PQ,
    pub nits: unsigned int gammaPredefinedPQ2084Interim :1; //flag for PQ gamma, lower max,
    pub gamma: unsigned int gammaPredefined36 :1; //flag for 3.6,
    pub gamma: unsigned int gammaPredefinedReset :1; //flag to return to previous,
    pub bits: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regamma_ramp {
    pub b: *mut *mut unsigned short gamma[2563]; // gamma ramp packed in same way as OS windows ,r , g &,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regamma_coeff {
    pub gamma: [c_int; 3],
    pub A0: [c_int; 3],
    pub A1: [c_int; 3],
    pub A2: [c_int; 3],
    pub A3: [c_int; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regamma_lut {
    pub flags: regamma_flags,
    pub ramp: regamma_ramp,
    pub coeff: regamma_coeff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_tm_params {
    pub sdr_white_level: c_uint,
    pub nits: unsigned int min_content; // luminance in 1/10000,
    pub nits: unsigned int max_content; // luminance in,
    pub nits: unsigned int min_display; // luminance in 1/10000,
    pub nits: unsigned int max_display; // luminance in,
    pub tm: unsigned int skip_tm; // skip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct calculate_buffer {
    pub buffer_index: c_int,
    pub buffer: [fixed31_32; NUM_PTS_IN_REGION],
    pub gamma_of_2: fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct translate_from_linear_space_args {
    pub arg: fixed31_32,
    pub a0: fixed31_32,
    pub a1: fixed31_32,
    pub a2: fixed31_32,
    pub a3: fixed31_32,
    pub gamma: fixed31_32,
    pub cal_buffer: *mut calculate_buffer,
}

extern "C" {
    pub fn setup_x_points_distribution();
}
extern "C" {
    pub fn log_x_points_distribution(logger: *mut dal_logger);
}
extern "C" {
    pub fn precompute_pq();
}
extern "C" {
    pub fn precompute_de_pq();
}
