//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/hubbub/dcn10/dcn10_hubbub.h
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
// Copyright 2016-2026 Advanced Micro Devices, Inc.
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

// Macro flag: #define TO_DCN10_HUBBUB(hubbub)\
// Macro flag: #define HUBBUB_REG_LIST_DCN_COMMON()\

// Macro flag: #define HUBBUB_SR_WATERMARK_REG_LIST()\
// Macro flag: #define HUBBUB_REG_LIST_DCN10(id)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubbub_registers {
    pub DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_A: u32,
    pub DCHUBBUB_ARB_PTE_META_URGENCY_WATERMARK_A: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_A: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_A: u32,
    pub DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_A: u32,
    pub DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_B: u32,
    pub DCHUBBUB_ARB_PTE_META_URGENCY_WATERMARK_B: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_B: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_B: u32,
    pub DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_B: u32,
    pub DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_C: u32,
    pub DCHUBBUB_ARB_PTE_META_URGENCY_WATERMARK_C: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_C: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_C: u32,
    pub DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_C: u32,
    pub DCHUBBUB_ARB_DATA_URGENCY_WATERMARK_D: u32,
    pub DCHUBBUB_ARB_PTE_META_URGENCY_WATERMARK_D: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_D: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_D: u32,
    pub DCHUBBUB_ARB_ALLOW_DRAM_CLK_CHANGE_WATERMARK_D: u32,
    pub DCHUBBUB_ARB_WATERMARK_CHANGE_CNTL: u32,
    pub DCHUBBUB_ARB_SAT_LEVEL: u32,
    pub DCHUBBUB_ARB_DF_REQ_OUTSTAND: u32,
    pub DCHUBBUB_GLOBAL_TIMER_CNTL: u32,
    pub DCHUBBUB_ARB_DRAM_STATE_CNTL: u32,
    pub DCHUBBUB_TEST_DEBUG_INDEX: u32,
    pub DCHUBBUB_TEST_DEBUG_DATA: u32,
    pub DCHUBBUB_SDPIF_FB_TOP: u32,
    pub DCHUBBUB_SDPIF_FB_BASE: u32,
    pub DCHUBBUB_SDPIF_FB_OFFSET: u32,
    pub DCHUBBUB_SDPIF_AGP_BASE: u32,
    pub DCHUBBUB_SDPIF_AGP_BOT: u32,
    pub DCHUBBUB_SDPIF_AGP_TOP: u32,
    pub DCHUBBUB_CRC_CTRL: u32,
    pub DCHUBBUB_SOFT_RESET: u32,
    pub DCN_VM_FB_LOCATION_BASE: u32,
    pub DCN_VM_FB_LOCATION_TOP: u32,
    pub DCN_VM_FB_OFFSET: u32,
    pub DCN_VM_AGP_BOT: u32,
    pub DCN_VM_AGP_TOP: u32,
    pub DCN_VM_AGP_BASE: u32,
    pub DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_MSB: u32,
    pub DCN_VM_PROTECTION_FAULT_DEFAULT_ADDR_LSB: u32,
    pub DCN_VM_FAULT_ADDR_MSB: u32,
    pub DCN_VM_FAULT_ADDR_LSB: u32,
    pub DCN_VM_FAULT_CNTL: u32,
    pub DCN_VM_FAULT_STATUS: u32,
    pub DCHUBBUB_ARB_FRAC_URG_BW_NOM_A: u32,
    pub DCHUBBUB_ARB_FRAC_URG_BW_NOM_B: u32,
    pub DCHUBBUB_ARB_FRAC_URG_BW_NOM_C: u32,
    pub DCHUBBUB_ARB_FRAC_URG_BW_NOM_D: u32,
    pub DCHUBBUB_ARB_FRAC_URG_BW_FLIP_A: u32,
    pub DCHUBBUB_ARB_FRAC_URG_BW_FLIP_B: u32,
    pub DCHUBBUB_ARB_FRAC_URG_BW_FLIP_C: u32,
    pub DCHUBBUB_ARB_FRAC_URG_BW_FLIP_D: u32,
    pub DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_A: u32,
    pub DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_B: u32,
    pub DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_C: u32,
    pub DCHUBBUB_ARB_REFCYC_PER_TRIP_TO_MEMORY_D: u32,
    pub DCHUBBUB_ARB_HOSTVM_CNTL: u32,
    pub DCHVM_CTRL0: u32,
    pub DCHVM_MEM_CTRL: u32,
    pub DCHVM_CLK_CTRL: u32,
    pub DCHVM_RIOMMU_CTRL0: u32,
    pub DCHVM_RIOMMU_STAT0: u32,
    pub DCHUBBUB_DET0_CTRL: u32,
    pub DCHUBBUB_DET1_CTRL: u32,
    pub DCHUBBUB_DET2_CTRL: u32,
    pub DCHUBBUB_DET3_CTRL: u32,
    pub DCHUBBUB_COMPBUF_CTRL: u32,
    pub COMPBUF_RESERVED_SPACE: u32,
    pub DCHUBBUB_DEBUG_CTRL_0: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_A: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_A: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_B: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_B: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_C: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_C: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK_Z8_D: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK_Z8_D: u32,
    pub DCHUBBUB_ARB_USR_RETRAINING_CNTL: u32,
    pub DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_A: u32,
    pub DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_B: u32,
    pub DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_C: u32,
    pub DCHUBBUB_ARB_USR_RETRAINING_WATERMARK_D: u32,
    pub DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_A: u32,
    pub DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_B: u32,
    pub DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_C: u32,
    pub DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK_D: u32,
    pub DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_A: u32,
    pub DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_B: u32,
    pub DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_C: u32,
    pub DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK_D: u32,
    pub DCHUBBUB_ARB_MALL_CNTL: u32,
    pub SDPIF_REQUEST_RATE_LIMIT: u32,
    pub DCHUBBUB_SDPIF_CFG0: u32,
    pub DCHUBBUB_SDPIF_CFG1: u32,
    pub DCHUBBUB_CLOCK_CNTL: u32,
    pub DCHUBBUB_MEM_PWR_MODE_CTRL: u32,
    pub DCHUBBUB_ARB_QOS_FORCE: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK1_A: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK1_A: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK1_B: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK1_B: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK2_A: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK2_A: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK2_B: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK2_B: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK3_A: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK3_A: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_ENTER_WATERMARK3_B: u32,
    pub DCHUBBUB_ARB_ALLOW_SR_EXIT_WATERMARK3_B: u32,
    pub DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK1_A: u32,
    pub DCHUBBUB_ARB_UCLK_PSTATE_CHANGE_WATERMARK1_B: u32,
    pub DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK1_A: u32,
    pub DCHUBBUB_ARB_FCLK_PSTATE_CHANGE_WATERMARK1_B: u32,
    pub DCHUBBUB_ARB_REFCYC_PER_META_TRIP_A: u32,
    pub DCHUBBUB_ARB_REFCYC_PER_META_TRIP_B: u32,
    pub DCHUBBUB_ARB_FRAC_URG_BW_MALL_A: u32,
    pub DCHUBBUB_ARB_FRAC_URG_BW_MALL_B: u32,
    pub DCHUBBUB_TIMEOUT_DETECTION_CTRL1: u32,
    pub DCHUBBUB_TIMEOUT_DETECTION_CTRL2: u32,
    pub DCHUBBUB_CTRL_STATUS: u32,
    pub DCHUBBUB_ARB_BUFFER_FULLNESS_WATERMARK_A: u32,
    pub DCHUBBUB_ARB_BUFFER_FULLNESS_WATERMARK_B: u32,
    pub DCHUBBUB_PERFORMANCE_MEASUREMENT_CNTL: u32,
    pub DCHUBBUB_PERFORMANCE_MEASUREMENT_CNTL2: u32,
    pub DC_PERFMON5_PERFCOUNTER_CNTL: u32,
    pub DC_PERFMON5_PERFCOUNTER_CNTL2: u32,
    pub DC_PERFMON5_PERFCOUNTER_STATE: u32,
    pub DC_PERFMON5_PERFMON_CNTL: u32,
    pub DC_PERFMON5_PERFMON_CNTL2: u32,
    pub DC_PERFMON5_PERFMON_CVALUE_INT_MISC: u32,
    pub DC_PERFMON5_PERFMON_CVALUE_LOW: u32,
    pub DC_PERFMON5_PERFMON_HI: u32,
    pub DC_PERFMON5_PERFMON_LOW: u32,
    pub FMON_CTRL: u32,
}

// set field name

// Macro flag: #define HUBBUB_MASK_SH_LIST_DCN_COMMON(mask_sh)\

// Macro flag: #define HUBBUB_MASK_SH_LIST_DCN10(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubbub_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_hubbub_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_hubbub {
    pub base: hubbub,
    pub regs: *const dcn_hubbub_registers,
    pub shifts: *const dcn_hubbub_shift,
    pub masks: *const dcn_hubbub_mask,
    pub debug_test_index_pstate: c_uint,
    pub watermarks: dcn_watermark_set,
}

extern "C" {
    pub fn hubbub1_wm_change_req_wa(hubbub: *mut hubbub);
}
extern "C" {
    pub fn hubbub1_allow_self_refresh_control(hubbub: *mut hubbub, allow: bool);
}
extern "C" {
    pub fn hubbub1_is_allow_self_refresh_enabled(hubub: *mut hubbub) -> bool;
}
extern "C" {
    pub fn hubbub1_soft_reset(hubbub: *mut hubbub, reset: bool);
}
extern "C" {
    pub fn dcn10_hubbub_global_timer_enable(hubbub: *mut hubbub, enable: bool, refdiv: u32);
}
extern "C" {
    pub fn dcn10_hubbub_read_fb_aperture(hubbub: *mut hubbub, fb_base_value: *mut u32, fb_offset_value: *mut u32);
}
