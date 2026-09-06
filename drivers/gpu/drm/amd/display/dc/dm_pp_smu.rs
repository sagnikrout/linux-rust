//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dm_pp_smu.h
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
// interface to PPLIB/SMU to setup clocks and pstate requirements on SoC
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_smu_ver {
//
// PP_SMU_INTERFACE_X should be interpreted as the interface defined
// starting from X, where X is some family of ASICs.  This is as
// opposed to interfaces used only for X.  There will be some degree
// of interface sharing between families of ASIcs.
//
    PP_SMU_UNSUPPORTED,
    PP_SMU_VER_RV,
    PP_SMU_VER_NV,

    PP_SMU_VER_RN,
    PP_SMU_VER_VG,
    PP_SMU_VER_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_smu {
    pub ver: pp_smu_ver,
    pub pp: *const c_void,
//
// interim extra handle for backwards compatibility
// as some existing functionality not yet implemented
// by ppsmu
//
    pub dm: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_smu_status {
    PP_SMU_RESULT_UNDEFINED = 0,
    PP_SMU_RESULT_OK = 1,
    PP_SMU_RESULT_FAIL,
    PP_SMU_RESULT_UNSUPPORTED
}

pub const PP_SMU_WM_SET_RANGE_CLK_UNCONSTRAINED_MIN: c_uint = 0x0;
pub const PP_SMU_WM_SET_RANGE_CLK_UNCONSTRAINED_MAX: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm_type {
    WM_TYPE_PSTATE_CHG = 0,
    WM_TYPE_RETRAINING = 1,
}

// This structure is a copy of WatermarkRowGeneric_t defined by smuxx_driver_if.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_smu_wm_set_range {
    pub min_fill_clk_mhz: u16,
    pub max_fill_clk_mhz: u16,
    pub min_drain_clk_mhz: u16,
    pub max_drain_clk_mhz: u16,
    pub wm_inst: u8,
    pub wm_type: u8,
}

pub const MAX_WATERMARK_SETS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_smu_wm_range_sets {
    pub num_reader_wm_sets: c_uint,
    pub reader_wm_sets: [pp_smu_wm_set_range; MAX_WATERMARK_SETS],
    pub num_writer_wm_sets: c_uint,
    pub writer_wm_sets: [pp_smu_wm_set_range; MAX_WATERMARK_SETS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_smu_funcs_rv {
    pub pp_smu: pp_smu,
// PPSMC_MSG_SetDisplayCount
// 0 triggers S0i2 optimization
//
    pub count): *mut *mut *mut void (set_display_count)(struct pp_smu pp, int,
// reader and writer WM's are sent together as part of one table
//
// PPSMC_MSG_SetDriverDramAddrHigh
// PPSMC_MSG_SetDriverDramAddrLow
// PPSMC_MSG_TransferTableDram2Smu
//
    pub ranges): *mut pp_smu_wm_range_sets,
// PPSMC_MSG_SetHardMinDcfclkByFreq
// fixed clock at requested freq, either from FCH bypass or DFS
//
    pub mhz): *mut *mut *mut void (set_hard_min_dcfclk_by_freq)(struct pp_smu pp, int,
// PPSMC_MSG_SetMinDeepSleepDcfclk
// when DF is in cstate, dcf clock is further divided down
// to just above given frequency
//
    pub mhz): *mut *mut *mut void (set_min_deep_sleep_dcfclk)(struct pp_smu pp, int,
// PPSMC_MSG_SetHardMinFclkByFreq
// FCLK will vary with DPM, but never below requested hard min
//
    pub mhz): *mut *mut *mut void (set_hard_min_fclk_by_freq)(struct pp_smu pp, int,
// PPSMC_MSG_SetHardMinSocclkByFreq
// Needed for DWB support
//
    pub mhz): *mut *mut *mut void (set_hard_min_socclk_by_freq)(struct pp_smu pp, int,
// PME w/a
    pub pp): *mut *mut void (set_pme_wa_enable)(struct pp_smu,
}

// Used by pp_smu_funcs_nv.set_voltage_by_freq
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pp_smu_nv_clock_id {
    PP_SMU_NV_DISPCLK,
    PP_SMU_NV_PHYCLK,
    PP_SMU_NV_PIXELCLK
}

//
// Used by pp_smu_funcs_nv.get_maximum_sustainable_clocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_smu_nv_clock_table {
// voltage managed SMU, freq set by driver
    pub displayClockInKhz: c_uint,
    pub dppClockInKhz: c_uint,
    pub phyClockInKhz: c_uint,
    pub pixelClockInKhz: c_uint,
    pub dscClockInKhz: c_uint,
// freq/voltage managed by SMU
    pub fabricClockInKhz: c_uint,
    pub socClockInKhz: c_uint,
    pub dcfClockInKhz: c_uint,
    pub uClockInKhz: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_smu_funcs_nv {
    pub pp_smu: pp_smu,
// PPSMC_MSG_SetDisplayCount
// 0 triggers S0i2 optimization
//
    pub count): *mut *mut *mut pp_smu_status (set_display_count)(struct pp_smu pp, int,
// PPSMC_MSG_SetHardMinDcfclkByFreq
// fixed clock at requested freq, either from FCH bypass or DFS
//
    pub Mhz): *mut *mut *mut pp_smu_status (set_hard_min_dcfclk_by_freq)(struct pp_smu pp, int,
// PPSMC_MSG_SetMinDeepSleepDcfclk
// when DF is in cstate, dcf clock is further divided down
// to just above given frequency
//
    pub Mhz): *mut *mut *mut pp_smu_status (set_min_deep_sleep_dcfclk)(struct pp_smu pp, int,
// PPSMC_MSG_SetHardMinUclkByFreq
// UCLK will vary with DPM, but never below requested hard min
//
    pub Mhz): *mut *mut *mut pp_smu_status (set_hard_min_uclk_by_freq)(struct pp_smu pp, int,
// PPSMC_MSG_SetHardMinSocclkByFreq
// Needed for DWB support
//
    pub Mhz): *mut *mut *mut pp_smu_status (set_hard_min_socclk_by_freq)(struct pp_smu pp, int,
// PME w/a
    pub pp): *mut *mut pp_smu_status (set_pme_wa_enable)(struct pp_smu,
// PPSMC_MSG_SetHardMinByFreq
// Needed to set ASIC voltages for clocks programmed by DAL
//
    pub Mhz): pp_smu_nv_clock_id clock_id, int,
// reader and writer WM's are sent together as part of one table
//
// PPSMC_MSG_SetDriverDramAddrHigh
// PPSMC_MSG_SetDriverDramAddrLow
// PPSMC_MSG_TransferTableDram2Smu
//
// on DCN20:
// reader fill clk = uclk
// reader drain clk = dcfclk
// writer fill clk = socclk
// writer drain clk = uclk
//
    pub ranges): *mut pp_smu_wm_range_sets,
// Not a single SMU message.  This call should return maximum sustainable limit for all
// clocks that DC depends on.  These will be used as basis for mode enumeration.
//
    pub max_clocks): *mut pp_smu_nv_clock_table,
// This call should return the discrete uclk DPM states available
//
    pub num_states): *mut *mut unsigned int clock_values_in_khz, unsigned int,
// Not a single SMU message.  This call informs PPLIB that display will not be able
// to perform pstate handshaking in its current state.  Typically this handshake
// is used to perform uCLK switching, so disabling pstate disables uCLK switching.
//
// Note that when setting handshake to unsupported, the call is pre-emptive.  That means
// DC will make the call BEFORE setting up the display state which would cause pstate
// request to go un-acked.  Only when the call completes should such a state be applied to
// DC hardware
//
    pub pstate_handshake_supported): bool,
}

pub const PP_SMU_NUM_SOCCLK_DPM_LEVELS: c_int = 8;
pub const PP_SMU_NUM_DCFCLK_DPM_LEVELS: c_int = 8;
pub const PP_SMU_NUM_FCLK_DPM_LEVELS: c_int = 4;
pub const PP_SMU_NUM_MEMCLK_DPM_LEVELS: c_int = 4;
pub const PP_SMU_NUM_DCLK_DPM_LEVELS: c_int = 8;
pub const PP_SMU_NUM_VCLK_DPM_LEVELS: c_int = 8;
pub const PP_SMU_NUM_VPECLK_DPM_LEVELS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpm_clock {
    pub MHz: uint32_t Freq; // In,
    pub bits: uint32_t Vol; // Millivolts with 2 fractional,
}

// this is a copy of the structure defined in smuxx_driver_if.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpm_clocks {
    pub DcfClocks: [dpm_clock; PP_SMU_NUM_DCFCLK_DPM_LEVELS],
    pub SocClocks: [dpm_clock; PP_SMU_NUM_SOCCLK_DPM_LEVELS],
    pub FClocks: [dpm_clock; PP_SMU_NUM_FCLK_DPM_LEVELS],
    pub MemClocks: [dpm_clock; PP_SMU_NUM_MEMCLK_DPM_LEVELS],
    pub VClocks: [dpm_clock; PP_SMU_NUM_VCLK_DPM_LEVELS],
    pub DClocks: [dpm_clock; PP_SMU_NUM_DCLK_DPM_LEVELS],
    pub VPEClocks: [dpm_clock; PP_SMU_NUM_VPECLK_DPM_LEVELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_smu_funcs_rn {
    pub pp_smu: pp_smu,
//
// reader and writer WM's are sent together as part of one table
//
// PPSMC_MSG_SetDriverDramAddrHigh
// PPSMC_MSG_SetDriverDramAddrLow
// PPSMC_MSG_TransferTableDram2Smu
//
    pub ranges): *mut pp_smu_wm_range_sets,
    pub clock_table): *mut dpm_clocks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_smu_funcs_vgh {
    pub pp_smu: pp_smu,
//
// reader and writer WM's are sent together as part of one table
//
// PPSMC_MSG_SetDriverDramAddrHigh
// PPSMC_MSG_SetDriverDramAddrLow
// PPSMC_MSG_TransferTableDram2Smu
//
// TODO: Check whether this is moved to DAL, and remove as needed
    pub ranges): *mut pp_smu_wm_range_sets,
// TODO: Check whether this is moved to DAL, and remove as needed
    pub clock_table): *mut dpm_clocks,
    pub pp): *mut *mut pp_smu_status (notify_smu_timeout) (struct pp_smu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pp_smu_funcs {
    pub ctx: pp_smu,
    pub rv_funcs: pp_smu_funcs_rv,
    pub nv_funcs: pp_smu_funcs_nv,
    pub rn_funcs: pp_smu_funcs_rn,
    pub vgh_funcs: pp_smu_funcs_vgh,
}
