//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/sid.h
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
// Authors: Alex Deucher
//
pub const TAHITI_RB_BITMAP_WIDTH_PER_SH: c_int = 2;
pub const TAHITI_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x12011003;
pub const VERDE_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x12010002;
pub const HAINAN_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02010001;
pub const SI_MAX_SH_GPRS: c_int = 256;
pub const SI_MAX_TEMP_GPRS: c_int = 16;
pub const SI_MAX_SH_THREADS: c_int = 256;
pub const SI_MAX_SH_STACK_ENTRIES: c_int = 4096;
pub const SI_MAX_FRC_EOV_CNT: c_int = 16384;
pub const SI_MAX_BACKENDS: c_int = 8;
pub const SI_MAX_BACKENDS_MASK: c_uint = 0xFF;
pub const SI_MAX_BACKENDS_PER_SE_MASK: c_uint = 0x0F;
pub const SI_MAX_SIMDS: c_int = 12;
pub const SI_MAX_SIMDS_MASK: c_uint = 0x0FFF;
pub const SI_MAX_SIMDS_PER_SE_MASK: c_uint = 0x00FF;
pub const SI_MAX_PIPES: c_int = 8;
pub const SI_MAX_PIPES_MASK: c_uint = 0xFF;
pub const SI_MAX_PIPES_PER_SIMD_MASK: c_uint = 0x3F;
pub const SI_MAX_LDS_NUM: c_uint = 0xFFFF;
pub const SI_MAX_TCC: c_int = 16;
pub const SI_MAX_TCC_MASK: c_uint = 0xFFFF;
// SMC IND accessor regs
pub const SMC_IND_INDEX_0: c_uint = 0x200;
pub const SMC_IND_DATA_0: c_uint = 0x204;
pub const SMC_IND_ACCESS_CNTL: c_uint = 0x228;

pub const SMC_MESSAGE_0: c_uint = 0x22c;
pub const SMC_RESP_0: c_uint = 0x230;
// CG IND registers are accessed via SMC indirect space + SMC_CG_IND_START
pub const SMC_CG_IND_START: c_uint = 0xc0030000;
pub const SMC_CG_IND_END: c_uint = 0xc0040000;
pub const CG_CGTT_LOCAL_0: c_uint = 0x400;
pub const CG_CGTT_LOCAL_1: c_uint = 0x401;
// SMC IND registers
pub const SMC_SYSCON_RESET_CNTL: c_uint = 0x80000000;

pub const SMC_SYSCON_CLOCK_CNTL_0: c_uint = 0x80000004;

pub const VGA_HDP_CONTROL: c_uint = 0x328;

pub const DCCG_DISP_SLOW_SELECT_REG: c_uint = 0x4fc;

pub const DCCG_DISP1_SLOW_SELECT_SHIFT: c_int = 0;

pub const DCCG_DISP2_SLOW_SELECT_SHIFT: c_int = 4;
pub const CG_SPLL_FUNC_CNTL: c_uint = 0x600;

pub const SPLL_PDIV_A_SHIFT: c_int = 20;
pub const CG_SPLL_FUNC_CNTL_2: c_uint = 0x604;

pub const CG_SPLL_FUNC_CNTL_3: c_uint = 0x608;

pub const SPLL_FB_DIV_SHIFT: c_int = 0;

pub const CG_SPLL_FUNC_CNTL_4: c_uint = 0x60c;
pub const SPLL_STATUS: c_uint = 0x614;

pub const SPLL_CNTL_MODE: c_uint = 0x618;

pub const CG_SPLL_SPREAD_SPECTRUM: c_uint = 0x620;

pub const CLK_S_SHIFT: c_int = 4;
pub const CG_SPLL_SPREAD_SPECTRUM_2: c_uint = 0x624;

pub const CLK_V_SHIFT: c_int = 0;
pub const CG_SPLL_AUTOSCALE_CNTL: c_uint = 0x62c;

// discrete uvd clocks
pub const CG_UPLL_FUNC_CNTL: c_uint = 0x634;

pub const CG_UPLL_FUNC_CNTL_2: c_uint = 0x638;

pub const CG_UPLL_FUNC_CNTL_3: c_uint = 0x63C;

pub const CG_UPLL_FUNC_CNTL_4: c_uint = 0x644;

pub const CG_UPLL_FUNC_CNTL_5: c_uint = 0x648;

pub const CG_UPLL_SPREAD_SPECTRUM: c_uint = 0x650;

pub const MPLL_BYPASSCLK_SEL: c_uint = 0x65c;

pub const CG_CLKPIN_CNTL: c_uint = 0x660;

pub const CG_CLKPIN_CNTL_2: c_uint = 0x664;

pub const THM_CLK_CNTL: c_uint = 0x66c;

pub const MISC_CLK_CNTL: c_uint = 0x670;

pub const CG_THERMAL_CTRL: c_uint = 0x700;

pub const DIG_THERM_DPM_MASK: c_uint = 0x003FC000;
pub const DIG_THERM_DPM_SHIFT: c_int = 14;
pub const CG_THERMAL_STATUS: c_uint = 0x704;

pub const FDO_PWM_DUTY_SHIFT: c_int = 9;
pub const CG_THERMAL_INT: c_uint = 0x708;

pub const DIG_THERM_INTH_MASK: c_uint = 0x0000FF00;
pub const DIG_THERM_INTH_SHIFT: c_int = 8;

pub const DIG_THERM_INTL_MASK: c_uint = 0x00FF0000;
pub const DIG_THERM_INTL_SHIFT: c_int = 16;

pub const CG_MULT_THERMAL_CTRL: c_uint = 0x710;

pub const TEMP_SEL_SHIFT: c_int = 20;
pub const CG_MULT_THERMAL_STATUS: c_uint = 0x714;

pub const ASIC_MAX_TEMP_MASK: c_uint = 0x000001ff;
pub const ASIC_MAX_TEMP_SHIFT: c_int = 0;

pub const CTF_TEMP_MASK: c_uint = 0x0003fe00;
pub const CTF_TEMP_SHIFT: c_int = 9;
pub const CG_FDO_CTRL0: c_uint = 0x754;

pub const FDO_STATIC_DUTY_MASK: c_uint = 0x000000FF;
pub const FDO_STATIC_DUTY_SHIFT: c_int = 0;
pub const CG_FDO_CTRL1: c_uint = 0x758;

pub const FMAX_DUTY100_MASK: c_uint = 0x000000FF;
pub const FMAX_DUTY100_SHIFT: c_int = 0;
pub const CG_FDO_CTRL2: c_uint = 0x75C;

pub const TMIN_MASK: c_uint = 0x000000FF;
pub const TMIN_SHIFT: c_int = 0;

pub const FDO_PWM_MODE_SHIFT: c_int = 11;

pub const TACH_PWM_RESP_RATE_SHIFT: c_int = 25;
pub const CG_TACH_CTRL: c_uint = 0x770;

pub const CG_TACH_STATUS: c_uint = 0x774;

pub const GENERAL_PWRMGT: c_uint = 0x780;

pub const CG_TPC: c_uint = 0x784;
pub const SCLK_PWRMGT_CNTL: c_uint = 0x788;

pub const TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0x798;

pub const CG_FTV: c_uint = 0x7bc;
pub const CG_FFCT_0: c_uint = 0x7c0;

pub const CG_BSP: c_uint = 0x7fc;

pub const CG_AT: c_uint = 0x800;

pub const CG_GIT: c_uint = 0x804;

pub const CG_SSP: c_uint = 0x80c;

pub const CG_DISPLAY_GAP_CNTL: c_uint = 0x828;

pub const CG_ULV_CONTROL: c_uint = 0x878;
pub const CG_ULV_PARAMETER: c_uint = 0x87c;
pub const SMC_SCRATCH0: c_uint = 0x884;
pub const CG_CAC_CTRL: c_uint = 0x8b8;

pub const DMIF_ADDR_CONFIG: c_uint = 0xBD4;
pub const DMIF_ADDR_CALC: c_uint = 0xC00;
pub const PIPE0_DMIF_BUFFER_CONTROL: c_uint = 0x0ca0;

pub const SRBM_STATUS: c_uint = 0xE50;

pub const SRBM_SOFT_RESET: c_uint = 0x0E60;

pub const CC_SYS_RB_BACKEND_DISABLE: c_uint = 0xe80;
pub const GC_USER_SYS_RB_BACKEND_DISABLE: c_uint = 0xe84;
pub const SRBM_READ_ERROR: c_uint = 0xE98;
pub const SRBM_INT_CNTL: c_uint = 0xEA0;
pub const SRBM_INT_ACK: c_uint = 0xEA8;
pub const SRBM_STATUS2: c_uint = 0x0EC4;

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
pub const VM_CONTEXT1_PROTECTION_FAULT_ADDR: c_uint = 0x14FC;
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
pub const VM_INVALIDATE_REQUEST: c_uint = 0x1478;
pub const VM_INVALIDATE_RESPONSE: c_uint = 0x147c;
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
pub const MC_VM_FB_LOCATION: c_uint = 0x2024;
pub const MC_VM_AGP_TOP: c_uint = 0x2028;
pub const MC_VM_AGP_BOT: c_uint = 0x202C;
pub const MC_VM_AGP_BASE: c_uint = 0x2030;
pub const MC_VM_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x2034;
pub const MC_VM_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x2038;
pub const MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR: c_uint = 0x203C;
pub const MC_VM_MX_L1_TLB_CNTL: c_uint = 0x2064;

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
pub const MC_SEQ_TRAIN_WAKEUP_CNTL: c_uint = 0x28e8;

pub const MC_SEQ_SUP_CNTL: c_uint = 0x28c8;

pub const MC_SEQ_SUP_PGM: c_uint = 0x28cc;
pub const MC_PMG_AUTO_CMD: c_uint = 0x28d0;
pub const MC_IO_PAD_CNTL_D0: c_uint = 0x29d0;

pub const MC_SEQ_RAS_TIMING: c_uint = 0x28a0;
pub const MC_SEQ_CAS_TIMING: c_uint = 0x28a4;
pub const MC_SEQ_MISC_TIMING: c_uint = 0x28a8;
pub const MC_SEQ_MISC_TIMING2: c_uint = 0x28ac;
pub const MC_SEQ_PMG_TIMING: c_uint = 0x28b0;
pub const MC_SEQ_RD_CTL_D0: c_uint = 0x28b4;
pub const MC_SEQ_RD_CTL_D1: c_uint = 0x28b8;
pub const MC_SEQ_WR_CTL_D0: c_uint = 0x28bc;
pub const MC_SEQ_WR_CTL_D1: c_uint = 0x28c0;
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

pub const MPLL_CNTL_MODE: c_uint = 0x2bb0;

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
pub const IH_RB_CNTL: c_uint = 0x3e00;

pub const IH_RB_BASE: c_uint = 0x3e04;
pub const IH_RB_RPTR: c_uint = 0x3e08;
pub const IH_RB_WPTR: c_uint = 0x3e0c;

pub const IH_RB_WPTR_ADDR_HI: c_uint = 0x3e10;
pub const IH_RB_WPTR_ADDR_LO: c_uint = 0x3e14;
pub const IH_CNTL: c_uint = 0x3e18;

pub const CONFIG_MEMSIZE: c_uint = 0x5428;
pub const INTERRUPT_CNTL: c_uint = 0x5468;

pub const INTERRUPT_CNTL2: c_uint = 0x546c;
pub const HDP_MEM_COHERENCY_FLUSH_CNTL: c_uint = 0x5480;
pub const BIF_FB_EN: c_uint = 0x5490;

pub const HDP_REG_COHERENCY_FLUSH_CNTL: c_uint = 0x54A0;
// DCE6 ELD audio interface
pub const AZ_F0_CODEC_ENDPOINT_INDEX: c_uint = 0x5E00;

pub const AZ_F0_CODEC_ENDPOINT_DATA: c_uint = 0x5E04;
pub const AZ_F0_CODEC_PIN_CONTROL_CHANNEL_SPEAKER: c_uint = 0x25;

pub const SPEAKER_ALLOCATION_SHIFT: c_int = 0;

pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR0: c_uint = 0x28 /* LPCM */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR1: c_uint = 0x29 /* AC3 */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR2: c_uint = 0x2A /* MPEG1 */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR3: c_uint = 0x2B /* MP3 */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR4: c_uint = 0x2C /* MPEG2 */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR5: c_uint = 0x2D /* AAC */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR6: c_uint = 0x2E /* DTS */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR7: c_uint = 0x2F /* ATRAC */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR8: c_uint = 0x30 /* one bit audio - leave at 0 (default) */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR9: c_uint = 0x31 /* Dolby Digital */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR10: c_uint = 0x32 /* DTS-HD */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR11: c_uint = 0x33 /* MAT-MLP */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR12: c_uint = 0x34 /* DTS */;
pub const AZ_F0_CODEC_PIN_CONTROL_AUDIO_DESCRIPTOR13: c_uint = 0x35 /* WMA Pro */;

// max channels minus one.  7 = 8 channels

// SUPPORTED_FREQUENCIES, SUPPORTED_FREQUENCIES_STEREO
// bit0 = 32 kHz
// bit1 = 44.1 kHz
// bit2 = 48 kHz
// bit3 = 88.2 kHz
// bit4 = 96 kHz
// bit5 = 176.4 kHz
// bit6 = 192 kHz
//
pub const AZ_F0_CODEC_PIN_CONTROL_RESPONSE_LIPSYNC: c_uint = 0x37;

// VIDEO_LIPSYNC, AUDIO_LIPSYNC
// 0   = invalid
// x   = legal delay value
// 255 = sync not supported
//
pub const AZ_F0_CODEC_PIN_CONTROL_RESPONSE_HBR: c_uint = 0x38;

pub const AZ_F0_CODEC_PIN_CONTROL_SINK_INFO0: c_uint = 0x3a;

pub const AZ_F0_CODEC_PIN_CONTROL_SINK_INFO1: c_uint = 0x3b;

pub const AZ_F0_CODEC_PIN_CONTROL_SINK_INFO2: c_uint = 0x3c;

pub const AZ_F0_CODEC_PIN_CONTROL_SINK_INFO3: c_uint = 0x3d;

pub const AZ_F0_CODEC_PIN_CONTROL_SINK_INFO4: c_uint = 0x3e;

pub const AZ_F0_CODEC_PIN_CONTROL_SINK_INFO5: c_uint = 0x3f;

pub const AZ_F0_CODEC_PIN_CONTROL_SINK_INFO6: c_uint = 0x40;

pub const AZ_F0_CODEC_PIN_CONTROL_SINK_INFO7: c_uint = 0x41;

pub const AZ_F0_CODEC_PIN_CONTROL_SINK_INFO8: c_uint = 0x42;

pub const AZ_F0_CODEC_PIN_CONTROL_HOT_PLUG_CONTROL: c_uint = 0x54;

pub const AZ_F0_CODEC_PIN_CONTROL_RESPONSE_CONFIGURATION_DEFAULT: c_uint = 0x56;

pub const PORT_CONNECTIVITY_SHIFT: c_int = 30;
pub const DC_LB_MEMORY_SPLIT: c_uint = 0x6b0c;

pub const PRIORITY_A_CNT: c_uint = 0x6b18;
pub const PRIORITY_MARK_MASK: c_uint = 0x7fff;

pub const PRIORITY_B_CNT: c_uint = 0x6b1c;
pub const DPG_PIPE_ARBITRATION_CONTROL3: c_uint = 0x6cc8;

pub const DPG_PIPE_LATENCY_CONTROL: c_uint = 0x6ccc;

// 0x6bb8, 0x77b8, 0x103b8, 0x10fb8, 0x11bb8, 0x127b8
pub const VLINE_STATUS: c_uint = 0x6bb8;

// 0x6bbc, 0x77bc, 0x103bc, 0x10fbc, 0x11bbc, 0x127bc
pub const VBLANK_STATUS: c_uint = 0x6bbc;

// 0x6b40, 0x7740, 0x10340, 0x10f40, 0x11b40, 0x12740
pub const INT_MASK: c_uint = 0x6b40;

pub const DISP_INTERRUPT_STATUS: c_uint = 0x60f4;

pub const DISP_INTERRUPT_STATUS_CONTINUE: c_uint = 0x60f8;

pub const DISP_INTERRUPT_STATUS_CONTINUE2: c_uint = 0x60fc;

pub const DISP_INTERRUPT_STATUS_CONTINUE3: c_uint = 0x6100;

pub const DISP_INTERRUPT_STATUS_CONTINUE4: c_uint = 0x614c;

pub const DISP_INTERRUPT_STATUS_CONTINUE5: c_uint = 0x6150;

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

// 0x6e98, 0x7a98, 0x10698, 0x11298, 0x11e98, 0x12a98
pub const CRTC_STATUS_FRAME_COUNT: c_uint = 0x6e98;
// Audio clocks
pub const DCCG_AUDIO_DTO_SOURCE: c_uint = 0x05ac;

pub const DCCG_AUDIO_DTO0_PHASE: c_uint = 0x05b0;
pub const DCCG_AUDIO_DTO0_MODULE: c_uint = 0x05b4;
pub const DCCG_AUDIO_DTO1_PHASE: c_uint = 0x05c0;
pub const DCCG_AUDIO_DTO1_MODULE: c_uint = 0x05c4;
pub const DENTIST_DISPCLK_CNTL: c_uint = 0x0490;

pub const AFMT_AUDIO_SRC_CONTROL: c_uint = 0x713c;

// AFMT_AUDIO_SRC_SELECT
// 0 = stream0
// 1 = stream1
// 2 = stream2
// 3 = stream3
// 4 = stream4
// 5 = stream5
//
pub const GRBM_CNTL: c_uint = 0x8000;

pub const GRBM_STATUS2: c_uint = 0x8008;

pub const GRBM_STATUS: c_uint = 0x8010;
pub const CMDFIFO_AVAIL_MASK: c_uint = 0x0000000F;

pub const GRBM_STATUS_SE0: c_uint = 0x8014;
pub const GRBM_STATUS_SE1: c_uint = 0x8018;

pub const GRBM_SOFT_RESET: c_uint = 0x8020;

pub const GRBM_GFX_INDEX: c_uint = 0x802C;

pub const GRBM_INT_CNTL: c_uint = 0x8060;

pub const CP_STRMOUT_CNTL: c_uint = 0x84FC;
pub const SCRATCH_REG0: c_uint = 0x8500;
pub const SCRATCH_REG1: c_uint = 0x8504;
pub const SCRATCH_REG2: c_uint = 0x8508;
pub const SCRATCH_REG3: c_uint = 0x850C;
pub const SCRATCH_REG4: c_uint = 0x8510;
pub const SCRATCH_REG5: c_uint = 0x8514;
pub const SCRATCH_REG6: c_uint = 0x8518;
pub const SCRATCH_REG7: c_uint = 0x851C;
pub const SCRATCH_UMSK: c_uint = 0x8540;
pub const SCRATCH_ADDR: c_uint = 0x8544;
pub const CP_SEM_WAIT_TIMER: c_uint = 0x85BC;
pub const CP_SEM_INCOMPLETE_TIMER_CNTL: c_uint = 0x85C8;
pub const CP_ME_CNTL: c_uint = 0x86D8;

pub const CP_COHER_CNTL2: c_uint = 0x85E8;
pub const CP_RB2_RPTR: c_uint = 0x86f8;
pub const CP_RB1_RPTR: c_uint = 0x86fc;
pub const CP_RB0_RPTR: c_uint = 0x8700;
pub const CP_RB_WPTR_DELAY: c_uint = 0x8704;
pub const CP_QUEUE_THRESHOLDS: c_uint = 0x8760;

pub const CP_MEQ_THRESHOLDS: c_uint = 0x8764;

pub const CP_PERFMON_CNTL: c_uint = 0x87FC;
pub const VGT_VTX_VECT_EJECT_REG: c_uint = 0x88B0;
pub const VGT_CACHE_INVALIDATION: c_uint = 0x88C4;

pub const VC_ONLY: c_int = 0;
pub const TC_ONLY: c_int = 1;
pub const VC_AND_TC: c_int = 2;

pub const NO_AUTO: c_int = 0;
pub const ES_AUTO: c_int = 1;
pub const GS_AUTO: c_int = 2;
pub const ES_AND_GS_AUTO: c_int = 3;
pub const VGT_ESGS_RING_SIZE: c_uint = 0x88C8;
pub const VGT_GSVS_RING_SIZE: c_uint = 0x88CC;
pub const VGT_GS_VERTEX_REUSE: c_uint = 0x88D4;
pub const VGT_PRIMITIVE_TYPE: c_uint = 0x8958;
pub const VGT_INDEX_TYPE: c_uint = 0x895C;
pub const VGT_NUM_INDICES: c_uint = 0x8970;
pub const VGT_NUM_INSTANCES: c_uint = 0x8974;
pub const VGT_TF_RING_SIZE: c_uint = 0x8988;
pub const VGT_HS_OFFCHIP_PARAM: c_uint = 0x89B0;
pub const VGT_TF_MEMORY_BASE: c_uint = 0x89B8;
pub const CC_GC_SHADER_ARRAY_CONFIG: c_uint = 0x89bc;
pub const INACTIVE_CUS_MASK: c_uint = 0xFFFF0000;
pub const INACTIVE_CUS_SHIFT: c_int = 16;
pub const GC_USER_SHADER_ARRAY_CONFIG: c_uint = 0x89c0;
pub const PA_CL_ENHANCE: c_uint = 0x8A14;

pub const PA_SU_LINE_STIPPLE_VALUE: c_uint = 0x8A60;
pub const PA_SC_LINE_STIPPLE_STATE: c_uint = 0x8B10;
pub const PA_SC_FORCE_EOV_MAX_CNTS: c_uint = 0x8B24;

pub const PA_SC_FIFO_SIZE: c_uint = 0x8BCC;

pub const PA_SC_ENHANCE: c_uint = 0x8BF0;
pub const SQ_CONFIG: c_uint = 0x8C00;
pub const SQC_CACHES: c_uint = 0x8C08;
pub const SQ_POWER_THROTTLE: c_uint = 0x8e58;

pub const MIN_POWER_SHIFT: c_int = 0;

pub const MAX_POWER_SHIFT: c_int = 16;
pub const SQ_POWER_THROTTLE2: c_uint = 0x8e5c;

pub const MAX_POWER_DELTA_SHIFT: c_int = 0;

pub const STI_SIZE_SHIFT: c_int = 16;

pub const LTI_RATIO_SHIFT: c_int = 27;
pub const SX_DEBUG_1: c_uint = 0x9060;
pub const SPI_STATIC_THREAD_MGMT_1: c_uint = 0x90E0;
pub const SPI_STATIC_THREAD_MGMT_2: c_uint = 0x90E4;
pub const SPI_STATIC_THREAD_MGMT_3: c_uint = 0x90E8;
pub const SPI_PS_MAX_WAVE_ID: c_uint = 0x90EC;
pub const SPI_CONFIG_CNTL: c_uint = 0x9100;
pub const SPI_CONFIG_CNTL_1: c_uint = 0x913C;

pub const CGTS_TCC_DISABLE: c_uint = 0x9148;
pub const CGTS_USER_TCC_DISABLE: c_uint = 0x914C;
pub const TCC_DISABLE_MASK: c_uint = 0xFFFF0000;
pub const TCC_DISABLE_SHIFT: c_int = 16;
pub const CGTS_SM_CTRL_REG: c_uint = 0x9150;

pub const SPI_LB_CU_MASK: c_uint = 0x9354;
pub const TA_CNTL_AUX: c_uint = 0x9508;
pub const TA_CS_BC_BASE_ADDR: c_uint = 0x950C;
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

pub const NUM_GPUS_MASK: c_uint = 0x00700000;
pub const NUM_GPUS_SHIFT: c_int = 20;

pub const MULTI_GPU_TILE_SIZE_MASK: c_uint = 0x03000000;
pub const MULTI_GPU_TILE_SIZE_SHIFT: c_int = 24;

pub const ROW_SIZE_MASK: c_uint = 0x30000000;
pub const ROW_SIZE_SHIFT: c_int = 28;
pub const GB_TILE_MODE0: c_uint = 0x9910;

pub const CB_PERFCOUNTER0_SELECT0: c_uint = 0x9a20;
pub const CB_PERFCOUNTER0_SELECT1: c_uint = 0x9a24;
pub const CB_PERFCOUNTER1_SELECT0: c_uint = 0x9a28;
pub const CB_PERFCOUNTER1_SELECT1: c_uint = 0x9a2c;
pub const CB_PERFCOUNTER2_SELECT0: c_uint = 0x9a30;
pub const CB_PERFCOUNTER2_SELECT1: c_uint = 0x9a34;
pub const CB_PERFCOUNTER3_SELECT0: c_uint = 0x9a38;
pub const CB_PERFCOUNTER3_SELECT1: c_uint = 0x9a3c;
pub const CB_CGTT_SCLK_CTRL: c_uint = 0x9a60;
pub const GC_USER_RB_BACKEND_DISABLE: c_uint = 0x9B7C;
pub const BACKEND_DISABLE_MASK: c_uint = 0x00FF0000;
pub const BACKEND_DISABLE_SHIFT: c_int = 16;
pub const TCP_CHAN_STEER_LO: c_uint = 0xac0c;
pub const TCP_CHAN_STEER_HI: c_uint = 0xac10;
pub const CP_RB0_BASE: c_uint = 0xC100;
pub const CP_RB0_CNTL: c_uint = 0xC104;

pub const CP_RB0_RPTR_ADDR: c_uint = 0xC10C;
pub const CP_RB0_RPTR_ADDR_HI: c_uint = 0xC110;
pub const CP_RB0_WPTR: c_uint = 0xC114;
pub const CP_PFP_UCODE_ADDR: c_uint = 0xC150;
pub const CP_PFP_UCODE_DATA: c_uint = 0xC154;
pub const CP_ME_RAM_RADDR: c_uint = 0xC158;
pub const CP_ME_RAM_WADDR: c_uint = 0xC15C;
pub const CP_ME_RAM_DATA: c_uint = 0xC160;
pub const CP_CE_UCODE_ADDR: c_uint = 0xC168;
pub const CP_CE_UCODE_DATA: c_uint = 0xC16C;
pub const CP_RB1_BASE: c_uint = 0xC180;
pub const CP_RB1_CNTL: c_uint = 0xC184;
pub const CP_RB1_RPTR_ADDR: c_uint = 0xC188;
pub const CP_RB1_RPTR_ADDR_HI: c_uint = 0xC18C;
pub const CP_RB1_WPTR: c_uint = 0xC190;
pub const CP_RB2_BASE: c_uint = 0xC194;
pub const CP_RB2_CNTL: c_uint = 0xC198;
pub const CP_RB2_RPTR_ADDR: c_uint = 0xC19C;
pub const CP_RB2_RPTR_ADDR_HI: c_uint = 0xC1A0;
pub const CP_RB2_WPTR: c_uint = 0xC1A4;
pub const CP_INT_CNTL_RING0: c_uint = 0xC1A8;
pub const CP_INT_CNTL_RING1: c_uint = 0xC1AC;
pub const CP_INT_CNTL_RING2: c_uint = 0xC1B0;

pub const CP_INT_STATUS_RING0: c_uint = 0xC1B4;
pub const CP_INT_STATUS_RING1: c_uint = 0xC1B8;
pub const CP_INT_STATUS_RING2: c_uint = 0xC1BC;

pub const CP_MEM_SLP_CNTL: c_uint = 0xC1E4;

pub const CP_DEBUG: c_uint = 0xC1FC;
pub const RLC_CNTL: c_uint = 0xC300;

pub const RLC_RL_BASE: c_uint = 0xC304;
pub const RLC_RL_SIZE: c_uint = 0xC308;
pub const RLC_LB_CNTL: c_uint = 0xC30C;

pub const RLC_SAVE_AND_RESTORE_BASE: c_uint = 0xC310;
pub const RLC_LB_CNTR_MAX: c_uint = 0xC314;
pub const RLC_LB_CNTR_INIT: c_uint = 0xC318;
pub const RLC_CLEAR_STATE_RESTORE_BASE: c_uint = 0xC320;
pub const RLC_UCODE_ADDR: c_uint = 0xC32C;
pub const RLC_UCODE_DATA: c_uint = 0xC330;
pub const RLC_GPU_CLOCK_COUNT_LSB: c_uint = 0xC338;
pub const RLC_GPU_CLOCK_COUNT_MSB: c_uint = 0xC33C;
pub const RLC_CAPTURE_GPU_CLOCK_COUNT: c_uint = 0xC340;
pub const RLC_MC_CNTL: c_uint = 0xC344;
pub const RLC_UCODE_CNTL: c_uint = 0xC348;
pub const RLC_STAT: c_uint = 0xC34C;

pub const RLC_PG_CNTL: c_uint = 0xC35C;

pub const RLC_CGTT_MGCG_OVERRIDE: c_uint = 0xC400;
pub const RLC_CGCG_CGLS_CTRL: c_uint = 0xC404;

pub const RLC_TTOP_D: c_uint = 0xC414;

pub const RLC_LB_INIT_CU_MASK: c_uint = 0xC41C;
pub const RLC_PG_AO_CU_MASK: c_uint = 0xC42C;
pub const RLC_MAX_PG_CU: c_uint = 0xC430;

pub const RLC_AUTO_PG_CTRL: c_uint = 0xC434;

pub const RLC_SERDES_WR_MASTER_MASK_0: c_uint = 0xC454;
pub const RLC_SERDES_WR_MASTER_MASK_1: c_uint = 0xC458;
pub const RLC_SERDES_WR_CTRL: c_uint = 0xC45C;
pub const RLC_SERDES_MASTER_BUSY_0: c_uint = 0xC464;
pub const RLC_SERDES_MASTER_BUSY_1: c_uint = 0xC468;
pub const RLC_GCPM_GENERAL_3: c_uint = 0xC478;
pub const DB_RENDER_CONTROL: c_uint = 0x28000;
pub const DB_DEPTH_INFO: c_uint = 0x2803c;
pub const PA_SC_RASTER_CONFIG: c_uint = 0x28350;

pub const VGT_EVENT_INITIATOR: c_uint = 0x28a90;

// PIF PHY0 registers idx/data 0x8/0xc
pub const PB0_PIF_CNTL: c_uint = 0x10;

pub const PB0_PIF_PAIRING: c_uint = 0x11;

pub const PB0_PIF_PWRDOWN_0: c_uint = 0x12;

pub const PB0_PIF_PWRDOWN_1: c_uint = 0x13;

pub const PB0_PIF_PWRDOWN_2: c_uint = 0x17;

pub const PB0_PIF_PWRDOWN_3: c_uint = 0x18;

// PIF PHY1 registers idx/data 0x10/0x14
pub const PB1_PIF_CNTL: c_uint = 0x10;
pub const PB1_PIF_PAIRING: c_uint = 0x11;
pub const PB1_PIF_PWRDOWN_0: c_uint = 0x12;
pub const PB1_PIF_PWRDOWN_1: c_uint = 0x13;
pub const PB1_PIF_PWRDOWN_2: c_uint = 0x17;
pub const PB1_PIF_PWRDOWN_3: c_uint = 0x18;
// PCIE registers idx/data 0x30/0x34
pub const PCIE_CNTL2: c_uint = 0x1c /* PCIE */;

pub const PCIE_LC_STATUS1: c_uint = 0x28 /* PCIE */;

pub const PCIE_P_CNTL: c_uint = 0x40 /* PCIE */;

// PCIE PORT registers idx/data 0x38/0x3c
pub const PCIE_LC_CNTL: c_uint = 0xa0;

pub const PCIE_LC_LINK_WIDTH_CNTL: c_uint = 0xa2 /* PCIE_P */;

pub const PCIE_LC_N_FTS_CNTL: c_uint = 0xa3 /* PCIE_P */;

pub const PCIE_LC_SPEED_CNTL: c_uint = 0xa4 /* PCIE_P */;

pub const PCIE_LC_CNTL2: c_uint = 0xb1;

pub const PCIE_LC_CNTL3: c_uint = 0xb5 /* PCIE_P */;

pub const PCIE_LC_CNTL4: c_uint = 0xb6 /* PCIE_P */;

//
// UVD
//
pub const UVD_UDEC_ADDR_CONFIG: c_uint = 0xEF4C;
pub const UVD_UDEC_DB_ADDR_CONFIG: c_uint = 0xEF50;
pub const UVD_UDEC_DBW_ADDR_CONFIG: c_uint = 0xEF54;
pub const UVD_NO_OP: c_uint = 0xEFFC;
pub const UVD_RBC_RB_RPTR: c_uint = 0xF690;
pub const UVD_RBC_RB_WPTR: c_uint = 0xF694;
pub const UVD_STATUS: c_uint = 0xf6bc;
pub const UVD_CGC_CTRL: c_uint = 0xF4B0;

// UVD CTX indirect
pub const UVD_CGC_MEM_CTRL: c_uint = 0xC0;
pub const UVD_CGC_CTRL2: c_uint = 0xC1;

//
// PM4
//

pub const CP_PACKET2: c_uint = 0x80000000;
pub const PACKET2_PAD_SHIFT: c_int = 0;

// Packet 3 types
pub const PACKET3_NOP: c_uint = 0x10;
pub const PACKET3_SET_BASE: c_uint = 0x11;

pub const GDS_PARTITION_BASE: c_int = 2;
pub const CE_PARTITION_BASE: c_int = 3;
pub const PACKET3_CLEAR_STATE: c_uint = 0x12;
pub const PACKET3_INDEX_BUFFER_SIZE: c_uint = 0x13;
pub const PACKET3_DISPATCH_DIRECT: c_uint = 0x15;
pub const PACKET3_DISPATCH_INDIRECT: c_uint = 0x16;
pub const PACKET3_ALLOC_GDS: c_uint = 0x1B;
pub const PACKET3_WRITE_GDS_RAM: c_uint = 0x1C;
pub const PACKET3_ATOMIC_GDS: c_uint = 0x1D;
pub const PACKET3_ATOMIC: c_uint = 0x1E;
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
pub const PACKET3_DRAW_INDEX_IMMD: c_uint = 0x2E;
pub const PACKET3_NUM_INSTANCES: c_uint = 0x2F;
pub const PACKET3_DRAW_INDEX_MULTI_AUTO: c_uint = 0x30;
pub const PACKET3_INDIRECT_BUFFER_CONST: c_uint = 0x31;
pub const PACKET3_INDIRECT_BUFFER: c_uint = 0x32;
pub const PACKET3_STRMOUT_BUFFER_UPDATE: c_uint = 0x34;
pub const PACKET3_DRAW_INDEX_OFFSET_2: c_uint = 0x35;
pub const PACKET3_DRAW_INDEX_MULTI_ELEMENT: c_uint = 0x36;
pub const PACKET3_WRITE_DATA: c_uint = 0x37;

// 0 - register
// 1 - memory (sync - via GRBM)
// 2 - tc/l2
// 3 - gds
// 4 - reserved
// 5 - memory (async - direct)
//

// 0 - me
// 1 - pfp
// 2 - ce
//
pub const PACKET3_DRAW_INDEX_INDIRECT_MULTI: c_uint = 0x38;
pub const PACKET3_MEM_SEMAPHORE: c_uint = 0x39;
pub const PACKET3_MPEG_INDEX: c_uint = 0x3A;
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

// 0 - me
// 1 - pfp
//
pub const PACKET3_MEM_WRITE: c_uint = 0x3D;
pub const PACKET3_COPY_DATA: c_uint = 0x40;
pub const PACKET3_CP_DMA: c_uint = 0x41;
// 1. header
// 2. SRC_ADDR_LO or DATA [31:0]
// 3. CP_SYNC [31] | SRC_SEL [30:29] | ENGINE [27] | DST_SEL [21:20] |
// SRC_ADDR_HI [7:0]
// 4. DST_ADDR_LO [31:0]
// 5. DST_ADDR_HI [7:0]
// 6. COMMAND [30:21] | BYTE_COUNT [20:0]
//

// 0 - DST_ADDR
// 1 - GDS
//

// 0 - ME
// 1 - PFP
//

// 0 - SRC_ADDR
// 1 - GDS
// 2 - DATA
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

pub const PACKET3_PFP_SYNC_ME: c_uint = 0x42;
pub const PACKET3_SURFACE_SYNC: c_uint = 0x43;

pub const PACKET3_ME_INITIALIZE: c_uint = 0x44;

pub const PACKET3_COND_WRITE: c_uint = 0x45;
pub const PACKET3_EVENT_WRITE: c_uint = 0x46;

// 0 - any non-TS event
// 1 - ZPASS_DONE
// 2 - SAMPLE_PIPELINESTAT
// 3 - SAMPLE_STREAMOUTSTAT
// 4 - *S_PARTIAL_FLUSH
// 5 - EOP events
// 6 - EOS events
// 7 - CACHE_FLUSH, CACHE_FLUSH_AND_INV_EVENT
//

// INV TC L2 cache when EVENT_INDEX = 7
pub const PACKET3_EVENT_WRITE_EOP: c_uint = 0x47;

// 0 - discard
// 1 - send low 32bit data
// 2 - send 64bit data
// 3 - send 64bit counter value
//

// 0 - none
// 1 - interrupt only (DATA_SEL = 0)
// 2 - interrupt when data write is confirmed
//
pub const PACKET3_EVENT_WRITE_EOS: c_uint = 0x48;
pub const PACKET3_PREAMBLE_CNTL: c_uint = 0x4A;

pub const PACKET3_ONE_REG_WRITE: c_uint = 0x57;
pub const PACKET3_LOAD_CONFIG_REG: c_uint = 0x5F;
pub const PACKET3_LOAD_CONTEXT_REG: c_uint = 0x60;
pub const PACKET3_LOAD_SH_REG: c_uint = 0x61;
pub const PACKET3_SET_CONFIG_REG: c_uint = 0x68;
pub const PACKET3_SET_CONFIG_REG_START: c_uint = 0x00008000;
pub const PACKET3_SET_CONFIG_REG_END: c_uint = 0x0000b000;
pub const PACKET3_SET_CONTEXT_REG: c_uint = 0x69;
pub const PACKET3_SET_CONTEXT_REG_START: c_uint = 0x00028000;
pub const PACKET3_SET_CONTEXT_REG_END: c_uint = 0x00029000;
pub const PACKET3_SET_CONTEXT_REG_INDIRECT: c_uint = 0x73;
pub const PACKET3_SET_RESOURCE_INDIRECT: c_uint = 0x74;
pub const PACKET3_SET_SH_REG: c_uint = 0x76;
pub const PACKET3_SET_SH_REG_START: c_uint = 0x0000b000;
pub const PACKET3_SET_SH_REG_END: c_uint = 0x0000c000;
pub const PACKET3_SET_SH_REG_OFFSET: c_uint = 0x77;
pub const PACKET3_ME_WRITE: c_uint = 0x7A;
pub const PACKET3_SCRATCH_RAM_WRITE: c_uint = 0x7D;
pub const PACKET3_SCRATCH_RAM_READ: c_uint = 0x7E;
pub const PACKET3_CE_WRITE: c_uint = 0x7F;
pub const PACKET3_LOAD_CONST_RAM: c_uint = 0x80;
pub const PACKET3_WRITE_CONST_RAM: c_uint = 0x81;
pub const PACKET3_WRITE_CONST_RAM_OFFSET: c_uint = 0x82;
pub const PACKET3_DUMP_CONST_RAM: c_uint = 0x83;
pub const PACKET3_INCREMENT_CE_COUNTER: c_uint = 0x84;
pub const PACKET3_INCREMENT_DE_COUNTER: c_uint = 0x85;
pub const PACKET3_WAIT_ON_CE_COUNTER: c_uint = 0x86;
pub const PACKET3_WAIT_ON_DE_COUNTER: c_uint = 0x87;
pub const PACKET3_WAIT_ON_DE_COUNTER_DIFF: c_uint = 0x88;
pub const PACKET3_SET_CE_DE_COUNTERS: c_uint = 0x89;
pub const PACKET3_WAIT_ON_AVAIL_BUFFER: c_uint = 0x8A;
pub const PACKET3_SWITCH_BUFFER: c_uint = 0x8B;
// ASYNC DMA - first instance at 0xd000, second at 0xd800
pub const DMA0_REGISTER_OFFSET: c_uint = 0x0 /* not a register */;
pub const DMA1_REGISTER_OFFSET: c_uint = 0x800 /* not a register */;
pub const DMA_RB_CNTL: c_uint = 0xd000;

pub const DMA_RB_BASE: c_uint = 0xd004;
pub const DMA_RB_RPTR: c_uint = 0xd008;
pub const DMA_RB_WPTR: c_uint = 0xd00c;
pub const DMA_RB_RPTR_ADDR_HI: c_uint = 0xd01c;
pub const DMA_RB_RPTR_ADDR_LO: c_uint = 0xd020;
pub const DMA_IB_CNTL: c_uint = 0xd024;

pub const DMA_IB_RPTR: c_uint = 0xd028;
pub const DMA_CNTL: c_uint = 0xd02c;

pub const DMA_STATUS_REG: c_uint = 0xd034;

pub const DMA_TILING_CONFIG: c_uint = 0xd0b8;
pub const DMA_POWER_CNTL: c_uint = 0xd0bc;

pub const DMA_CLK_CTRL: c_uint = 0xd0c0;
pub const DMA_PG: c_uint = 0xd0d4;

pub const DMA_PGFSM_CONFIG: c_uint = 0xd0d8;
pub const DMA_PGFSM_WRITE: c_uint = 0xd0dc;

// async DMA Packet types
pub const DMA_PACKET_WRITE: c_uint = 0x2;
pub const DMA_PACKET_COPY: c_uint = 0x3;
pub const DMA_PACKET_INDIRECT_BUFFER: c_uint = 0x4;
pub const DMA_PACKET_SEMAPHORE: c_uint = 0x5;
pub const DMA_PACKET_FENCE: c_uint = 0x6;
pub const DMA_PACKET_TRAP: c_uint = 0x7;
pub const DMA_PACKET_SRBM_WRITE: c_uint = 0x9;
pub const DMA_PACKET_CONSTANT_FILL: c_uint = 0xd;
pub const DMA_PACKET_POLL_REG_MEM: c_uint = 0xe;
pub const DMA_PACKET_NOP: c_uint = 0xf;
pub const VCE_STATUS: c_uint = 0x20004;
pub const VCE_VCPU_CNTL: c_uint = 0x20014;

pub const VCE_VCPU_CACHE_OFFSET0: c_uint = 0x20024;
pub const VCE_VCPU_CACHE_SIZE0: c_uint = 0x20028;
pub const VCE_VCPU_CACHE_OFFSET1: c_uint = 0x2002c;
pub const VCE_VCPU_CACHE_SIZE1: c_uint = 0x20030;
pub const VCE_VCPU_CACHE_OFFSET2: c_uint = 0x20034;
pub const VCE_VCPU_CACHE_SIZE2: c_uint = 0x20038;
pub const VCE_VCPU_SCRATCH7: c_uint = 0x200dc;
pub const VCE_SOFT_RESET: c_uint = 0x20120;

pub const VCE_RB_BASE_LO2: c_uint = 0x2016c;
pub const VCE_RB_BASE_HI2: c_uint = 0x20170;
pub const VCE_RB_SIZE2: c_uint = 0x20174;
pub const VCE_RB_RPTR2: c_uint = 0x20178;
pub const VCE_RB_WPTR2: c_uint = 0x2017c;
pub const VCE_RB_BASE_LO: c_uint = 0x20180;
pub const VCE_RB_BASE_HI: c_uint = 0x20184;
pub const VCE_RB_SIZE: c_uint = 0x20188;
pub const VCE_RB_RPTR: c_uint = 0x2018c;
pub const VCE_RB_WPTR: c_uint = 0x20190;
pub const VCE_CLOCK_GATING_A: c_uint = 0x202f8;

pub const VCE_CLOCK_GATING_B: c_uint = 0x202fc;
pub const VCE_UENC_CLOCK_GATING: c_uint = 0x205bc;
pub const VCE_UENC_REG_CLOCK_GATING: c_uint = 0x205c0;
pub const VCE_FW_REG_STATUS: c_uint = 0x20e10;

pub const VCE_LMI_FW_START_KEYSEL: c_uint = 0x20e18;
pub const VCE_LMI_FW_PERIODIC_CTRL: c_uint = 0x20e20;
pub const VCE_LMI_CTRL2: c_uint = 0x20e74;
pub const VCE_LMI_CTRL: c_uint = 0x20e98;
pub const VCE_LMI_VM_CTRL: c_uint = 0x20ea0;
pub const VCE_LMI_SWAP_CNTL: c_uint = 0x20eb4;
pub const VCE_LMI_SWAP_CNTL1: c_uint = 0x20eb8;
pub const VCE_LMI_CACHE_CTRL: c_uint = 0x20ef4;
pub const VCE_CMD_NO_OP: c_uint = 0x00000000;
pub const VCE_CMD_END: c_uint = 0x00000001;
pub const VCE_CMD_IB: c_uint = 0x00000002;
pub const VCE_CMD_FENCE: c_uint = 0x00000003;
pub const VCE_CMD_TRAP: c_uint = 0x00000004;
pub const VCE_CMD_IB_AUTO: c_uint = 0x00000005;
pub const VCE_CMD_SEMAPHORE: c_uint = 0x00000006;
// discrete vce clocks
pub const CG_VCEPLL_FUNC_CNTL: c_uint = 0xc0030600;

pub const CG_VCEPLL_FUNC_CNTL_2: c_uint = 0xc0030601;

pub const CG_VCEPLL_FUNC_CNTL_3: c_uint = 0xc0030602;

pub const CG_VCEPLL_FUNC_CNTL_4: c_uint = 0xc0030603;
pub const CG_VCEPLL_FUNC_CNTL_5: c_uint = 0xc0030604;
pub const CG_VCEPLL_SPREAD_SPECTRUM: c_uint = 0xc0030606;

