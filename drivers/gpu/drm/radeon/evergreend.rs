//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/evergreend.h
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
// Copyright 2010 Advanced Micro Devices, Inc.
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
pub const EVERGREEN_MAX_SH_GPRS: c_int = 256;
pub const EVERGREEN_MAX_TEMP_GPRS: c_int = 16;
pub const EVERGREEN_MAX_SH_THREADS: c_int = 256;
pub const EVERGREEN_MAX_SH_STACK_ENTRIES: c_int = 4096;
pub const EVERGREEN_MAX_FRC_EOV_CNT: c_int = 16384;
pub const EVERGREEN_MAX_BACKENDS: c_int = 8;
pub const EVERGREEN_MAX_BACKENDS_MASK: c_uint = 0xFF;
pub const EVERGREEN_MAX_SIMDS: c_int = 16;
pub const EVERGREEN_MAX_SIMDS_MASK: c_uint = 0xFFFF;
pub const EVERGREEN_MAX_PIPES: c_int = 8;
pub const EVERGREEN_MAX_PIPES_MASK: c_uint = 0xFF;
pub const EVERGREEN_MAX_LDS_NUM: c_uint = 0xFFFF;
pub const CYPRESS_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02011003;
pub const BARTS_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02011003;
pub const CAYMAN_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02011003;
pub const JUNIPER_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02010002;
pub const REDWOOD_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02010002;
pub const TURKS_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02010002;
pub const CEDAR_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02010001;
pub const CAICOS_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02010001;
pub const SUMO_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02010002;
pub const SUMO2_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02010002;
// pm registers
pub const SMC_MSG: c_uint = 0x20c;

pub const HOST_SMC_MSG_SHIFT: c_int = 0;

pub const HOST_SMC_RESP_SHIFT: c_int = 8;

pub const SMC_HOST_MSG_SHIFT: c_int = 16;

pub const SMC_HOST_RESP_SHIFT: c_int = 24;
pub const DCCG_DISP_SLOW_SELECT_REG: c_uint = 0x4fc;

pub const DCCG_DISP1_SLOW_SELECT_SHIFT: c_int = 0;

pub const DCCG_DISP2_SLOW_SELECT_SHIFT: c_int = 4;
pub const CG_SPLL_FUNC_CNTL: c_uint = 0x600;

pub const CG_SPLL_FUNC_CNTL_2: c_uint = 0x604;

pub const CG_SPLL_FUNC_CNTL_3: c_uint = 0x608;

pub const CG_SPLL_STATUS: c_uint = 0x60c;

pub const MPLL_CNTL_MODE: c_uint = 0x61c;

pub const MPLL_AD_FUNC_CNTL: c_uint = 0x624;

pub const MPLL_AD_FUNC_CNTL_2: c_uint = 0x628;

pub const MPLL_DQ_FUNC_CNTL: c_uint = 0x62c;
pub const MPLL_DQ_FUNC_CNTL_2: c_uint = 0x630;
pub const GENERAL_PWRMGT: c_uint = 0x63c;

pub const SCLK_PWRMGT_CNTL: c_uint = 0x644;

pub const MCLK_PWRMGT_CNTL: c_uint = 0x648;

pub const DLL_CNTL: c_uint = 0x64c;

pub const CG_AT: c_uint = 0x6d4;

pub const CG_DISPLAY_GAP_CNTL: c_uint = 0x714;

pub const CG_BIF_REQ_AND_RSP: c_uint = 0x7f4;

pub const CG_CLIENT_REQ_SHIFT: c_int = 0;

pub const CG_CLIENT_RESP_SHIFT: c_int = 8;

pub const CLIENT_CG_REQ_SHIFT: c_int = 16;

pub const CLIENT_CG_RESP_SHIFT: c_int = 24;
pub const CG_SPLL_SPREAD_SPECTRUM: c_uint = 0x790;

pub const CG_SPLL_SPREAD_SPECTRUM_2: c_uint = 0x794;
pub const MPLL_SS1: c_uint = 0x85c;

pub const MPLL_SS2: c_uint = 0x860;

pub const CG_IND_ADDR: c_uint = 0x8f8;
pub const CG_IND_DATA: c_uint = 0x8fc;
// CGIND regs
pub const CG_CGTT_LOCAL_0: c_uint = 0x00;
pub const CG_CGTT_LOCAL_1: c_uint = 0x01;
pub const CG_CGTT_LOCAL_2: c_uint = 0x02;
pub const CG_CGTT_LOCAL_3: c_uint = 0x03;
pub const CG_CGLS_TILE_0: c_uint = 0x20;
pub const CG_CGLS_TILE_1: c_uint = 0x21;
pub const CG_CGLS_TILE_2: c_uint = 0x22;
pub const CG_CGLS_TILE_3: c_uint = 0x23;
pub const CG_CGLS_TILE_4: c_uint = 0x24;
pub const CG_CGLS_TILE_5: c_uint = 0x25;
pub const CG_CGLS_TILE_6: c_uint = 0x26;
pub const CG_CGLS_TILE_7: c_uint = 0x27;
pub const CG_CGLS_TILE_8: c_uint = 0x28;
pub const CG_CGLS_TILE_9: c_uint = 0x29;
pub const CG_CGLS_TILE_10: c_uint = 0x2a;
pub const CG_CGLS_TILE_11: c_uint = 0x2b;
pub const VM_L2_CG: c_uint = 0x15c0;
pub const MC_CONFIG: c_uint = 0x2000;
pub const MC_CONFIG_MCD: c_uint = 0x20a0;
pub const MC_CG_CONFIG_MCD: c_uint = 0x20a4;

pub const MC_HUB_MISC_HUB_CG: c_uint = 0x20b8;
pub const MC_HUB_MISC_VM_CG: c_uint = 0x20bc;
pub const MC_HUB_MISC_SIP_CG: c_uint = 0x20c0;
pub const MC_XPB_CLK_GAT: c_uint = 0x2478;
pub const MC_CG_CONFIG: c_uint = 0x25bc;

pub const MC_CITF_MISC_RD_CG: c_uint = 0x2648;
pub const MC_CITF_MISC_WR_CG: c_uint = 0x264c;
pub const MC_CITF_MISC_VM_CG: c_uint = 0x2650;

pub const MC_ARB_BURST_TIME: c_uint = 0x2808;

pub const MC_SEQ_RAS_TIMING: c_uint = 0x28a0;
pub const MC_SEQ_CAS_TIMING: c_uint = 0x28a4;
pub const MC_SEQ_MISC_TIMING: c_uint = 0x28a8;
pub const MC_SEQ_MISC_TIMING2: c_uint = 0x28ac;
pub const MC_SEQ_RD_CTL_D0: c_uint = 0x28b4;
pub const MC_SEQ_RD_CTL_D1: c_uint = 0x28b8;
pub const MC_SEQ_WR_CTL_D0: c_uint = 0x28bc;
pub const MC_SEQ_WR_CTL_D1: c_uint = 0x28c0;
pub const MC_SEQ_STATUS_M: c_uint = 0x29f4;

pub const MC_SEQ_MISC1: c_uint = 0x2a04;
pub const MC_SEQ_RESERVE_M: c_uint = 0x2a08;
pub const MC_PMG_CMD_EMRS: c_uint = 0x2a0c;
pub const MC_SEQ_MISC3: c_uint = 0x2a2c;
pub const MC_SEQ_MISC5: c_uint = 0x2a54;
pub const MC_SEQ_MISC6: c_uint = 0x2a58;
pub const MC_SEQ_MISC7: c_uint = 0x2a64;
pub const MC_SEQ_CG: c_uint = 0x2a68;

pub const CG_SEQ_REQ_SHIFT: c_int = 0;

pub const CG_SEQ_RESP_SHIFT: c_int = 8;

pub const SEQ_CG_REQ_SHIFT: c_int = 16;

pub const SEQ_CG_RESP_SHIFT: c_int = 24;
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
pub const CGTS_SM_CTRL_REG: c_uint = 0x9150;
// Registers
pub const RCU_IND_INDEX: c_uint = 0x100;
pub const RCU_IND_DATA: c_uint = 0x104;
// discrete uvd clocks
pub const CG_UPLL_FUNC_CNTL: c_uint = 0x718;

pub const CG_UPLL_FUNC_CNTL_2: c_uint = 0x71c;

pub const CG_UPLL_FUNC_CNTL_3: c_uint = 0x720;

pub const CG_UPLL_FUNC_CNTL_4: c_uint = 0x854;

pub const CG_UPLL_SPREAD_SPECTRUM: c_uint = 0x79c;

// fusion uvd clocks
pub const CG_DCLK_CNTL: c_uint = 0x610;

pub const CG_DCLK_STATUS: c_uint = 0x614;

pub const CG_VCLK_CNTL: c_uint = 0x618;
pub const CG_VCLK_STATUS: c_uint = 0x61c;
pub const CG_SCRATCH1: c_uint = 0x820;
pub const RLC_CNTL: c_uint = 0x3f00;

pub const RLC_HB_BASE: c_uint = 0x3f10;
pub const RLC_HB_CNTL: c_uint = 0x3f0c;
pub const RLC_HB_RPTR: c_uint = 0x3f20;
pub const RLC_HB_WPTR: c_uint = 0x3f1c;
pub const RLC_HB_WPTR_LSB_ADDR: c_uint = 0x3f14;
pub const RLC_HB_WPTR_MSB_ADDR: c_uint = 0x3f18;
pub const RLC_MC_CNTL: c_uint = 0x3f44;
pub const RLC_UCODE_CNTL: c_uint = 0x3f48;
pub const RLC_UCODE_ADDR: c_uint = 0x3f2c;
pub const RLC_UCODE_DATA: c_uint = 0x3f30;
// new for TN
pub const TN_RLC_SAVE_AND_RESTORE_BASE: c_uint = 0x3f10;
pub const TN_RLC_LB_CNTR_MAX: c_uint = 0x3f14;
pub const TN_RLC_LB_CNTR_INIT: c_uint = 0x3f18;
pub const TN_RLC_CLEAR_STATE_RESTORE_BASE: c_uint = 0x3f20;
pub const TN_RLC_LB_INIT_SIMD_MASK: c_uint = 0x3fe4;
pub const TN_RLC_LB_ALWAYS_ACTIVE_SIMD_MASK: c_uint = 0x3fe8;
pub const TN_RLC_LB_PARAMS: c_uint = 0x3fec;
pub const GRBM_GFX_INDEX: c_uint = 0x802C;

pub const RLC_GFX_INDEX: c_uint = 0x3fC4;
pub const CC_GC_SHADER_PIPE_CONFIG: c_uint = 0x8950;

pub const CC_RB_BACKEND_DISABLE: c_uint = 0x98F4;

pub const GB_ADDR_CONFIG: c_uint = 0x98F8;

pub const NUM_PIPES_MASK: c_uint = 0x0000000f;

pub const GB_BACKEND_MAP: c_uint = 0x98FC;
pub const DMIF_ADDR_CONFIG: c_uint = 0xBD4;
pub const HDP_ADDR_CONFIG: c_uint = 0x2F48;
pub const HDP_MISC_CNTL: c_uint = 0x2F4C;

pub const CC_SYS_RB_BACKEND_DISABLE: c_uint = 0x3F88;
pub const GC_USER_RB_BACKEND_DISABLE: c_uint = 0x9B7C;
pub const CGTS_SYS_TCC_DISABLE: c_uint = 0x3F90;
pub const CGTS_TCC_DISABLE: c_uint = 0x9148;
pub const CGTS_USER_SYS_TCC_DISABLE: c_uint = 0x3F94;
pub const CGTS_USER_TCC_DISABLE: c_uint = 0x914C;
pub const CONFIG_MEMSIZE: c_uint = 0x5428;
pub const BIF_FB_EN: c_uint = 0x5490;

pub const CP_STRMOUT_CNTL: c_uint = 0x84FC;
pub const CP_COHER_CNTL: c_uint = 0x85F0;
pub const CP_COHER_SIZE: c_uint = 0x85F4;
pub const CP_COHER_BASE: c_uint = 0x85F8;
pub const CP_STALLED_STAT1: c_uint = 0x8674;
pub const CP_STALLED_STAT2: c_uint = 0x8678;
pub const CP_BUSY_STAT: c_uint = 0x867C;
pub const CP_STAT: c_uint = 0x8680;
pub const CP_ME_CNTL: c_uint = 0x86D8;

pub const CP_ME_RAM_DATA: c_uint = 0xC160;
pub const CP_ME_RAM_RADDR: c_uint = 0xC158;
pub const CP_ME_RAM_WADDR: c_uint = 0xC15C;
pub const CP_MEQ_THRESHOLDS: c_uint = 0x8764;

pub const CP_PERFMON_CNTL: c_uint = 0x87FC;
pub const CP_PFP_UCODE_ADDR: c_uint = 0xC150;
pub const CP_PFP_UCODE_DATA: c_uint = 0xC154;
pub const CP_QUEUE_THRESHOLDS: c_uint = 0x8760;

pub const CP_RB_BASE: c_uint = 0xC100;
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
pub const CP_SEM_INCOMPLETE_TIMER_CNTL: c_uint = 0x85C8;
pub const CP_DEBUG: c_uint = 0xC1FC;
// Audio clocks
pub const DCCG_AUDIO_DTO_SOURCE: c_uint = 0x05ac;

pub const DCCG_AUDIO_DTO0_PHASE: c_uint = 0x05b0;
pub const DCCG_AUDIO_DTO0_MODULE: c_uint = 0x05b4;
pub const DCCG_AUDIO_DTO0_LOAD: c_uint = 0x05b8;
pub const DCCG_AUDIO_DTO0_CNTL: c_uint = 0x05bc;

pub const DCCG_AUDIO_DTO1_PHASE: c_uint = 0x05c0;
pub const DCCG_AUDIO_DTO1_MODULE: c_uint = 0x05c4;
pub const DCCG_AUDIO_DTO1_LOAD: c_uint = 0x05c8;
pub const DCCG_AUDIO_DTO1_CNTL: c_uint = 0x05cc;

pub const DCE41_DENTIST_DISPCLK_CNTL: c_uint = 0x049c;

// DCE 4.0 AFMT
pub const HDMI_CONTROL: c_uint = 0x7030;

pub const HDMI_STATUS: c_uint = 0x7034;

pub const HDMI_AUDIO_PACKET_CONTROL: c_uint = 0x7038;

pub const HDMI_ACR_PACKET_CONTROL: c_uint = 0x703c;

pub const HDMI_VBI_PACKET_CONTROL: c_uint = 0x7040;

pub const HDMI_INFOFRAME_CONTROL0: c_uint = 0x7044;

pub const HDMI_INFOFRAME_CONTROL1: c_uint = 0x7048;

pub const HDMI_GENERIC_PACKET_CONTROL: c_uint = 0x704c;

pub const HDMI_GC: c_uint = 0x7058;

pub const AFMT_AUDIO_PACKET_CONTROL2: c_uint = 0x705c;

pub const AFMT_AVI_INFO0: c_uint = 0x7084;

pub const AFMT_AVI_INFO1: c_uint = 0x7088;

pub const AFMT_AVI_INFO2: c_uint = 0x708c;

pub const AFMT_AVI_INFO3: c_uint = 0x7090;

pub const AFMT_MPEG_INFO0: c_uint = 0x7094;

pub const AFMT_MPEG_INFO1: c_uint = 0x7098;

pub const AFMT_GENERIC0_HDR: c_uint = 0x709c;
pub const AFMT_GENERIC0_0: c_uint = 0x70a0;
pub const AFMT_GENERIC0_1: c_uint = 0x70a4;
pub const AFMT_GENERIC0_2: c_uint = 0x70a8;
pub const AFMT_GENERIC0_3: c_uint = 0x70ac;
pub const AFMT_GENERIC0_4: c_uint = 0x70b0;
pub const AFMT_GENERIC0_5: c_uint = 0x70b4;
pub const AFMT_GENERIC0_6: c_uint = 0x70b8;
pub const AFMT_GENERIC1_HDR: c_uint = 0x70bc;
pub const AFMT_GENERIC1_0: c_uint = 0x70c0;
pub const AFMT_GENERIC1_1: c_uint = 0x70c4;
pub const AFMT_GENERIC1_2: c_uint = 0x70c8;
pub const AFMT_GENERIC1_3: c_uint = 0x70cc;
pub const AFMT_GENERIC1_4: c_uint = 0x70d0;
pub const AFMT_GENERIC1_5: c_uint = 0x70d4;
pub const AFMT_GENERIC1_6: c_uint = 0x70d8;
pub const HDMI_ACR_32_0: c_uint = 0x70dc;

pub const HDMI_ACR_32_1: c_uint = 0x70e0;

pub const HDMI_ACR_44_0: c_uint = 0x70e4;

pub const HDMI_ACR_44_1: c_uint = 0x70e8;

pub const HDMI_ACR_48_0: c_uint = 0x70ec;

pub const HDMI_ACR_48_1: c_uint = 0x70f0;

pub const HDMI_ACR_STATUS_0: c_uint = 0x70f4;
pub const HDMI_ACR_STATUS_1: c_uint = 0x70f8;
pub const AFMT_AUDIO_INFO0: c_uint = 0x70fc;

pub const AFMT_AUDIO_INFO1: c_uint = 0x7100;

pub const AFMT_60958_0: c_uint = 0x7104;

pub const AFMT_60958_1: c_uint = 0x7108;

pub const AFMT_AUDIO_CRC_CONTROL: c_uint = 0x710c;

pub const AFMT_RAMP_CONTROL0: c_uint = 0x7110;

pub const AFMT_RAMP_CONTROL1: c_uint = 0x7114;

pub const AFMT_RAMP_CONTROL2: c_uint = 0x7118;

pub const AFMT_RAMP_CONTROL3: c_uint = 0x711c;

pub const AFMT_60958_2: c_uint = 0x7120;

pub const AFMT_STATUS: c_uint = 0x7128;

pub const AFMT_AUDIO_PACKET_CONTROL: c_uint = 0x712c;

pub const AFMT_VBI_PACKET_CONTROL: c_uint = 0x7130;

pub const AFMT_INFOFRAME_CONTROL0: c_uint = 0x7134;

pub const AFMT_GENERIC0_7: c_uint = 0x7138;
// DCE4/5 ELD audio interface
pub const AZ_F0_CODEC_PIN0_CONTROL_CHANNEL_SPEAKER: c_uint = 0x5f78;

pub const SPEAKER_ALLOCATION_SHIFT: c_int = 0;

pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR0: c_uint = 0x5f84 /* LPCM */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR1: c_uint = 0x5f88 /* AC3 */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR2: c_uint = 0x5f8c /* MPEG1 */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR3: c_uint = 0x5f90 /* MP3 */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR4: c_uint = 0x5f94 /* MPEG2 */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR5: c_uint = 0x5f98 /* AAC */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR6: c_uint = 0x5f9c /* DTS */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR7: c_uint = 0x5fa0 /* ATRAC */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR8: c_uint = 0x5fa4 /* one bit audio - leave at 0 (default) */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR9: c_uint = 0x5fa8 /* Dolby Digital */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR10: c_uint = 0x5fac /* DTS-HD */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR11: c_uint = 0x5fb0 /* MAT-MLP */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR12: c_uint = 0x5fb4 /* DTS */;
pub const AZ_F0_CODEC_PIN0_CONTROL_AUDIO_DESCRIPTOR13: c_uint = 0x5fb8 /* WMA Pro */;

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
pub const AZ_CHANNEL_COUNT_CONTROL: c_uint = 0x5fe4;

// HBR_CHANNEL_COUNT, COMPRESSED_CHANNEL_COUNT
// 0   = use stream header
// 1-7 = channel count - 1
//
pub const AZ_F0_CODEC_PIN0_CONTROL_RESPONSE_LIPSYNC: c_uint = 0x5fe8;

// VIDEO_LIPSYNC, AUDIO_LIPSYNC
// 0   = invalid
// x   = legal delay value
// 255 = sync not supported
//
pub const AZ_F0_CODEC_PIN0_CONTROL_RESPONSE_HBR: c_uint = 0x5fec;

pub const AZ_F0_CODEC_PIN0_CONTROL_RESPONSE_AV_ASSOCIATION0: c_uint = 0x5ff4;

pub const AZ_F0_CODEC_PIN0_CONTROL_RESPONSE_AV_ASSOCIATION1: c_uint = 0x5ff8;

pub const AZ_F0_CODEC_PIN0_CONTROL_RESPONSE_AV_NUMBER: c_uint = 0x5ffc;

pub const AZ_HOT_PLUG_CONTROL: c_uint = 0x5e78;

pub const GC_USER_SHADER_PIPE_CONFIG: c_uint = 0x8954;

pub const INACTIVE_QD_PIPES_MASK: c_uint = 0x0000FF00;

pub const INACTIVE_SIMDS_MASK: c_uint = 0x00FF0000;
pub const GRBM_CNTL: c_uint = 0x8000;

pub const GRBM_SOFT_RESET: c_uint = 0x8020;

pub const GRBM_STATUS: c_uint = 0x8010;
pub const CMDFIFO_AVAIL_MASK: c_uint = 0x0000000F;

pub const GRBM_STATUS_SE0: c_uint = 0x8014;
pub const GRBM_STATUS_SE1: c_uint = 0x8018;

// evergreen
pub const CG_THERMAL_CTRL: c_uint = 0x72c;
pub const TOFFSET_MASK: c_uint = 0x00003FE0;
pub const TOFFSET_SHIFT: c_int = 5;

pub const DIG_THERM_DPM_MASK: c_uint = 0x003FC000;
pub const DIG_THERM_DPM_SHIFT: c_int = 14;
pub const CG_THERMAL_INT: c_uint = 0x734;

pub const DIG_THERM_INTH_MASK: c_uint = 0x0000FF00;
pub const DIG_THERM_INTH_SHIFT: c_int = 8;

pub const DIG_THERM_INTL_MASK: c_uint = 0x00FF0000;
pub const DIG_THERM_INTL_SHIFT: c_int = 16;

pub const TN_CG_THERMAL_INT_CTRL: c_uint = 0x738;

pub const TN_DIG_THERM_INTH_MASK: c_uint = 0x000000FF;
pub const TN_DIG_THERM_INTH_SHIFT: c_int = 0;

pub const TN_DIG_THERM_INTL_MASK: c_uint = 0x0000FF00;
pub const TN_DIG_THERM_INTL_SHIFT: c_int = 8;

pub const CG_MULT_THERMAL_STATUS: c_uint = 0x740;

pub const ASIC_T_MASK: c_uint = 0x07FF0000;
pub const ASIC_T_SHIFT: c_int = 16;
pub const CG_TS0_STATUS: c_uint = 0x760;
pub const TS0_ADC_DOUT_MASK: c_uint = 0x000003FF;
pub const TS0_ADC_DOUT_SHIFT: c_int = 0;
// APU
pub const CG_THERMAL_STATUS: c_uint = 0x678;
pub const HDP_HOST_PATH_CNTL: c_uint = 0x2C00;
pub const HDP_NONSURFACE_BASE: c_uint = 0x2C04;
pub const HDP_NONSURFACE_INFO: c_uint = 0x2C08;
pub const HDP_NONSURFACE_SIZE: c_uint = 0x2C0C;
pub const HDP_MEM_COHERENCY_FLUSH_CNTL: c_uint = 0x5480;
pub const HDP_REG_COHERENCY_FLUSH_CNTL: c_uint = 0x54A0;
pub const HDP_TILING_CONFIG: c_uint = 0x2F3C;
pub const MC_SHARED_CHMAP: c_uint = 0x2004;
pub const NOOFCHAN_SHIFT: c_int = 12;
pub const NOOFCHAN_MASK: c_uint = 0x00003000;
pub const MC_SHARED_CHREMAP: c_uint = 0x2008;
pub const MC_SHARED_BLACKOUT_CNTL: c_uint = 0x20ac;
pub const BLACKOUT_MODE_MASK: c_uint = 0x00000007;
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

pub const FUS_MC_ARB_RAMCFG: c_uint = 0x2768;
pub const MC_VM_AGP_TOP: c_uint = 0x2028;
pub const MC_VM_AGP_BOT: c_uint = 0x202C;
pub const MC_VM_AGP_BASE: c_uint = 0x2030;
pub const MC_VM_FB_LOCATION: c_uint = 0x2024;
pub const MC_FUS_VM_FB_OFFSET: c_uint = 0x2898;
pub const MC_VM_MB_L1_TLB0_CNTL: c_uint = 0x2234;
pub const MC_VM_MB_L1_TLB1_CNTL: c_uint = 0x2238;
pub const MC_VM_MB_L1_TLB2_CNTL: c_uint = 0x223C;
pub const MC_VM_MB_L1_TLB3_CNTL: c_uint = 0x2240;

pub const MC_VM_MD_L1_TLB0_CNTL: c_uint = 0x2654;
pub const MC_VM_MD_L1_TLB1_CNTL: c_uint = 0x2658;
pub const MC_VM_MD_L1_TLB2_CNTL: c_uint = 0x265C;
pub const MC_VM_MD_L1_TLB3_CNTL: c_uint = 0x2698;
pub const FUS_MC_VM_MD_L1_TLB0_CNTL: c_uint = 0x265C;
pub const FUS_MC_VM_MD_L1_TLB1_CNTL: c_uint = 0x2660;
pub const FUS_MC_VM_MD_L1_TLB2_CNTL: c_uint = 0x2664;
pub const MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR: c_uint = 0x203C;
pub const MC_VM_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x2038;
pub const MC_VM_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x2034;
pub const PA_CL_ENHANCE: c_uint = 0x8A14;

pub const PA_SC_ENHANCE: c_uint = 0x8BF0;
pub const PA_SC_AA_CONFIG: c_uint = 0x28C04;
pub const MSAA_NUM_SAMPLES_SHIFT: c_int = 0;
pub const MSAA_NUM_SAMPLES_MASK: c_uint = 0x3;
pub const PA_SC_CLIPRECT_RULE: c_uint = 0x2820C;
pub const PA_SC_EDGERULE: c_uint = 0x28230;
pub const PA_SC_FIFO_SIZE: c_uint = 0x8BCC;

pub const PA_SC_FORCE_EOV_MAX_CNTS: c_uint = 0x8B24;

pub const PA_SC_LINE_STIPPLE: c_uint = 0x28A0C;
pub const PA_SU_LINE_STIPPLE_VALUE: c_uint = 0x8A60;
pub const PA_SC_LINE_STIPPLE_STATE: c_uint = 0x8B10;
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

pub const SQ_GPR_RESOURCE_MGMT_1: c_uint = 0x8C04;

pub const SQ_GPR_RESOURCE_MGMT_2: c_uint = 0x8C08;

pub const SQ_GPR_RESOURCE_MGMT_3: c_uint = 0x8C0C;

pub const SQ_GLOBAL_GPR_RESOURCE_MGMT_1: c_uint = 0x8C10;
pub const SQ_GLOBAL_GPR_RESOURCE_MGMT_2: c_uint = 0x8C14;
pub const SQ_THREAD_RESOURCE_MGMT: c_uint = 0x8C18;

pub const SQ_THREAD_RESOURCE_MGMT_2: c_uint = 0x8C1C;

pub const SQ_STACK_RESOURCE_MGMT_1: c_uint = 0x8C20;

pub const SQ_STACK_RESOURCE_MGMT_2: c_uint = 0x8C24;

pub const SQ_STACK_RESOURCE_MGMT_3: c_uint = 0x8C28;

pub const SQ_DYN_GPR_CNTL_PS_FLUSH_REQ: c_uint = 0x8D8C;
pub const SQ_DYN_GPR_SIMD_LOCK_EN: c_uint = 0x8D94;
pub const SQ_STATIC_THREAD_MGMT_1: c_uint = 0x8E20;
pub const SQ_STATIC_THREAD_MGMT_2: c_uint = 0x8E24;
pub const SQ_STATIC_THREAD_MGMT_3: c_uint = 0x8E28;
pub const SQ_LDS_RESOURCE_MGMT: c_uint = 0x8E2C;
pub const SQ_MS_FIFO_SIZES: c_uint = 0x8CF0;

pub const SX_DEBUG_1: c_uint = 0x9058;

pub const SX_EXPORT_BUFFER_SIZES: c_uint = 0x900C;

pub const SX_MEMORY_EXPORT_BASE: c_uint = 0x9010;
pub const SX_MISC: c_uint = 0x28350;
pub const CB_PERF_CTR0_SEL_0: c_uint = 0x9A20;
pub const CB_PERF_CTR0_SEL_1: c_uint = 0x9A24;
pub const CB_PERF_CTR1_SEL_0: c_uint = 0x9A28;
pub const CB_PERF_CTR1_SEL_1: c_uint = 0x9A2C;
pub const CB_PERF_CTR2_SEL_0: c_uint = 0x9A30;
pub const CB_PERF_CTR2_SEL_1: c_uint = 0x9A34;
pub const CB_PERF_CTR3_SEL_0: c_uint = 0x9A38;
pub const CB_PERF_CTR3_SEL_1: c_uint = 0x9A3C;
pub const TA_CNTL_AUX: c_uint = 0x9508;

pub const TCP_CHAN_STEER_LO: c_uint = 0x960c;
pub const TCP_CHAN_STEER_HI: c_uint = 0x9610;
pub const VGT_CACHE_INVALIDATION: c_uint = 0x88C4;

pub const VC_ONLY: c_int = 0;
pub const TC_ONLY: c_int = 1;
pub const VC_AND_TC: c_int = 2;

pub const NO_AUTO: c_int = 0;
pub const ES_AUTO: c_int = 1;
pub const GS_AUTO: c_int = 2;
pub const ES_AND_GS_AUTO: c_int = 3;
pub const VGT_GS_VERTEX_REUSE: c_uint = 0x88D4;
pub const VGT_NUM_INSTANCES: c_uint = 0x8974;
pub const VGT_OUT_DEALLOC_CNTL: c_uint = 0x28C5C;
pub const DEALLOC_DIST_MASK: c_uint = 0x0000007F;
pub const VGT_VERTEX_REUSE_BLOCK_CNTL: c_uint = 0x28C58;
pub const VTX_REUSE_DEPTH_MASK: c_uint = 0x000000FF;
pub const VM_CONTEXT0_CNTL: c_uint = 0x1410;

pub const VM_CONTEXT1_CNTL: c_uint = 0x1414;
pub const VM_CONTEXT1_CNTL2: c_uint = 0x1434;
pub const VM_CONTEXT0_PAGE_TABLE_BASE_ADDR: c_uint = 0x153C;
pub const VM_CONTEXT0_PAGE_TABLE_END_ADDR: c_uint = 0x157C;
pub const VM_CONTEXT0_PAGE_TABLE_START_ADDR: c_uint = 0x155C;
pub const VM_CONTEXT0_PROTECTION_FAULT_DEFAULT_ADDR: c_uint = 0x1518;
pub const VM_CONTEXT0_REQUEST_RESPONSE: c_uint = 0x1470;

pub const RESPONSE_TYPE_MASK: c_uint = 0x000000F0;
pub const RESPONSE_TYPE_SHIFT: c_int = 4;
pub const VM_L2_CNTL: c_uint = 0x1400;

pub const VM_L2_CNTL2: c_uint = 0x1404;

pub const VM_L2_CNTL3: c_uint = 0x1408;

pub const VM_L2_STATUS: c_uint = 0x140C;

pub const VM_CONTEXT1_PROTECTION_FAULT_ADDR: c_uint = 0x14FC;
pub const VM_CONTEXT1_PROTECTION_FAULT_STATUS: c_uint = 0x14DC;
pub const WAIT_UNTIL: c_uint = 0x8040;
pub const SRBM_STATUS: c_uint = 0x0E50;

pub const SRBM_STATUS2: c_uint = 0x0EC4;

pub const SRBM_SOFT_RESET: c_uint = 0x0E60;
pub const SRBM_SOFT_RESET_ALL_MASK: c_uint = 0x00FEEFA6;

pub const SRBM_READ_ERROR: c_uint = 0xE98;
pub const SRBM_INT_CNTL: c_uint = 0xEA0;
pub const SRBM_INT_ACK: c_uint = 0xEA8;
// display watermarks
pub const DC_LB_MEMORY_SPLIT: c_uint = 0x6b0c;
pub const PRIORITY_A_CNT: c_uint = 0x6b18;
pub const PRIORITY_MARK_MASK: c_uint = 0x7fff;

pub const PRIORITY_B_CNT: c_uint = 0x6b1c;
pub const PIPE0_ARBITRATION_CONTROL3: c_uint = 0x0bf0;

pub const PIPE0_LATENCY_CONTROL: c_uint = 0x0bf4;

pub const PIPE0_DMIF_BUFFER_CONTROL: c_uint = 0x0ca0;

pub const IH_RB_CNTL: c_uint = 0x3e00;

pub const IH_RB_BASE: c_uint = 0x3e04;
pub const IH_RB_RPTR: c_uint = 0x3e08;
pub const IH_RB_WPTR: c_uint = 0x3e0c;

pub const IH_RB_WPTR_ADDR_HI: c_uint = 0x3e10;
pub const IH_RB_WPTR_ADDR_LO: c_uint = 0x3e14;
pub const IH_CNTL: c_uint = 0x3e18;

pub const CP_INT_CNTL: c_uint = 0xc124;

pub const CP_INT_STATUS: c_uint = 0xc128;

pub const GRBM_INT_CNTL: c_uint = 0x8060;

// 0x6e98, 0x7a98, 0x10698, 0x11298, 0x11e98, 0x12a98
pub const CRTC_STATUS_FRAME_COUNT: c_uint = 0x6e98;
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

pub const DACA_AUTODETECT_INT_CONTROL: c_uint = 0x66c8;
pub const DACB_AUTODETECT_INT_CONTROL: c_uint = 0x67c8;
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

// DCE4/5/6 FMT blocks
pub const FMT_DYNAMIC_EXP_CNTL: c_uint = 0x6fb4;

// 0 = 10bit -> 12bit, 1 = 8bit -> 12bit
pub const FMT_CONTROL: c_uint = 0x6fb8;

// 0 = RGB 4:4:4 or YCbCr 4:4:4, 1 = YCbCr 4:2:2
pub const FMT_BIT_DEPTH_CONTROL: c_uint = 0x6fc8;

pub const FMT_CLAMP_CONTROL: c_uint = 0x6fe4;

// ASYNC DMA
pub const DMA_RB_RPTR: c_uint = 0xd008;
pub const DMA_RB_WPTR: c_uint = 0xd00c;
pub const DMA_CNTL: c_uint = 0xd02c;

pub const DMA_TILING_CONFIG: c_uint = 0xD0B8;
pub const CAYMAN_DMA1_CNTL: c_uint = 0xd82c;
// async DMA packets

// async DMA Packet types
pub const DMA_PACKET_WRITE: c_uint = 0x2;
pub const DMA_PACKET_COPY: c_uint = 0x3;
pub const DMA_PACKET_INDIRECT_BUFFER: c_uint = 0x4;
pub const DMA_PACKET_SEMAPHORE: c_uint = 0x5;
pub const DMA_PACKET_FENCE: c_uint = 0x6;
pub const DMA_PACKET_TRAP: c_uint = 0x7;
pub const DMA_PACKET_SRBM_WRITE: c_uint = 0x9;
pub const DMA_PACKET_CONSTANT_FILL: c_uint = 0xd;
pub const DMA_PACKET_NOP: c_uint = 0xf;
// PIF PHY0 indirect regs
pub const PB0_PIF_CNTL: c_uint = 0x10;

pub const PB0_PIF_PAIRING: c_uint = 0x11;

pub const PB0_PIF_PWRDOWN_0: c_uint = 0x12;

pub const PB0_PIF_PWRDOWN_1: c_uint = 0x13;

// PIF PHY1 indirect regs
pub const PB1_PIF_CNTL: c_uint = 0x10;
pub const PB1_PIF_PAIRING: c_uint = 0x11;
pub const PB1_PIF_PWRDOWN_0: c_uint = 0x12;
pub const PB1_PIF_PWRDOWN_1: c_uint = 0x13;
// PCIE PORT indirect regs
pub const PCIE_LC_CNTL: c_uint = 0xa0;

pub const PCIE_LC_TRAINING_CNTL: c_uint = 0xa1 /* PCIE_P */;
pub const PCIE_LC_LINK_WIDTH_CNTL: c_uint = 0xa2 /* PCIE_P */;

pub const PCIE_LC_SPEED_CNTL: c_uint = 0xa4 /* PCIE_P */;

pub const MM_CFGREGS_CNTL: c_uint = 0x544c;

pub const LINK_CNTL2: c_uint = 0x88 /* F0 */;

//
// UVD
//
pub const UVD_UDEC_ADDR_CONFIG: c_uint = 0xef4c;
pub const UVD_UDEC_DB_ADDR_CONFIG: c_uint = 0xef50;
pub const UVD_UDEC_DBW_ADDR_CONFIG: c_uint = 0xef54;
pub const UVD_NO_OP: c_uint = 0xeffc;
pub const UVD_RBC_RB_RPTR: c_uint = 0xf690;
pub const UVD_RBC_RB_WPTR: c_uint = 0xf694;
pub const UVD_STATUS: c_uint = 0xf6bc;
//
// PM4
//

pub const CP_PACKET2: c_uint = 0x80000000;
pub const PACKET2_PAD_SHIFT: c_int = 0;

// Packet 3 types
pub const PACKET3_NOP: c_uint = 0x10;
pub const PACKET3_SET_BASE: c_uint = 0x11;
pub const PACKET3_CLEAR_STATE: c_uint = 0x12;
pub const PACKET3_INDEX_BUFFER_SIZE: c_uint = 0x13;
pub const PACKET3_DISPATCH_DIRECT: c_uint = 0x15;
pub const PACKET3_DISPATCH_INDIRECT: c_uint = 0x16;
pub const PACKET3_INDIRECT_BUFFER_END: c_uint = 0x17;
pub const PACKET3_MODE_CONTROL: c_uint = 0x18;
pub const PACKET3_SET_PREDICATION: c_uint = 0x20;
pub const PACKET3_REG_RMW: c_uint = 0x21;
pub const PACKET3_COND_EXEC: c_uint = 0x22;
pub const PACKET3_PRED_EXEC: c_uint = 0x23;
pub const PACKET3_DRAW_INDIRECT: c_uint = 0x24;
pub const PACKET3_DRAW_INDEX_INDIRECT: c_uint = 0x25;
pub const PACKET3_INDEX_BASE: c_uint = 0x26;
pub const PACKET3_DRAW_INDEX_2: c_uint = 0x27;
pub const PACKET3_CONTEXT_CONTROL: c_uint = 0x28;
pub const PACKET3_DRAW_INDEX_OFFSET: c_uint = 0x29;
pub const PACKET3_INDEX_TYPE: c_uint = 0x2A;
pub const PACKET3_DRAW_INDEX: c_uint = 0x2B;
pub const PACKET3_DRAW_INDEX_AUTO: c_uint = 0x2D;
pub const PACKET3_DRAW_INDEX_IMMD: c_uint = 0x2E;
pub const PACKET3_NUM_INSTANCES: c_uint = 0x2F;
pub const PACKET3_DRAW_INDEX_MULTI_AUTO: c_uint = 0x30;
pub const PACKET3_STRMOUT_BUFFER_UPDATE: c_uint = 0x34;
pub const PACKET3_DRAW_INDEX_OFFSET_2: c_uint = 0x35;
pub const PACKET3_DRAW_INDEX_MULTI_ELEMENT: c_uint = 0x36;
pub const PACKET3_MEM_SEMAPHORE: c_uint = 0x39;
pub const PACKET3_MPEG_INDEX: c_uint = 0x3A;
pub const PACKET3_COPY_DW: c_uint = 0x3B;
pub const PACKET3_WAIT_REG_MEM: c_uint = 0x3C;
pub const PACKET3_MEM_WRITE: c_uint = 0x3D;
pub const PACKET3_INDIRECT_BUFFER: c_uint = 0x32;
pub const PACKET3_CP_DMA: c_uint = 0x41;
// 1. header
// 2. SRC_ADDR_LO or DATA [31:0]
// 3. CP_SYNC [31] | SRC_SEL [30:29] | ENGINE [27] | DST_SEL [21:20] |
// SRC_ADDR_HI [7:0]
// 4. DST_ADDR_LO [31:0]
// 5. DST_ADDR_HI [7:0]
// 6. COMMAND [29:22] | BYTE_COUNT [20:0]
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
pub const PACKET3_EVENT_WRITE_EOP: c_uint = 0x47;
pub const PACKET3_EVENT_WRITE_EOS: c_uint = 0x48;
pub const PACKET3_PREAMBLE_CNTL: c_uint = 0x4A;

pub const PACKET3_RB_OFFSET: c_uint = 0x4B;
pub const PACKET3_ALU_PS_CONST_BUFFER_COPY: c_uint = 0x4C;
pub const PACKET3_ALU_VS_CONST_BUFFER_COPY: c_uint = 0x4D;
pub const PACKET3_ALU_PS_CONST_UPDATE: c_uint = 0x4E;
pub const PACKET3_ALU_VS_CONST_UPDATE: c_uint = 0x4F;
pub const PACKET3_ONE_REG_WRITE: c_uint = 0x57;
pub const PACKET3_SET_CONFIG_REG: c_uint = 0x68;
pub const PACKET3_SET_CONFIG_REG_START: c_uint = 0x00008000;
pub const PACKET3_SET_CONFIG_REG_END: c_uint = 0x0000ac00;
pub const PACKET3_SET_CONTEXT_REG: c_uint = 0x69;
pub const PACKET3_SET_CONTEXT_REG_START: c_uint = 0x00028000;
pub const PACKET3_SET_CONTEXT_REG_END: c_uint = 0x00029000;
pub const PACKET3_SET_ALU_CONST: c_uint = 0x6A;
// alu const buffers only; no reg file
pub const PACKET3_SET_BOOL_CONST: c_uint = 0x6B;
pub const PACKET3_SET_BOOL_CONST_START: c_uint = 0x0003a500;
pub const PACKET3_SET_BOOL_CONST_END: c_uint = 0x0003a518;
pub const PACKET3_SET_LOOP_CONST: c_uint = 0x6C;
pub const PACKET3_SET_LOOP_CONST_START: c_uint = 0x0003a200;
pub const PACKET3_SET_LOOP_CONST_END: c_uint = 0x0003a500;
pub const PACKET3_SET_RESOURCE: c_uint = 0x6D;
pub const PACKET3_SET_RESOURCE_START: c_uint = 0x00030000;
pub const PACKET3_SET_RESOURCE_END: c_uint = 0x00038000;
pub const PACKET3_SET_SAMPLER: c_uint = 0x6E;
pub const PACKET3_SET_SAMPLER_START: c_uint = 0x0003c000;
pub const PACKET3_SET_SAMPLER_END: c_uint = 0x0003c600;
pub const PACKET3_SET_CTL_CONST: c_uint = 0x6F;
pub const PACKET3_SET_CTL_CONST_START: c_uint = 0x0003cff0;
pub const PACKET3_SET_CTL_CONST_END: c_uint = 0x0003ff0c;
pub const PACKET3_SET_RESOURCE_OFFSET: c_uint = 0x70;
pub const PACKET3_SET_ALU_CONST_VS: c_uint = 0x71;
pub const PACKET3_SET_ALU_CONST_DI: c_uint = 0x72;
pub const PACKET3_SET_CONTEXT_REG_INDIRECT: c_uint = 0x73;
pub const PACKET3_SET_RESOURCE_INDIRECT: c_uint = 0x74;
pub const PACKET3_SET_APPEND_CNT: c_uint = 0x75;
// SET_APPEND_CNT - documentation
// 1. header
// 2. COMMAND
// 1:0 - SOURCE SEL
// 15:2 - Reserved
// 31:16 - WR_REG_OFFSET - context register to write source data to.
// (one of R_02872C_GDS_APPEND_COUNT_0-11)
// 3. CONTROL
// (for source == mem)
// 31:2 SRC_ADDRESS_LO
// 0:1 SWAP
// (for source == GDS)
// 31:0 GDS offset
// (for source == DATA)
// 31:0 DATA
// (for source == REG)
// 31:0 REG
// 4. SRC_ADDRESS_HI[7:0]
// kernel driver 2.44 only supports SRC == MEM.
//

// source is from the data in CONTROL
pub const PACKET3_SAC_SRC_SEL_DATA: c_uint = 0x0;
// source is from register
pub const PACKET3_SAC_SRC_SEL_REG: c_uint = 0x1;
// source is from GDS offset in CONTROL
pub const PACKET3_SAC_SRC_SEL_GDS: c_uint = 0x2;
// source is from memory address
pub const PACKET3_SAC_SRC_SEL_MEM: c_uint = 0x3;
pub const SQ_RESOURCE_CONSTANT_WORD7_0: c_uint = 0x3001c;

pub const SQ_TEX_VTX_INVALID_TEXTURE: c_uint = 0x0;
pub const SQ_TEX_VTX_INVALID_BUFFER: c_uint = 0x1;
pub const SQ_TEX_VTX_VALID_TEXTURE: c_uint = 0x2;
pub const SQ_TEX_VTX_VALID_BUFFER: c_uint = 0x3;
pub const VGT_VTX_VECT_EJECT_REG: c_uint = 0x88b0;
pub const SQ_CONST_MEM_BASE: c_uint = 0x8df8;
pub const SQ_ESGS_RING_BASE: c_uint = 0x8c40;
pub const SQ_ESGS_RING_SIZE: c_uint = 0x8c44;
pub const SQ_GSVS_RING_BASE: c_uint = 0x8c48;
pub const SQ_GSVS_RING_SIZE: c_uint = 0x8c4c;
pub const SQ_ESTMP_RING_BASE: c_uint = 0x8c50;
pub const SQ_ESTMP_RING_SIZE: c_uint = 0x8c54;
pub const SQ_GSTMP_RING_BASE: c_uint = 0x8c58;
pub const SQ_GSTMP_RING_SIZE: c_uint = 0x8c5c;
pub const SQ_VSTMP_RING_BASE: c_uint = 0x8c60;
pub const SQ_VSTMP_RING_SIZE: c_uint = 0x8c64;
pub const SQ_PSTMP_RING_BASE: c_uint = 0x8c68;
pub const SQ_PSTMP_RING_SIZE: c_uint = 0x8c6c;
pub const SQ_LSTMP_RING_BASE: c_uint = 0x8e10;
pub const SQ_LSTMP_RING_SIZE: c_uint = 0x8e14;
pub const SQ_HSTMP_RING_BASE: c_uint = 0x8e18;
pub const SQ_HSTMP_RING_SIZE: c_uint = 0x8e1c;
pub const VGT_TF_RING_SIZE: c_uint = 0x8988;
pub const SQ_ESGS_RING_ITEMSIZE: c_uint = 0x28900;
pub const SQ_GSVS_RING_ITEMSIZE: c_uint = 0x28904;
pub const SQ_ESTMP_RING_ITEMSIZE: c_uint = 0x28908;
pub const SQ_GSTMP_RING_ITEMSIZE: c_uint = 0x2890c;
pub const SQ_VSTMP_RING_ITEMSIZE: c_uint = 0x28910;
pub const SQ_PSTMP_RING_ITEMSIZE: c_uint = 0x28914;
pub const SQ_LSTMP_RING_ITEMSIZE: c_uint = 0x28830;
pub const SQ_HSTMP_RING_ITEMSIZE: c_uint = 0x28834;
pub const SQ_GS_VERT_ITEMSIZE: c_uint = 0x2891c;
pub const SQ_GS_VERT_ITEMSIZE_1: c_uint = 0x28920;
pub const SQ_GS_VERT_ITEMSIZE_2: c_uint = 0x28924;
pub const SQ_GS_VERT_ITEMSIZE_3: c_uint = 0x28928;
pub const SQ_GSVS_RING_OFFSET_1: c_uint = 0x2892c;
pub const SQ_GSVS_RING_OFFSET_2: c_uint = 0x28930;
pub const SQ_GSVS_RING_OFFSET_3: c_uint = 0x28934;
pub const SQ_ALU_CONST_BUFFER_SIZE_PS_0: c_uint = 0x28140;
pub const SQ_ALU_CONST_BUFFER_SIZE_HS_0: c_uint = 0x28f80;
pub const SQ_ALU_CONST_CACHE_PS_0: c_uint = 0x28940;
pub const SQ_ALU_CONST_CACHE_PS_1: c_uint = 0x28944;
pub const SQ_ALU_CONST_CACHE_PS_2: c_uint = 0x28948;
pub const SQ_ALU_CONST_CACHE_PS_3: c_uint = 0x2894c;
pub const SQ_ALU_CONST_CACHE_PS_4: c_uint = 0x28950;
pub const SQ_ALU_CONST_CACHE_PS_5: c_uint = 0x28954;
pub const SQ_ALU_CONST_CACHE_PS_6: c_uint = 0x28958;
pub const SQ_ALU_CONST_CACHE_PS_7: c_uint = 0x2895c;
pub const SQ_ALU_CONST_CACHE_PS_8: c_uint = 0x28960;
pub const SQ_ALU_CONST_CACHE_PS_9: c_uint = 0x28964;
pub const SQ_ALU_CONST_CACHE_PS_10: c_uint = 0x28968;
pub const SQ_ALU_CONST_CACHE_PS_11: c_uint = 0x2896c;
pub const SQ_ALU_CONST_CACHE_PS_12: c_uint = 0x28970;
pub const SQ_ALU_CONST_CACHE_PS_13: c_uint = 0x28974;
pub const SQ_ALU_CONST_CACHE_PS_14: c_uint = 0x28978;
pub const SQ_ALU_CONST_CACHE_PS_15: c_uint = 0x2897c;
pub const SQ_ALU_CONST_CACHE_VS_0: c_uint = 0x28980;
pub const SQ_ALU_CONST_CACHE_VS_1: c_uint = 0x28984;
pub const SQ_ALU_CONST_CACHE_VS_2: c_uint = 0x28988;
pub const SQ_ALU_CONST_CACHE_VS_3: c_uint = 0x2898c;
pub const SQ_ALU_CONST_CACHE_VS_4: c_uint = 0x28990;
pub const SQ_ALU_CONST_CACHE_VS_5: c_uint = 0x28994;
pub const SQ_ALU_CONST_CACHE_VS_6: c_uint = 0x28998;
pub const SQ_ALU_CONST_CACHE_VS_7: c_uint = 0x2899c;
pub const SQ_ALU_CONST_CACHE_VS_8: c_uint = 0x289a0;
pub const SQ_ALU_CONST_CACHE_VS_9: c_uint = 0x289a4;
pub const SQ_ALU_CONST_CACHE_VS_10: c_uint = 0x289a8;
pub const SQ_ALU_CONST_CACHE_VS_11: c_uint = 0x289ac;
pub const SQ_ALU_CONST_CACHE_VS_12: c_uint = 0x289b0;
pub const SQ_ALU_CONST_CACHE_VS_13: c_uint = 0x289b4;
pub const SQ_ALU_CONST_CACHE_VS_14: c_uint = 0x289b8;
pub const SQ_ALU_CONST_CACHE_VS_15: c_uint = 0x289bc;
pub const SQ_ALU_CONST_CACHE_GS_0: c_uint = 0x289c0;
pub const SQ_ALU_CONST_CACHE_GS_1: c_uint = 0x289c4;
pub const SQ_ALU_CONST_CACHE_GS_2: c_uint = 0x289c8;
pub const SQ_ALU_CONST_CACHE_GS_3: c_uint = 0x289cc;
pub const SQ_ALU_CONST_CACHE_GS_4: c_uint = 0x289d0;
pub const SQ_ALU_CONST_CACHE_GS_5: c_uint = 0x289d4;
pub const SQ_ALU_CONST_CACHE_GS_6: c_uint = 0x289d8;
pub const SQ_ALU_CONST_CACHE_GS_7: c_uint = 0x289dc;
pub const SQ_ALU_CONST_CACHE_GS_8: c_uint = 0x289e0;
pub const SQ_ALU_CONST_CACHE_GS_9: c_uint = 0x289e4;
pub const SQ_ALU_CONST_CACHE_GS_10: c_uint = 0x289e8;
pub const SQ_ALU_CONST_CACHE_GS_11: c_uint = 0x289ec;
pub const SQ_ALU_CONST_CACHE_GS_12: c_uint = 0x289f0;
pub const SQ_ALU_CONST_CACHE_GS_13: c_uint = 0x289f4;
pub const SQ_ALU_CONST_CACHE_GS_14: c_uint = 0x289f8;
pub const SQ_ALU_CONST_CACHE_GS_15: c_uint = 0x289fc;
pub const SQ_ALU_CONST_CACHE_HS_0: c_uint = 0x28f00;
pub const SQ_ALU_CONST_CACHE_HS_1: c_uint = 0x28f04;
pub const SQ_ALU_CONST_CACHE_HS_2: c_uint = 0x28f08;
pub const SQ_ALU_CONST_CACHE_HS_3: c_uint = 0x28f0c;
pub const SQ_ALU_CONST_CACHE_HS_4: c_uint = 0x28f10;
pub const SQ_ALU_CONST_CACHE_HS_5: c_uint = 0x28f14;
pub const SQ_ALU_CONST_CACHE_HS_6: c_uint = 0x28f18;
pub const SQ_ALU_CONST_CACHE_HS_7: c_uint = 0x28f1c;
pub const SQ_ALU_CONST_CACHE_HS_8: c_uint = 0x28f20;
pub const SQ_ALU_CONST_CACHE_HS_9: c_uint = 0x28f24;
pub const SQ_ALU_CONST_CACHE_HS_10: c_uint = 0x28f28;
pub const SQ_ALU_CONST_CACHE_HS_11: c_uint = 0x28f2c;
pub const SQ_ALU_CONST_CACHE_HS_12: c_uint = 0x28f30;
pub const SQ_ALU_CONST_CACHE_HS_13: c_uint = 0x28f34;
pub const SQ_ALU_CONST_CACHE_HS_14: c_uint = 0x28f38;
pub const SQ_ALU_CONST_CACHE_HS_15: c_uint = 0x28f3c;
pub const SQ_ALU_CONST_CACHE_LS_0: c_uint = 0x28f40;
pub const SQ_ALU_CONST_CACHE_LS_1: c_uint = 0x28f44;
pub const SQ_ALU_CONST_CACHE_LS_2: c_uint = 0x28f48;
pub const SQ_ALU_CONST_CACHE_LS_3: c_uint = 0x28f4c;
pub const SQ_ALU_CONST_CACHE_LS_4: c_uint = 0x28f50;
pub const SQ_ALU_CONST_CACHE_LS_5: c_uint = 0x28f54;
pub const SQ_ALU_CONST_CACHE_LS_6: c_uint = 0x28f58;
pub const SQ_ALU_CONST_CACHE_LS_7: c_uint = 0x28f5c;
pub const SQ_ALU_CONST_CACHE_LS_8: c_uint = 0x28f60;
pub const SQ_ALU_CONST_CACHE_LS_9: c_uint = 0x28f64;
pub const SQ_ALU_CONST_CACHE_LS_10: c_uint = 0x28f68;
pub const SQ_ALU_CONST_CACHE_LS_11: c_uint = 0x28f6c;
pub const SQ_ALU_CONST_CACHE_LS_12: c_uint = 0x28f70;
pub const SQ_ALU_CONST_CACHE_LS_13: c_uint = 0x28f74;
pub const SQ_ALU_CONST_CACHE_LS_14: c_uint = 0x28f78;
pub const SQ_ALU_CONST_CACHE_LS_15: c_uint = 0x28f7c;
pub const PA_SC_SCREEN_SCISSOR_TL: c_uint = 0x28030;
pub const PA_SC_GENERIC_SCISSOR_TL: c_uint = 0x28240;
pub const PA_SC_WINDOW_SCISSOR_TL: c_uint = 0x28204;
pub const VGT_PRIMITIVE_TYPE: c_uint = 0x8958;
pub const VGT_INDEX_TYPE: c_uint = 0x895C;
pub const VGT_NUM_INDICES: c_uint = 0x8970;
pub const VGT_COMPUTE_DIM_X: c_uint = 0x8990;
pub const VGT_COMPUTE_DIM_Y: c_uint = 0x8994;
pub const VGT_COMPUTE_DIM_Z: c_uint = 0x8998;
pub const VGT_COMPUTE_START_X: c_uint = 0x899C;
pub const VGT_COMPUTE_START_Y: c_uint = 0x89A0;
pub const VGT_COMPUTE_START_Z: c_uint = 0x89A4;
pub const VGT_COMPUTE_INDEX: c_uint = 0x89A8;
pub const VGT_COMPUTE_THREAD_GROUP_SIZE: c_uint = 0x89AC;
pub const VGT_HS_OFFCHIP_PARAM: c_uint = 0x89B0;
pub const DB_DEBUG: c_uint = 0x9830;
pub const DB_DEBUG2: c_uint = 0x9834;
pub const DB_DEBUG3: c_uint = 0x9838;
pub const DB_DEBUG4: c_uint = 0x983C;
pub const DB_WATERMARKS: c_uint = 0x9854;
pub const DB_DEPTH_CONTROL: c_uint = 0x28800;
pub const R_028800_DB_DEPTH_CONTROL: c_uint = 0x028800;

pub const C_028800_STENCIL_ENABLE: c_uint = 0xFFFFFFFE;

pub const C_028800_Z_ENABLE: c_uint = 0xFFFFFFFD;

pub const C_028800_Z_WRITE_ENABLE: c_uint = 0xFFFFFFFB;

pub const C_028800_ZFUNC: c_uint = 0xFFFFFF8F;

pub const C_028800_BACKFACE_ENABLE: c_uint = 0xFFFFFF7F;

pub const C_028800_STENCILFUNC: c_uint = 0xFFFFF8FF;
pub const V_028800_STENCILFUNC_NEVER: c_uint = 0x00000000;
pub const V_028800_STENCILFUNC_LESS: c_uint = 0x00000001;
pub const V_028800_STENCILFUNC_EQUAL: c_uint = 0x00000002;
pub const V_028800_STENCILFUNC_LEQUAL: c_uint = 0x00000003;
pub const V_028800_STENCILFUNC_GREATER: c_uint = 0x00000004;
pub const V_028800_STENCILFUNC_NOTEQUAL: c_uint = 0x00000005;
pub const V_028800_STENCILFUNC_GEQUAL: c_uint = 0x00000006;
pub const V_028800_STENCILFUNC_ALWAYS: c_uint = 0x00000007;

pub const C_028800_STENCILFAIL: c_uint = 0xFFFFC7FF;
pub const V_028800_STENCIL_KEEP: c_uint = 0x00000000;
pub const V_028800_STENCIL_ZERO: c_uint = 0x00000001;
pub const V_028800_STENCIL_REPLACE: c_uint = 0x00000002;
pub const V_028800_STENCIL_INCR: c_uint = 0x00000003;
pub const V_028800_STENCIL_DECR: c_uint = 0x00000004;
pub const V_028800_STENCIL_INVERT: c_uint = 0x00000005;
pub const V_028800_STENCIL_INCR_WRAP: c_uint = 0x00000006;
pub const V_028800_STENCIL_DECR_WRAP: c_uint = 0x00000007;

pub const C_028800_STENCILZPASS: c_uint = 0xFFFE3FFF;

pub const C_028800_STENCILZFAIL: c_uint = 0xFFF1FFFF;

pub const C_028800_STENCILFUNC_BF: c_uint = 0xFF8FFFFF;

pub const C_028800_STENCILFAIL_BF: c_uint = 0xFC7FFFFF;

pub const C_028800_STENCILZPASS_BF: c_uint = 0xE3FFFFFF;

pub const C_028800_STENCILZFAIL_BF: c_uint = 0x1FFFFFFF;
pub const DB_DEPTH_VIEW: c_uint = 0x28008;
pub const R_028008_DB_DEPTH_VIEW: c_uint = 0x00028008;

pub const C_028008_SLICE_START: c_uint = 0xFFFFF800;

pub const C_028008_SLICE_MAX: c_uint = 0xFF001FFF;
pub const DB_HTILE_DATA_BASE: c_uint = 0x28014;
pub const DB_HTILE_SURFACE: c_uint = 0x28abc;

pub const C_028ABC_HTILE_WIDTH: c_uint = 0xFFFFFFFE;

pub const C_028ABC_HTILE_HEIGHT: c_uint = 0xFFFFFFFD;

pub const DB_Z_INFO: c_uint = 0x28040;

pub const R_028040_DB_Z_INFO: c_uint = 0x028040;

pub const C_028040_FORMAT: c_uint = 0xFFFFFFFC;
pub const V_028040_Z_INVALID: c_uint = 0x00000000;
pub const V_028040_Z_16: c_uint = 0x00000001;
pub const V_028040_Z_24: c_uint = 0x00000002;
pub const V_028040_Z_32_FLOAT: c_uint = 0x00000003;

pub const C_028040_ARRAY_MODE: c_uint = 0xFFFFFF0F;

pub const C_028040_READ_SIZE: c_uint = 0xEFFFFFFF;

pub const C_028040_TILE_SURFACE_ENABLE: c_uint = 0xDFFFFFFF;

pub const C_028040_ZRANGE_PRECISION: c_uint = 0x7FFFFFFF;

pub const DB_STENCIL_INFO: c_uint = 0x28044;
pub const R_028044_DB_STENCIL_INFO: c_uint = 0x028044;

pub const C_028044_FORMAT: c_uint = 0xFFFFFFFE;
pub const V_028044_STENCIL_INVALID: c_int = 0;
pub const V_028044_STENCIL_8: c_int = 1;

pub const DB_Z_READ_BASE: c_uint = 0x28048;
pub const DB_STENCIL_READ_BASE: c_uint = 0x2804c;
pub const DB_Z_WRITE_BASE: c_uint = 0x28050;
pub const DB_STENCIL_WRITE_BASE: c_uint = 0x28054;
pub const DB_DEPTH_SIZE: c_uint = 0x28058;
pub const R_028058_DB_DEPTH_SIZE: c_uint = 0x028058;

pub const C_028058_PITCH_TILE_MAX: c_uint = 0xFFFFF800;

pub const C_028058_HEIGHT_TILE_MAX: c_uint = 0xFFC007FF;
pub const R_02805C_DB_DEPTH_SLICE: c_uint = 0x02805C;

pub const C_02805C_SLICE_TILE_MAX: c_uint = 0xFFC00000;
pub const SQ_PGM_START_PS: c_uint = 0x28840;
pub const SQ_PGM_START_VS: c_uint = 0x2885c;
pub const SQ_PGM_START_GS: c_uint = 0x28874;
pub const SQ_PGM_START_ES: c_uint = 0x2888c;
pub const SQ_PGM_START_FS: c_uint = 0x288a4;
pub const SQ_PGM_START_HS: c_uint = 0x288b8;
pub const SQ_PGM_START_LS: c_uint = 0x288d0;
pub const VGT_STRMOUT_BUFFER_BASE_0: c_uint = 0x28AD8;
pub const VGT_STRMOUT_BUFFER_BASE_1: c_uint = 0x28AE8;
pub const VGT_STRMOUT_BUFFER_BASE_2: c_uint = 0x28AF8;
pub const VGT_STRMOUT_BUFFER_BASE_3: c_uint = 0x28B08;
pub const VGT_STRMOUT_BUFFER_SIZE_0: c_uint = 0x28AD0;
pub const VGT_STRMOUT_BUFFER_SIZE_1: c_uint = 0x28AE0;
pub const VGT_STRMOUT_BUFFER_SIZE_2: c_uint = 0x28AF0;
pub const VGT_STRMOUT_BUFFER_SIZE_3: c_uint = 0x28B00;
pub const VGT_STRMOUT_CONFIG: c_uint = 0x28b94;
pub const VGT_STRMOUT_BUFFER_CONFIG: c_uint = 0x28b98;
pub const CB_TARGET_MASK: c_uint = 0x28238;
pub const CB_SHADER_MASK: c_uint = 0x2823c;
pub const GDS_ADDR_BASE: c_uint = 0x28720;
pub const GDS_APPEND_COUNT_0: c_uint = 0x2872C;
pub const GDS_APPEND_COUNT_1: c_uint = 0x28730;
pub const GDS_APPEND_COUNT_2: c_uint = 0x28734;
pub const GDS_APPEND_COUNT_3: c_uint = 0x28738;
pub const GDS_APPEND_COUNT_4: c_uint = 0x2873C;
pub const GDS_APPEND_COUNT_5: c_uint = 0x28740;
pub const GDS_APPEND_COUNT_6: c_uint = 0x28744;
pub const GDS_APPEND_COUNT_7: c_uint = 0x28748;
pub const GDS_APPEND_COUNT_8: c_uint = 0x2874c;
pub const GDS_APPEND_COUNT_9: c_uint = 0x28750;
pub const GDS_APPEND_COUNT_10: c_uint = 0x28754;
pub const GDS_APPEND_COUNT_11: c_uint = 0x28758;
pub const CB_IMMED0_BASE: c_uint = 0x28b9c;
pub const CB_IMMED1_BASE: c_uint = 0x28ba0;
pub const CB_IMMED2_BASE: c_uint = 0x28ba4;
pub const CB_IMMED3_BASE: c_uint = 0x28ba8;
pub const CB_IMMED4_BASE: c_uint = 0x28bac;
pub const CB_IMMED5_BASE: c_uint = 0x28bb0;
pub const CB_IMMED6_BASE: c_uint = 0x28bb4;
pub const CB_IMMED7_BASE: c_uint = 0x28bb8;
pub const CB_IMMED8_BASE: c_uint = 0x28bbc;
pub const CB_IMMED9_BASE: c_uint = 0x28bc0;
pub const CB_IMMED10_BASE: c_uint = 0x28bc4;
pub const CB_IMMED11_BASE: c_uint = 0x28bc8;
// all 12 CB blocks have these regs
pub const CB_COLOR0_BASE: c_uint = 0x28c60;
pub const CB_COLOR0_PITCH: c_uint = 0x28c64;
pub const CB_COLOR0_SLICE: c_uint = 0x28c68;
pub const CB_COLOR0_VIEW: c_uint = 0x28c6c;
pub const R_028C6C_CB_COLOR0_VIEW: c_uint = 0x00028C6C;

pub const C_028C6C_SLICE_START: c_uint = 0xFFFFF800;

pub const C_028C6C_SLICE_MAX: c_uint = 0xFF001FFF;
pub const R_028C70_CB_COLOR0_INFO: c_uint = 0x028C70;

pub const C_028C70_ENDIAN: c_uint = 0xFFFFFFFC;

pub const C_028C70_FORMAT: c_uint = 0xFFFFFF03;
pub const V_028C70_COLOR_INVALID: c_uint = 0x00000000;
pub const V_028C70_COLOR_8: c_uint = 0x00000001;
pub const V_028C70_COLOR_4_4: c_uint = 0x00000002;
pub const V_028C70_COLOR_3_3_2: c_uint = 0x00000003;
pub const V_028C70_COLOR_16: c_uint = 0x00000005;
pub const V_028C70_COLOR_16_FLOAT: c_uint = 0x00000006;
pub const V_028C70_COLOR_8_8: c_uint = 0x00000007;
pub const V_028C70_COLOR_5_6_5: c_uint = 0x00000008;
pub const V_028C70_COLOR_6_5_5: c_uint = 0x00000009;
pub const V_028C70_COLOR_1_5_5_5: c_uint = 0x0000000A;
pub const V_028C70_COLOR_4_4_4_4: c_uint = 0x0000000B;
pub const V_028C70_COLOR_5_5_5_1: c_uint = 0x0000000C;
pub const V_028C70_COLOR_32: c_uint = 0x0000000D;
pub const V_028C70_COLOR_32_FLOAT: c_uint = 0x0000000E;
pub const V_028C70_COLOR_16_16: c_uint = 0x0000000F;
pub const V_028C70_COLOR_16_16_FLOAT: c_uint = 0x00000010;
pub const V_028C70_COLOR_8_24: c_uint = 0x00000011;
pub const V_028C70_COLOR_8_24_FLOAT: c_uint = 0x00000012;
pub const V_028C70_COLOR_24_8: c_uint = 0x00000013;
pub const V_028C70_COLOR_24_8_FLOAT: c_uint = 0x00000014;
pub const V_028C70_COLOR_10_11_11: c_uint = 0x00000015;
pub const V_028C70_COLOR_10_11_11_FLOAT: c_uint = 0x00000016;
pub const V_028C70_COLOR_11_11_10: c_uint = 0x00000017;
pub const V_028C70_COLOR_11_11_10_FLOAT: c_uint = 0x00000018;
pub const V_028C70_COLOR_2_10_10_10: c_uint = 0x00000019;
pub const V_028C70_COLOR_8_8_8_8: c_uint = 0x0000001A;
pub const V_028C70_COLOR_10_10_10_2: c_uint = 0x0000001B;
pub const V_028C70_COLOR_X24_8_32_FLOAT: c_uint = 0x0000001C;
pub const V_028C70_COLOR_32_32: c_uint = 0x0000001D;
pub const V_028C70_COLOR_32_32_FLOAT: c_uint = 0x0000001E;
pub const V_028C70_COLOR_16_16_16_16: c_uint = 0x0000001F;
pub const V_028C70_COLOR_16_16_16_16_FLOAT: c_uint = 0x00000020;
pub const V_028C70_COLOR_32_32_32_32: c_uint = 0x00000022;
pub const V_028C70_COLOR_32_32_32_32_FLOAT: c_uint = 0x00000023;
pub const V_028C70_COLOR_32_32_32_FLOAT: c_uint = 0x00000030;

pub const C_028C70_ARRAY_MODE: c_uint = 0xFFFFF0FF;
pub const V_028C70_ARRAY_LINEAR_GENERAL: c_uint = 0x00000000;
pub const V_028C70_ARRAY_LINEAR_ALIGNED: c_uint = 0x00000001;
pub const V_028C70_ARRAY_1D_TILED_THIN1: c_uint = 0x00000002;
pub const V_028C70_ARRAY_2D_TILED_THIN1: c_uint = 0x00000004;

pub const C_028C70_NUMBER_TYPE: c_uint = 0xFFFF8FFF;
pub const V_028C70_NUMBER_UNORM: c_uint = 0x00000000;
pub const V_028C70_NUMBER_SNORM: c_uint = 0x00000001;
pub const V_028C70_NUMBER_USCALED: c_uint = 0x00000002;
pub const V_028C70_NUMBER_SSCALED: c_uint = 0x00000003;
pub const V_028C70_NUMBER_UINT: c_uint = 0x00000004;
pub const V_028C70_NUMBER_SINT: c_uint = 0x00000005;
pub const V_028C70_NUMBER_SRGB: c_uint = 0x00000006;
pub const V_028C70_NUMBER_FLOAT: c_uint = 0x00000007;

pub const C_028C70_COMP_SWAP: c_uint = 0xFFFE7FFF;
pub const V_028C70_SWAP_STD: c_uint = 0x00000000;
pub const V_028C70_SWAP_ALT: c_uint = 0x00000001;
pub const V_028C70_SWAP_STD_REV: c_uint = 0x00000002;
pub const V_028C70_SWAP_ALT_REV: c_uint = 0x00000003;

pub const C_028C70_FAST_CLEAR: c_uint = 0xFFFDFFFF;

pub const C_028C70_COMPRESSION: c_uint = 0xFFF3FFFF;

pub const C_028C70_BLEND_CLAMP: c_uint = 0xFFF7FFFF;

pub const C_028C70_BLEND_BYPASS: c_uint = 0xFFEFFFFF;

pub const C_028C70_SIMPLE_FLOAT: c_uint = 0xFFDFFFFF;

pub const C_028C70_ROUND_MODE: c_uint = 0xFFBFFFFF;

pub const C_028C70_TILE_COMPACT: c_uint = 0xFF7FFFFF;

pub const C_028C70_SOURCE_FORMAT: c_uint = 0xFCFFFFFF;
pub const V_028C70_EXPORT_4C_32BPC: c_uint = 0x0;
pub const V_028C70_EXPORT_4C_16BPC: c_uint = 0x1;
pub const V_028C70_EXPORT_2C_32BPC: c_uint = 0x2 /* Do not use */;

pub const C_028C70_RAT: c_uint = 0xFBFFFFFF;

pub const C_028C70_RESOURCE_TYPE: c_uint = 0xC7FFFFFF;
pub const CB_COLOR0_INFO: c_uint = 0x28c70;

pub const R_028C74_CB_COLOR0_ATTRIB: c_uint = 0x028C74;

pub const C_028C74_NON_DISP_TILING_ORDER: c_uint = 0xFFFFFFEF;

pub const CB_COLOR0_ATTRIB: c_uint = 0x28c74;

pub const CB_COLOR0_DIM: c_uint = 0x28c78;
// only CB0-7 blocks have these regs
pub const CB_COLOR0_CMASK: c_uint = 0x28c7c;
pub const CB_COLOR0_CMASK_SLICE: c_uint = 0x28c80;
pub const CB_COLOR0_FMASK: c_uint = 0x28c84;
pub const CB_COLOR0_FMASK_SLICE: c_uint = 0x28c88;
pub const CB_COLOR0_CLEAR_WORD0: c_uint = 0x28c8c;
pub const CB_COLOR0_CLEAR_WORD1: c_uint = 0x28c90;
pub const CB_COLOR0_CLEAR_WORD2: c_uint = 0x28c94;
pub const CB_COLOR0_CLEAR_WORD3: c_uint = 0x28c98;
pub const CB_COLOR1_BASE: c_uint = 0x28c9c;
pub const CB_COLOR2_BASE: c_uint = 0x28cd8;
pub const CB_COLOR3_BASE: c_uint = 0x28d14;
pub const CB_COLOR4_BASE: c_uint = 0x28d50;
pub const CB_COLOR5_BASE: c_uint = 0x28d8c;
pub const CB_COLOR6_BASE: c_uint = 0x28dc8;
pub const CB_COLOR7_BASE: c_uint = 0x28e04;
pub const CB_COLOR8_BASE: c_uint = 0x28e40;
pub const CB_COLOR9_BASE: c_uint = 0x28e5c;
pub const CB_COLOR10_BASE: c_uint = 0x28e78;
pub const CB_COLOR11_BASE: c_uint = 0x28e94;
pub const CB_COLOR1_PITCH: c_uint = 0x28ca0;
pub const CB_COLOR2_PITCH: c_uint = 0x28cdc;
pub const CB_COLOR3_PITCH: c_uint = 0x28d18;
pub const CB_COLOR4_PITCH: c_uint = 0x28d54;
pub const CB_COLOR5_PITCH: c_uint = 0x28d90;
pub const CB_COLOR6_PITCH: c_uint = 0x28dcc;
pub const CB_COLOR7_PITCH: c_uint = 0x28e08;
pub const CB_COLOR8_PITCH: c_uint = 0x28e44;
pub const CB_COLOR9_PITCH: c_uint = 0x28e60;
pub const CB_COLOR10_PITCH: c_uint = 0x28e7c;
pub const CB_COLOR11_PITCH: c_uint = 0x28e98;
pub const CB_COLOR1_SLICE: c_uint = 0x28ca4;
pub const CB_COLOR2_SLICE: c_uint = 0x28ce0;
pub const CB_COLOR3_SLICE: c_uint = 0x28d1c;
pub const CB_COLOR4_SLICE: c_uint = 0x28d58;
pub const CB_COLOR5_SLICE: c_uint = 0x28d94;
pub const CB_COLOR6_SLICE: c_uint = 0x28dd0;
pub const CB_COLOR7_SLICE: c_uint = 0x28e0c;
pub const CB_COLOR8_SLICE: c_uint = 0x28e48;
pub const CB_COLOR9_SLICE: c_uint = 0x28e64;
pub const CB_COLOR10_SLICE: c_uint = 0x28e80;
pub const CB_COLOR11_SLICE: c_uint = 0x28e9c;
pub const CB_COLOR1_VIEW: c_uint = 0x28ca8;
pub const CB_COLOR2_VIEW: c_uint = 0x28ce4;
pub const CB_COLOR3_VIEW: c_uint = 0x28d20;
pub const CB_COLOR4_VIEW: c_uint = 0x28d5c;
pub const CB_COLOR5_VIEW: c_uint = 0x28d98;
pub const CB_COLOR6_VIEW: c_uint = 0x28dd4;
pub const CB_COLOR7_VIEW: c_uint = 0x28e10;
pub const CB_COLOR8_VIEW: c_uint = 0x28e4c;
pub const CB_COLOR9_VIEW: c_uint = 0x28e68;
pub const CB_COLOR10_VIEW: c_uint = 0x28e84;
pub const CB_COLOR11_VIEW: c_uint = 0x28ea0;
pub const CB_COLOR1_INFO: c_uint = 0x28cac;
pub const CB_COLOR2_INFO: c_uint = 0x28ce8;
pub const CB_COLOR3_INFO: c_uint = 0x28d24;
pub const CB_COLOR4_INFO: c_uint = 0x28d60;
pub const CB_COLOR5_INFO: c_uint = 0x28d9c;
pub const CB_COLOR6_INFO: c_uint = 0x28dd8;
pub const CB_COLOR7_INFO: c_uint = 0x28e14;
pub const CB_COLOR8_INFO: c_uint = 0x28e50;
pub const CB_COLOR9_INFO: c_uint = 0x28e6c;
pub const CB_COLOR10_INFO: c_uint = 0x28e88;
pub const CB_COLOR11_INFO: c_uint = 0x28ea4;
pub const CB_COLOR1_ATTRIB: c_uint = 0x28cb0;
pub const CB_COLOR2_ATTRIB: c_uint = 0x28cec;
pub const CB_COLOR3_ATTRIB: c_uint = 0x28d28;
pub const CB_COLOR4_ATTRIB: c_uint = 0x28d64;
pub const CB_COLOR5_ATTRIB: c_uint = 0x28da0;
pub const CB_COLOR6_ATTRIB: c_uint = 0x28ddc;
pub const CB_COLOR7_ATTRIB: c_uint = 0x28e18;
pub const CB_COLOR8_ATTRIB: c_uint = 0x28e54;
pub const CB_COLOR9_ATTRIB: c_uint = 0x28e70;
pub const CB_COLOR10_ATTRIB: c_uint = 0x28e8c;
pub const CB_COLOR11_ATTRIB: c_uint = 0x28ea8;
pub const CB_COLOR1_DIM: c_uint = 0x28cb4;
pub const CB_COLOR2_DIM: c_uint = 0x28cf0;
pub const CB_COLOR3_DIM: c_uint = 0x28d2c;
pub const CB_COLOR4_DIM: c_uint = 0x28d68;
pub const CB_COLOR5_DIM: c_uint = 0x28da4;
pub const CB_COLOR6_DIM: c_uint = 0x28de0;
pub const CB_COLOR7_DIM: c_uint = 0x28e1c;
pub const CB_COLOR8_DIM: c_uint = 0x28e58;
pub const CB_COLOR9_DIM: c_uint = 0x28e74;
pub const CB_COLOR10_DIM: c_uint = 0x28e90;
pub const CB_COLOR11_DIM: c_uint = 0x28eac;
pub const CB_COLOR1_CMASK: c_uint = 0x28cb8;
pub const CB_COLOR2_CMASK: c_uint = 0x28cf4;
pub const CB_COLOR3_CMASK: c_uint = 0x28d30;
pub const CB_COLOR4_CMASK: c_uint = 0x28d6c;
pub const CB_COLOR5_CMASK: c_uint = 0x28da8;
pub const CB_COLOR6_CMASK: c_uint = 0x28de4;
pub const CB_COLOR7_CMASK: c_uint = 0x28e20;
pub const CB_COLOR1_CMASK_SLICE: c_uint = 0x28cbc;
pub const CB_COLOR2_CMASK_SLICE: c_uint = 0x28cf8;
pub const CB_COLOR3_CMASK_SLICE: c_uint = 0x28d34;
pub const CB_COLOR4_CMASK_SLICE: c_uint = 0x28d70;
pub const CB_COLOR5_CMASK_SLICE: c_uint = 0x28dac;
pub const CB_COLOR6_CMASK_SLICE: c_uint = 0x28de8;
pub const CB_COLOR7_CMASK_SLICE: c_uint = 0x28e24;
pub const CB_COLOR1_FMASK: c_uint = 0x28cc0;
pub const CB_COLOR2_FMASK: c_uint = 0x28cfc;
pub const CB_COLOR3_FMASK: c_uint = 0x28d38;
pub const CB_COLOR4_FMASK: c_uint = 0x28d74;
pub const CB_COLOR5_FMASK: c_uint = 0x28db0;
pub const CB_COLOR6_FMASK: c_uint = 0x28dec;
pub const CB_COLOR7_FMASK: c_uint = 0x28e28;
pub const CB_COLOR1_FMASK_SLICE: c_uint = 0x28cc4;
pub const CB_COLOR2_FMASK_SLICE: c_uint = 0x28d00;
pub const CB_COLOR3_FMASK_SLICE: c_uint = 0x28d3c;
pub const CB_COLOR4_FMASK_SLICE: c_uint = 0x28d78;
pub const CB_COLOR5_FMASK_SLICE: c_uint = 0x28db4;
pub const CB_COLOR6_FMASK_SLICE: c_uint = 0x28df0;
pub const CB_COLOR7_FMASK_SLICE: c_uint = 0x28e2c;
pub const CB_COLOR1_CLEAR_WORD0: c_uint = 0x28cc8;
pub const CB_COLOR2_CLEAR_WORD0: c_uint = 0x28d04;
pub const CB_COLOR3_CLEAR_WORD0: c_uint = 0x28d40;
pub const CB_COLOR4_CLEAR_WORD0: c_uint = 0x28d7c;
pub const CB_COLOR5_CLEAR_WORD0: c_uint = 0x28db8;
pub const CB_COLOR6_CLEAR_WORD0: c_uint = 0x28df4;
pub const CB_COLOR7_CLEAR_WORD0: c_uint = 0x28e30;
pub const CB_COLOR1_CLEAR_WORD1: c_uint = 0x28ccc;
pub const CB_COLOR2_CLEAR_WORD1: c_uint = 0x28d08;
pub const CB_COLOR3_CLEAR_WORD1: c_uint = 0x28d44;
pub const CB_COLOR4_CLEAR_WORD1: c_uint = 0x28d80;
pub const CB_COLOR5_CLEAR_WORD1: c_uint = 0x28dbc;
pub const CB_COLOR6_CLEAR_WORD1: c_uint = 0x28df8;
pub const CB_COLOR7_CLEAR_WORD1: c_uint = 0x28e34;
pub const CB_COLOR1_CLEAR_WORD2: c_uint = 0x28cd0;
pub const CB_COLOR2_CLEAR_WORD2: c_uint = 0x28d0c;
pub const CB_COLOR3_CLEAR_WORD2: c_uint = 0x28d48;
pub const CB_COLOR4_CLEAR_WORD2: c_uint = 0x28d84;
pub const CB_COLOR5_CLEAR_WORD2: c_uint = 0x28dc0;
pub const CB_COLOR6_CLEAR_WORD2: c_uint = 0x28dfc;
pub const CB_COLOR7_CLEAR_WORD2: c_uint = 0x28e38;
pub const CB_COLOR1_CLEAR_WORD3: c_uint = 0x28cd4;
pub const CB_COLOR2_CLEAR_WORD3: c_uint = 0x28d10;
pub const CB_COLOR3_CLEAR_WORD3: c_uint = 0x28d4c;
pub const CB_COLOR4_CLEAR_WORD3: c_uint = 0x28d88;
pub const CB_COLOR5_CLEAR_WORD3: c_uint = 0x28dc4;
pub const CB_COLOR6_CLEAR_WORD3: c_uint = 0x28e00;
pub const CB_COLOR7_CLEAR_WORD3: c_uint = 0x28e3c;
pub const SQ_TEX_RESOURCE_WORD0_0: c_uint = 0x30000;

pub const SQ_TEX_RESOURCE_WORD1_0: c_uint = 0x30004;

pub const SQ_TEX_RESOURCE_WORD2_0: c_uint = 0x30008;
pub const SQ_TEX_RESOURCE_WORD3_0: c_uint = 0x3000C;
pub const SQ_TEX_RESOURCE_WORD4_0: c_uint = 0x30010;

pub const SQ_TEX_RESOURCE_WORD5_0: c_uint = 0x30014;
pub const SQ_TEX_RESOURCE_WORD6_0: c_uint = 0x30018;

pub const SQ_TEX_RESOURCE_WORD7_0: c_uint = 0x3001c;

pub const R_030000_SQ_TEX_RESOURCE_WORD0_0: c_uint = 0x030000;

pub const C_030000_DIM: c_uint = 0xFFFFFFF8;
pub const V_030000_SQ_TEX_DIM_1D: c_uint = 0x00000000;
pub const V_030000_SQ_TEX_DIM_2D: c_uint = 0x00000001;
pub const V_030000_SQ_TEX_DIM_3D: c_uint = 0x00000002;
pub const V_030000_SQ_TEX_DIM_CUBEMAP: c_uint = 0x00000003;
pub const V_030000_SQ_TEX_DIM_1D_ARRAY: c_uint = 0x00000004;
pub const V_030000_SQ_TEX_DIM_2D_ARRAY: c_uint = 0x00000005;
pub const V_030000_SQ_TEX_DIM_2D_MSAA: c_uint = 0x00000006;
pub const V_030000_SQ_TEX_DIM_2D_ARRAY_MSAA: c_uint = 0x00000007;

pub const C_030000_NON_DISP_TILING_ORDER: c_uint = 0xFFFFFFDF;

pub const C_030000_PITCH: c_uint = 0xFFFC003F;

pub const C_030000_TEX_WIDTH: c_uint = 0x0003FFFF;
pub const R_030004_SQ_TEX_RESOURCE_WORD1_0: c_uint = 0x030004;

pub const C_030004_TEX_HEIGHT: c_uint = 0xFFFFC000;

pub const C_030004_TEX_DEPTH: c_uint = 0xF8003FFF;

pub const C_030004_ARRAY_MODE: c_uint = 0x0FFFFFFF;
pub const R_030008_SQ_TEX_RESOURCE_WORD2_0: c_uint = 0x030008;

pub const C_030008_BASE_ADDRESS: c_uint = 0x00000000;
pub const R_03000C_SQ_TEX_RESOURCE_WORD3_0: c_uint = 0x03000C;

pub const C_03000C_MIP_ADDRESS: c_uint = 0x00000000;
pub const R_030010_SQ_TEX_RESOURCE_WORD4_0: c_uint = 0x030010;

pub const C_030010_FORMAT_COMP_X: c_uint = 0xFFFFFFFC;
pub const V_030010_SQ_FORMAT_COMP_UNSIGNED: c_uint = 0x00000000;
pub const V_030010_SQ_FORMAT_COMP_SIGNED: c_uint = 0x00000001;
pub const V_030010_SQ_FORMAT_COMP_UNSIGNED_BIASED: c_uint = 0x00000002;

pub const C_030010_FORMAT_COMP_Y: c_uint = 0xFFFFFFF3;

pub const C_030010_FORMAT_COMP_Z: c_uint = 0xFFFFFFCF;

pub const C_030010_FORMAT_COMP_W: c_uint = 0xFFFFFF3F;

pub const C_030010_NUM_FORMAT_ALL: c_uint = 0xFFFFFCFF;
pub const V_030010_SQ_NUM_FORMAT_NORM: c_uint = 0x00000000;
pub const V_030010_SQ_NUM_FORMAT_INT: c_uint = 0x00000001;
pub const V_030010_SQ_NUM_FORMAT_SCALED: c_uint = 0x00000002;

pub const C_030010_SRF_MODE_ALL: c_uint = 0xFFFFFBFF;
pub const V_030010_SRF_MODE_ZERO_CLAMP_MINUS_ONE: c_uint = 0x00000000;
pub const V_030010_SRF_MODE_NO_ZERO: c_uint = 0x00000001;

pub const C_030010_FORCE_DEGAMMA: c_uint = 0xFFFFF7FF;

pub const C_030010_ENDIAN_SWAP: c_uint = 0xFFFFCFFF;

pub const C_030010_DST_SEL_X: c_uint = 0xFFF8FFFF;
pub const V_030010_SQ_SEL_X: c_uint = 0x00000000;
pub const V_030010_SQ_SEL_Y: c_uint = 0x00000001;
pub const V_030010_SQ_SEL_Z: c_uint = 0x00000002;
pub const V_030010_SQ_SEL_W: c_uint = 0x00000003;
pub const V_030010_SQ_SEL_0: c_uint = 0x00000004;
pub const V_030010_SQ_SEL_1: c_uint = 0x00000005;

pub const C_030010_DST_SEL_Y: c_uint = 0xFFC7FFFF;

pub const C_030010_DST_SEL_Z: c_uint = 0xFE3FFFFF;

pub const C_030010_DST_SEL_W: c_uint = 0xF1FFFFFF;

pub const C_030010_BASE_LEVEL: c_uint = 0x0FFFFFFF;
pub const R_030014_SQ_TEX_RESOURCE_WORD5_0: c_uint = 0x030014;

pub const C_030014_LAST_LEVEL: c_uint = 0xFFFFFFF0;

pub const C_030014_BASE_ARRAY: c_uint = 0xFFFE000F;

pub const C_030014_LAST_ARRAY: c_uint = 0xC001FFFF;
pub const R_030018_SQ_TEX_RESOURCE_WORD6_0: c_uint = 0x030018;

pub const C_030018_MAX_ANISO: c_uint = 0xFFFFFFF8;

pub const C_030018_PERF_MODULATION: c_uint = 0xFFFFFFC7;

pub const C_030018_INTERLACED: c_uint = 0xFFFFFFBF;

pub const R_03001C_SQ_TEX_RESOURCE_WORD7_0: c_uint = 0x03001C;

pub const C_03001C_TYPE: c_uint = 0x3FFFFFFF;
pub const V_03001C_SQ_TEX_VTX_INVALID_TEXTURE: c_uint = 0x00000000;
pub const V_03001C_SQ_TEX_VTX_INVALID_BUFFER: c_uint = 0x00000001;
pub const V_03001C_SQ_TEX_VTX_VALID_TEXTURE: c_uint = 0x00000002;
pub const V_03001C_SQ_TEX_VTX_VALID_BUFFER: c_uint = 0x00000003;

pub const C_03001C_DATA_FORMAT: c_uint = 0xFFFFFFC0;
pub const SQ_VTX_CONSTANT_WORD0_0: c_uint = 0x30000;
pub const SQ_VTX_CONSTANT_WORD1_0: c_uint = 0x30004;
pub const SQ_VTX_CONSTANT_WORD2_0: c_uint = 0x30008;

pub const SQ_VTX_CONSTANT_WORD3_0: c_uint = 0x3000C;

pub const SQ_VTX_CONSTANT_WORD4_0: c_uint = 0x30010;
pub const SQ_VTX_CONSTANT_WORD5_0: c_uint = 0x30014;
pub const SQ_VTX_CONSTANT_WORD6_0: c_uint = 0x30018;
pub const SQ_VTX_CONSTANT_WORD7_0: c_uint = 0x3001c;
pub const TD_PS_BORDER_COLOR_INDEX: c_uint = 0xA400;
pub const TD_PS_BORDER_COLOR_RED: c_uint = 0xA404;
pub const TD_PS_BORDER_COLOR_GREEN: c_uint = 0xA408;
pub const TD_PS_BORDER_COLOR_BLUE: c_uint = 0xA40C;
pub const TD_PS_BORDER_COLOR_ALPHA: c_uint = 0xA410;
pub const TD_VS_BORDER_COLOR_INDEX: c_uint = 0xA414;
pub const TD_VS_BORDER_COLOR_RED: c_uint = 0xA418;
pub const TD_VS_BORDER_COLOR_GREEN: c_uint = 0xA41C;
pub const TD_VS_BORDER_COLOR_BLUE: c_uint = 0xA420;
pub const TD_VS_BORDER_COLOR_ALPHA: c_uint = 0xA424;
pub const TD_GS_BORDER_COLOR_INDEX: c_uint = 0xA428;
pub const TD_GS_BORDER_COLOR_RED: c_uint = 0xA42C;
pub const TD_GS_BORDER_COLOR_GREEN: c_uint = 0xA430;
pub const TD_GS_BORDER_COLOR_BLUE: c_uint = 0xA434;
pub const TD_GS_BORDER_COLOR_ALPHA: c_uint = 0xA438;
pub const TD_HS_BORDER_COLOR_INDEX: c_uint = 0xA43C;
pub const TD_HS_BORDER_COLOR_RED: c_uint = 0xA440;
pub const TD_HS_BORDER_COLOR_GREEN: c_uint = 0xA444;
pub const TD_HS_BORDER_COLOR_BLUE: c_uint = 0xA448;
pub const TD_HS_BORDER_COLOR_ALPHA: c_uint = 0xA44C;
pub const TD_LS_BORDER_COLOR_INDEX: c_uint = 0xA450;
pub const TD_LS_BORDER_COLOR_RED: c_uint = 0xA454;
pub const TD_LS_BORDER_COLOR_GREEN: c_uint = 0xA458;
pub const TD_LS_BORDER_COLOR_BLUE: c_uint = 0xA45C;
pub const TD_LS_BORDER_COLOR_ALPHA: c_uint = 0xA460;
pub const TD_CS_BORDER_COLOR_INDEX: c_uint = 0xA464;
pub const TD_CS_BORDER_COLOR_RED: c_uint = 0xA468;
pub const TD_CS_BORDER_COLOR_GREEN: c_uint = 0xA46C;
pub const TD_CS_BORDER_COLOR_BLUE: c_uint = 0xA470;
pub const TD_CS_BORDER_COLOR_ALPHA: c_uint = 0xA474;
// cayman 3D regs
pub const CAYMAN_VGT_OFFCHIP_LDS_BASE: c_uint = 0x89B4;
pub const CAYMAN_SQ_EX_ALLOC_TABLE_SLOTS: c_uint = 0x8E48;
pub const CAYMAN_DB_EQAA: c_uint = 0x28804;
pub const CAYMAN_DB_DEPTH_INFO: c_uint = 0x2803C;
pub const CAYMAN_PA_SC_AA_CONFIG: c_uint = 0x28BE0;
pub const CAYMAN_MSAA_NUM_SAMPLES_SHIFT: c_int = 0;
pub const CAYMAN_MSAA_NUM_SAMPLES_MASK: c_uint = 0x7;
pub const CAYMAN_SX_SCATTER_EXPORT_BASE: c_uint = 0x28358;
// cayman packet3 addition
pub const CAYMAN_PACKET3_DEALLOC_STATE: c_uint = 0x14;
// DMA regs common on r6xx/r7xx/evergreen/ni
pub const DMA_RB_CNTL: c_uint = 0xd000;

pub const DMA_STATUS_REG: c_uint = 0xd034;

