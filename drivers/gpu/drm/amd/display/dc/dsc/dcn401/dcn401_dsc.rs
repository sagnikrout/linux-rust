//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dsc/dcn401/dcn401_dsc.h
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

// Macro flag: #define TO_DCN401_DSC(dsc)\
// Macro flag: #define DSC_REG_LIST_SH_MASK_DCN401(mask_sh)\
// DSC_SF(DSCC0_DSCC_CONFIG1, DSCC_DISABLE_ICH, mask_sh),*/ \
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_dsc_registers {
    pub DSC_TOP_CONTROL: u32,
    pub DSC_DEBUG_CONTROL: u32,
    pub DSCC_CONFIG0: u32,
    pub DSCC_CONFIG1: u32,
    pub DSCC_STATUS: u32,
    pub DSCC_INTERRUPT_CONTROL0: u32,
    pub DSCC_INTERRUPT_CONTROL1: u32,
    pub DSCC_INTERRUPT_STATUS0: u32,
    pub DSCC_INTERRUPT_STATUS1: u32,
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
    pub DSCC_MEM_POWER_CONTROL0: u32,
    pub DSCC_MEM_POWER_CONTROL1: u32,
    pub DSCC_R_Y_SQUARED_ERROR_LOWER: u32,
    pub DSCC_R_Y_SQUARED_ERROR_UPPER: u32,
    pub DSCC_G_CB_SQUARED_ERROR_LOWER: u32,
    pub DSCC_G_CB_SQUARED_ERROR_UPPER: u32,
    pub DSCC_B_CR_SQUARED_ERROR_LOWER: u32,
    pub DSCC_B_CR_SQUARED_ERROR_UPPER: u32,
    pub DSCC_MAX_ABS_ERROR0: u32,
    pub DSCC_MAX_ABS_ERROR1: u32,
    pub DSCC_TEST_DEBUG_BUS_ROTATE: u32,
    pub DSCCIF_CONFIG0: u32,
    pub DSCRM_DSC_FORWARD_CONFIG: u32,
    pub DSCC_RATE_BUFFER_MODEL_MAX_FULLNESS_LEVEL0: u32,
    pub DSCC_RATE_BUFFER_MODEL_MAX_FULLNESS_LEVEL1: u32,
    pub DSCC_RATE_BUFFER_MODEL_MAX_FULLNESS_LEVEL2: u32,
    pub DSCC_RATE_BUFFER_MODEL_MAX_FULLNESS_LEVEL3: u32,
    pub DSCC_OUTPUT_BUFFER_MAX_FULLNESS_LEVEL0: u32,
    pub DSCC_OUTPUT_BUFFER_MAX_FULLNESS_LEVEL1: u32,
    pub DSCC_OUTPUT_BUFFER_MAX_FULLNESS_LEVEL2: u32,
    pub DSCC_OUTPUT_BUFFER_MAX_FULLNESS_LEVEL3: u32,
}

// Macro flag: #define DSC_FIELD_LIST_DCN401(type)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_dsc_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_dsc_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn401_dsc {
    pub base: display_stream_compressor,
    pub dsc_regs: *const dcn401_dsc_registers,
    pub dsc_shift: *const dcn401_dsc_shift,
    pub dsc_mask: *const dcn401_dsc_mask,
    pub reg_vals: dsc_reg_values,
    pub max_image_width: c_int,
}

extern "C" {
    pub fn dsc401_set_fgcg(dsc401: *mut dcn401_dsc, enable: bool);
}
extern "C" {
    pub fn dsc401_read_state(dsc: *mut display_stream_compressor, s: *mut dcn_dsc_state);
}
extern "C" {
    pub fn dsc401_validate_stream(dsc: *mut display_stream_compressor, dsc_cfg: *const dsc_config) -> bool;
}
extern "C" {
    pub fn dsc401_enable(dsc: *mut display_stream_compressor, opp_pipe: c_int);
}
extern "C" {
    pub fn dsc401_disable(dsc: *mut display_stream_compressor);
}
extern "C" {
    pub fn dsc401_disconnect(dsc: *mut display_stream_compressor);
}
extern "C" {
    pub fn dsc401_wait_disconnect_pending_clear(dsc: *mut display_stream_compressor);
}
