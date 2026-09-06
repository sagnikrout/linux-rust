//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rv770_smc.h
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
// Copyright 2011 Advanced Micro Devices, Inc.
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

pub const RV770_SMC_TABLE_ADDRESS: c_uint = 0xB000;
pub const RV770_SMC_PERFORMANCE_LEVELS_PER_SWSTATE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RV770_SMC_SCLK_VALUE {
    pub vCG_SPLL_FUNC_CNTL: u32,
    pub vCG_SPLL_FUNC_CNTL_2: u32,
    pub vCG_SPLL_FUNC_CNTL_3: u32,
    pub vCG_SPLL_SPREAD_SPECTRUM: u32,
    pub vCG_SPLL_SPREAD_SPECTRUM_2: u32,
    pub sclk_value: u32,
}

pub type RV770_SMC_SCLK_VALUE = RV770_SMC_SCLK_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RV770_SMC_MCLK_VALUE {
    pub vMPLL_AD_FUNC_CNTL: u32,
    pub vMPLL_AD_FUNC_CNTL_2: u32,
    pub vMPLL_DQ_FUNC_CNTL: u32,
    pub vMPLL_DQ_FUNC_CNTL_2: u32,
    pub vMCLK_PWRMGT_CNTL: u32,
    pub vDLL_CNTL: u32,
    pub vMPLL_SS: u32,
    pub vMPLL_SS2: u32,
    pub mclk_value: u32,
}

pub type RV770_SMC_MCLK_VALUE = RV770_SMC_MCLK_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RV730_SMC_MCLK_VALUE {
    pub vMCLK_PWRMGT_CNTL: u32,
    pub vDLL_CNTL: u32,
    pub vMPLL_FUNC_CNTL: u32,
    pub vMPLL_FUNC_CNTL2: u32,
    pub vMPLL_FUNC_CNTL3: u32,
    pub vMPLL_SS: u32,
    pub vMPLL_SS2: u32,
    pub mclk_value: u32,
}

pub type RV730_SMC_MCLK_VALUE = RV730_SMC_MCLK_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RV770_SMC_VOLTAGE_VALUE {
    pub value: u16,
    pub index: u8,
    pub padding: u8,
}

pub type RV770_SMC_VOLTAGE_VALUE = RV770_SMC_VOLTAGE_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub union RV7XX_SMC_MCLK_VALUE {
    pub mclk770: RV770_SMC_MCLK_VALUE,
    pub mclk730: RV730_SMC_MCLK_VALUE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RV770_SMC_HW_PERFORMANCE_LEVEL {
    pub arbValue: u8,
    pub seqValue: u8,
    pub ACIndex: u8,
}

pub const SMC_STROBE_RATIO: c_uint = 0x0F;
pub const SMC_STROBE_ENABLE: c_uint = 0x10;
pub const SMC_MC_EDC_RD_FLAG: c_uint = 0x01;
pub const SMC_MC_EDC_WR_FLAG: c_uint = 0x02;
pub const SMC_MC_RTT_ENABLE: c_uint = 0x04;
pub const SMC_MC_STUTTER_EN: c_uint = 0x08;
pub type RV770_SMC_HW_PERFORMANCE_LEVEL = RV770_SMC_HW_PERFORMANCE_LEVEL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RV770_SMC_SWSTATE {
    pub flags: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub levels: [RV770_SMC_HW_PERFORMANCE_LEVEL; RV770_SMC_PERFORMANCE_LEVELS_PER_SWSTATE],
}

pub type RV770_SMC_SWSTATE = RV770_SMC_SWSTATE;
pub const RV770_SMC_VOLTAGEMASK_VDDC: c_int = 0;
pub const RV770_SMC_VOLTAGEMASK_MVDD: c_int = 1;
pub const RV770_SMC_VOLTAGEMASK_VDDCI: c_int = 2;
pub const RV770_SMC_VOLTAGEMASK_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RV770_SMC_VOLTAGEMASKTABLE {
    pub highMask: [u8; RV770_SMC_VOLTAGEMASK_MAX],
    pub lowMask: [u32; RV770_SMC_VOLTAGEMASK_MAX],
}

pub type RV770_SMC_VOLTAGEMASKTABLE = RV770_SMC_VOLTAGEMASKTABLE;
pub const MAX_NO_VREG_STEPS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RV770_SMC_STATETABLE {
    pub thermalProtectType: u8,
    pub systemFlags: u8,
    pub maxVDDCIndexInPPTable: u8,
    pub extraFlags: u8,
    pub highSMIO: [u8; MAX_NO_VREG_STEPS],
    pub lowSMIO: [u32; MAX_NO_VREG_STEPS],
    pub voltageMaskTable: RV770_SMC_VOLTAGEMASKTABLE,
    pub initialState: RV770_SMC_SWSTATE,
    pub ACPIState: RV770_SMC_SWSTATE,
    pub driverState: RV770_SMC_SWSTATE,
    pub ULVState: RV770_SMC_SWSTATE,
}

pub type RV770_SMC_STATETABLE = RV770_SMC_STATETABLE;
pub const PPSMC_STATEFLAG_AUTO_PULSE_SKIP: c_uint = 0x01;

pub const RV770_SMC_SOFT_REGISTERS_START: c_uint = 0x104;
pub const RV770_SMC_SOFT_REGISTER_mclk_chg_timeout: c_uint = 0x0;
pub const RV770_SMC_SOFT_REGISTER_baby_step_timer: c_uint = 0x8;
pub const RV770_SMC_SOFT_REGISTER_delay_bbias: c_uint = 0xC;
pub const RV770_SMC_SOFT_REGISTER_delay_vreg: c_uint = 0x10;
pub const RV770_SMC_SOFT_REGISTER_delay_acpi: c_uint = 0x2C;
pub const RV770_SMC_SOFT_REGISTER_seq_index: c_uint = 0x64;
pub const RV770_SMC_SOFT_REGISTER_mvdd_chg_time: c_uint = 0x68;
pub const RV770_SMC_SOFT_REGISTER_mclk_switch_lim: c_uint = 0x78;
pub const RV770_SMC_SOFT_REGISTER_mc_block_delay: c_uint = 0x90;
pub const RV770_SMC_SOFT_REGISTER_uvd_enabled: c_uint = 0x9C;
pub const RV770_SMC_SOFT_REGISTER_is_asic_lombok: c_uint = 0xA0;
extern "C" {
    pub fn rv770_start_smc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_reset_smc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_stop_smc_clock(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_start_smc_clock(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_is_smc_running(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn rv770_send_msg_to_smc(rdev: *mut radeon_device, msg: PPSMC_Msg) -> PPSMC_Result;
}
extern "C" {
    pub fn rv770_wait_for_smc_inactive(rdev: *mut radeon_device) -> PPSMC_Result;
}
