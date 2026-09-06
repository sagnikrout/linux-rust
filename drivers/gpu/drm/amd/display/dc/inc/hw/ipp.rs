//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/ipp.h
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
// Copyright 2017 Advanced Micro Devices, Inc.
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

pub const MAXTRIX_COEFFICIENTS_NUMBER: c_int = 12;

pub const MAX_OVL_MATRIX_COUNT: c_int = 12;
// IPP RELATED
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_pixel_processor {
    pub ctx: *mut dc_context,
    pub inst: c_uint,
    pub funcs: *const ipp_funcs,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipp_prescale_mode {
    IPP_PRESCALE_MODE_BYPASS,
    IPP_PRESCALE_MODE_FIXED_SIGNED,
    IPP_PRESCALE_MODE_FLOAT_SIGNED,
    IPP_PRESCALE_MODE_FIXED_UNSIGNED,
    IPP_PRESCALE_MODE_FLOAT_UNSIGNED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipp_prescale_params {
    pub mode: ipp_prescale_mode,
    pub bias: u16,
    pub scale: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovl_color_space {
    OVL_COLOR_SPACE_UNKNOWN = 0,
    OVL_COLOR_SPACE_RGB,
    OVL_COLOR_SPACE_YUV601,
    OVL_COLOR_SPACE_YUV709
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipp_funcs {
// cursor
    pub param): *const dc_cursor_mi_param,
    pub attributes): *const dc_cursor_attributes,
// setup input pixel processing
// put the entire pixel processor to bypass
    pub ipp): *mut input_pixel_processor,
// setup ipp to expand/convert input to pixel processor internal format
    pub input_color_space): dc_color_space,
// DCE function to setup IPP.  TODO: see if we can consolidate to setup
    pub params): *mut ipp_prescale_params,
    pub gamma): *const dc_gamma,
// DEGAMMA RELATED
    pub mode): ipp_degamma_mode,
    pub params): *const pwl_params,
    pub ipp): *mut *mut void (ipp_destroy)(struct input_pixel_processor,
}
