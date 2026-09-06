//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/powerplay/smumgr/tonga_smumgr.h
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
// Copyright 2015 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tonga_pt_defaults {
    pub svi_load_line_en: u8,
    pub svi_load_line_vddC: u8,
    pub tdc_vddc_throttle_release_limit_perc: u8,
    pub tdc_mawt: u8,
    pub tdc_waterfall_ctl: u8,
    pub dte_ambient_temp_base: u8,
    pub display_cac: u32,
    pub bapm_temp_gradient: u32,
    pub SMU72_DTE_SINKS]: *mut *mut *mut uint16_t bapmti_r[SMU72_DTE_ITERATIONS  SMU72_DTE_SOURCES,
    pub SMU72_DTE_SINKS]: *mut *mut *mut uint16_t bapmti_rc[SMU72_DTE_ITERATIONS  SMU72_DTE_SOURCES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tonga_mc_reg_entry {
    pub mclk_max: u32,
    pub mc_data: [u32; SMU72_DISCRETE_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tonga_mc_reg_table {
    pub registers*/: *mut *mut uint8_t last; / number of,
    pub used*/: *mut *mut uint8_t num_entries; / number of entries in mc_reg_table_entry,
    pub etc.*/: *mut *mut uint16_t validflag; / indicate the corresponding register is valid or not. 1: valid, 0: invalid. bit0->address[0], bit1->address[1],,
    pub mc_reg_table_entry: [tonga_mc_reg_entry; MAX_AC_TIMING_ENTRIES],
    pub mc_reg_address: [SMU72_Discrete_MCRegisterAddress; SMU72_DISCRETE_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tonga_smumgr {
    pub smu7_data: smu7_smumgr,
    pub smc_state_table: SMU72_Discrete_DpmTable,
    pub ulv_setting: SMU72_Discrete_Ulv,
    pub power_tune_table: SMU72_Discrete_PmFuses,
    pub power_tune_defaults: *const tonga_pt_defaults,
    pub mc_regs: SMU72_Discrete_MCRegisters,
    pub mc_reg_table: tonga_mc_reg_table,
}
