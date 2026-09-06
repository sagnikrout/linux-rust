//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/compressor.h
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
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
pub enum fbc_compress_ratio {
    FBC_COMPRESS_RATIO_INVALID = 0,
    FBC_COMPRESS_RATIO_1TO1 = 1,
    FBC_COMPRESS_RATIO_2TO1 = 2,
    FBC_COMPRESS_RATIO_4TO1 = 4,
    FBC_COMPRESS_RATIO_8TO1 = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fbc_physical_address {
    pub low_part: u32,
    pub high_part: i32,
    pub addr: },
    pub quad_part: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compr_addr_and_pitch_params {
// enum controller_id controller_id;
    pub inst: u32,
    pub source_view_width: u32,
    pub source_view_height: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fbc_hw_max_resolution_supported {
    FBC_MAX_X = 3840,
    FBC_MAX_Y = 2400,
    FBC_MAX_X_SG = 1920,
    FBC_MAX_Y_SG = 1080,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compressor_funcs {
    pub cp): *mut *mut void (power_up_fbc)(struct compressor,
    pub params): *mut compr_addr_and_pitch_params,
    pub cp): *mut *mut void (disable_fbc)(struct compressor,
    pub fbc_trigger): u32,
    pub params): *mut compr_addr_and_pitch_params,
    pub fbc_mapped_crtc_id): *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compressor {
    pub ctx: *mut dc_context,
// CONTROLLER_ID_D0 + instance, CONTROLLER_ID_UNDEFINED = 0
    pub attached_inst: u32,
    pub is_enabled: bool,
    pub funcs: *const compressor_funcs,
    pub raw: u32,
    pub FBC_SUPPORT:1: u32,
    pub FB_POOL:1: u32,
    pub DYNAMIC_ALLOC:1: u32,
    pub LPT_SUPPORT:1: u32,
    pub LPT_MC_CONFIG:1: u32,
    pub DUMMY_BACKEND:1: u32,
    pub CLK_GATING_DISABLED:1: u32,
    pub bits: },
    pub options: },
    pub compr_surface_address: fbc_physical_address,
    pub embedded_panel_h_size: u32,
    pub embedded_panel_v_size: u32,
    pub memory_bus_width: u32,
    pub banks_num: u32,
    pub raw_size: u32,
    pub channel_interleave_size: u32,
    pub dram_channels_num: u32,
    pub allocated_size: u32,
    pub preferred_requested_size: u32,
    pub lpt_channels_num: u32,
    pub min_compress_ratio: fbc_compress_ratio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbc_input_info {
    pub dynamic_fbc_buffer_alloc: bool,
    pub source_view_width: c_uint,
    pub source_view_height: c_uint,
    pub num_of_active_targets: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fbc_requested_compressed_size {
    pub preferred_size: c_uint,
    pub preferred_size_alignment: c_uint,
    pub min_size: c_uint,
    pub min_size_alignment: c_uint,
// Above preferedSize must be allocated in FB pool
    pub 1: unsigned int preferred_must_be_framebuffer_pool :,
// Above minSize must be allocated in FB pool
    pub 1: unsigned int min_must_be_framebuffer_pool :,
    pub bits: },
    pub flags: c_uint,
}
