//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dsc/dcn20/dcn20_dsc.h
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

// Macro flag: #define TO_DCN20_DSC(dsc)\

// Used in resolving the corner case with duplicate field name

// Macro flag: #define DSC_REG_LIST_SH_MASK_DCN20(mask_sh)\
// DSC_SF(DSCC0_DSCC_CONFIG1, DSCC_DISABLE_ICH, mask_sh),*/ \
// Macro flag: #define DSC_FIELD_LIST_DCN20(type)\
// type DSCC_DISABLE_ICH;*/ \
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsc_bits_per_comp {
    DSC_BPC_8 = 8,
    DSC_BPC_10 = 10,
    DSC_BPC_12 = 12,
    DSC_BPC_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn20_dsc_registers {
    pub DSC_TOP_CONTROL: u32,
    pub DSC_DEBUG_CONTROL: u32,
    pub DSCC_CONFIG0: u32,
    pub DSCC_CONFIG1: u32,
    pub DSCC_STATUS: u32,
    pub DSCC_INTERRUPT_CONTROL_STATUS: u32,
    pub DSCC_PPS_CONFIG0: u32,
    pub DSCC_PPS_CONFIG1: u32,
    pub DSCC_PPS_CONFIG2: u32,
    pub DSCC_PPS_CONFIG3: u32,
    pub DSCC_PPS_CONFIG4: u32,
    pub DSCC_PPS_CONFIG5: u32,
    pub DSCC_PPS_CONFIG6: u32,
    pub DSCC_PPS_CONFIG7: u32,
    pub DSCC_PPS_CONFIG8: u32,
    pub DSCC_PPS_CONFIG9: u32,
    pub DSCC_PPS_CONFIG10: u32,
    pub DSCC_PPS_CONFIG11: u32,
    pub DSCC_PPS_CONFIG12: u32,
    pub DSCC_PPS_CONFIG13: u32,
    pub DSCC_PPS_CONFIG14: u32,
    pub DSCC_PPS_CONFIG15: u32,
    pub DSCC_PPS_CONFIG16: u32,
    pub DSCC_PPS_CONFIG17: u32,
    pub DSCC_PPS_CONFIG18: u32,
    pub DSCC_PPS_CONFIG19: u32,
    pub DSCC_PPS_CONFIG20: u32,
    pub DSCC_PPS_CONFIG21: u32,
    pub DSCC_PPS_CONFIG22: u32,
    pub DSCC_MEM_POWER_CONTROL: u32,
    pub DSCC_R_Y_SQUARED_ERROR_LOWER: u32,
    pub DSCC_R_Y_SQUARED_ERROR_UPPER: u32,
    pub DSCC_G_CB_SQUARED_ERROR_LOWER: u32,
    pub DSCC_G_CB_SQUARED_ERROR_UPPER: u32,
    pub DSCC_B_CR_SQUARED_ERROR_LOWER: u32,
    pub DSCC_B_CR_SQUARED_ERROR_UPPER: u32,
    pub DSCC_MAX_ABS_ERROR0: u32,
    pub DSCC_MAX_ABS_ERROR1: u32,
    pub DSCC_RATE_BUFFER0_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_RATE_BUFFER1_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_RATE_BUFFER2_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_RATE_BUFFER3_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_RATE_CONTROL_BUFFER0_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_RATE_CONTROL_BUFFER1_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_RATE_CONTROL_BUFFER2_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_RATE_CONTROL_BUFFER3_MAX_FULLNESS_LEVEL: u32,
    pub DSCC_TEST_DEBUG_BUS_ROTATE: u32,
    pub DSCCIF_CONFIG0: u32,
    pub DSCCIF_CONFIG1: u32,
    pub DSCRM_DSC_FORWARD_CONFIG: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn20_dsc_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn20_dsc_mask {
}

// DSCCIF_CONFIG.INPUT_PIXEL_FORMAT values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsc_pixel_format {
    DSC_PIXFMT_RGB,
    DSC_PIXFMT_YCBCR444,
    DSC_PIXFMT_SIMPLE_YCBCR422,
    DSC_PIXFMT_NATIVE_YCBCR422,
    DSC_PIXFMT_NATIVE_YCBCR420,
    DSC_PIXFMT_UNKNOWN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_reg_values {
// PPS registers
    pub pps: drm_dsc_config,
// Additional registers
    pub dsc_clock_enable: u32,
    pub dsc_clock_gating_disable: u32,
    pub underflow_recovery_en: u32,
    pub underflow_occurred_int_en: u32,
    pub underflow_occurred_status: u32,
    pub pixel_format: dsc_pixel_format,
    pub ich_reset_at_eol: u32,
    pub alternate_ich_encoding_en: u32,
    pub num_slices_h: u32,
    pub num_slices_v: u32,
    pub rc_buffer_model_size: u32,
    pub disable_ich: u32,
    pub bpp_x32: u32,
    pub dsc_dbg_en: u32,
    pub rc_buffer_model_overflow_int_en: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn20_dsc {
    pub base: display_stream_compressor,
    pub dsc_regs: *const dcn20_dsc_registers,
    pub dsc_shift: *const dcn20_dsc_shift,
    pub dsc_mask: *const dcn20_dsc_mask,
    pub reg_vals: dsc_reg_values,
    pub max_image_width: c_int,
}

extern "C" {
    pub fn dsc_dc_color_depth_to_dsc_bits_per_comp(dc_color_depth: dc_color_depth) -> dsc_bits_per_comp;
}
extern "C" {
    pub fn dsc_init_reg_values(reg_vals: *mut dsc_reg_values);
}
extern "C" {
    pub fn dsc_update_from_dsc_parameters(reg_vals: *mut dsc_reg_values, dsc_params: *const dsc_parameters);
}
extern "C" {
    pub fn dsc2_read_state(dsc: *mut display_stream_compressor, s: *mut dcn_dsc_state);
}
extern "C" {
    pub fn dsc2_read_reg_state(dsc: *mut display_stream_compressor, dccg_reg_state: *mut dcn_dsc_reg_state);
}
extern "C" {
    pub fn dsc2_validate_stream(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config) -> bool;
}
extern "C" {
    pub fn dsc2_enable(dsc: *mut display_stream_compressor, opp_pipe: c_int);
}
extern "C" {
    pub fn dsc2_disable(dsc: *mut display_stream_compressor);
}
extern "C" {
    pub fn dsc2_disconnect(dsc: *mut display_stream_compressor);
}
extern "C" {
    pub fn dsc2_wait_disconnect_pending_clear(dsc: *mut display_stream_compressor);
}
