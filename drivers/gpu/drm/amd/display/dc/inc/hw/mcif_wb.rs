//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/mcif_wb.h
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


// Copyright 2012-17 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmhubbub_wbif_mode {
    PACKED_444 = 0,
    PACKED_444_FP16 = 1,
    PLANAR_420_8BPC = 2,
    PLANAR_420_10BPC = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_arb_params {
    pub time_per_pixel: c_uint,
    pub cli_watermark: [c_uint; 4],
    pub pstate_watermark: [c_uint; 4],
    pub arbitration_slice: c_uint,
    pub slice_lines: c_uint,
    pub max_scaled_time: c_uint,
    pub dram_speed_change_duration: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_irq_params {
    pub sw_int_en: c_uint,
    pub sw_slice_int_en: c_uint,
    pub sw_overrun_int_en: c_uint,
    pub vce_int_en: c_uint,
    pub vce_slice_int_en: c_uint,
}

// / - mcif_wb_frame_dump_info is the info of the dumping WB data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_wb_frame_dump_info {
    pub size: c_uint,
    pub width: c_uint,
    pub height: c_uint,
    pub luma_pitch: c_uint,
    pub chroma_pitch: c_uint,
    pub format: dwb_scaler_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_wb {
    pub funcs: *const mcif_wb_funcs,
    pub ctx: *mut dc_context,
    pub inst: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcif_wb_funcs {
    pub params): *mut mcif_warmup_params,
    pub mcif_wb): *mut *mut void (enable_mcif)(struct mcif_wb,
    pub mcif_wb): *mut *mut void (disable_mcif)(struct mcif_wb,
    pub dest_height): c_uint,
    pub params): *mut mcif_arb_params,
    pub params): *mut mcif_irq_params,
    pub dest_chroma_buffer): *mut c_uchar,
}
