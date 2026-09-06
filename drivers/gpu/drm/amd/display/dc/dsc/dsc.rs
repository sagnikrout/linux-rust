//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dsc/dsc.h
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

// do not include any other headers
// or else it might break Edid Utility functionality.
//
// Input parameters for configuring DSC from the outside of DSC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_config {
    pub pic_width: u32,
    pub pic_height: u32,
    pub pixel_encoding: dc_pixel_encoding,
    pub /: *mut *mut dc_color_depth color_depth; / Bits per component,
    pub is_odm: bool,
    pub dc_dsc_cfg: dc_dsc_config,
    pub dsc_padding: u32,
}

// Output parameters for configuring DSC-related part of OPTC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_optc_config {
    pub /: *mut *mut uint32_t slice_width; / Slice width in pixels,
    pub /: *mut *mut uint32_t bytes_per_pixel; / Bytes per pixel in u3.28 format,
    pub /: *mut *mut bool is_pixel_format_444; / 'true' if pixel format is 'RGB 444' or 'Simple YCbCr 4:2:2' (4:2:2 upsampled to 4:4:4)',
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_dsc_state {
    pub dsc_clock_en: u32,
    pub dsc_slice_width: u32,
    pub dsc_bits_per_pixel: u32,
    pub dsc_slice_height: u32,
    pub dsc_pic_width: u32,
    pub dsc_pic_height: u32,
    pub dsc_slice_bpg_offset: u32,
    pub dsc_chunk_size: u32,
    pub dsc_fw_en: u32,
    pub dsc_opp_source: u32,
    pub dsc_block_pred_enable: u32,
    pub dsc_line_buf_depth: u32,
    pub dsc_version_minor: u32,
    pub dsc_rc_buffer_size: u32,
    pub dsc_simple_422: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_dsc_reg_state {
    pub dsc_top_control: u32,
    pub dscc_interrupt_control_status: u32,
}

// DSC encoder capabilities
// They differ from the DPCD DSC caps because they are based on AMD DSC encoder caps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union dsc_enc_slice_caps {
    pub 1: uint8_t NUM_SLICES_1 :,
    pub 1: uint8_t NUM_SLICES_2 :,
    pub /: *mut *mut uint8_t NUM_SLICES_3 : 1; / This one is not per DSC spec, but our encoder supports it,
    pub 1: uint8_t NUM_SLICES_4 :,
    pub 1: uint8_t NUM_SLICES_8 :,
    pub 1: uint8_t NUM_SLICES_12 :,
    pub 1: uint8_t NUM_SLICES_16 :,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_enc_caps {
    pub dsc_version: u8,
    pub slice_caps: dsc_enc_slice_caps,
    pub lb_bit_depth: i32,
    pub is_block_pred_supported: bool,
    pub color_formats: dsc_color_formats,
    pub color_depth: dsc_color_depth,
    pub /: *mut *mut int32_t max_total_throughput_mps; / Maximum total throughput with all the slices combined,
    pub max_slice_width: i32,
    pub /: *mut *mut uint32_t bpp_increment_div; / bpp increment divisor, e.g. if 16, it's 1/16th of a bit,
    pub is_frl: bool,
    pub is_vic_all_bpp: bool,
    pub total_chunk_kbytes: u32,
    pub num_lanes: u32,
    pub frl_rate: u32,
    pub edp_sink_max_bits_per_pixel: u32,
    pub is_dp: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_funcs {
    pub pixel_clock_100Hz): *mut *mut *mut void (dsc_get_enc_caps)(struct dsc_enc_caps dsc_enc_caps, int,
    pub s): *mut *mut *mut void (dsc_read_state)(struct display_stream_compressor dsc, struct dcn_dsc_state,
    pub dccg_reg_state): *mut *mut *mut void (dsc_read_reg_state)(struct display_stream_compressor dsc, struct dcn_dsc_reg_state,
    pub dsc_cfg): *const *const *const bool (dsc_validate_stream)(struct display_stream_compressor dsc, struct dsc_config,
    pub dsc_optc_cfg): *mut dsc_optc_config,
    pub dsc_packed_pps): *mut u8,
    pub opp_pipe): *mut *mut *mut void (dsc_enable)(struct display_stream_compressor dsc, int,
    pub dsc): *mut *mut void (dsc_disable)(struct display_stream_compressor,
    pub dsc): *mut *mut void (dsc_disconnect)(struct display_stream_compressor,
    pub dsc): *mut *mut void (dsc_wait_disconnect_pending_clear)(struct display_stream_compressor,
    pub max_dscclk_khz): *mut *mut *mut void (dsc_get_single_enc_caps)(struct dsc_enc_caps dsc_enc_caps, unsigned int,
    pub enable): *mut *mut *mut void (set_fgcg)(struct display_stream_compressor dsc, bool,
}
