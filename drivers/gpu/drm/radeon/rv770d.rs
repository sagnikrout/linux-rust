//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rv770d.h
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
// Copyright 2009 Advanced Micro Devices, Inc.
// Copyright 2009 Red Hat Inc.
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
// Authors: Dave Airlie
// Alex Deucher
// Jerome Glisse
//
pub const R7XX_MAX_SH_GPRS: c_int = 256;
pub const R7XX_MAX_TEMP_GPRS: c_int = 16;
pub const R7XX_MAX_SH_THREADS: c_int = 256;
pub const R7XX_MAX_SH_STACK_ENTRIES: c_int = 4096;
pub const R7XX_MAX_BACKENDS: c_int = 8;
pub const R7XX_MAX_BACKENDS_MASK: c_uint = 0xff;
pub const R7XX_MAX_SIMDS: c_int = 16;
pub const R7XX_MAX_SIMDS_MASK: c_uint = 0xffff;
pub const R7XX_MAX_PIPES: c_int = 8;
pub const R7XX_MAX_PIPES_MASK: c_uint = 0xff;
// discrete uvd clocks
pub const CG_UPLL_FUNC_CNTL: c_uint = 0x718;

pub const CG_UPLL_FUNC_CNTL_2: c_uint = 0x71c;

pub const CG_UPLL_FUNC_CNTL_3: c_uint = 0x720;

// pm registers
pub const SMC_SRAM_ADDR: c_uint = 0x200;

pub const SMC_SRAM_DATA: c_uint = 0x204;
pub const SMC_IO: c_uint = 0x208;

pub const SMC_MSG: c_uint = 0x20c;

pub const HOST_SMC_MSG_SHIFT: c_int = 0;

pub const HOST_SMC_RESP_SHIFT: c_int = 8;

pub const SMC_HOST_MSG_SHIFT: c_int = 16;

pub const SMC_HOST_RESP_SHIFT: c_int = 24;
pub const SMC_ISR_FFD8_FFDB: c_uint = 0x218;
pub const CG_SPLL_FUNC_CNTL: c_uint = 0x600;

pub const CG_SPLL_FUNC_CNTL_2: c_uint = 0x604;

pub const CG_SPLL_FUNC_CNTL_3: c_uint = 0x608;

pub const CG_SPLL_STATUS: c_uint = 0x60c;

pub const SPLL_CNTL_MODE: c_uint = 0x610;

pub const MPLL_CNTL_MODE: c_uint = 0x61c;

pub const MPLL_AD_FUNC_CNTL: c_uint = 0x624;

pub const MPLL_AD_FUNC_CNTL_2: c_uint = 0x628;

pub const MPLL_DQ_FUNC_CNTL: c_uint = 0x62c;
pub const MPLL_DQ_FUNC_CNTL_2: c_uint = 0x630;
pub const GENERAL_PWRMGT: c_uint = 0x63c;

pub const CG_TPC: c_uint = 0x640;
pub const SCLK_PWRMGT_CNTL: c_uint = 0x644;

pub const MCLK_PWRMGT_CNTL: c_uint = 0x648;

pub const DLL_CNTL: c_uint = 0x64c;

pub const MPLL_TIME: c_uint = 0x654;

pub const CG_CLKPIN_CNTL: c_uint = 0x660;

pub const TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0x66c;

pub const S0_VID_LOWER_SMIO_CNTL: c_uint = 0x678;
pub const S1_VID_LOWER_SMIO_CNTL: c_uint = 0x67c;
pub const S2_VID_LOWER_SMIO_CNTL: c_uint = 0x680;
pub const S3_VID_LOWER_SMIO_CNTL: c_uint = 0x684;
pub const CG_FTV: c_uint = 0x690;
pub const CG_FFCT_0: c_uint = 0x694;

pub const CG_BSP: c_uint = 0x6d0;

pub const CG_AT: c_uint = 0x6d4;

pub const CG_GIT: c_uint = 0x6d8;

pub const CG_SSP: c_uint = 0x6e8;

pub const CG_DISPLAY_GAP_CNTL: c_uint = 0x714;

pub const CG_SPLL_SPREAD_SPECTRUM: c_uint = 0x790;

pub const CG_SPLL_SPREAD_SPECTRUM_2: c_uint = 0x794;

pub const CG_MPLL_SPREAD_SPECTRUM: c_uint = 0x798;
pub const CG_UPLL_SPREAD_SPECTRUM: c_uint = 0x79c;

pub const CG_CGTT_LOCAL_0: c_uint = 0x7d0;
pub const CG_CGTT_LOCAL_1: c_uint = 0x7d4;
pub const BIOS_SCRATCH_4: c_uint = 0x1734;
pub const MC_SEQ_MISC0: c_uint = 0x2a00;
pub const MC_SEQ_MISC0_GDDR5_SHIFT: c_int = 28;
pub const MC_SEQ_MISC0_GDDR5_MASK: c_uint = 0xf0000000;
pub const MC_SEQ_MISC0_GDDR5_VALUE: c_int = 5;
pub const MC_ARB_SQM_RATIO: c_uint = 0x2770;

pub const MC_ARB_RFSH_RATE: c_uint = 0x27b0;

pub const CGTS_SM_CTRL_REG: c_uint = 0x9150;
// Registers
pub const CB_COLOR0_BASE: c_uint = 0x28040;
pub const CB_COLOR1_BASE: c_uint = 0x28044;
pub const CB_COLOR2_BASE: c_uint = 0x28048;
pub const CB_COLOR3_BASE: c_uint = 0x2804C;
pub const CB_COLOR4_BASE: c_uint = 0x28050;
pub const CB_COLOR5_BASE: c_uint = 0x28054;
pub const CB_COLOR6_BASE: c_uint = 0x28058;
pub const CB_COLOR7_BASE: c_uint = 0x2805C;
pub const CB_COLOR7_FRAG: c_uint = 0x280FC;
pub const CC_GC_SHADER_PIPE_CONFIG: c_uint = 0x8950;
pub const CC_RB_BACKEND_DISABLE: c_uint = 0x98F4;

pub const CC_SYS_RB_BACKEND_DISABLE: c_uint = 0x3F88;
pub const CGTS_SYS_TCC_DISABLE: c_uint = 0x3F90;
pub const CGTS_TCC_DISABLE: c_uint = 0x9148;
pub const CGTS_USER_SYS_TCC_DISABLE: c_uint = 0x3F94;
pub const CGTS_USER_TCC_DISABLE: c_uint = 0x914C;
pub const CONFIG_MEMSIZE: c_uint = 0x5428;
pub const CP_ME_CNTL: c_uint = 0x86D8;

pub const CP_ME_RAM_DATA: c_uint = 0xC160;
pub const CP_ME_RAM_RADDR: c_uint = 0xC158;
pub const CP_ME_RAM_WADDR: c_uint = 0xC15C;
pub const CP_MEQ_THRESHOLDS: c_uint = 0x8764;

pub const CP_PERFMON_CNTL: c_uint = 0x87FC;
pub const CP_PFP_UCODE_ADDR: c_uint = 0xC150;
pub const CP_PFP_UCODE_DATA: c_uint = 0xC154;
pub const CP_QUEUE_THRESHOLDS: c_uint = 0x8760;

pub const CP_RB_CNTL: c_uint = 0xC104;

pub const CP_RB_RPTR: c_uint = 0x8700;
pub const CP_RB_RPTR_ADDR: c_uint = 0xC10C;
pub const CP_RB_RPTR_ADDR_HI: c_uint = 0xC110;
pub const CP_RB_RPTR_WR: c_uint = 0xC108;
pub const CP_RB_WPTR: c_uint = 0xC114;
pub const CP_RB_WPTR_ADDR: c_uint = 0xC118;
pub const CP_RB_WPTR_ADDR_HI: c_uint = 0xC11C;
pub const CP_RB_WPTR_DELAY: c_uint = 0x8704;
pub const CP_SEM_WAIT_TIMER: c_uint = 0x85BC;
pub const DB_DEBUG3: c_uint = 0x98B0;

pub const DB_DEBUG4: c_uint = 0x9B8C;

pub const DCP_TILING_CONFIG: c_uint = 0x6CA0;

pub const GB_TILING_CONFIG: c_uint = 0x98F0;
pub const PIPE_TILING__SHIFT: c_int = 1;
pub const PIPE_TILING__MASK: c_uint = 0x0000000e;
pub const DMA_TILING_CONFIG: c_uint = 0x3ec8;
pub const DMA_TILING_CONFIG2: c_uint = 0xd0b8;
// RV730 only
pub const UVD_UDEC_TILING_CONFIG: c_uint = 0xef40;
pub const UVD_UDEC_DB_TILING_CONFIG: c_uint = 0xef44;
pub const UVD_UDEC_DBW_TILING_CONFIG: c_uint = 0xef48;
pub const UVD_NO_OP: c_uint = 0xeffc;
pub const GC_USER_SHADER_PIPE_CONFIG: c_uint = 0x8954;

pub const INACTIVE_QD_PIPES_MASK: c_uint = 0x0000FF00;
pub const INACTIVE_QD_PIPES_SHIFT: c_int = 8;

pub const INACTIVE_SIMDS_MASK: c_uint = 0x00FF0000;
pub const GRBM_CNTL: c_uint = 0x8000;

pub const GRBM_SOFT_RESET: c_uint = 0x8020;

pub const GRBM_STATUS: c_uint = 0x8010;
pub const CMDFIFO_AVAIL_MASK: c_uint = 0x0000000F;

pub const GRBM_STATUS2: c_uint = 0x8014;
pub const CG_THERMAL_CTRL: c_uint = 0x72C;

pub const DIG_THERM_DPM_MASK: c_uint = 0x003FC000;
pub const DIG_THERM_DPM_SHIFT: c_int = 14;
pub const CG_THERMAL_INT: c_uint = 0x734;

pub const DIG_THERM_INTH_MASK: c_uint = 0x0000FF00;
pub const DIG_THERM_INTH_SHIFT: c_int = 8;

pub const DIG_THERM_INTL_MASK: c_uint = 0x00FF0000;
pub const DIG_THERM_INTL_SHIFT: c_int = 16;

pub const CG_MULT_THERMAL_STATUS: c_uint = 0x740;

pub const ASIC_T_MASK: c_uint = 0x3FF0000;
pub const ASIC_T_SHIFT: c_int = 16;
pub const HDP_HOST_PATH_CNTL: c_uint = 0x2C00;
pub const HDP_NONSURFACE_BASE: c_uint = 0x2C04;
pub const HDP_NONSURFACE_INFO: c_uint = 0x2C08;
pub const HDP_NONSURFACE_SIZE: c_uint = 0x2C0C;
pub const HDP_REG_COHERENCY_FLUSH_CNTL: c_uint = 0x54A0;
pub const HDP_TILING_CONFIG: c_uint = 0x2F3C;
pub const HDP_DEBUG1: c_uint = 0x2F34;
pub const MC_SHARED_CHMAP: c_uint = 0x2004;
pub const NOOFCHAN_SHIFT: c_int = 12;
pub const NOOFCHAN_MASK: c_uint = 0x00003000;
pub const MC_SHARED_CHREMAP: c_uint = 0x2008;
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
pub const BURSTLENGTH_SHIFT: c_int = 9;
pub const BURSTLENGTH_MASK: c_uint = 0x00000200;

pub const MC_VM_AGP_TOP: c_uint = 0x2028;
pub const MC_VM_AGP_BOT: c_uint = 0x202C;
pub const MC_VM_AGP_BASE: c_uint = 0x2030;
pub const MC_VM_FB_LOCATION: c_uint = 0x2024;
pub const MC_VM_MB_L1_TLB0_CNTL: c_uint = 0x2234;
pub const MC_VM_MB_L1_TLB1_CNTL: c_uint = 0x2238;
pub const MC_VM_MB_L1_TLB2_CNTL: c_uint = 0x223C;
pub const MC_VM_MB_L1_TLB3_CNTL: c_uint = 0x2240;

pub const MC_VM_MD_L1_TLB0_CNTL: c_uint = 0x2654;
pub const MC_VM_MD_L1_TLB1_CNTL: c_uint = 0x2658;
pub const MC_VM_MD_L1_TLB2_CNTL: c_uint = 0x265C;
pub const MC_VM_MD_L1_TLB3_CNTL: c_uint = 0x2698;
pub const MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR: c_uint = 0x203C;
pub const MC_VM_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x2038;
pub const MC_VM_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x2034;
pub const PA_CL_ENHANCE: c_uint = 0x8A14;

pub const PA_SC_AA_CONFIG: c_uint = 0x28C04;
pub const PA_SC_CLIPRECT_RULE: c_uint = 0x2820C;
pub const PA_SC_EDGERULE: c_uint = 0x28230;
pub const PA_SC_FIFO_SIZE: c_uint = 0x8BCC;

pub const PA_SC_FORCE_EOV_MAX_CNTS: c_uint = 0x8B24;

pub const PA_SC_LINE_STIPPLE: c_uint = 0x28A0C;
pub const PA_SC_LINE_STIPPLE_STATE: c_uint = 0x8B10;
pub const PA_SC_MODE_CNTL: c_uint = 0x28A4C;
pub const PA_SC_MULTI_CHIP_CNTL: c_uint = 0x8B20;

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
pub const SMX_SAR_CTL0: c_uint = 0xA008;
pub const SMX_DC_CTL0: c_uint = 0xA020;

pub const SMX_EVENT_CTL: c_uint = 0xA02C;

pub const SPI_CONFIG_CNTL: c_uint = 0x9100;

pub const SPI_CONFIG_CNTL_1: c_uint = 0x913C;

pub const SPI_INPUT_Z: c_uint = 0x286D8;
pub const SPI_PS_IN_CONTROL_0: c_uint = 0x286CC;

pub const SQ_CONFIG: c_uint = 0x8C00;

pub const SQ_DYN_GPR_SIZE_SIMD_AB_0: c_uint = 0x8DB0;

pub const SQ_DYN_GPR_SIZE_SIMD_AB_1: c_uint = 0x8DB4;
pub const SQ_DYN_GPR_SIZE_SIMD_AB_2: c_uint = 0x8DB8;
pub const SQ_DYN_GPR_SIZE_SIMD_AB_3: c_uint = 0x8DBC;
pub const SQ_DYN_GPR_SIZE_SIMD_AB_4: c_uint = 0x8DC0;
pub const SQ_DYN_GPR_SIZE_SIMD_AB_5: c_uint = 0x8DC4;
pub const SQ_DYN_GPR_SIZE_SIMD_AB_6: c_uint = 0x8DC8;
pub const SQ_DYN_GPR_SIZE_SIMD_AB_7: c_uint = 0x8DCC;

pub const SQ_GPR_RESOURCE_MGMT_1: c_uint = 0x8C04;

pub const SQ_GPR_RESOURCE_MGMT_2: c_uint = 0x8C08;

pub const SQ_MS_FIFO_SIZES: c_uint = 0x8CF0;

pub const SQ_STACK_RESOURCE_MGMT_1: c_uint = 0x8C10;

pub const SQ_STACK_RESOURCE_MGMT_2: c_uint = 0x8C14;

pub const SQ_THREAD_RESOURCE_MGMT: c_uint = 0x8C0C;

pub const SX_DEBUG_1: c_uint = 0x9058;

pub const SX_EXPORT_BUFFER_SIZES: c_uint = 0x900C;

pub const SX_MISC: c_uint = 0x28350;
pub const TA_CNTL_AUX: c_uint = 0x9508;

pub const TCP_CNTL: c_uint = 0x9610;
pub const TCP_CHAN_STEER: c_uint = 0x9614;
pub const VC_ENHANCE: c_uint = 0x9714;
pub const VGT_CACHE_INVALIDATION: c_uint = 0x88C4;

pub const VC_ONLY: c_int = 0;
pub const TC_ONLY: c_int = 1;
pub const VC_AND_TC: c_int = 2;

pub const NO_AUTO: c_int = 0;
pub const ES_AUTO: c_int = 1;
pub const GS_AUTO: c_int = 2;
pub const ES_AND_GS_AUTO: c_int = 3;
pub const VGT_ES_PER_GS: c_uint = 0x88CC;
pub const VGT_GS_PER_ES: c_uint = 0x88C8;
pub const VGT_GS_PER_VS: c_uint = 0x88E8;
pub const VGT_GS_VERTEX_REUSE: c_uint = 0x88D4;
pub const VGT_NUM_INSTANCES: c_uint = 0x8974;
pub const VGT_OUT_DEALLOC_CNTL: c_uint = 0x28C5C;
pub const DEALLOC_DIST_MASK: c_uint = 0x0000007F;
pub const VGT_STRMOUT_EN: c_uint = 0x28AB0;
pub const VGT_VERTEX_REUSE_BLOCK_CNTL: c_uint = 0x28C58;
pub const VTX_REUSE_DEPTH_MASK: c_uint = 0x000000FF;
pub const VM_CONTEXT0_CNTL: c_uint = 0x1410;

pub const VM_CONTEXT0_PAGE_TABLE_BASE_ADDR: c_uint = 0x153C;
pub const VM_CONTEXT0_PAGE_TABLE_END_ADDR: c_uint = 0x157C;
pub const VM_CONTEXT0_PAGE_TABLE_START_ADDR: c_uint = 0x155C;
pub const VM_CONTEXT0_PROTECTION_FAULT_DEFAULT_ADDR: c_uint = 0x1518;
pub const VM_L2_CNTL: c_uint = 0x1400;

pub const VM_L2_CNTL2: c_uint = 0x1404;

pub const VM_L2_CNTL3: c_uint = 0x1408;

pub const VM_L2_STATUS: c_uint = 0x140C;

pub const WAIT_UNTIL: c_uint = 0x8040;
// async DMA
pub const DMA_RB_RPTR: c_uint = 0xd008;
pub const DMA_RB_WPTR: c_uint = 0xd00c;
// async DMA packets

// async DMA Packet types
pub const DMA_PACKET_WRITE: c_uint = 0x2;
pub const DMA_PACKET_COPY: c_uint = 0x3;
pub const DMA_PACKET_INDIRECT_BUFFER: c_uint = 0x4;
pub const DMA_PACKET_SEMAPHORE: c_uint = 0x5;
pub const DMA_PACKET_FENCE: c_uint = 0x6;
pub const DMA_PACKET_TRAP: c_uint = 0x7;
pub const DMA_PACKET_CONSTANT_FILL: c_uint = 0xd;
pub const DMA_PACKET_NOP: c_uint = 0xf;
pub const SRBM_STATUS: c_uint = 0x0E50;
// DCE 3.2 HDMI
pub const HDMI_CONTROL: c_uint = 0x7400;

pub const HDMI_STATUS: c_uint = 0x7404;

pub const HDMI_AUDIO_PACKET_CONTROL: c_uint = 0x7408;

pub const HDMI_ACR_PACKET_CONTROL: c_uint = 0x740c;

pub const HDMI_VBI_PACKET_CONTROL: c_uint = 0x7410;

pub const HDMI_INFOFRAME_CONTROL0: c_uint = 0x7414;

pub const HDMI_INFOFRAME_CONTROL1: c_uint = 0x7418;

pub const HDMI_GENERIC_PACKET_CONTROL: c_uint = 0x741c;

pub const HDMI_GC: c_uint = 0x7428;

pub const AFMT_AUDIO_PACKET_CONTROL2: c_uint = 0x742c;

pub const AFMT_AVI_INFO0: c_uint = 0x7454;

pub const AFMT_AVI_INFO1: c_uint = 0x7458;

pub const AFMT_AVI_INFO2: c_uint = 0x745c;

pub const AFMT_AVI_INFO3: c_uint = 0x7460;

pub const AFMT_MPEG_INFO0: c_uint = 0x7464;

pub const AFMT_MPEG_INFO1: c_uint = 0x7468;

pub const AFMT_GENERIC0_HDR: c_uint = 0x746c;
pub const AFMT_GENERIC0_0: c_uint = 0x7470;
pub const AFMT_GENERIC0_1: c_uint = 0x7474;
pub const AFMT_GENERIC0_2: c_uint = 0x7478;
pub const AFMT_GENERIC0_3: c_uint = 0x747c;
pub const AFMT_GENERIC0_4: c_uint = 0x7480;
pub const AFMT_GENERIC0_5: c_uint = 0x7484;
pub const AFMT_GENERIC0_6: c_uint = 0x7488;
pub const AFMT_GENERIC1_HDR: c_uint = 0x748c;
pub const AFMT_GENERIC1_0: c_uint = 0x7490;
pub const AFMT_GENERIC1_1: c_uint = 0x7494;
pub const AFMT_GENERIC1_2: c_uint = 0x7498;
pub const AFMT_GENERIC1_3: c_uint = 0x749c;
pub const AFMT_GENERIC1_4: c_uint = 0x74a0;
pub const AFMT_GENERIC1_5: c_uint = 0x74a4;
pub const AFMT_GENERIC1_6: c_uint = 0x74a8;
pub const HDMI_ACR_32_0: c_uint = 0x74ac;

pub const HDMI_ACR_32_1: c_uint = 0x74b0;

pub const HDMI_ACR_44_0: c_uint = 0x74b4;

pub const HDMI_ACR_44_1: c_uint = 0x74b8;

pub const HDMI_ACR_48_0: c_uint = 0x74bc;

pub const HDMI_ACR_48_1: c_uint = 0x74c0;

pub const HDMI_ACR_STATUS_0: c_uint = 0x74c4;
pub const HDMI_ACR_STATUS_1: c_uint = 0x74c8;
pub const AFMT_AUDIO_INFO0: c_uint = 0x74cc;

pub const AFMT_AUDIO_INFO1: c_uint = 0x74d0;

pub const AFMT_60958_0: c_uint = 0x74d4;

pub const AFMT_60958_1: c_uint = 0x74d8;

pub const AFMT_AUDIO_CRC_CONTROL: c_uint = 0x74dc;

pub const AFMT_RAMP_CONTROL0: c_uint = 0x74e0;

pub const AFMT_RAMP_CONTROL1: c_uint = 0x74e4;

pub const AFMT_RAMP_CONTROL2: c_uint = 0x74e8;

pub const AFMT_RAMP_CONTROL3: c_uint = 0x74ec;

pub const AFMT_60958_2: c_uint = 0x74f0;

pub const AFMT_STATUS: c_uint = 0x7600;

pub const AFMT_AUDIO_PACKET_CONTROL: c_uint = 0x7604;

pub const AFMT_VBI_PACKET_CONTROL: c_uint = 0x7608;

pub const AFMT_INFOFRAME_CONTROL0: c_uint = 0x760c;

pub const AFMT_GENERIC0_7: c_uint = 0x7610;
// second instance starts at 0x7800

// DCE3.2 ELD audio interface
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR0: c_uint = 0x71c8 /* LPCM */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR1: c_uint = 0x71cc /* AC3 */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR2: c_uint = 0x71d0 /* MPEG1 */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR3: c_uint = 0x71d4 /* MP3 */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR4: c_uint = 0x71d8 /* MPEG2 */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR5: c_uint = 0x71dc /* AAC */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR6: c_uint = 0x71e0 /* DTS */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR7: c_uint = 0x71e4 /* ATRAC */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR8: c_uint = 0x71e8 /* one bit audio - leave at 0 (default) */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR9: c_uint = 0x71ec /* Dolby Digital */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR10: c_uint = 0x71f0 /* DTS-HD */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR11: c_uint = 0x71f4 /* MAT-MLP */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR12: c_uint = 0x71f8 /* DTS */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR13: c_uint = 0x71fc /* WMA Pro */;

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
pub const AZ_HOT_PLUG_CONTROL: c_uint = 0x7300;

pub const D1GRPH_PRIMARY_SURFACE_ADDRESS: c_uint = 0x6110;
pub const D1GRPH_PRIMARY_SURFACE_ADDRESS_HIGH: c_uint = 0x6914;
pub const D2GRPH_PRIMARY_SURFACE_ADDRESS_HIGH: c_uint = 0x6114;
pub const D1GRPH_SECONDARY_SURFACE_ADDRESS: c_uint = 0x6118;
pub const D1GRPH_SECONDARY_SURFACE_ADDRESS_HIGH: c_uint = 0x691c;
pub const D2GRPH_SECONDARY_SURFACE_ADDRESS_HIGH: c_uint = 0x611c;
// PCIE indirect regs
pub const PCIE_P_CNTL: c_uint = 0x40;

// PCIE PORT regs
pub const PCIE_LC_CNTL: c_uint = 0xa0;

pub const PCIE_LC_TRAINING_CNTL: c_uint = 0xa1 /* PCIE_P */;
pub const PCIE_LC_LINK_WIDTH_CNTL: c_uint = 0xa2 /* PCIE_P */;

pub const PCIE_LC_SPEED_CNTL: c_uint = 0xa4 /* PCIE_P */;

pub const MM_CFGREGS_CNTL: c_uint = 0x544c;

pub const LINK_CNTL2: c_uint = 0x88 /* F0 */;

//
// PM4
//

// UVD
pub const UVD_SEMA_ADDR_LOW: c_uint = 0xef00;
pub const UVD_SEMA_ADDR_HIGH: c_uint = 0xef04;
pub const UVD_SEMA_CMD: c_uint = 0xef08;
pub const UVD_GPCOM_VCPU_CMD: c_uint = 0xef0c;
pub const UVD_GPCOM_VCPU_DATA0: c_uint = 0xef10;
pub const UVD_GPCOM_VCPU_DATA1: c_uint = 0xef14;
pub const UVD_LMI_EXT40_ADDR: c_uint = 0xf498;
pub const UVD_VCPU_CHIP_ID: c_uint = 0xf4d4;
pub const UVD_VCPU_CACHE_OFFSET0: c_uint = 0xf4d8;
pub const UVD_VCPU_CACHE_SIZE0: c_uint = 0xf4dc;
pub const UVD_VCPU_CACHE_OFFSET1: c_uint = 0xf4e0;
pub const UVD_VCPU_CACHE_SIZE1: c_uint = 0xf4e4;
pub const UVD_VCPU_CACHE_OFFSET2: c_uint = 0xf4e8;
pub const UVD_VCPU_CACHE_SIZE2: c_uint = 0xf4ec;
pub const UVD_LMI_ADDR_EXT: c_uint = 0xf594;
pub const UVD_RBC_RB_RPTR: c_uint = 0xf690;
pub const UVD_RBC_RB_WPTR: c_uint = 0xf694;
pub const UVD_CONTEXT_ID: c_uint = 0xf6f4;
