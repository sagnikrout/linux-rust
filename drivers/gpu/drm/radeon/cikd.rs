//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/cikd.h
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
// Authors: Alex Deucher
//
pub const BONAIRE_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x12010001;
pub const HAWAII_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x12011003;
pub const CIK_RB_BITMAP_WIDTH_PER_SH: c_int = 2;
pub const HAWAII_RB_BITMAP_WIDTH_PER_SH: c_int = 4;
// DIDT IND registers
pub const DIDT_SQ_CTRL0: c_uint = 0x0;

pub const DIDT_DB_CTRL0: c_uint = 0x20;
pub const DIDT_TD_CTRL0: c_uint = 0x40;
pub const DIDT_TCP_CTRL0: c_uint = 0x60;
// SMC IND registers
pub const DPM_TABLE_475: c_uint = 0x3F768;

pub const FIRMWARE_FLAGS: c_uint = 0x3F800;

pub const NB_DPM_CONFIG_1: c_uint = 0x3F9E8;

pub const SMC_SYSCON_RESET_CNTL: c_uint = 0x80000000;

pub const SMC_SYSCON_CLOCK_CNTL_0: c_uint = 0x80000004;

pub const SMC_SYSCON_MISC_CNTL: c_uint = 0x80000010;
pub const SMC_SYSCON_MSG_ARG_0: c_uint = 0x80000068;
pub const SMC_PC_C: c_uint = 0x80000370;
pub const SMC_SCRATCH9: c_uint = 0x80000424;
pub const RCU_UC_EVENTS: c_uint = 0xC0000004;

pub const GENERAL_PWRMGT: c_uint = 0xC0200000;

pub const CNB_PWRMGT_CNTL: c_uint = 0xC0200004;

pub const SCLK_PWRMGT_CNTL: c_uint = 0xC0200008;

pub const TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0xC0200014;

pub const CG_SSP: c_uint = 0xC0200044;

pub const CG_DISPLAY_GAP_CNTL: c_uint = 0xC0200060;

pub const SMU_VOLTAGE_STATUS: c_uint = 0xC0200094;

pub const TARGET_AND_CURRENT_PROFILE_INDEX_1: c_uint = 0xC02000F0;

pub const CG_ULV_PARAMETER: c_uint = 0xC0200158;
pub const CG_FTV_0: c_uint = 0xC02001A8;
pub const CG_FTV_1: c_uint = 0xC02001AC;
pub const CG_FTV_2: c_uint = 0xC02001B0;
pub const CG_FTV_3: c_uint = 0xC02001B4;
pub const CG_FTV_4: c_uint = 0xC02001B8;
pub const CG_FTV_5: c_uint = 0xC02001BC;
pub const CG_FTV_6: c_uint = 0xC02001C0;
pub const CG_FTV_7: c_uint = 0xC02001C4;
pub const CG_DISPLAY_GAP_CNTL2: c_uint = 0xC0200230;
pub const LCAC_SX0_OVR_SEL: c_uint = 0xC0400D04;
pub const LCAC_SX0_OVR_VAL: c_uint = 0xC0400D08;
pub const LCAC_MC0_CNTL: c_uint = 0xC0400D30;
pub const LCAC_MC0_OVR_SEL: c_uint = 0xC0400D34;
pub const LCAC_MC0_OVR_VAL: c_uint = 0xC0400D38;
pub const LCAC_MC1_CNTL: c_uint = 0xC0400D3C;
pub const LCAC_MC1_OVR_SEL: c_uint = 0xC0400D40;
pub const LCAC_MC1_OVR_VAL: c_uint = 0xC0400D44;
pub const LCAC_MC2_OVR_SEL: c_uint = 0xC0400D4C;
pub const LCAC_MC2_OVR_VAL: c_uint = 0xC0400D50;
pub const LCAC_MC3_OVR_SEL: c_uint = 0xC0400D58;
pub const LCAC_MC3_OVR_VAL: c_uint = 0xC0400D5C;
pub const LCAC_CPL_CNTL: c_uint = 0xC0400D80;
pub const LCAC_CPL_OVR_SEL: c_uint = 0xC0400D84;
pub const LCAC_CPL_OVR_VAL: c_uint = 0xC0400D88;
// dGPU
pub const CG_THERMAL_CTRL: c_uint = 0xC0300004;

pub const DIG_THERM_DPM_MASK: c_uint = 0x003FC000;
pub const DIG_THERM_DPM_SHIFT: c_int = 14;
pub const CG_THERMAL_STATUS: c_uint = 0xC0300008;

pub const FDO_PWM_DUTY_SHIFT: c_int = 9;
pub const CG_THERMAL_INT: c_uint = 0xC030000C;

pub const CI_DIG_THERM_INTH_MASK: c_uint = 0x0000FF00;
pub const CI_DIG_THERM_INTH_SHIFT: c_int = 8;

pub const CI_DIG_THERM_INTL_MASK: c_uint = 0x00FF0000;
pub const CI_DIG_THERM_INTL_SHIFT: c_int = 16;

pub const CG_MULT_THERMAL_CTRL: c_uint = 0xC0300010;

pub const TEMP_SEL_SHIFT: c_int = 20;
pub const CG_MULT_THERMAL_STATUS: c_uint = 0xC0300014;

pub const ASIC_MAX_TEMP_MASK: c_uint = 0x000001ff;
pub const ASIC_MAX_TEMP_SHIFT: c_int = 0;

pub const CTF_TEMP_MASK: c_uint = 0x0003fe00;
pub const CTF_TEMP_SHIFT: c_int = 9;
pub const CG_FDO_CTRL0: c_uint = 0xC0300064;

pub const FDO_STATIC_DUTY_MASK: c_uint = 0x000000FF;
pub const FDO_STATIC_DUTY_SHIFT: c_int = 0;
pub const CG_FDO_CTRL1: c_uint = 0xC0300068;

pub const FMAX_DUTY100_MASK: c_uint = 0x000000FF;
pub const FMAX_DUTY100_SHIFT: c_int = 0;
pub const CG_FDO_CTRL2: c_uint = 0xC030006C;

pub const TMIN_MASK: c_uint = 0x000000FF;
pub const TMIN_SHIFT: c_int = 0;

pub const FDO_PWM_MODE_SHIFT: c_int = 11;

pub const TACH_PWM_RESP_RATE_SHIFT: c_int = 25;
pub const CG_TACH_CTRL: c_uint = 0xC0300070;

pub const CG_TACH_STATUS: c_uint = 0xC0300074;

pub const CG_ECLK_CNTL: c_uint = 0xC05000AC;

pub const CG_ECLK_STATUS: c_uint = 0xC05000B0;

pub const CG_SPLL_FUNC_CNTL: c_uint = 0xC0500140;

pub const SPLL_PDIV_A_SHIFT: c_int = 20;
pub const CG_SPLL_FUNC_CNTL_2: c_uint = 0xC0500144;

pub const CG_SPLL_FUNC_CNTL_3: c_uint = 0xC0500148;

pub const SPLL_FB_DIV_SHIFT: c_int = 0;

pub const CG_SPLL_FUNC_CNTL_4: c_uint = 0xC050014C;
pub const CG_SPLL_SPREAD_SPECTRUM: c_uint = 0xC0500164;

pub const CLK_S_SHIFT: c_int = 4;
pub const CG_SPLL_SPREAD_SPECTRUM_2: c_uint = 0xC0500168;

pub const CLK_V_SHIFT: c_int = 0;
pub const MPLL_BYPASSCLK_SEL: c_uint = 0xC050019C;

pub const CG_CLKPIN_CNTL: c_uint = 0xC05001A0;

pub const CG_CLKPIN_CNTL_2: c_uint = 0xC05001A4;

pub const THM_CLK_CNTL: c_uint = 0xC05001A8;

pub const MISC_CLK_CTRL: c_uint = 0xC05001AC;

// KV/KB
pub const CG_THERMAL_INT_CTRL: c_uint = 0xC2100028;

pub const DIG_THERM_INTH_MASK: c_uint = 0x000000FF;
pub const DIG_THERM_INTH_SHIFT: c_int = 0;

pub const DIG_THERM_INTL_MASK: c_uint = 0x0000FF00;
pub const DIG_THERM_INTL_SHIFT: c_int = 8;

// PCIE registers idx/data 0x38/0x3c
pub const PB0_PIF_PWRDOWN_0: c_uint = 0x1100012 /* PCIE */;

pub const PB0_PIF_PWRDOWN_1: c_uint = 0x1100013 /* PCIE */;

pub const PCIE_CNTL2: c_uint = 0x1001001c /* PCIE */;

pub const PCIE_LC_STATUS1: c_uint = 0x1400028 /* PCIE */;

pub const PCIE_P_CNTL: c_uint = 0x1400040 /* PCIE */;

pub const PB1_PIF_PWRDOWN_0: c_uint = 0x2100012 /* PCIE */;
pub const PB1_PIF_PWRDOWN_1: c_uint = 0x2100013 /* PCIE */;
pub const PCIE_LC_CNTL: c_uint = 0x100100A0 /* PCIE */;

pub const PCIE_LC_LINK_WIDTH_CNTL: c_uint = 0x100100A2 /* PCIE */;

pub const PCIE_LC_N_FTS_CNTL: c_uint = 0x100100a3 /* PCIE */;

pub const PCIE_LC_SPEED_CNTL: c_uint = 0x100100A4 /* PCIE */;

pub const PCIE_LC_CNTL2: c_uint = 0x100100B1 /* PCIE */;

pub const PCIE_LC_CNTL3: c_uint = 0x100100B5 /* PCIE */;

pub const PCIE_LC_CNTL4: c_uint = 0x100100B6 /* PCIE */;

// direct registers
pub const PCIE_INDEX: c_uint = 0x38;
pub const PCIE_DATA: c_uint = 0x3C;
pub const SMC_IND_INDEX_0: c_uint = 0x200;
pub const SMC_IND_DATA_0: c_uint = 0x204;
pub const SMC_IND_ACCESS_CNTL: c_uint = 0x240;

pub const SMC_MESSAGE_0: c_uint = 0x250;
pub const SMC_MSG_MASK: c_uint = 0xffff;
pub const SMC_RESP_0: c_uint = 0x254;
pub const SMC_RESP_MASK: c_uint = 0xffff;
pub const SMC_MSG_ARG_0: c_uint = 0x290;
pub const VGA_HDP_CONTROL: c_uint = 0x328;

pub const DMIF_ADDR_CALC: c_uint = 0xC00;
pub const PIPE0_DMIF_BUFFER_CONTROL: c_uint = 0x0ca0;

pub const SRBM_GFX_CNTL: c_uint = 0xE44;

pub const SRBM_STATUS2: c_uint = 0xE4C;

pub const SRBM_STATUS: c_uint = 0xE50;

pub const SRBM_SOFT_RESET: c_uint = 0xE60;

pub const SRBM_READ_ERROR: c_uint = 0xE98;
pub const SRBM_INT_CNTL: c_uint = 0xEA0;
pub const SRBM_INT_ACK: c_uint = 0xEA8;
pub const VM_L2_CNTL: c_uint = 0x1400;

pub const VM_L2_CNTL2: c_uint = 0x1404;

pub const INVALIDATE_PTE_AND_PDE_CACHES: c_int = 0;
pub const INVALIDATE_ONLY_PTE_CACHES: c_int = 1;
pub const INVALIDATE_ONLY_PDE_CACHES: c_int = 2;
pub const VM_L2_CNTL3: c_uint = 0x1408;

pub const VM_L2_STATUS: c_uint = 0x140C;

pub const VM_CONTEXT0_CNTL: c_uint = 0x1410;

pub const VM_CONTEXT1_CNTL: c_uint = 0x1414;
pub const VM_CONTEXT0_CNTL2: c_uint = 0x1430;
pub const VM_CONTEXT1_CNTL2: c_uint = 0x1434;
pub const VM_CONTEXT8_PAGE_TABLE_BASE_ADDR: c_uint = 0x1438;
pub const VM_CONTEXT9_PAGE_TABLE_BASE_ADDR: c_uint = 0x143c;
pub const VM_CONTEXT10_PAGE_TABLE_BASE_ADDR: c_uint = 0x1440;
pub const VM_CONTEXT11_PAGE_TABLE_BASE_ADDR: c_uint = 0x1444;
pub const VM_CONTEXT12_PAGE_TABLE_BASE_ADDR: c_uint = 0x1448;
pub const VM_CONTEXT13_PAGE_TABLE_BASE_ADDR: c_uint = 0x144c;
pub const VM_CONTEXT14_PAGE_TABLE_BASE_ADDR: c_uint = 0x1450;
pub const VM_CONTEXT15_PAGE_TABLE_BASE_ADDR: c_uint = 0x1454;
pub const VM_INVALIDATE_REQUEST: c_uint = 0x1478;
pub const VM_INVALIDATE_RESPONSE: c_uint = 0x147c;
pub const VM_CONTEXT1_PROTECTION_FAULT_STATUS: c_uint = 0x14DC;

pub const PROTECTIONS_SHIFT: c_int = 0;
// bit 0: range
// bit 1: pde0
// bit 2: valid
// bit 3: read
// bit 4: write
//

pub const MEMORY_CLIENT_ID_SHIFT: c_int = 12;

pub const MEMORY_CLIENT_RW_SHIFT: c_int = 24;

pub const FAULT_VMID_SHIFT: c_int = 25;
pub const VM_CONTEXT1_PROTECTION_FAULT_MCCLIENT: c_uint = 0x14E4;
pub const VM_CONTEXT1_PROTECTION_FAULT_ADDR: c_uint = 0x14FC;
pub const VM_CONTEXT0_PROTECTION_FAULT_DEFAULT_ADDR: c_uint = 0x1518;
pub const VM_CONTEXT1_PROTECTION_FAULT_DEFAULT_ADDR: c_uint = 0x151c;
pub const VM_CONTEXT0_PAGE_TABLE_BASE_ADDR: c_uint = 0x153c;
pub const VM_CONTEXT1_PAGE_TABLE_BASE_ADDR: c_uint = 0x1540;
pub const VM_CONTEXT2_PAGE_TABLE_BASE_ADDR: c_uint = 0x1544;
pub const VM_CONTEXT3_PAGE_TABLE_BASE_ADDR: c_uint = 0x1548;
pub const VM_CONTEXT4_PAGE_TABLE_BASE_ADDR: c_uint = 0x154c;
pub const VM_CONTEXT5_PAGE_TABLE_BASE_ADDR: c_uint = 0x1550;
pub const VM_CONTEXT6_PAGE_TABLE_BASE_ADDR: c_uint = 0x1554;
pub const VM_CONTEXT7_PAGE_TABLE_BASE_ADDR: c_uint = 0x1558;
pub const VM_CONTEXT0_PAGE_TABLE_START_ADDR: c_uint = 0x155c;
pub const VM_CONTEXT1_PAGE_TABLE_START_ADDR: c_uint = 0x1560;
pub const VM_CONTEXT0_PAGE_TABLE_END_ADDR: c_uint = 0x157C;
pub const VM_CONTEXT1_PAGE_TABLE_END_ADDR: c_uint = 0x1580;
pub const VM_L2_CG: c_uint = 0x15c0;

pub const MC_SHARED_CHMAP: c_uint = 0x2004;
pub const NOOFCHAN_SHIFT: c_int = 12;
pub const NOOFCHAN_MASK: c_uint = 0x0000f000;
pub const MC_SHARED_CHREMAP: c_uint = 0x2008;
pub const CHUB_CONTROL: c_uint = 0x1864;

pub const MC_VM_FB_LOCATION: c_uint = 0x2024;
pub const MC_VM_AGP_TOP: c_uint = 0x2028;
pub const MC_VM_AGP_BOT: c_uint = 0x202C;
pub const MC_VM_AGP_BASE: c_uint = 0x2030;
pub const MC_VM_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x2034;
pub const MC_VM_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x2038;
pub const MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR: c_uint = 0x203C;
pub const MC_VM_MX_L1_TLB_CNTL: c_uint = 0x2064;

pub const MC_VM_FB_OFFSET: c_uint = 0x2068;
pub const MC_SHARED_BLACKOUT_CNTL: c_uint = 0x20ac;
pub const MC_HUB_MISC_HUB_CG: c_uint = 0x20b8;
pub const MC_HUB_MISC_VM_CG: c_uint = 0x20bc;
pub const MC_HUB_MISC_SIP_CG: c_uint = 0x20c0;
pub const MC_XPB_CLK_GAT: c_uint = 0x2478;
pub const MC_CITF_MISC_RD_CG: c_uint = 0x2648;
pub const MC_CITF_MISC_WR_CG: c_uint = 0x264c;
pub const MC_CITF_MISC_VM_CG: c_uint = 0x2650;
pub const MC_ARB_RAMCFG: c_uint = 0x2760;
pub const NOOFBANK_SHIFT: c_int = 0;
pub const NOOFBANK_MASK: c_uint = 0x00000003;
pub const NOOFRANK_SHIFT: c_int = 2;
pub const NOOFRANK_MASK: c_uint = 0x00000004;
pub const NOOFROWS_SHIFT: c_int = 3;
pub const NOOFROWS_MASK: c_uint = 0x00000038;
pub const NOOFCOLS_SHIFT: c_int = 6;
pub const NOOFCOLS_MASK: c_uint = 0x000000C0;
pub const CHANSIZE_SHIFT: c_int = 8;
pub const CHANSIZE_MASK: c_uint = 0x00000100;
pub const NOOFGROUPS_SHIFT: c_int = 12;
pub const NOOFGROUPS_MASK: c_uint = 0x00001000;
pub const MC_ARB_DRAM_TIMING: c_uint = 0x2774;
pub const MC_ARB_DRAM_TIMING2: c_uint = 0x2778;
pub const MC_ARB_BURST_TIME: c_uint = 0x2808;

pub const STATE0_SHIFT: c_int = 0;

pub const STATE1_SHIFT: c_int = 5;

pub const STATE2_SHIFT: c_int = 10;

pub const STATE3_SHIFT: c_int = 15;
pub const MC_SEQ_RAS_TIMING: c_uint = 0x28a0;
pub const MC_SEQ_CAS_TIMING: c_uint = 0x28a4;
pub const MC_SEQ_MISC_TIMING: c_uint = 0x28a8;
pub const MC_SEQ_MISC_TIMING2: c_uint = 0x28ac;
pub const MC_SEQ_PMG_TIMING: c_uint = 0x28b0;
pub const MC_SEQ_RD_CTL_D0: c_uint = 0x28b4;
pub const MC_SEQ_RD_CTL_D1: c_uint = 0x28b8;
pub const MC_SEQ_WR_CTL_D0: c_uint = 0x28bc;
pub const MC_SEQ_WR_CTL_D1: c_uint = 0x28c0;
pub const MC_SEQ_SUP_CNTL: c_uint = 0x28c8;

pub const MC_SEQ_SUP_PGM: c_uint = 0x28cc;
pub const MC_PMG_AUTO_CMD: c_uint = 0x28d0;
pub const MC_SEQ_TRAIN_WAKEUP_CNTL: c_uint = 0x28e8;

pub const MC_IO_PAD_CNTL_D0: c_uint = 0x29d0;

pub const MC_SEQ_MISC0: c_uint = 0x2a00;
pub const MC_SEQ_MISC0_VEN_ID_SHIFT: c_int = 8;
pub const MC_SEQ_MISC0_VEN_ID_MASK: c_uint = 0x00000f00;
pub const MC_SEQ_MISC0_VEN_ID_VALUE: c_int = 3;
pub const MC_SEQ_MISC0_REV_ID_SHIFT: c_int = 12;
pub const MC_SEQ_MISC0_REV_ID_MASK: c_uint = 0x0000f000;
pub const MC_SEQ_MISC0_REV_ID_VALUE: c_int = 1;
pub const MC_SEQ_MISC0_GDDR5_SHIFT: c_int = 28;
pub const MC_SEQ_MISC0_GDDR5_MASK: c_uint = 0xf0000000;
pub const MC_SEQ_MISC0_GDDR5_VALUE: c_int = 5;
pub const MC_SEQ_MISC1: c_uint = 0x2a04;
pub const MC_SEQ_RESERVE_M: c_uint = 0x2a08;
pub const MC_PMG_CMD_EMRS: c_uint = 0x2a0c;
pub const MC_SEQ_IO_DEBUG_INDEX: c_uint = 0x2a44;
pub const MC_SEQ_IO_DEBUG_DATA: c_uint = 0x2a48;
pub const MC_SEQ_MISC5: c_uint = 0x2a54;
pub const MC_SEQ_MISC6: c_uint = 0x2a58;
pub const MC_SEQ_MISC7: c_uint = 0x2a64;
pub const MC_SEQ_RAS_TIMING_LP: c_uint = 0x2a6c;
pub const MC_SEQ_CAS_TIMING_LP: c_uint = 0x2a70;
pub const MC_SEQ_MISC_TIMING_LP: c_uint = 0x2a74;
pub const MC_SEQ_MISC_TIMING2_LP: c_uint = 0x2a78;
pub const MC_SEQ_WR_CTL_D0_LP: c_uint = 0x2a7c;
pub const MC_SEQ_WR_CTL_D1_LP: c_uint = 0x2a80;
pub const MC_SEQ_PMG_CMD_EMRS_LP: c_uint = 0x2a84;
pub const MC_SEQ_PMG_CMD_MRS_LP: c_uint = 0x2a88;
pub const MC_PMG_CMD_MRS: c_uint = 0x2aac;
pub const MC_SEQ_RD_CTL_D0_LP: c_uint = 0x2b1c;
pub const MC_SEQ_RD_CTL_D1_LP: c_uint = 0x2b20;
pub const MC_PMG_CMD_MRS1: c_uint = 0x2b44;
pub const MC_SEQ_PMG_CMD_MRS1_LP: c_uint = 0x2b48;
pub const MC_SEQ_PMG_TIMING_LP: c_uint = 0x2b4c;
pub const MC_SEQ_WR_CTL_2: c_uint = 0x2b54;
pub const MC_SEQ_WR_CTL_2_LP: c_uint = 0x2b58;
pub const MC_PMG_CMD_MRS2: c_uint = 0x2b5c;
pub const MC_SEQ_PMG_CMD_MRS2_LP: c_uint = 0x2b60;
pub const MCLK_PWRMGT_CNTL: c_uint = 0x2ba0;

pub const DLL_CNTL: c_uint = 0x2ba4;

pub const MPLL_FUNC_CNTL: c_uint = 0x2bb4;

pub const MPLL_FUNC_CNTL_1: c_uint = 0x2bb8;

pub const MPLL_FUNC_CNTL_2: c_uint = 0x2bbc;
pub const MPLL_AD_FUNC_CNTL: c_uint = 0x2bc0;

pub const MPLL_DQ_FUNC_CNTL: c_uint = 0x2bc4;

pub const MPLL_SS1: c_uint = 0x2bcc;

pub const MPLL_SS2: c_uint = 0x2bd0;

pub const HDP_HOST_PATH_CNTL: c_uint = 0x2C00;

pub const HDP_NONSURFACE_BASE: c_uint = 0x2C04;
pub const HDP_NONSURFACE_INFO: c_uint = 0x2C08;
pub const HDP_NONSURFACE_SIZE: c_uint = 0x2C0C;
pub const HDP_ADDR_CONFIG: c_uint = 0x2F48;
pub const HDP_MISC_CNTL: c_uint = 0x2F4C;

pub const HDP_MEM_POWER_LS: c_uint = 0x2F50;

pub const ATC_MISC_CG: c_uint = 0x3350;
pub const GMCON_RENG_EXECUTE: c_uint = 0x3508;

pub const GMCON_MISC: c_uint = 0x350c;

pub const GMCON_PGFSM_CONFIG: c_uint = 0x3538;
pub const GMCON_PGFSM_WRITE: c_uint = 0x353c;
pub const GMCON_PGFSM_READ: c_uint = 0x3540;
pub const GMCON_MISC3: c_uint = 0x3544;
pub const MC_SEQ_CNTL_3: c_uint = 0x3600;

pub const MC_SEQ_G5PDX_CTRL: c_uint = 0x3604;
pub const MC_SEQ_G5PDX_CTRL_LP: c_uint = 0x3608;
pub const MC_SEQ_G5PDX_CMD0: c_uint = 0x360c;
pub const MC_SEQ_G5PDX_CMD0_LP: c_uint = 0x3610;
pub const MC_SEQ_G5PDX_CMD1: c_uint = 0x3614;
pub const MC_SEQ_G5PDX_CMD1_LP: c_uint = 0x3618;
pub const MC_SEQ_PMG_DVS_CTL: c_uint = 0x3628;
pub const MC_SEQ_PMG_DVS_CTL_LP: c_uint = 0x362c;
pub const MC_SEQ_PMG_DVS_CMD: c_uint = 0x3630;
pub const MC_SEQ_PMG_DVS_CMD_LP: c_uint = 0x3634;
pub const MC_SEQ_DLL_STBY: c_uint = 0x3638;
pub const MC_SEQ_DLL_STBY_LP: c_uint = 0x363c;
pub const IH_RB_CNTL: c_uint = 0x3e00;

pub const IH_RB_BASE: c_uint = 0x3e04;
pub const IH_RB_RPTR: c_uint = 0x3e08;
pub const IH_RB_WPTR: c_uint = 0x3e0c;

pub const IH_RB_WPTR_ADDR_HI: c_uint = 0x3e10;
pub const IH_RB_WPTR_ADDR_LO: c_uint = 0x3e14;
pub const IH_CNTL: c_uint = 0x3e18;

pub const BIF_LNCNT_RESET: c_uint = 0x5220;

pub const CONFIG_MEMSIZE: c_uint = 0x5428;
pub const INTERRUPT_CNTL: c_uint = 0x5468;

pub const INTERRUPT_CNTL2: c_uint = 0x546c;
pub const HDP_MEM_COHERENCY_FLUSH_CNTL: c_uint = 0x5480;
pub const BIF_FB_EN: c_uint = 0x5490;

pub const HDP_REG_COHERENCY_FLUSH_CNTL: c_uint = 0x54A0;
pub const GPU_HDP_FLUSH_REQ: c_uint = 0x54DC;
pub const GPU_HDP_FLUSH_DONE: c_uint = 0x54E0;

// 0x6b04, 0x7704, 0x10304, 0x10f04, 0x11b04, 0x12704
pub const LB_MEMORY_CTRL: c_uint = 0x6b04;

pub const DPG_WATERMARK_MASK_CONTROL: c_uint = 0x6cc8;

pub const DPG_PIPE_LATENCY_CONTROL: c_uint = 0x6ccc;

// 0x6b24, 0x7724, 0x10324, 0x10f24, 0x11b24, 0x12724
pub const LB_VLINE_STATUS: c_uint = 0x6b24;

// 0x6b2c, 0x772c, 0x1032c, 0x10f2c, 0x11b2c, 0x1272c
pub const LB_VBLANK_STATUS: c_uint = 0x6b2c;

// 0x6b20, 0x7720, 0x10320, 0x10f20, 0x11b20, 0x12720
pub const LB_INTERRUPT_MASK: c_uint = 0x6b20;

pub const DISP_INTERRUPT_STATUS: c_uint = 0x60f4;

pub const DISP_INTERRUPT_STATUS_CONTINUE: c_uint = 0x60f8;

pub const DISP_INTERRUPT_STATUS_CONTINUE2: c_uint = 0x60fc;

pub const DISP_INTERRUPT_STATUS_CONTINUE3: c_uint = 0x6100;

pub const DISP_INTERRUPT_STATUS_CONTINUE4: c_uint = 0x614c;

pub const DISP_INTERRUPT_STATUS_CONTINUE5: c_uint = 0x6150;

pub const DISP_INTERRUPT_STATUS_CONTINUE6: c_uint = 0x6780;
// 0x6858, 0x7458, 0x10058, 0x10c58, 0x11858, 0x12458
pub const GRPH_INT_STATUS: c_uint = 0x6858;

// 0x685c, 0x745c, 0x1005c, 0x10c5c, 0x1185c, 0x1245c
pub const GRPH_INT_CONTROL: c_uint = 0x685c;

pub const DAC_AUTODETECT_INT_CONTROL: c_uint = 0x67c8;
pub const DC_HPD1_INT_STATUS: c_uint = 0x601c;
pub const DC_HPD2_INT_STATUS: c_uint = 0x6028;
pub const DC_HPD3_INT_STATUS: c_uint = 0x6034;
pub const DC_HPD4_INT_STATUS: c_uint = 0x6040;
pub const DC_HPD5_INT_STATUS: c_uint = 0x604c;
pub const DC_HPD6_INT_STATUS: c_uint = 0x6058;

pub const DC_HPD1_INT_CONTROL: c_uint = 0x6020;
pub const DC_HPD2_INT_CONTROL: c_uint = 0x602c;
pub const DC_HPD3_INT_CONTROL: c_uint = 0x6038;
pub const DC_HPD4_INT_CONTROL: c_uint = 0x6044;
pub const DC_HPD5_INT_CONTROL: c_uint = 0x6050;
pub const DC_HPD6_INT_CONTROL: c_uint = 0x605c;

pub const DC_HPD1_CONTROL: c_uint = 0x6024;
pub const DC_HPD2_CONTROL: c_uint = 0x6030;
pub const DC_HPD3_CONTROL: c_uint = 0x603c;
pub const DC_HPD4_CONTROL: c_uint = 0x6048;
pub const DC_HPD5_CONTROL: c_uint = 0x6054;
pub const DC_HPD6_CONTROL: c_uint = 0x6060;

pub const DPG_PIPE_STUTTER_CONTROL: c_uint = 0x6cd4;

// DCE8 FMT blocks
pub const FMT_DYNAMIC_EXP_CNTL: c_uint = 0x6fb4;

// 0 = 10bit -> 12bit, 1 = 8bit -> 12bit
pub const FMT_CONTROL: c_uint = 0x6fb8;

// 0 = RGB 4:4:4 or YCbCr 4:4:4, 1 = YCbCr 4:2:2
pub const FMT_BIT_DEPTH_CONTROL: c_uint = 0x6fc8;

pub const FMT_CLAMP_CONTROL: c_uint = 0x6fe4;

pub const GRBM_CNTL: c_uint = 0x8000;

pub const GRBM_STATUS2: c_uint = 0x8008;
pub const ME0PIPE1_CMDFIFO_AVAIL_MASK: c_uint = 0x0000000F;

pub const GRBM_STATUS: c_uint = 0x8010;
pub const ME0PIPE0_CMDFIFO_AVAIL_MASK: c_uint = 0x0000000F;

pub const GRBM_STATUS_SE0: c_uint = 0x8014;
pub const GRBM_STATUS_SE1: c_uint = 0x8018;
pub const GRBM_STATUS_SE2: c_uint = 0x8038;
pub const GRBM_STATUS_SE3: c_uint = 0x803C;

pub const GRBM_SOFT_RESET: c_uint = 0x8020;

pub const GRBM_INT_CNTL: c_uint = 0x8060;

pub const CP_CPC_STATUS: c_uint = 0x8210;
pub const CP_CPC_BUSY_STAT: c_uint = 0x8214;
pub const CP_CPC_STALLED_STAT1: c_uint = 0x8218;
pub const CP_CPF_STATUS: c_uint = 0x821c;
pub const CP_CPF_BUSY_STAT: c_uint = 0x8220;
pub const CP_CPF_STALLED_STAT1: c_uint = 0x8224;
pub const CP_MEC_CNTL: c_uint = 0x8234;

pub const CP_MEC_CNTL: c_uint = 0x8234;

pub const CP_STALLED_STAT3: c_uint = 0x8670;
pub const CP_STALLED_STAT1: c_uint = 0x8674;
pub const CP_STALLED_STAT2: c_uint = 0x8678;
pub const CP_STAT: c_uint = 0x8680;
pub const CP_ME_CNTL: c_uint = 0x86D8;

pub const CP_RB0_RPTR: c_uint = 0x8700;
pub const CP_RB_WPTR_DELAY: c_uint = 0x8704;
pub const CP_RB_WPTR_POLL_CNTL: c_uint = 0x8708;

pub const CP_MEQ_THRESHOLDS: c_uint = 0x8764;

pub const VGT_VTX_VECT_EJECT_REG: c_uint = 0x88B0;
pub const VGT_CACHE_INVALIDATION: c_uint = 0x88C4;

pub const VC_ONLY: c_int = 0;
pub const TC_ONLY: c_int = 1;
pub const VC_AND_TC: c_int = 2;

pub const NO_AUTO: c_int = 0;
pub const ES_AUTO: c_int = 1;
pub const GS_AUTO: c_int = 2;
pub const ES_AND_GS_AUTO: c_int = 3;
pub const VGT_GS_VERTEX_REUSE: c_uint = 0x88D4;
pub const CC_GC_SHADER_ARRAY_CONFIG: c_uint = 0x89bc;
pub const INACTIVE_CUS_MASK: c_uint = 0xFFFF0000;
pub const INACTIVE_CUS_SHIFT: c_int = 16;
pub const GC_USER_SHADER_ARRAY_CONFIG: c_uint = 0x89c0;
pub const PA_CL_ENHANCE: c_uint = 0x8A14;

pub const PA_SC_FORCE_EOV_MAX_CNTS: c_uint = 0x8B24;

pub const PA_SC_FIFO_SIZE: c_uint = 0x8BCC;

pub const PA_SC_ENHANCE: c_uint = 0x8BF0;

pub const SQ_CONFIG: c_uint = 0x8C00;
pub const SH_MEM_BASES: c_uint = 0x8C28;
// if PTR32, these are the bases for scratch and lds

pub const SH_MEM_APE1_BASE: c_uint = 0x8C2C;
// if PTR32, this is the base location of GPUVM
pub const SH_MEM_APE1_LIMIT: c_uint = 0x8C30;
// if PTR32, this is the upper limit of GPUVM
pub const SH_MEM_CONFIG: c_uint = 0x8C34;

pub const SH_MEM_ALIGNMENT_MODE_DWORD: c_int = 0;
pub const SH_MEM_ALIGNMENT_MODE_DWORD_STRICT: c_int = 1;
pub const SH_MEM_ALIGNMENT_MODE_STRICT: c_int = 2;
pub const SH_MEM_ALIGNMENT_MODE_UNALIGNED: c_int = 3;

// valid for both DEFAULT_MTYPE and APE1_MTYPE
pub const MTYPE_CACHED: c_int = 0;
pub const MTYPE_NONCACHED: c_int = 3;
pub const SX_DEBUG_1: c_uint = 0x9060;
pub const SPI_CONFIG_CNTL: c_uint = 0x9100;
pub const SPI_CONFIG_CNTL_1: c_uint = 0x913C;

pub const TA_CNTL_AUX: c_uint = 0x9508;
pub const DB_DEBUG: c_uint = 0x9830;
pub const DB_DEBUG2: c_uint = 0x9834;
pub const DB_DEBUG3: c_uint = 0x9838;
pub const CC_RB_BACKEND_DISABLE: c_uint = 0x98F4;

pub const GB_ADDR_CONFIG: c_uint = 0x98F8;

pub const NUM_PIPES_MASK: c_uint = 0x00000007;
pub const NUM_PIPES_SHIFT: c_int = 0;

pub const PIPE_INTERLEAVE_SIZE_MASK: c_uint = 0x00000070;
pub const PIPE_INTERLEAVE_SIZE_SHIFT: c_int = 4;

pub const NUM_SHADER_ENGINES_MASK: c_uint = 0x00003000;
pub const NUM_SHADER_ENGINES_SHIFT: c_int = 12;

pub const SHADER_ENGINE_TILE_SIZE_MASK: c_uint = 0x00070000;
pub const SHADER_ENGINE_TILE_SIZE_SHIFT: c_int = 16;

pub const ROW_SIZE_MASK: c_uint = 0x30000000;
pub const ROW_SIZE_SHIFT: c_int = 28;
pub const GB_TILE_MODE0: c_uint = 0x9910;

pub const GB_MACROTILE_MODE0: c_uint = 0x9990;

pub const CB_HW_CONTROL: c_uint = 0x9A10;
pub const GC_USER_RB_BACKEND_DISABLE: c_uint = 0x9B7C;
pub const BACKEND_DISABLE_MASK: c_uint = 0x00FF0000;
pub const BACKEND_DISABLE_SHIFT: c_int = 16;
pub const TCP_CHAN_STEER_LO: c_uint = 0xac0c;
pub const TCP_CHAN_STEER_HI: c_uint = 0xac10;
pub const TC_CFG_L1_LOAD_POLICY0: c_uint = 0xAC68;
pub const TC_CFG_L1_LOAD_POLICY1: c_uint = 0xAC6C;
pub const TC_CFG_L1_STORE_POLICY: c_uint = 0xAC70;
pub const TC_CFG_L2_LOAD_POLICY0: c_uint = 0xAC74;
pub const TC_CFG_L2_LOAD_POLICY1: c_uint = 0xAC78;
pub const TC_CFG_L2_STORE_POLICY0: c_uint = 0xAC7C;
pub const TC_CFG_L2_STORE_POLICY1: c_uint = 0xAC80;
pub const TC_CFG_L2_ATOMIC_POLICY: c_uint = 0xAC84;
pub const TC_CFG_L1_VOLATILE: c_uint = 0xAC88;
pub const TC_CFG_L2_VOLATILE: c_uint = 0xAC8C;
pub const CP_RB0_BASE: c_uint = 0xC100;
pub const CP_RB0_CNTL: c_uint = 0xC104;

pub const CP_RB0_RPTR_ADDR: c_uint = 0xC10C;

pub const CP_RB0_RPTR_ADDR_HI: c_uint = 0xC110;
pub const CP_RB0_WPTR: c_uint = 0xC114;
pub const CP_DEVICE_ID: c_uint = 0xC12C;
pub const CP_ENDIAN_SWAP: c_uint = 0xC140;
pub const CP_RB_VMID: c_uint = 0xC144;
pub const CP_PFP_UCODE_ADDR: c_uint = 0xC150;
pub const CP_PFP_UCODE_DATA: c_uint = 0xC154;
pub const CP_ME_RAM_RADDR: c_uint = 0xC158;
pub const CP_ME_RAM_WADDR: c_uint = 0xC15C;
pub const CP_ME_RAM_DATA: c_uint = 0xC160;
pub const CP_CE_UCODE_ADDR: c_uint = 0xC168;
pub const CP_CE_UCODE_DATA: c_uint = 0xC16C;
pub const CP_MEC_ME1_UCODE_ADDR: c_uint = 0xC170;
pub const CP_MEC_ME1_UCODE_DATA: c_uint = 0xC174;
pub const CP_MEC_ME2_UCODE_ADDR: c_uint = 0xC178;
pub const CP_MEC_ME2_UCODE_DATA: c_uint = 0xC17C;
pub const CP_INT_CNTL_RING0: c_uint = 0xC1A8;

pub const CP_INT_STATUS_RING0: c_uint = 0xC1B4;

pub const CP_MEM_SLP_CNTL: c_uint = 0xC1E4;

pub const CP_CPF_DEBUG: c_uint = 0xC200;
pub const CP_PQ_WPTR_POLL_CNTL: c_uint = 0xC20C;

pub const CP_ME1_PIPE0_INT_CNTL: c_uint = 0xC214;
pub const CP_ME1_PIPE1_INT_CNTL: c_uint = 0xC218;
pub const CP_ME1_PIPE2_INT_CNTL: c_uint = 0xC21C;
pub const CP_ME1_PIPE3_INT_CNTL: c_uint = 0xC220;
pub const CP_ME2_PIPE0_INT_CNTL: c_uint = 0xC224;
pub const CP_ME2_PIPE1_INT_CNTL: c_uint = 0xC228;
pub const CP_ME2_PIPE2_INT_CNTL: c_uint = 0xC22C;
pub const CP_ME2_PIPE3_INT_CNTL: c_uint = 0xC230;

pub const CP_ME1_PIPE0_INT_STATUS: c_uint = 0xC214;
pub const CP_ME1_PIPE1_INT_STATUS: c_uint = 0xC218;
pub const CP_ME1_PIPE2_INT_STATUS: c_uint = 0xC21C;
pub const CP_ME1_PIPE3_INT_STATUS: c_uint = 0xC220;
pub const CP_ME2_PIPE0_INT_STATUS: c_uint = 0xC224;
pub const CP_ME2_PIPE1_INT_STATUS: c_uint = 0xC228;
pub const CP_ME2_PIPE2_INT_STATUS: c_uint = 0xC22C;
pub const CP_ME2_PIPE3_INT_STATUS: c_uint = 0xC230;

pub const CP_MAX_CONTEXT: c_uint = 0xC2B8;
pub const CP_RB0_BASE_HI: c_uint = 0xC2C4;
pub const RLC_CNTL: c_uint = 0xC300;

pub const RLC_MC_CNTL: c_uint = 0xC30C;
pub const RLC_MEM_SLP_CNTL: c_uint = 0xC318;

pub const RLC_LB_CNTR_MAX: c_uint = 0xC348;
pub const RLC_LB_CNTL: c_uint = 0xC364;

pub const RLC_LB_CNTR_INIT: c_uint = 0xC36C;
pub const RLC_SAVE_AND_RESTORE_BASE: c_uint = 0xC374;
pub const RLC_DRIVER_DMA_STATUS: c_uint = 0xC378 /* dGPU */;
pub const RLC_CP_TABLE_RESTORE: c_uint = 0xC378 /* APU */;
pub const RLC_PG_DELAY_2: c_uint = 0xC37C;
pub const RLC_GPM_UCODE_ADDR: c_uint = 0xC388;
pub const RLC_GPM_UCODE_DATA: c_uint = 0xC38C;
pub const RLC_GPU_CLOCK_COUNT_LSB: c_uint = 0xC390;
pub const RLC_GPU_CLOCK_COUNT_MSB: c_uint = 0xC394;
pub const RLC_CAPTURE_GPU_CLOCK_COUNT: c_uint = 0xC398;
pub const RLC_UCODE_CNTL: c_uint = 0xC39C;
pub const RLC_GPM_STAT: c_uint = 0xC400;

pub const RLC_PG_CNTL: c_uint = 0xC40C;

pub const RLC_CGTT_MGCG_OVERRIDE: c_uint = 0xC420;
pub const RLC_CGCG_CGLS_CTRL: c_uint = 0xC424;

pub const RLC_PG_DELAY: c_uint = 0xC434;
pub const RLC_LB_INIT_CU_MASK: c_uint = 0xC43C;
pub const RLC_LB_PARAMS: c_uint = 0xC444;
pub const RLC_PG_AO_CU_MASK: c_uint = 0xC44C;
pub const RLC_MAX_PG_CU: c_uint = 0xC450;

pub const RLC_AUTO_PG_CTRL: c_uint = 0xC454;

pub const RLC_SERDES_WR_CU_MASTER_MASK: c_uint = 0xC474;
pub const RLC_SERDES_WR_NONCU_MASTER_MASK: c_uint = 0xC478;
pub const RLC_SERDES_WR_CTRL: c_uint = 0xC47C;

pub const RLC_SERDES_CU_MASTER_BUSY: c_uint = 0xC484;
pub const RLC_SERDES_NONCU_MASTER_BUSY: c_uint = 0xC488;

pub const RLC_GPM_SCRATCH_ADDR: c_uint = 0xC4B0;
pub const RLC_GPM_SCRATCH_DATA: c_uint = 0xC4B4;
pub const RLC_GPR_REG2: c_uint = 0xC4E8;
pub const REQ: c_uint = 0x00000001;

pub const MESSAGE_MASK: c_uint = 0x0000001e;
pub const MSG_ENTER_RLC_SAFE_MODE: c_int = 1;
pub const MSG_EXIT_RLC_SAFE_MODE: c_int = 0;
pub const CP_HPD_EOP_BASE_ADDR: c_uint = 0xC904;
pub const CP_HPD_EOP_BASE_ADDR_HI: c_uint = 0xC908;
pub const CP_HPD_EOP_VMID: c_uint = 0xC90C;
pub const CP_HPD_EOP_CONTROL: c_uint = 0xC910;

pub const CP_MQD_BASE_ADDR: c_uint = 0xC914;
pub const CP_MQD_BASE_ADDR_HI: c_uint = 0xC918;
pub const CP_HQD_ACTIVE: c_uint = 0xC91C;
pub const CP_HQD_VMID: c_uint = 0xC920;
pub const CP_HQD_PERSISTENT_STATE: c_uint = 0xC924u;

pub const CP_HQD_PIPE_PRIORITY: c_uint = 0xC928u;
pub const CP_HQD_QUEUE_PRIORITY: c_uint = 0xC92Cu;
pub const CP_HQD_QUANTUM: c_uint = 0xC930u;

pub const CP_HQD_PQ_BASE: c_uint = 0xC934;
pub const CP_HQD_PQ_BASE_HI: c_uint = 0xC938;
pub const CP_HQD_PQ_RPTR: c_uint = 0xC93C;
pub const CP_HQD_PQ_RPTR_REPORT_ADDR: c_uint = 0xC940;
pub const CP_HQD_PQ_RPTR_REPORT_ADDR_HI: c_uint = 0xC944;
pub const CP_HQD_PQ_WPTR_POLL_ADDR: c_uint = 0xC948;
pub const CP_HQD_PQ_WPTR_POLL_ADDR_HI: c_uint = 0xC94C;
pub const CP_HQD_PQ_DOORBELL_CONTROL: c_uint = 0xC950;

pub const CP_HQD_PQ_WPTR: c_uint = 0xC954;
pub const CP_HQD_PQ_CONTROL: c_uint = 0xC958;

pub const CP_HQD_IB_BASE_ADDR: c_uint = 0xC95Cu;
pub const CP_HQD_IB_BASE_ADDR_HI: c_uint = 0xC960u;
pub const CP_HQD_IB_RPTR: c_uint = 0xC964u;
pub const CP_HQD_IB_CONTROL: c_uint = 0xC968u;

pub const CP_HQD_DEQUEUE_REQUEST: c_uint = 0xC974;
pub const DEQUEUE_REQUEST_DRAIN: c_int = 1;
pub const DEQUEUE_REQUEST_RESET: c_int = 2;
pub const CP_MQD_CONTROL: c_uint = 0xC99C;

pub const CP_HQD_SEMA_CMD: c_uint = 0xC97Cu;
pub const CP_HQD_MSG_TYPE: c_uint = 0xC980u;
pub const CP_HQD_ATOMIC0_PREOP_LO: c_uint = 0xC984u;
pub const CP_HQD_ATOMIC0_PREOP_HI: c_uint = 0xC988u;
pub const CP_HQD_ATOMIC1_PREOP_LO: c_uint = 0xC98Cu;
pub const CP_HQD_ATOMIC1_PREOP_HI: c_uint = 0xC990u;
pub const CP_HQD_HQ_SCHEDULER0: c_uint = 0xC994u;
pub const CP_HQD_HQ_SCHEDULER1: c_uint = 0xC998u;
pub const SH_STATIC_MEM_CONFIG: c_uint = 0x9604u;
pub const DB_RENDER_CONTROL: c_uint = 0x28000;
pub const PA_SC_RASTER_CONFIG: c_uint = 0x28350;

pub const VGT_EVENT_INITIATOR: c_uint = 0x28a90;

pub const SCRATCH_REG0: c_uint = 0x30100;
pub const SCRATCH_REG1: c_uint = 0x30104;
pub const SCRATCH_REG2: c_uint = 0x30108;
pub const SCRATCH_REG3: c_uint = 0x3010C;
pub const SCRATCH_REG4: c_uint = 0x30110;
pub const SCRATCH_REG5: c_uint = 0x30114;
pub const SCRATCH_REG6: c_uint = 0x30118;
pub const SCRATCH_REG7: c_uint = 0x3011C;
pub const SCRATCH_UMSK: c_uint = 0x30140;
pub const SCRATCH_ADDR: c_uint = 0x30144;
pub const CP_SEM_WAIT_TIMER: c_uint = 0x301BC;
pub const CP_SEM_INCOMPLETE_TIMER_CNTL: c_uint = 0x301C8;
pub const CP_WAIT_REG_MEM_TIMEOUT: c_uint = 0x301D0;
pub const GRBM_GFX_INDEX: c_uint = 0x30800;

pub const VGT_ESGS_RING_SIZE: c_uint = 0x30900;
pub const VGT_GSVS_RING_SIZE: c_uint = 0x30904;
pub const VGT_PRIMITIVE_TYPE: c_uint = 0x30908;
pub const VGT_INDEX_TYPE: c_uint = 0x3090C;
pub const VGT_NUM_INDICES: c_uint = 0x30930;
pub const VGT_NUM_INSTANCES: c_uint = 0x30934;
pub const VGT_TF_RING_SIZE: c_uint = 0x30938;
pub const VGT_HS_OFFCHIP_PARAM: c_uint = 0x3093C;
pub const VGT_TF_MEMORY_BASE: c_uint = 0x30940;
pub const PA_SU_LINE_STIPPLE_VALUE: c_uint = 0x30a00;
pub const PA_SC_LINE_STIPPLE_STATE: c_uint = 0x30a04;
pub const SQC_CACHES: c_uint = 0x30d20;
pub const CP_PERFMON_CNTL: c_uint = 0x36020;
pub const CGTS_SM_CTRL_REG: c_uint = 0x3c000;

pub const CGTS_TCC_DISABLE: c_uint = 0x3c00c;
pub const CGTS_USER_TCC_DISABLE: c_uint = 0x3c010;
pub const TCC_DISABLE_MASK: c_uint = 0xFFFF0000;
pub const TCC_DISABLE_SHIFT: c_int = 16;
pub const CB_CGTT_SCLK_CTRL: c_uint = 0x3c2a0;
//
// PM4
//
pub const PACKET_TYPE0: c_int = 0;
pub const PACKET_TYPE1: c_int = 1;
pub const PACKET_TYPE2: c_int = 2;
pub const PACKET_TYPE3: c_int = 3;

pub const CP_PACKET2: c_uint = 0x80000000;
pub const PACKET2_PAD_SHIFT: c_int = 0;

// Packet 3 types
pub const PACKET3_NOP: c_uint = 0x10;
pub const PACKET3_SET_BASE: c_uint = 0x11;

pub const CE_PARTITION_BASE: c_int = 3;
pub const PACKET3_CLEAR_STATE: c_uint = 0x12;
pub const PACKET3_INDEX_BUFFER_SIZE: c_uint = 0x13;
pub const PACKET3_DISPATCH_DIRECT: c_uint = 0x15;
pub const PACKET3_DISPATCH_INDIRECT: c_uint = 0x16;
pub const PACKET3_ATOMIC_GDS: c_uint = 0x1D;
pub const PACKET3_ATOMIC_MEM: c_uint = 0x1E;
pub const PACKET3_OCCLUSION_QUERY: c_uint = 0x1F;
pub const PACKET3_SET_PREDICATION: c_uint = 0x20;
pub const PACKET3_REG_RMW: c_uint = 0x21;
pub const PACKET3_COND_EXEC: c_uint = 0x22;
pub const PACKET3_PRED_EXEC: c_uint = 0x23;
pub const PACKET3_DRAW_INDIRECT: c_uint = 0x24;
pub const PACKET3_DRAW_INDEX_INDIRECT: c_uint = 0x25;
pub const PACKET3_INDEX_BASE: c_uint = 0x26;
pub const PACKET3_DRAW_INDEX_2: c_uint = 0x27;
pub const PACKET3_CONTEXT_CONTROL: c_uint = 0x28;
pub const PACKET3_INDEX_TYPE: c_uint = 0x2A;
pub const PACKET3_DRAW_INDIRECT_MULTI: c_uint = 0x2C;
pub const PACKET3_DRAW_INDEX_AUTO: c_uint = 0x2D;
pub const PACKET3_NUM_INSTANCES: c_uint = 0x2F;
pub const PACKET3_DRAW_INDEX_MULTI_AUTO: c_uint = 0x30;
pub const PACKET3_INDIRECT_BUFFER_CONST: c_uint = 0x33;
pub const PACKET3_STRMOUT_BUFFER_UPDATE: c_uint = 0x34;
pub const PACKET3_DRAW_INDEX_OFFSET_2: c_uint = 0x35;
pub const PACKET3_DRAW_PREAMBLE: c_uint = 0x36;
pub const PACKET3_WRITE_DATA: c_uint = 0x37;

// 0 - register
// 1 - memory (sync - via GRBM)
// 2 - gl2
// 3 - gds
// 4 - reserved
// 5 - memory (async - direct)
//

// 0 - LRU
// 1 - Stream
//

// 0 - me
// 1 - pfp
// 2 - ce
//
pub const PACKET3_DRAW_INDEX_INDIRECT_MULTI: c_uint = 0x38;
pub const PACKET3_MEM_SEMAPHORE: c_uint = 0x39;

pub const PACKET3_COPY_DW: c_uint = 0x3B;
pub const PACKET3_WAIT_REG_MEM: c_uint = 0x3C;

// 0 - always
// 1 - <
// 2 - <=
// 3 - ==
// 4 - !=
// 5 - >=
// 6 - >
//

// 0 - reg
// 1 - mem
//

// 0 - wait_reg_mem
// 1 - wr_wait_wr_reg
//

// 0 - me
// 1 - pfp
//
pub const PACKET3_INDIRECT_BUFFER: c_uint = 0x3F;

// 0 - LRU
// 1 - Stream
// 2 - Bypass
//
pub const PACKET3_COPY_DATA: c_uint = 0x40;
pub const PACKET3_PFP_SYNC_ME: c_uint = 0x42;
pub const PACKET3_SURFACE_SYNC: c_uint = 0x43;

pub const PACKET3_COND_WRITE: c_uint = 0x45;
pub const PACKET3_EVENT_WRITE: c_uint = 0x46;

// 0 - any non-TS event
// 1 - ZPASS_DONE, PIXEL_PIPE_STAT_
// 2 - SAMPLE_PIPELINESTAT
// 3 - SAMPLE_STREAMOUTSTAT
// 4 - *S_PARTIAL_FLUSH
// 5 - EOP events
// 6 - EOS events
//
pub const PACKET3_EVENT_WRITE_EOP: c_uint = 0x47;

// 0 - LRU
// 1 - Stream
// 2 - Bypass
//

// 0 - discard
// 1 - send low 32bit data
// 2 - send 64bit data
// 3 - send 64bit GPU counter value
// 4 - send 64bit sys counter value
//

// 0 - none
// 1 - interrupt only (DATA_SEL = 0)
// 2 - interrupt when data write is confirmed
//

// 0 - MC
// 1 - TC/L2
//
pub const PACKET3_EVENT_WRITE_EOS: c_uint = 0x48;
pub const PACKET3_RELEASE_MEM: c_uint = 0x49;
pub const PACKET3_PREAMBLE_CNTL: c_uint = 0x4A;

pub const PACKET3_DMA_DATA: c_uint = 0x50;
// 1. header
// 2. CONTROL
// 3. SRC_ADDR_LO or DATA [31:0]
// 4. SRC_ADDR_HI [31:0]
// 5. DST_ADDR_LO [31:0]
// 6. DST_ADDR_HI [7:0]
// 7. COMMAND [30:21] | BYTE_COUNT [20:0]
//
// CONTROL

// 0 - ME
// 1 - PFP
//

// 0 - LRU
// 1 - Stream
// 2 - Bypass
//

// 0 - DST_ADDR using DAS
// 1 - GDS
// 3 - DST_ADDR using L2
//

// 0 - LRU
// 1 - Stream
// 2 - Bypass
//

// 0 - SRC_ADDR using SAS
// 1 - GDS
// 2 - DATA
// 3 - SRC_ADDR using L2
//

// COMMAND

// 0 - none
// 1 - 8 in 16
// 2 - 8 in 32
// 3 - 8 in 64
//

// 0 - none
// 1 - 8 in 16
// 2 - 8 in 32
// 3 - 8 in 64
//

// 0 - memory
// 1 - register
//

// 0 - memory
// 1 - register
//

pub const PACKET3_AQUIRE_MEM: c_uint = 0x58;
pub const PACKET3_REWIND: c_uint = 0x59;
pub const PACKET3_LOAD_UCONFIG_REG: c_uint = 0x5E;
pub const PACKET3_LOAD_SH_REG: c_uint = 0x5F;
pub const PACKET3_LOAD_CONFIG_REG: c_uint = 0x60;
pub const PACKET3_LOAD_CONTEXT_REG: c_uint = 0x61;
pub const PACKET3_SET_CONFIG_REG: c_uint = 0x68;
pub const PACKET3_SET_CONFIG_REG_START: c_uint = 0x00008000;
pub const PACKET3_SET_CONFIG_REG_END: c_uint = 0x0000b000;
pub const PACKET3_SET_CONTEXT_REG: c_uint = 0x69;
pub const PACKET3_SET_CONTEXT_REG_START: c_uint = 0x00028000;
pub const PACKET3_SET_CONTEXT_REG_END: c_uint = 0x00029000;
pub const PACKET3_SET_CONTEXT_REG_INDIRECT: c_uint = 0x73;
pub const PACKET3_SET_SH_REG: c_uint = 0x76;
pub const PACKET3_SET_SH_REG_START: c_uint = 0x0000b000;
pub const PACKET3_SET_SH_REG_END: c_uint = 0x0000c000;
pub const PACKET3_SET_SH_REG_OFFSET: c_uint = 0x77;
pub const PACKET3_SET_QUEUE_REG: c_uint = 0x78;
pub const PACKET3_SET_UCONFIG_REG: c_uint = 0x79;
pub const PACKET3_SET_UCONFIG_REG_START: c_uint = 0x00030000;
pub const PACKET3_SET_UCONFIG_REG_END: c_uint = 0x00031000;
pub const PACKET3_SCRATCH_RAM_WRITE: c_uint = 0x7D;
pub const PACKET3_SCRATCH_RAM_READ: c_uint = 0x7E;
pub const PACKET3_LOAD_CONST_RAM: c_uint = 0x80;
pub const PACKET3_WRITE_CONST_RAM: c_uint = 0x81;
pub const PACKET3_DUMP_CONST_RAM: c_uint = 0x83;
pub const PACKET3_INCREMENT_CE_COUNTER: c_uint = 0x84;
pub const PACKET3_INCREMENT_DE_COUNTER: c_uint = 0x85;
pub const PACKET3_WAIT_ON_CE_COUNTER: c_uint = 0x86;
pub const PACKET3_WAIT_ON_DE_COUNTER_DIFF: c_uint = 0x88;
pub const PACKET3_SWITCH_BUFFER: c_uint = 0x8B;
// SDMA - first instance at 0xd000, second at 0xd800
pub const SDMA0_REGISTER_OFFSET: c_uint = 0x0 /* not a register */;
pub const SDMA1_REGISTER_OFFSET: c_uint = 0x800 /* not a register */;
pub const SDMA0_UCODE_ADDR: c_uint = 0xD000;
pub const SDMA0_UCODE_DATA: c_uint = 0xD004;
pub const SDMA0_POWER_CNTL: c_uint = 0xD008;
pub const SDMA0_CLK_CTRL: c_uint = 0xD00C;
pub const SDMA0_CNTL: c_uint = 0xD010;

pub const SDMA0_TILING_CONFIG: c_uint = 0xD018;
pub const SDMA0_SEM_INCOMPLETE_TIMER_CNTL: c_uint = 0xD020;
pub const SDMA0_SEM_WAIT_FAIL_TIMER_CNTL: c_uint = 0xD024;
pub const SDMA0_STATUS_REG: c_uint = 0xd034;

pub const SDMA0_ME_CNTL: c_uint = 0xD048;

pub const SDMA0_GFX_RB_CNTL: c_uint = 0xD200;

pub const SDMA0_GFX_RB_BASE: c_uint = 0xD204;
pub const SDMA0_GFX_RB_BASE_HI: c_uint = 0xD208;
pub const SDMA0_GFX_RB_RPTR: c_uint = 0xD20C;
pub const SDMA0_GFX_RB_WPTR: c_uint = 0xD210;
pub const SDMA0_GFX_RB_RPTR_ADDR_HI: c_uint = 0xD220;
pub const SDMA0_GFX_RB_RPTR_ADDR_LO: c_uint = 0xD224;
pub const SDMA0_GFX_IB_CNTL: c_uint = 0xD228;

pub const SDMA0_GFX_VIRTUAL_ADDR: c_uint = 0xD29C;
pub const SDMA0_GFX_APE1_CNTL: c_uint = 0xD2A0;

// sDMA opcodes
pub const SDMA_OPCODE_NOP: c_int = 0;
pub const SDMA_OPCODE_COPY: c_int = 1;

pub const SDMA_OPCODE_WRITE: c_int = 2;

pub const SDMA_OPCODE_INDIRECT_BUFFER: c_int = 4;
pub const SDMA_OPCODE_FENCE: c_int = 5;
pub const SDMA_OPCODE_TRAP: c_int = 6;
pub const SDMA_OPCODE_SEMAPHORE: c_int = 7;

// 0 - increment
// 1 - write 1
//

// 0 - wait
// 1 - signal
//

// mailbox
pub const SDMA_OPCODE_POLL_REG_MEM: c_int = 8;

// 0 - wait_reg_mem
// 1 - wr_wait_wr_reg
//

// 0 - always
// 1 - <
// 2 - <=
// 3 - ==
// 4 - !=
// 5 - >=
// 6 - >
//

// 0 = register
// 1 = memory
//
pub const SDMA_OPCODE_COND_EXEC: c_int = 9;
pub const SDMA_OPCODE_CONSTANT_FILL: c_int = 11;

// 0 = byte fill
// 2 = DW fill
//
pub const SDMA_OPCODE_GENERATE_PTE_PDE: c_int = 12;
pub const SDMA_OPCODE_TIMESTAMP: c_int = 13;

pub const SDMA_OPCODE_SRBM_WRITE: c_int = 14;

// byte mask
// UVD
pub const UVD_UDEC_ADDR_CONFIG: c_uint = 0xef4c;
pub const UVD_UDEC_DB_ADDR_CONFIG: c_uint = 0xef50;
pub const UVD_UDEC_DBW_ADDR_CONFIG: c_uint = 0xef54;
pub const UVD_NO_OP: c_uint = 0xeffc;
pub const UVD_LMI_EXT40_ADDR: c_uint = 0xf498;
pub const UVD_GP_SCRATCH4: c_uint = 0xf4e0;
pub const UVD_LMI_ADDR_EXT: c_uint = 0xf594;
pub const UVD_VCPU_CACHE_OFFSET0: c_uint = 0xf608;
pub const UVD_VCPU_CACHE_SIZE0: c_uint = 0xf60c;
pub const UVD_VCPU_CACHE_OFFSET1: c_uint = 0xf610;
pub const UVD_VCPU_CACHE_SIZE1: c_uint = 0xf614;
pub const UVD_VCPU_CACHE_OFFSET2: c_uint = 0xf618;
pub const UVD_VCPU_CACHE_SIZE2: c_uint = 0xf61c;
pub const UVD_RBC_RB_RPTR: c_uint = 0xf690;
pub const UVD_RBC_RB_WPTR: c_uint = 0xf694;
pub const UVD_CGC_CTRL: c_uint = 0xF4B0;

pub const UVD_STATUS: c_uint = 0xf6bc;
// UVD clocks
pub const CG_DCLK_CNTL: c_uint = 0xC050009C;

pub const CG_DCLK_STATUS: c_uint = 0xC05000A0;

pub const CG_VCLK_CNTL: c_uint = 0xC05000A4;
pub const CG_VCLK_STATUS: c_uint = 0xC05000A8;
// UVD CTX indirect
pub const UVD_CGC_MEM_CTRL: c_uint = 0xC0;
// VCE
pub const VCE_VCPU_CACHE_OFFSET0: c_uint = 0x20024;
pub const VCE_VCPU_CACHE_SIZE0: c_uint = 0x20028;
pub const VCE_VCPU_CACHE_OFFSET1: c_uint = 0x2002c;
pub const VCE_VCPU_CACHE_SIZE1: c_uint = 0x20030;
pub const VCE_VCPU_CACHE_OFFSET2: c_uint = 0x20034;
pub const VCE_VCPU_CACHE_SIZE2: c_uint = 0x20038;
pub const VCE_RB_RPTR2: c_uint = 0x20178;
pub const VCE_RB_WPTR2: c_uint = 0x2017c;
pub const VCE_RB_RPTR: c_uint = 0x2018c;
pub const VCE_RB_WPTR: c_uint = 0x20190;
pub const VCE_CLOCK_GATING_A: c_uint = 0x202f8;

pub const VCE_CLOCK_GATING_B: c_uint = 0x202fc;
pub const VCE_CGTT_CLK_OVERRIDE: c_uint = 0x207a0;
pub const VCE_UENC_CLOCK_GATING: c_uint = 0x207bc;

pub const VCE_UENC_REG_CLOCK_GATING: c_uint = 0x207c0;
pub const VCE_SYS_INT_EN: c_uint = 0x21300;

pub const VCE_LMI_VCPU_CACHE_40BIT_BAR: c_uint = 0x2145c;
pub const VCE_LMI_CTRL2: c_uint = 0x21474;
pub const VCE_LMI_CTRL: c_uint = 0x21498;
pub const VCE_LMI_VM_CTRL: c_uint = 0x214a0;
pub const VCE_LMI_SWAP_CNTL: c_uint = 0x214b4;
pub const VCE_LMI_SWAP_CNTL1: c_uint = 0x214b8;
pub const VCE_LMI_CACHE_CTRL: c_uint = 0x214f4;
pub const VCE_CMD_NO_OP: c_uint = 0x00000000;
pub const VCE_CMD_END: c_uint = 0x00000001;
pub const VCE_CMD_IB: c_uint = 0x00000002;
pub const VCE_CMD_FENCE: c_uint = 0x00000003;
pub const VCE_CMD_TRAP: c_uint = 0x00000004;
pub const VCE_CMD_IB_AUTO: c_uint = 0x00000005;
pub const VCE_CMD_SEMAPHORE: c_uint = 0x00000006;
pub const ATC_VMID_PASID_MAPPING_UPDATE_STATUS: c_uint = 0x3398u;
pub const ATC_VMID0_PASID_MAPPING: c_uint = 0x339Cu;

pub const ATC_VMID_PASID_MAPPING_PASID_SHIFT: c_int = 0;

pub const ATC_VMID_PASID_MAPPING_VALID_SHIFT: c_int = 31;
pub const ATC_VM_APERTURE0_CNTL: c_uint = 0x3310u;
pub const ATS_ACCESS_MODE_NEVER: c_int = 0;
pub const ATS_ACCESS_MODE_ALWAYS: c_int = 1;
pub const ATC_VM_APERTURE0_CNTL2: c_uint = 0x3318u;
pub const ATC_VM_APERTURE0_HIGH_ADDR: c_uint = 0x3308u;
pub const ATC_VM_APERTURE0_LOW_ADDR: c_uint = 0x3300u;
pub const ATC_VM_APERTURE1_CNTL: c_uint = 0x3314u;
pub const ATC_VM_APERTURE1_CNTL2: c_uint = 0x331Cu;
pub const ATC_VM_APERTURE1_HIGH_ADDR: c_uint = 0x330Cu;
pub const ATC_VM_APERTURE1_LOW_ADDR: c_uint = 0x3304u;
pub const IH_VMID_0_LUT: c_uint = 0x3D40u;
