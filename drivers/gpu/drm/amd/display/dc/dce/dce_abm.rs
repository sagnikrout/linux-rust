//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_abm.h
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
// Copyright 2012-2026 Advanced Micro Devices, Inc.
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

// Macro flag: #define ABM_DCN10_REG_LIST(id)\

// Macro flag: #define ABM_DCN301_REG_LIST(id)\
// Macro flag: #define ABM_DCN302_REG_LIST(id)\
// Macro flag: #define ABM_DCN30_REG_LIST(id)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_abm_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_abm_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_abm_registers {
    pub DC_ABM1_HG_SAMPLE_RATE: u32,
    pub DC_ABM1_LS_SAMPLE_RATE: u32,
    pub BL1_PWM_BL_UPDATE_SAMPLE_RATE: u32,
    pub DC_ABM1_HG_MISC_CTRL: u32,
    pub DC_ABM1_IPCSC_COEFF_SEL: u32,
    pub BL1_PWM_CURRENT_ABM_LEVEL: u32,
    pub BL1_PWM_TARGET_ABM_LEVEL: u32,
    pub BL1_PWM_USER_LEVEL: u32,
    pub DC_ABM1_LS_MIN_MAX_PIXEL_VALUE_THRES: u32,
    pub DC_ABM1_HGLS_REG_READ_PROGRESS: u32,
    pub DC_ABM1_ACE_OFFSET_SLOPE_0: u32,
    pub DC_ABM1_ACE_OFFSET_SLOPE_DATA: u32,
    pub DC_ABM1_ACE_PWL_CNTL: u32,
    pub DC_ABM1_HG_BIN_33_40_SHIFT_INDEX: u32,
    pub DC_ABM1_HG_BIN_33_64_SHIFT_FLAG: u32,
    pub DC_ABM1_HG_BIN_41_48_SHIFT_INDEX: u32,
    pub DC_ABM1_HG_BIN_49_56_SHIFT_INDEX: u32,
    pub DC_ABM1_HG_BIN_57_64_SHIFT_INDEX: u32,
    pub DC_ABM1_HG_RESULT_DATA: u32,
    pub DC_ABM1_HG_RESULT_INDEX: u32,
    pub DC_ABM1_ACE_THRES_DATA: u32,
    pub DC_ABM1_ACE_THRES_12: u32,
    pub MASTER_COMM_CNTL_REG: u32,
    pub MASTER_COMM_CMD_REG: u32,
    pub MASTER_COMM_DATA_REG1: u32,
    pub BIOS_SCRATCH_2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_abm {
    pub base: abm,
    pub regs: *const dce_abm_registers,
    pub abm_shift: *const dce_abm_shift,
    pub abm_mask: *const dce_abm_mask,
}

extern "C" {
    pub fn dce_abm_destroy(abm: *mut abm);
}
