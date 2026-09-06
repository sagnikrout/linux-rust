//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/nislands_smc.h
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
// Copyright 2012 Advanced Micro Devices, Inc.
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

pub const NISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_NIslands_Dpm2PerfLevel {
    pub MaxPS: u8,
    pub TgtAct: u8,
    pub MaxPS_StepInc: u8,
    pub MaxPS_StepDec: u8,
    pub PSST: u8,
    pub NearTDPDec: u8,
    pub AboveSafeInc: u8,
    pub BelowSafeInc: u8,
    pub PSDeltaLimit: u8,
    pub PSDeltaWin: u8,
    pub Reserved: [u8; 6],
}

pub type PP_NIslands_Dpm2PerfLevel = PP_NIslands_Dpm2PerfLevel;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_NIslands_DPM2Parameters {
    pub TDPLimit: u32,
    pub NearTDPLimit: u32,
    pub SafePowerLimit: u32,
    pub PowerBoostLimit: u32,
}

pub type PP_NIslands_DPM2Parameters = PP_NIslands_DPM2Parameters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NISLANDS_SMC_SCLK_VALUE {
    pub vCG_SPLL_FUNC_CNTL: u32,
    pub vCG_SPLL_FUNC_CNTL_2: u32,
    pub vCG_SPLL_FUNC_CNTL_3: u32,
    pub vCG_SPLL_FUNC_CNTL_4: u32,
    pub vCG_SPLL_SPREAD_SPECTRUM: u32,
    pub vCG_SPLL_SPREAD_SPECTRUM_2: u32,
    pub sclk_value: u32,
}

pub type NISLANDS_SMC_SCLK_VALUE = NISLANDS_SMC_SCLK_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NISLANDS_SMC_MCLK_VALUE {
    pub vMPLL_FUNC_CNTL: u32,
    pub vMPLL_FUNC_CNTL_1: u32,
    pub vMPLL_FUNC_CNTL_2: u32,
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

pub type NISLANDS_SMC_MCLK_VALUE = NISLANDS_SMC_MCLK_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NISLANDS_SMC_VOLTAGE_VALUE {
    pub value: u16,
    pub index: u8,
    pub padding: u8,
}

pub type NISLANDS_SMC_VOLTAGE_VALUE = NISLANDS_SMC_VOLTAGE_VALUE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NISLANDS_SMC_HW_PERFORMANCE_LEVEL {
    pub arbValue: u8,
    pub ACIndex: u8,
    pub displayWatermark: u8,
    pub gen2PCIE: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub strobeMode: u8,
    pub mcFlags: u8,
    pub aT: u32,
    pub bSP: u32,
    pub sclk: NISLANDS_SMC_SCLK_VALUE,
    pub mclk: NISLANDS_SMC_MCLK_VALUE,
    pub vddc: NISLANDS_SMC_VOLTAGE_VALUE,
    pub mvdd: NISLANDS_SMC_VOLTAGE_VALUE,
    pub vddci: NISLANDS_SMC_VOLTAGE_VALUE,
    pub std_vddc: NISLANDS_SMC_VOLTAGE_VALUE,
    pub powergate_en: u32,
    pub hUp: u8,
    pub hDown: u8,
    pub stateFlags: u8,
    pub arbRefreshState: u8,
    pub SQPowerThrottle: u32,
    pub SQPowerThrottle_2: u32,
    pub reserved: [u32; 2],
    pub dpm2: PP_NIslands_Dpm2PerfLevel,
}

pub const NISLANDS_SMC_STROBE_RATIO: c_uint = 0x0F;
pub const NISLANDS_SMC_STROBE_ENABLE: c_uint = 0x10;
pub const NISLANDS_SMC_MC_EDC_RD_FLAG: c_uint = 0x01;
pub const NISLANDS_SMC_MC_EDC_WR_FLAG: c_uint = 0x02;
pub const NISLANDS_SMC_MC_RTT_ENABLE: c_uint = 0x04;
pub const NISLANDS_SMC_MC_STUTTER_EN: c_uint = 0x08;
pub type NISLANDS_SMC_HW_PERFORMANCE_LEVEL = NISLANDS_SMC_HW_PERFORMANCE_LEVEL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NISLANDS_SMC_SWSTATE {
    pub flags: u8,
    pub levelCount: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub levels: [NISLANDS_SMC_HW_PERFORMANCE_LEVEL; ],
}

pub type NISLANDS_SMC_SWSTATE = NISLANDS_SMC_SWSTATE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NISLANDS_SMC_SWSTATE_SINGLE {
    pub flags: u8,
    pub levelCount: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub level: NISLANDS_SMC_HW_PERFORMANCE_LEVEL,
}

pub const NISLANDS_SMC_VOLTAGEMASK_VDDC: c_int = 0;
pub const NISLANDS_SMC_VOLTAGEMASK_MVDD: c_int = 1;
pub const NISLANDS_SMC_VOLTAGEMASK_VDDCI: c_int = 2;
pub const NISLANDS_SMC_VOLTAGEMASK_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NISLANDS_SMC_VOLTAGEMASKTABLE {
    pub highMask: [u8; NISLANDS_SMC_VOLTAGEMASK_MAX],
    pub lowMask: [u32; NISLANDS_SMC_VOLTAGEMASK_MAX],
}

pub type NISLANDS_SMC_VOLTAGEMASKTABLE = NISLANDS_SMC_VOLTAGEMASKTABLE;
pub const NISLANDS_MAX_NO_VREG_STEPS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NISLANDS_SMC_STATETABLE {
    pub thermalProtectType: u8,
    pub systemFlags: u8,
    pub maxVDDCIndexInPPTable: u8,
    pub extraFlags: u8,
    pub highSMIO: [u8; NISLANDS_MAX_NO_VREG_STEPS],
    pub lowSMIO: [u32; NISLANDS_MAX_NO_VREG_STEPS],
    pub voltageMaskTable: NISLANDS_SMC_VOLTAGEMASKTABLE,
    pub dpm2Params: PP_NIslands_DPM2Parameters,
    pub initialState: NISLANDS_SMC_SWSTATE_SINGLE,
    pub ACPIState: NISLANDS_SMC_SWSTATE_SINGLE,
    pub ULVState: NISLANDS_SMC_SWSTATE_SINGLE,
    pub driverState: NISLANDS_SMC_SWSTATE,
    pub dpmLevels: [NISLANDS_SMC_HW_PERFORMANCE_LEVEL; NISLANDS_MAX_SMC_PERFORMANCE_LEVELS_PER_SWSTATE],
}

pub type NISLANDS_SMC_STATETABLE = NISLANDS_SMC_STATETABLE;
pub const NI_SMC_SOFT_REGISTERS_START: c_uint = 0x108;
pub const NI_SMC_SOFT_REGISTER_mclk_chg_timeout: c_uint = 0x0;
pub const NI_SMC_SOFT_REGISTER_delay_bbias: c_uint = 0xC;
pub const NI_SMC_SOFT_REGISTER_delay_vreg: c_uint = 0x10;
pub const NI_SMC_SOFT_REGISTER_delay_acpi: c_uint = 0x2C;
pub const NI_SMC_SOFT_REGISTER_seq_index: c_uint = 0x64;
pub const NI_SMC_SOFT_REGISTER_mvdd_chg_time: c_uint = 0x68;
pub const NI_SMC_SOFT_REGISTER_mclk_switch_lim: c_uint = 0x78;
pub const NI_SMC_SOFT_REGISTER_watermark_threshold: c_uint = 0x80;
pub const NI_SMC_SOFT_REGISTER_mc_block_delay: c_uint = 0x84;
pub const NI_SMC_SOFT_REGISTER_uvd_enabled: c_uint = 0x98;
pub const SMC_NISLANDS_MC_TPP_CAC_NUM_OF_ENTRIES: c_int = 16;
pub const SMC_NISLANDS_LKGE_LUT_NUM_OF_TEMP_ENTRIES: c_int = 16;
pub const SMC_NISLANDS_LKGE_LUT_NUM_OF_VOLT_ENTRIES: c_int = 16;
pub const SMC_NISLANDS_BIF_LUT_NUM_OF_ENTRIES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_NISLANDS_MC_TPP_CAC_TABLE {
    pub tpp: [u32; SMC_NISLANDS_MC_TPP_CAC_NUM_OF_ENTRIES],
    pub cacValue: [u32; SMC_NISLANDS_MC_TPP_CAC_NUM_OF_ENTRIES],
}

pub type SMC_NISLANDS_MC_TPP_CAC_TABLE = SMC_NISLANDS_MC_TPP_CAC_TABLE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PP_NIslands_CACTABLES {
    pub cac_bif_lut: [u32; SMC_NISLANDS_BIF_LUT_NUM_OF_ENTRIES],
    pub cac_lkge_lut: [u32; SMC_NISLANDS_LKGE_LUT_NUM_OF_TEMP_ENTRIES][SMC_NISLANDS_LKGE_LUT_NUM_OF_VOLT_ENTRIES],
    pub pwr_const: u32,
    pub dc_cacValue: u32,
    pub bif_cacValue: u32,
    pub lkge_pwr: u32,
    pub cac_width: u8,
    pub window_size_p2: u8,
    pub num_drop_lsb: u8,
    pub padding_0: u8,
    pub last_power: u32,
    pub AllowOvrflw: u8,
    pub MCWrWeight: u8,
    pub MCRdWeight: u8,
    pub padding_1: [u8; 9],
    pub enableWinAvg: u8,
    pub numWin_TDP: u8,
    pub l2numWin_TDP: u8,
    pub WinIndex: u8,
    pub dynPwr_TDP: [u32; 4],
    pub lkgePwr_TDP: [u32; 4],
    pub power_TDP: [u32; 4],
    pub avg_dynPwr_TDP: u32,
    pub avg_lkgePwr_TDP: u32,
    pub avg_power_TDP: u32,
    pub lts_power_TDP: u32,
    pub lts_truncate_n: u8,
    pub padding_2: [u8; 7],
}

pub type PP_NIslands_CACTABLES = PP_NIslands_CACTABLES;
pub const SMC_NISLANDS_MC_REGISTER_ARRAY_SIZE: c_int = 32;
pub const SMC_NISLANDS_MC_REGISTER_ARRAY_SET_COUNT: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_NIslands_MCRegisterAddress {
    pub s0: u16,
    pub s1: u16,
}

pub type SMC_NIslands_MCRegisterAddress = SMC_NIslands_MCRegisterAddress;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_NIslands_MCRegisterSet {
    pub value: [u32; SMC_NISLANDS_MC_REGISTER_ARRAY_SIZE],
}

pub type SMC_NIslands_MCRegisterSet = SMC_NIslands_MCRegisterSet;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_NIslands_MCRegisters {
    pub last: u8,
    pub reserved: [u8; 3],
    pub address: [SMC_NIslands_MCRegisterAddress; SMC_NISLANDS_MC_REGISTER_ARRAY_SIZE],
    pub data: [SMC_NIslands_MCRegisterSet; SMC_NISLANDS_MC_REGISTER_ARRAY_SET_COUNT],
}

pub type SMC_NIslands_MCRegisters = SMC_NIslands_MCRegisters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_NIslands_MCArbDramTimingRegisterSet {
    pub mc_arb_dram_timing: u32,
    pub mc_arb_dram_timing2: u32,
    pub mc_arb_rfsh_rate: u8,
    pub padding: [u8; 3],
}

pub type SMC_NIslands_MCArbDramTimingRegisterSet = SMC_NIslands_MCArbDramTimingRegisterSet;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_NIslands_MCArbDramTimingRegisters {
    pub arb_current: u8,
    pub reserved: [u8; 3],
    pub data: [SMC_NIslands_MCArbDramTimingRegisterSet; 20],
}

pub type SMC_NIslands_MCArbDramTimingRegisters = SMC_NIslands_MCArbDramTimingRegisters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SMC_NISLANDS_SPLL_DIV_TABLE {
    pub freq: [u32; 256],
    pub ss: [u32; 256],
}

pub const SMC_NISLANDS_SPLL_DIV_TABLE_FBDIV_MASK: c_uint = 0x01ffffff;
pub const SMC_NISLANDS_SPLL_DIV_TABLE_FBDIV_SHIFT: c_int = 0;
pub const SMC_NISLANDS_SPLL_DIV_TABLE_PDIV_MASK: c_uint = 0xfe000000;
pub const SMC_NISLANDS_SPLL_DIV_TABLE_PDIV_SHIFT: c_int = 25;
pub const SMC_NISLANDS_SPLL_DIV_TABLE_CLKV_MASK: c_uint = 0x000fffff;
pub const SMC_NISLANDS_SPLL_DIV_TABLE_CLKV_SHIFT: c_int = 0;
pub const SMC_NISLANDS_SPLL_DIV_TABLE_CLKS_MASK: c_uint = 0xfff00000;
pub const SMC_NISLANDS_SPLL_DIV_TABLE_CLKS_SHIFT: c_int = 20;
pub type SMC_NISLANDS_SPLL_DIV_TABLE = SMC_NISLANDS_SPLL_DIV_TABLE;
pub const NISLANDS_SMC_FIRMWARE_HEADER_LOCATION: c_uint = 0x100;
pub const NISLANDS_SMC_FIRMWARE_HEADER_version: c_uint = 0x0;
pub const NISLANDS_SMC_FIRMWARE_HEADER_flags: c_uint = 0x4;
pub const NISLANDS_SMC_FIRMWARE_HEADER_softRegisters: c_uint = 0x8;
pub const NISLANDS_SMC_FIRMWARE_HEADER_stateTable: c_uint = 0xC;
pub const NISLANDS_SMC_FIRMWARE_HEADER_fanTable: c_uint = 0x10;
pub const NISLANDS_SMC_FIRMWARE_HEADER_cacTable: c_uint = 0x14;
pub const NISLANDS_SMC_FIRMWARE_HEADER_mcRegisterTable: c_uint = 0x20;
pub const NISLANDS_SMC_FIRMWARE_HEADER_mcArbDramAutoRefreshTable: c_uint = 0x2C;
pub const NISLANDS_SMC_FIRMWARE_HEADER_spllTable: c_uint = 0x30;

