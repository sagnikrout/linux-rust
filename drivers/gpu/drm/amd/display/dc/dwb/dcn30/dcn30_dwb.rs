//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dwb/dcn30/dcn30_dwb.h
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


// Copyright 2020 Advanced Micro Devices, Inc.
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
#[derive(Copy, Clone)]
pub struct dcn30_dwbc_registers {
// DCN3AG
// DWB_TOP
    pub DWB_ENABLE_CLK_CTRL: u32,
    pub DWB_MEM_PWR_CTRL: u32,
    pub FC_MODE_CTRL: u32,
    pub FC_FLOW_CTRL: u32,
    pub FC_WINDOW_START: u32,
    pub FC_WINDOW_SIZE: u32,
    pub FC_SOURCE_SIZE: u32,
    pub DWB_UPDATE_CTRL: u32,
    pub DWB_CRC_CTRL: u32,
    pub DWB_CRC_MASK_R_G: u32,
    pub DWB_CRC_MASK_B_A: u32,
    pub DWB_CRC_VAL_R_G: u32,
    pub DWB_CRC_VAL_B_A: u32,
    pub DWB_OUT_CTRL: u32,
    pub DWB_MMHUBBUB_BACKPRESSURE_CNT_EN: u32,
    pub DWB_MMHUBBUB_BACKPRESSURE_CNT: u32,
    pub DWB_HOST_READ_CONTROL: u32,
    pub DWB_SOFT_RESET: u32,
    pub DWB_DEBUG_CTRL: u32,
    pub DWB_DEBUG: u32,
    pub DWB_TEST_DEBUG_INDEX: u32,
    pub DWB_TEST_DEBUG_DATA: u32,
// DWBSCL
    pub DWBSCL_COEF_RAM_TAP_SELECT: u32,
    pub DWBSCL_COEF_RAM_TAP_DATA: u32,
    pub DWBSCL_MODE: u32,
    pub DWBSCL_TAP_CONTROL: u32,
    pub DWBSCL_HORZ_FILTER_SCALE_RATIO: u32,
    pub DWBSCL_HORZ_FILTER_INIT: u32,
    pub DWBSCL_VERT_FILTER_SCALE_RATIO: u32,
    pub DWBSCL_VERT_FILTER_INIT: u32,
    pub DWBSCL_BOUNDARY_CTRL: u32,
    pub DWBSCL_DEST_SIZE: u32,
    pub DWBSCL_OVERFLOW_STATUS: u32,
    pub DWBSCL_OVERFLOW_COUNTER: u32,
    pub DWBSCL_DEBUG: u32,
    pub DWBSCL_TEST_DEBUG_INDEX: u32,
    pub DWBSCL_TEST_DEBUG_DATA: u32,
// DWBCP
    pub DWB_HDR_MULT_COEF: u32,
    pub DWB_GAMUT_REMAP_MODE: u32,
    pub DWB_GAMUT_REMAP_COEF_FORMAT: u32,
    pub DWB_GAMUT_REMAPA_C11_C12: u32,
    pub DWB_GAMUT_REMAPA_C13_C14: u32,
    pub DWB_GAMUT_REMAPA_C21_C22: u32,
    pub DWB_GAMUT_REMAPA_C23_C24: u32,
    pub DWB_GAMUT_REMAPA_C31_C32: u32,
    pub DWB_GAMUT_REMAPA_C33_C34: u32,
    pub DWB_GAMUT_REMAPB_C11_C12: u32,
    pub DWB_GAMUT_REMAPB_C13_C14: u32,
    pub DWB_GAMUT_REMAPB_C21_C22: u32,
    pub DWB_GAMUT_REMAPB_C23_C24: u32,
    pub DWB_GAMUT_REMAPB_C31_C32: u32,
    pub DWB_GAMUT_REMAPB_C33_C34: u32,
    pub DWB_OGAM_CONTROL: u32,
    pub DWB_OGAM_LUT_INDEX: u32,
    pub DWB_OGAM_LUT_DATA: u32,
    pub DWB_OGAM_LUT_CONTROL: u32,
    pub DWB_OGAM_RAMA_START_CNTL_B: u32,
    pub DWB_OGAM_RAMA_START_CNTL_G: u32,
    pub DWB_OGAM_RAMA_START_CNTL_R: u32,
    pub DWB_OGAM_RAMA_START_BASE_CNTL_B: u32,
    pub DWB_OGAM_RAMA_START_SLOPE_CNTL_B: u32,
    pub DWB_OGAM_RAMA_START_BASE_CNTL_G: u32,
    pub DWB_OGAM_RAMA_START_SLOPE_CNTL_G: u32,
    pub DWB_OGAM_RAMA_START_BASE_CNTL_R: u32,
    pub DWB_OGAM_RAMA_START_SLOPE_CNTL_R: u32,
    pub DWB_OGAM_RAMA_END_CNTL1_B: u32,
    pub DWB_OGAM_RAMA_END_CNTL2_B: u32,
    pub DWB_OGAM_RAMA_END_CNTL1_G: u32,
    pub DWB_OGAM_RAMA_END_CNTL2_G: u32,
    pub DWB_OGAM_RAMA_END_CNTL1_R: u32,
    pub DWB_OGAM_RAMA_END_CNTL2_R: u32,
    pub DWB_OGAM_RAMA_OFFSET_B: u32,
    pub DWB_OGAM_RAMA_OFFSET_G: u32,
    pub DWB_OGAM_RAMA_OFFSET_R: u32,
    pub DWB_OGAM_RAMA_REGION_0_1: u32,
    pub DWB_OGAM_RAMA_REGION_2_3: u32,
    pub DWB_OGAM_RAMA_REGION_4_5: u32,
    pub DWB_OGAM_RAMA_REGION_6_7: u32,
    pub DWB_OGAM_RAMA_REGION_8_9: u32,
    pub DWB_OGAM_RAMA_REGION_10_11: u32,
    pub DWB_OGAM_RAMA_REGION_12_13: u32,
    pub DWB_OGAM_RAMA_REGION_14_15: u32,
    pub DWB_OGAM_RAMA_REGION_16_17: u32,
    pub DWB_OGAM_RAMA_REGION_18_19: u32,
    pub DWB_OGAM_RAMA_REGION_20_21: u32,
    pub DWB_OGAM_RAMA_REGION_22_23: u32,
    pub DWB_OGAM_RAMA_REGION_24_25: u32,
    pub DWB_OGAM_RAMA_REGION_26_27: u32,
    pub DWB_OGAM_RAMA_REGION_28_29: u32,
    pub DWB_OGAM_RAMA_REGION_30_31: u32,
    pub DWB_OGAM_RAMA_REGION_32_33: u32,
    pub DWB_OGAM_RAMB_START_CNTL_B: u32,
    pub DWB_OGAM_RAMB_START_CNTL_G: u32,
    pub DWB_OGAM_RAMB_START_CNTL_R: u32,
    pub DWB_OGAM_RAMB_START_BASE_CNTL_B: u32,
    pub DWB_OGAM_RAMB_START_SLOPE_CNTL_B: u32,
    pub DWB_OGAM_RAMB_START_BASE_CNTL_G: u32,
    pub DWB_OGAM_RAMB_START_SLOPE_CNTL_G: u32,
    pub DWB_OGAM_RAMB_START_BASE_CNTL_R: u32,
    pub DWB_OGAM_RAMB_START_SLOPE_CNTL_R: u32,
    pub DWB_OGAM_RAMB_END_CNTL1_B: u32,
    pub DWB_OGAM_RAMB_END_CNTL2_B: u32,
    pub DWB_OGAM_RAMB_END_CNTL1_G: u32,
    pub DWB_OGAM_RAMB_END_CNTL2_G: u32,
    pub DWB_OGAM_RAMB_END_CNTL1_R: u32,
    pub DWB_OGAM_RAMB_END_CNTL2_R: u32,
    pub DWB_OGAM_RAMB_OFFSET_B: u32,
    pub DWB_OGAM_RAMB_OFFSET_G: u32,
    pub DWB_OGAM_RAMB_OFFSET_R: u32,
    pub DWB_OGAM_RAMB_REGION_0_1: u32,
    pub DWB_OGAM_RAMB_REGION_2_3: u32,
    pub DWB_OGAM_RAMB_REGION_4_5: u32,
    pub DWB_OGAM_RAMB_REGION_6_7: u32,
    pub DWB_OGAM_RAMB_REGION_8_9: u32,
    pub DWB_OGAM_RAMB_REGION_10_11: u32,
    pub DWB_OGAM_RAMB_REGION_12_13: u32,
    pub DWB_OGAM_RAMB_REGION_14_15: u32,
    pub DWB_OGAM_RAMB_REGION_16_17: u32,
    pub DWB_OGAM_RAMB_REGION_18_19: u32,
    pub DWB_OGAM_RAMB_REGION_20_21: u32,
    pub DWB_OGAM_RAMB_REGION_22_23: u32,
    pub DWB_OGAM_RAMB_REGION_24_25: u32,
    pub DWB_OGAM_RAMB_REGION_26_27: u32,
    pub DWB_OGAM_RAMB_REGION_28_29: u32,
    pub DWB_OGAM_RAMB_REGION_30_31: u32,
    pub DWB_OGAM_RAMB_REGION_32_33: u32,
    pub DWBCP_DEBUG: u32,
    pub DWBCP_TEST_DEBUG_INDEX: u32,
    pub DWBCP_TEST_DEBUG_DATA: u32,
}

// Internal enums / structs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwbscl_coef_filter_type_sel {
    DWBSCL_COEF_RAM_FILTER_TYPE_VERT_RGB = 0,
    DWBSCL_COEF_RAM_FILTER_TYPE_HORZ_RGB = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn30_dwbc_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn30_dwbc_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn30_dwbc {
    pub base: dwbc,
    pub dwbc_regs: *const dcn30_dwbc_registers,
    pub dwbc_shift: *const dcn30_dwbc_shift,
    pub dwbc_mask: *const dcn30_dwbc_mask,
}

extern "C" {
    pub fn dwb3_enable(dwbc: *mut dwbc, params: *mut dc_dwb_params) -> bool;
}
extern "C" {
    pub fn dwb3_disable(dwbc: *mut dwbc) -> bool;
}
extern "C" {
    pub fn dwb3_update(dwbc: *mut dwbc, params: *mut dc_dwb_params) -> bool;
}
extern "C" {
    pub fn dwb3_is_enabled(dwbc: *mut dwbc) -> bool;
}
extern "C" {
    pub fn dwb3_set_fc_enable(dwbc: *mut dwbc, enable: dwb_frame_capture_enable);
}
extern "C" {
    pub fn dwb3_set_denorm(dwbc: *mut dwbc, params: *mut dc_dwb_params);
}
