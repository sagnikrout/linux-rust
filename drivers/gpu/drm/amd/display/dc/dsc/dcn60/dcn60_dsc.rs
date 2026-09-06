//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dsc/dcn60/dcn60_dsc.h
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
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Macro flag: #define TO_DCN60_DSC(dsc)\
// Macro flag: #define DSC_REG_LIST_SH_MASK_DCN60(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_dsc_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_dsc_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc60_reg_values {
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
    pub rc_buffer_model_overflow_int_en: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn60_dsc {
    pub base: display_stream_compressor,
    pub dsc_regs: *const dcn401_dsc_registers,
    pub dsc_shift: *const dcn60_dsc_shift,
    pub dsc_mask: *const dcn60_dsc_mask,
    pub reg_vals: dsc60_reg_values,
    pub max_image_width: c_int,
}

extern "C" {
    pub fn dsc60_set_fgcg(dsc: *mut display_stream_compressor, enable: bool);
}
extern "C" {
    pub fn dsc60_get_single_enc_caps(dsc_enc_caps: *mut dsc_enc_caps, max_dscclk_khz: c_uint);
}
