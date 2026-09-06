//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/r600d.h
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
pub const CP_PACKET2: c_uint = 0x80000000;
pub const PACKET2_PAD_SHIFT: c_int = 0;

pub const R6XX_MAX_SH_GPRS: c_int = 256;
pub const R6XX_MAX_TEMP_GPRS: c_int = 16;
pub const R6XX_MAX_SH_THREADS: c_int = 256;
pub const R6XX_MAX_SH_STACK_ENTRIES: c_int = 4096;
pub const R6XX_MAX_BACKENDS: c_int = 8;
pub const R6XX_MAX_BACKENDS_MASK: c_uint = 0xff;
pub const R6XX_MAX_SIMDS: c_int = 8;
pub const R6XX_MAX_SIMDS_MASK: c_uint = 0xff;
pub const R6XX_MAX_PIPES: c_int = 8;
pub const R6XX_MAX_PIPES_MASK: c_uint = 0xff;
// tiling bits
pub const ARRAY_LINEAR_GENERAL: c_uint = 0x00000000;
pub const ARRAY_LINEAR_ALIGNED: c_uint = 0x00000001;
pub const ARRAY_1D_TILED_THIN1: c_uint = 0x00000002;
pub const ARRAY_2D_TILED_THIN1: c_uint = 0x00000004;
// Registers
pub const ARB_POP: c_uint = 0x2418;

pub const ARB_GDEC_RD_CNTL: c_uint = 0x246C;
pub const CC_GC_SHADER_PIPE_CONFIG: c_uint = 0x8950;
pub const CC_RB_BACKEND_DISABLE: c_uint = 0x98F4;

pub const R_028808_CB_COLOR_CONTROL: c_uint = 0x28808;

pub const C_028808_SPECIAL_OP: c_uint = 0xFFFFFF8F;
pub const V_028808_SPECIAL_NORMAL: c_uint = 0x00;
pub const V_028808_SPECIAL_DISABLE: c_uint = 0x01;
pub const V_028808_SPECIAL_RESOLVE_BOX: c_uint = 0x07;
pub const CB_COLOR0_BASE: c_uint = 0x28040;
pub const CB_COLOR1_BASE: c_uint = 0x28044;
pub const CB_COLOR2_BASE: c_uint = 0x28048;
pub const CB_COLOR3_BASE: c_uint = 0x2804C;
pub const CB_COLOR4_BASE: c_uint = 0x28050;
pub const CB_COLOR5_BASE: c_uint = 0x28054;
pub const CB_COLOR6_BASE: c_uint = 0x28058;
pub const CB_COLOR7_BASE: c_uint = 0x2805C;
pub const CB_COLOR7_FRAG: c_uint = 0x280FC;
pub const CB_COLOR0_SIZE: c_uint = 0x28060;
pub const CB_COLOR0_VIEW: c_uint = 0x28080;
pub const R_028080_CB_COLOR0_VIEW: c_uint = 0x028080;

pub const C_028080_SLICE_START: c_uint = 0xFFFFF800;

pub const C_028080_SLICE_MAX: c_uint = 0xFF001FFF;
pub const R_028084_CB_COLOR1_VIEW: c_uint = 0x028084;
pub const R_028088_CB_COLOR2_VIEW: c_uint = 0x028088;
pub const R_02808C_CB_COLOR3_VIEW: c_uint = 0x02808C;
pub const R_028090_CB_COLOR4_VIEW: c_uint = 0x028090;
pub const R_028094_CB_COLOR5_VIEW: c_uint = 0x028094;
pub const R_028098_CB_COLOR6_VIEW: c_uint = 0x028098;
pub const R_02809C_CB_COLOR7_VIEW: c_uint = 0x02809C;
pub const R_028100_CB_COLOR0_MASK: c_uint = 0x028100;

pub const C_028100_CMASK_BLOCK_MAX: c_uint = 0xFFFFF000;

pub const C_028100_FMASK_TILE_MAX: c_uint = 0x00000FFF;
pub const R_028104_CB_COLOR1_MASK: c_uint = 0x028104;
pub const R_028108_CB_COLOR2_MASK: c_uint = 0x028108;
pub const R_02810C_CB_COLOR3_MASK: c_uint = 0x02810C;
pub const R_028110_CB_COLOR4_MASK: c_uint = 0x028110;
pub const R_028114_CB_COLOR5_MASK: c_uint = 0x028114;
pub const R_028118_CB_COLOR6_MASK: c_uint = 0x028118;
pub const R_02811C_CB_COLOR7_MASK: c_uint = 0x02811C;
pub const CB_COLOR0_INFO: c_uint = 0x280a0;

pub const CB_COLOR0_TILE: c_uint = 0x280c0;
pub const CB_COLOR0_FRAG: c_uint = 0x280e0;
pub const CB_COLOR0_MASK: c_uint = 0x28100;
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
pub const CONFIG_MEMSIZE: c_uint = 0x5428;
pub const CONFIG_CNTL: c_uint = 0x5424;
pub const CP_STALLED_STAT1: c_uint = 0x8674;
pub const CP_STALLED_STAT2: c_uint = 0x8678;
pub const CP_BUSY_STAT: c_uint = 0x867C;
pub const CP_STAT: c_uint = 0x8680;
pub const CP_COHER_BASE: c_uint = 0x85F8;
pub const CP_DEBUG: c_uint = 0xC1FC;
pub const R_0086D8_CP_ME_CNTL: c_uint = 0x86D8;

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
pub const CP_ROQ_IB1_STAT: c_uint = 0x8784;
pub const CP_ROQ_IB2_STAT: c_uint = 0x8788;
pub const CP_SEM_WAIT_TIMER: c_uint = 0x85BC;
pub const DB_DEBUG: c_uint = 0x9830;

pub const DB_DEPTH_BASE: c_uint = 0x2800C;
pub const DB_HTILE_DATA_BASE: c_uint = 0x28014;
pub const DB_HTILE_SURFACE: c_uint = 0x28D24;

pub const C_028D24_HTILE_WIDTH: c_uint = 0xFFFFFFFE;

pub const C_028D24_HTILE_HEIGHT: c_uint = 0xFFFFFFFD;

pub const DB_WATERMARKS: c_uint = 0x9838;

pub const DCP_TILING_CONFIG: c_uint = 0x6CA0;

pub const GB_TILING_CONFIG: c_uint = 0x98F0;
pub const PIPE_TILING__SHIFT: c_int = 1;
pub const PIPE_TILING__MASK: c_uint = 0x0000000e;
pub const GC_USER_SHADER_PIPE_CONFIG: c_uint = 0x8954;

pub const INACTIVE_QD_PIPES_MASK: c_uint = 0x0000FF00;

pub const INACTIVE_SIMDS_MASK: c_uint = 0x00FF0000;
pub const SQ_CONFIG: c_uint = 0x8c00;

pub const SQ_GPR_RESOURCE_MGMT_1: c_uint = 0x8c04;

pub const SQ_GPR_RESOURCE_MGMT_2: c_uint = 0x8c08;

pub const SQ_THREAD_RESOURCE_MGMT: c_uint = 0x8c0c;

pub const SQ_STACK_RESOURCE_MGMT_1: c_uint = 0x8c10;

pub const SQ_STACK_RESOURCE_MGMT_2: c_uint = 0x8c14;

pub const SQ_ESGS_RING_BASE: c_uint = 0x8c40;
pub const SQ_GSVS_RING_BASE: c_uint = 0x8c48;
pub const SQ_ESTMP_RING_BASE: c_uint = 0x8c50;
pub const SQ_GSTMP_RING_BASE: c_uint = 0x8c58;
pub const SQ_VSTMP_RING_BASE: c_uint = 0x8c60;
pub const SQ_PSTMP_RING_BASE: c_uint = 0x8c68;
pub const SQ_FBUF_RING_BASE: c_uint = 0x8c70;
pub const SQ_REDUC_RING_BASE: c_uint = 0x8c78;
pub const GRBM_CNTL: c_uint = 0x8000;

pub const GRBM_STATUS: c_uint = 0x8010;
pub const CMDFIFO_AVAIL_MASK: c_uint = 0x0000001F;

pub const GRBM_STATUS2: c_uint = 0x8014;
pub const GRBM_SOFT_RESET: c_uint = 0x8020;

pub const CG_THERMAL_CTRL: c_uint = 0x7F0;

pub const DIG_THERM_DPM_MASK: c_uint = 0x000FF000;
pub const DIG_THERM_DPM_SHIFT: c_int = 12;
pub const CG_THERMAL_STATUS: c_uint = 0x7F4;

pub const ASIC_T_MASK: c_uint = 0x1FF;
pub const ASIC_T_SHIFT: c_int = 0;
pub const CG_THERMAL_INT: c_uint = 0x7F8;

pub const DIG_THERM_INTH_MASK: c_uint = 0x0000FF00;
pub const DIG_THERM_INTH_SHIFT: c_int = 8;

pub const DIG_THERM_INTL_MASK: c_uint = 0x00FF0000;
pub const DIG_THERM_INTL_SHIFT: c_int = 16;

pub const RV770_CG_THERMAL_INT: c_uint = 0x734;
pub const HDP_HOST_PATH_CNTL: c_uint = 0x2C00;
pub const HDP_NONSURFACE_BASE: c_uint = 0x2C04;
pub const HDP_NONSURFACE_INFO: c_uint = 0x2C08;
pub const HDP_NONSURFACE_SIZE: c_uint = 0x2C0C;
pub const HDP_REG_COHERENCY_FLUSH_CNTL: c_uint = 0x54A0;
pub const HDP_TILING_CONFIG: c_uint = 0x2F3C;
pub const HDP_DEBUG1: c_uint = 0x2F34;
pub const MC_CONFIG: c_uint = 0x2000;
pub const MC_VM_AGP_TOP: c_uint = 0x2184;
pub const MC_VM_AGP_BOT: c_uint = 0x2188;
pub const MC_VM_AGP_BASE: c_uint = 0x218C;
pub const MC_VM_FB_LOCATION: c_uint = 0x2180;
pub const MC_VM_L1_TLB_MCB_RD_UVD_CNTL: c_uint = 0x2124;

pub const SYSTEM_ACCESS_MODE_MASK: c_uint = 0x000000C0;
pub const SYSTEM_ACCESS_MODE_SHIFT: c_int = 6;

pub const EFFECTIVE_L1_TLB_SIZE_MASK: c_uint = 0x00007000;
pub const EFFECTIVE_L1_TLB_SIZE_SHIFT: c_int = 12;

pub const EFFECTIVE_L1_QUEUE_SIZE_MASK: c_uint = 0x00038000;
pub const EFFECTIVE_L1_QUEUE_SIZE_SHIFT: c_int = 15;
pub const MC_VM_L1_TLB_MCD_RD_A_CNTL: c_uint = 0x219C;
pub const MC_VM_L1_TLB_MCD_RD_B_CNTL: c_uint = 0x21A0;
pub const MC_VM_L1_TLB_MCB_RD_GFX_CNTL: c_uint = 0x21FC;
pub const MC_VM_L1_TLB_MCB_RD_HDP_CNTL: c_uint = 0x2204;
pub const MC_VM_L1_TLB_MCB_RD_PDMA_CNTL: c_uint = 0x2208;
pub const MC_VM_L1_TLB_MCB_RD_SEM_CNTL: c_uint = 0x220C;
pub const MC_VM_L1_TLB_MCB_RD_SYS_CNTL: c_uint = 0x2200;
pub const MC_VM_L1_TLB_MCB_WR_UVD_CNTL: c_uint = 0x212c;
pub const MC_VM_L1_TLB_MCD_WR_A_CNTL: c_uint = 0x21A4;
pub const MC_VM_L1_TLB_MCD_WR_B_CNTL: c_uint = 0x21A8;
pub const MC_VM_L1_TLB_MCB_WR_GFX_CNTL: c_uint = 0x2210;
pub const MC_VM_L1_TLB_MCB_WR_HDP_CNTL: c_uint = 0x2218;
pub const MC_VM_L1_TLB_MCB_WR_PDMA_CNTL: c_uint = 0x221C;
pub const MC_VM_L1_TLB_MCB_WR_SEM_CNTL: c_uint = 0x2220;
pub const MC_VM_L1_TLB_MCB_WR_SYS_CNTL: c_uint = 0x2214;
pub const MC_VM_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x2190;
pub const LOGICAL_PAGE_NUMBER_MASK: c_uint = 0x000FFFFF;
pub const LOGICAL_PAGE_NUMBER_SHIFT: c_int = 0;
pub const MC_VM_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x2194;
pub const MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR: c_uint = 0x2198;
pub const RS_DQ_RD_RET_CONF: c_uint = 0x2348;
pub const PA_CL_ENHANCE: c_uint = 0x8A14;

pub const PA_SC_AA_CONFIG: c_uint = 0x28C04;
pub const PA_SC_AA_SAMPLE_LOCS_2S: c_uint = 0x8B40;
pub const PA_SC_AA_SAMPLE_LOCS_4S: c_uint = 0x8B44;
pub const PA_SC_AA_SAMPLE_LOCS_8S_WD0: c_uint = 0x8B48;
pub const PA_SC_AA_SAMPLE_LOCS_8S_WD1: c_uint = 0x8B4C;

pub const PA_SC_CLIPRECT_RULE: c_uint = 0x2820c;
pub const PA_SC_ENHANCE: c_uint = 0x8BF0;

pub const PA_SC_LINE_STIPPLE: c_uint = 0x28A0C;
pub const PA_SC_LINE_STIPPLE_STATE: c_uint = 0x8B10;
pub const PA_SC_MODE_CNTL: c_uint = 0x28A4C;
pub const PA_SC_MULTI_CHIP_CNTL: c_uint = 0x8B20;
pub const PA_SC_SCREEN_SCISSOR_TL: c_uint = 0x28030;
pub const PA_SC_GENERIC_SCISSOR_TL: c_uint = 0x28240;
pub const PA_SC_WINDOW_SCISSOR_TL: c_uint = 0x28204;
pub const PCIE_PORT_INDEX: c_uint = 0x0038;
pub const PCIE_PORT_DATA: c_uint = 0x003C;
pub const CHMAP: c_uint = 0x2004;
pub const NOOFCHAN_SHIFT: c_int = 12;
pub const NOOFCHAN_MASK: c_uint = 0x00003000;
pub const RAMCFG: c_uint = 0x2408;
pub const NOOFBANK_SHIFT: c_int = 0;
pub const NOOFBANK_MASK: c_uint = 0x00000001;
pub const NOOFRANK_SHIFT: c_int = 1;
pub const NOOFRANK_MASK: c_uint = 0x00000002;
pub const NOOFROWS_SHIFT: c_int = 2;
pub const NOOFROWS_MASK: c_uint = 0x0000001C;
pub const NOOFCOLS_SHIFT: c_int = 5;
pub const NOOFCOLS_MASK: c_uint = 0x00000060;
pub const CHANSIZE_SHIFT: c_int = 7;
pub const CHANSIZE_MASK: c_uint = 0x00000080;
pub const BURSTLENGTH_SHIFT: c_int = 8;
pub const BURSTLENGTH_MASK: c_uint = 0x00000100;

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
pub const SPI_CONFIG_CNTL: c_uint = 0x9100;

pub const SPI_CONFIG_CNTL_1: c_uint = 0x913C;

pub const SPI_INPUT_Z: c_uint = 0x286D8;
pub const SPI_PS_IN_CONTROL_0: c_uint = 0x286CC;

pub const SPI_PS_IN_CONTROL_1: c_uint = 0x286D0;

pub const SQ_MS_FIFO_SIZES: c_uint = 0x8CF0;

pub const SQ_PGM_START_ES: c_uint = 0x28880;
pub const SQ_PGM_START_FS: c_uint = 0x28894;
pub const SQ_PGM_START_GS: c_uint = 0x2886C;
pub const SQ_PGM_START_PS: c_uint = 0x28840;
pub const SQ_PGM_RESOURCES_PS: c_uint = 0x28850;
pub const SQ_PGM_EXPORTS_PS: c_uint = 0x28854;
pub const SQ_PGM_CF_OFFSET_PS: c_uint = 0x288cc;
pub const SQ_PGM_START_VS: c_uint = 0x28858;
pub const SQ_PGM_RESOURCES_VS: c_uint = 0x28868;
pub const SQ_PGM_CF_OFFSET_VS: c_uint = 0x288d0;
pub const SQ_VTX_CONSTANT_WORD0_0: c_uint = 0x30000;
pub const SQ_VTX_CONSTANT_WORD1_0: c_uint = 0x30004;
pub const SQ_VTX_CONSTANT_WORD2_0: c_uint = 0x30008;

pub const SQ_VTX_CONSTANT_WORD3_0: c_uint = 0x3000c;
pub const SQ_VTX_CONSTANT_WORD6_0: c_uint = 0x38018;

pub const SQ_TEX_VTX_INVALID_TEXTURE: c_uint = 0x0;
pub const SQ_TEX_VTX_INVALID_BUFFER: c_uint = 0x1;
pub const SQ_TEX_VTX_VALID_TEXTURE: c_uint = 0x2;
pub const SQ_TEX_VTX_VALID_BUFFER: c_uint = 0x3;
pub const SX_MISC: c_uint = 0x28350;
pub const SX_MEMORY_EXPORT_BASE: c_uint = 0x9010;
pub const SX_DEBUG_1: c_uint = 0x9054;

pub const TA_CNTL_AUX: c_uint = 0x9508;

pub const TC_CNTL: c_uint = 0x9608;

pub const VC_ENHANCE: c_uint = 0x9714;
pub const VGT_CACHE_INVALIDATION: c_uint = 0x88C4;

pub const VC_ONLY: c_int = 0;
pub const TC_ONLY: c_int = 1;
pub const VC_AND_TC: c_int = 2;
pub const VGT_DMA_BASE: c_uint = 0x287E8;
pub const VGT_DMA_BASE_HI: c_uint = 0x287E4;
pub const VGT_ES_PER_GS: c_uint = 0x88CC;
pub const VGT_GS_PER_ES: c_uint = 0x88C8;
pub const VGT_GS_PER_VS: c_uint = 0x88E8;
pub const VGT_GS_VERTEX_REUSE: c_uint = 0x88D4;
pub const VGT_PRIMITIVE_TYPE: c_uint = 0x8958;
pub const VGT_NUM_INSTANCES: c_uint = 0x8974;
pub const VGT_OUT_DEALLOC_CNTL: c_uint = 0x28C5C;
pub const DEALLOC_DIST_MASK: c_uint = 0x0000007F;
pub const VGT_STRMOUT_BASE_OFFSET_0: c_uint = 0x28B10;
pub const VGT_STRMOUT_BASE_OFFSET_1: c_uint = 0x28B14;
pub const VGT_STRMOUT_BASE_OFFSET_2: c_uint = 0x28B18;
pub const VGT_STRMOUT_BASE_OFFSET_3: c_uint = 0x28B1c;
pub const VGT_STRMOUT_BASE_OFFSET_HI_0: c_uint = 0x28B44;
pub const VGT_STRMOUT_BASE_OFFSET_HI_1: c_uint = 0x28B48;
pub const VGT_STRMOUT_BASE_OFFSET_HI_2: c_uint = 0x28B4c;
pub const VGT_STRMOUT_BASE_OFFSET_HI_3: c_uint = 0x28B50;
pub const VGT_STRMOUT_BUFFER_BASE_0: c_uint = 0x28AD8;
pub const VGT_STRMOUT_BUFFER_BASE_1: c_uint = 0x28AE8;
pub const VGT_STRMOUT_BUFFER_BASE_2: c_uint = 0x28AF8;
pub const VGT_STRMOUT_BUFFER_BASE_3: c_uint = 0x28B08;
pub const VGT_STRMOUT_BUFFER_OFFSET_0: c_uint = 0x28ADC;
pub const VGT_STRMOUT_BUFFER_OFFSET_1: c_uint = 0x28AEC;
pub const VGT_STRMOUT_BUFFER_OFFSET_2: c_uint = 0x28AFC;
pub const VGT_STRMOUT_BUFFER_OFFSET_3: c_uint = 0x28B0C;
pub const VGT_STRMOUT_BUFFER_SIZE_0: c_uint = 0x28AD0;
pub const VGT_STRMOUT_BUFFER_SIZE_1: c_uint = 0x28AE0;
pub const VGT_STRMOUT_BUFFER_SIZE_2: c_uint = 0x28AF0;
pub const VGT_STRMOUT_BUFFER_SIZE_3: c_uint = 0x28B00;
pub const VGT_STRMOUT_EN: c_uint = 0x28AB0;
pub const VGT_VERTEX_REUSE_BLOCK_CNTL: c_uint = 0x28C58;
pub const VTX_REUSE_DEPTH_MASK: c_uint = 0x000000FF;
pub const VGT_EVENT_INITIATOR: c_uint = 0x28a90;

pub const VM_CONTEXT0_CNTL: c_uint = 0x1410;

pub const VM_CONTEXT0_INVALIDATION_LOW_ADDR: c_uint = 0x1490;
pub const VM_CONTEXT0_INVALIDATION_HIGH_ADDR: c_uint = 0x14B0;
pub const VM_CONTEXT0_PAGE_TABLE_BASE_ADDR: c_uint = 0x1574;
pub const VM_CONTEXT0_PAGE_TABLE_START_ADDR: c_uint = 0x1594;
pub const VM_CONTEXT0_PAGE_TABLE_END_ADDR: c_uint = 0x15B4;
pub const VM_CONTEXT0_PROTECTION_FAULT_DEFAULT_ADDR: c_uint = 0x1554;
pub const VM_CONTEXT0_REQUEST_RESPONSE: c_uint = 0x1470;

pub const RESPONSE_TYPE_MASK: c_uint = 0x000000F0;
pub const RESPONSE_TYPE_SHIFT: c_int = 4;
pub const VM_L2_CNTL: c_uint = 0x1400;

pub const VM_L2_CNTL2: c_uint = 0x1404;

pub const VM_L2_CNTL3: c_uint = 0x1408;

pub const VM_L2_STATUS: c_uint = 0x140C;

pub const WAIT_UNTIL: c_uint = 0x8040;

// async DMA
pub const DMA_TILING_CONFIG: c_uint = 0x3ec4;
pub const DMA_CONFIG: c_uint = 0x3e4c;
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

pub const DMA_SEM_INCOMPLETE_TIMER_CNTL: c_uint = 0xd044;
pub const DMA_SEM_WAIT_FAIL_TIMER_CNTL: c_uint = 0xd048;
pub const DMA_MODE: c_uint = 0xd0bc;
// async DMA packets

// async DMA Packet types
pub const DMA_PACKET_WRITE: c_uint = 0x2;
pub const DMA_PACKET_COPY: c_uint = 0x3;
pub const DMA_PACKET_INDIRECT_BUFFER: c_uint = 0x4;
pub const DMA_PACKET_SEMAPHORE: c_uint = 0x5;
pub const DMA_PACKET_FENCE: c_uint = 0x6;
pub const DMA_PACKET_TRAP: c_uint = 0x7;
pub const DMA_PACKET_CONSTANT_FILL: c_uint = 0xd /* 7xx only */;
pub const DMA_PACKET_NOP: c_uint = 0xf;
pub const IH_RB_CNTL: c_uint = 0x3e00;

pub const IH_RB_BASE: c_uint = 0x3e04;
pub const IH_RB_RPTR: c_uint = 0x3e08;
pub const IH_RB_WPTR: c_uint = 0x3e0c;

pub const IH_RB_WPTR_ADDR_HI: c_uint = 0x3e10;
pub const IH_RB_WPTR_ADDR_LO: c_uint = 0x3e14;
pub const IH_CNTL: c_uint = 0x3e18;

pub const RLC_CNTL: c_uint = 0x3f00;

pub const RLC_HB_BASE: c_uint = 0x3f10;
pub const RLC_HB_CNTL: c_uint = 0x3f0c;
pub const RLC_HB_RPTR: c_uint = 0x3f20;
pub const RLC_HB_WPTR: c_uint = 0x3f1c;
pub const RLC_HB_WPTR_LSB_ADDR: c_uint = 0x3f14;
pub const RLC_HB_WPTR_MSB_ADDR: c_uint = 0x3f18;
pub const RLC_GPU_CLOCK_COUNT_LSB: c_uint = 0x3f38;
pub const RLC_GPU_CLOCK_COUNT_MSB: c_uint = 0x3f3c;
pub const RLC_CAPTURE_GPU_CLOCK_COUNT: c_uint = 0x3f40;
pub const RLC_MC_CNTL: c_uint = 0x3f44;
pub const RLC_UCODE_CNTL: c_uint = 0x3f48;
pub const RLC_UCODE_ADDR: c_uint = 0x3f2c;
pub const RLC_UCODE_DATA: c_uint = 0x3f30;
pub const SRBM_SOFT_RESET: c_uint = 0xe60;

pub const BIF_SCRATCH0: c_uint = 0x5438;
pub const BUS_CNTL: c_uint = 0x5420;

pub const CP_INT_CNTL: c_uint = 0xc124;

pub const CP_INT_STATUS: c_uint = 0xc128;

pub const GRBM_INT_CNTL: c_uint = 0x8060;

pub const INTERRUPT_CNTL: c_uint = 0x5468;

pub const INTERRUPT_CNTL2: c_uint = 0x546c;
pub const D1MODE_VBLANK_STATUS: c_uint = 0x6534;
pub const D2MODE_VBLANK_STATUS: c_uint = 0x6d34;

pub const D1MODE_VLINE_STATUS: c_uint = 0x653c;
pub const D2MODE_VLINE_STATUS: c_uint = 0x6d3c;

pub const DxMODE_INT_MASK: c_uint = 0x6540;

pub const DCE3_DISP_INTERRUPT_STATUS: c_uint = 0x7ddc;

pub const DISP_INTERRUPT_STATUS: c_uint = 0x7edc;

pub const DISP_INTERRUPT_STATUS_CONTINUE: c_uint = 0x7ee8;
pub const DCE3_DISP_INTERRUPT_STATUS_CONTINUE: c_uint = 0x7de8;

pub const DCE3_DISP_INTERRUPT_STATUS_CONTINUE2: c_uint = 0x7dec;

// DCE 3.2

pub const DACA_AUTO_DETECT_CONTROL: c_uint = 0x7828;
pub const DACB_AUTO_DETECT_CONTROL: c_uint = 0x7a28;
pub const DCE3_DACA_AUTO_DETECT_CONTROL: c_uint = 0x7028;
pub const DCE3_DACB_AUTO_DETECT_CONTROL: c_uint = 0x7128;

// bit 18 = R/C, 17 = G/Y, 16 = B/Comp

pub const DCE3_DACA_AUTODETECT_INT_CONTROL: c_uint = 0x7038;
pub const DCE3_DACB_AUTODETECT_INT_CONTROL: c_uint = 0x7138;
pub const DACA_AUTODETECT_INT_CONTROL: c_uint = 0x7838;
pub const DACB_AUTODETECT_INT_CONTROL: c_uint = 0x7a38;

pub const DC_HOT_PLUG_DETECT1_CONTROL: c_uint = 0x7d00;
pub const DC_HOT_PLUG_DETECT2_CONTROL: c_uint = 0x7d10;
pub const DC_HOT_PLUG_DETECT3_CONTROL: c_uint = 0x7d24;

pub const DC_HOT_PLUG_DETECT1_INT_STATUS: c_uint = 0x7d04;
pub const DC_HOT_PLUG_DETECT2_INT_STATUS: c_uint = 0x7d14;
pub const DC_HOT_PLUG_DETECT3_INT_STATUS: c_uint = 0x7d28;

// DCE 3.0
pub const DC_HPD1_INT_STATUS: c_uint = 0x7d00;
pub const DC_HPD2_INT_STATUS: c_uint = 0x7d0c;
pub const DC_HPD3_INT_STATUS: c_uint = 0x7d18;
pub const DC_HPD4_INT_STATUS: c_uint = 0x7d24;
// DCE 3.2
pub const DC_HPD5_INT_STATUS: c_uint = 0x7dc0;
pub const DC_HPD6_INT_STATUS: c_uint = 0x7df4;

pub const DC_HOT_PLUG_DETECT1_INT_CONTROL: c_uint = 0x7d08;
pub const DC_HOT_PLUG_DETECT2_INT_CONTROL: c_uint = 0x7d18;
pub const DC_HOT_PLUG_DETECT3_INT_CONTROL: c_uint = 0x7d2c;

// DCE 3.0
pub const DC_HPD1_INT_CONTROL: c_uint = 0x7d04;
pub const DC_HPD2_INT_CONTROL: c_uint = 0x7d10;
pub const DC_HPD3_INT_CONTROL: c_uint = 0x7d1c;
pub const DC_HPD4_INT_CONTROL: c_uint = 0x7d28;
// DCE 3.2
pub const DC_HPD5_INT_CONTROL: c_uint = 0x7dc4;
pub const DC_HPD6_INT_CONTROL: c_uint = 0x7df8;

// DCE 3.0
pub const DC_HPD1_CONTROL: c_uint = 0x7d08;
pub const DC_HPD2_CONTROL: c_uint = 0x7d14;
pub const DC_HPD3_CONTROL: c_uint = 0x7d20;
pub const DC_HPD4_CONTROL: c_uint = 0x7d2c;
// DCE 3.2
pub const DC_HPD5_CONTROL: c_uint = 0x7dc8;
pub const DC_HPD6_CONTROL: c_uint = 0x7dfc;

// DCE 3.2

pub const D1GRPH_INTERRUPT_STATUS: c_uint = 0x6158;
pub const D2GRPH_INTERRUPT_STATUS: c_uint = 0x6958;

pub const D1GRPH_INTERRUPT_CONTROL: c_uint = 0x615c;
pub const D2GRPH_INTERRUPT_CONTROL: c_uint = 0x695c;

// PCIE link stuff
pub const PCIE_LC_TRAINING_CNTL: c_uint = 0xa1 /* PCIE_P */;

pub const PCIE_LC_LINK_WIDTH_CNTL: c_uint = 0xa2 /* PCIE_P */;

pub const PCIE_LC_SPEED_CNTL: c_uint = 0xa4 /* PCIE_P */;

pub const MM_CFGREGS_CNTL: c_uint = 0x544c;

pub const LINK_CNTL2: c_uint = 0x88 /* F0 */;

// Audio
pub const AZ_HOT_PLUG_CONTROL: c_uint = 0x7300;

// DCE3 adds

// Audio clocks DCE 2.0/3.0
pub const AUDIO_DTO: c_uint = 0x7340;

// Audio clocks DCE 3.2
pub const DCCG_AUDIO_DTO0_PHASE: c_uint = 0x0514;
pub const DCCG_AUDIO_DTO0_MODULE: c_uint = 0x0518;
pub const DCCG_AUDIO_DTO0_LOAD: c_uint = 0x051c;

pub const DCCG_AUDIO_DTO0_CNTL: c_uint = 0x0520;

pub const DCCG_AUDIO_DTO1_PHASE: c_uint = 0x0524;
pub const DCCG_AUDIO_DTO1_MODULE: c_uint = 0x0528;
pub const DCCG_AUDIO_DTO1_LOAD: c_uint = 0x052c;
pub const DCCG_AUDIO_DTO1_CNTL: c_uint = 0x0530;
pub const DCCG_AUDIO_DTO_SELECT: c_uint = 0x0534;
// digital blocks
pub const TMDSA_CNTL: c_uint = 0x7880;

pub const LVTMA_CNTL: c_uint = 0x7a80;

pub const DDIA_CNTL: c_uint = 0x7200;

pub const DIG0_CNTL: c_uint = 0x75a0;

pub const DIG1_CNTL: c_uint = 0x79a0;
pub const AZ_F0_CODEC_PIN0_CONTROL_CHANNEL_SPEAKER: c_uint = 0x71bc;

pub const SPEAKER_ALLOCATION_SHIFT: c_int = 0;

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
// rs6xx/rs740 and r6xx share the same HDMI blocks, however, rs6xx has only one
// instance of the blocks while r6xx has 2.  DCE 3.0 cards are slightly
// different due to the new DIG blocks, but also have 2 instances.
// DCE 3.0 HDMI blocks are part of each DIG encoder.
//
// rs6xx/rs740/r6xx/dce3
pub const HDMI0_CONTROL: c_uint = 0x7400;
// rs6xx/rs740/r6xx

// rs6xx/r6xx/dce3

pub const HDMI0_STATUS: c_uint = 0x7404;

pub const HDMI0_AUDIO_PACKET_CONTROL: c_uint = 0x7408;

pub const HDMI0_AUDIO_CRC_CONTROL: c_uint = 0x740c;

pub const DCE3_HDMI0_ACR_PACKET_CONTROL: c_uint = 0x740c;
pub const HDMI0_VBI_PACKET_CONTROL: c_uint = 0x7410;

pub const HDMI0_INFOFRAME_CONTROL0: c_uint = 0x7414;

pub const HDMI0_INFOFRAME_CONTROL1: c_uint = 0x7418;

pub const HDMI0_GENERIC_PACKET_CONTROL: c_uint = 0x741c;

pub const HDMI0_GC: c_uint = 0x7428;

pub const HDMI0_AVI_INFO0: c_uint = 0x7454;

pub const HDMI0_AVI_INFO1: c_uint = 0x7458;

pub const HDMI0_AVI_INFO2: c_uint = 0x745c;

pub const HDMI0_AVI_INFO3: c_uint = 0x7460;

pub const HDMI0_MPEG_INFO0: c_uint = 0x7464;

pub const HDMI0_MPEG_INFO1: c_uint = 0x7468;

pub const HDMI0_GENERIC0_HDR: c_uint = 0x746c;
pub const HDMI0_GENERIC0_0: c_uint = 0x7470;
pub const HDMI0_GENERIC0_1: c_uint = 0x7474;
pub const HDMI0_GENERIC0_2: c_uint = 0x7478;
pub const HDMI0_GENERIC0_3: c_uint = 0x747c;
pub const HDMI0_GENERIC0_4: c_uint = 0x7480;
pub const HDMI0_GENERIC0_5: c_uint = 0x7484;
pub const HDMI0_GENERIC0_6: c_uint = 0x7488;
pub const HDMI0_GENERIC1_HDR: c_uint = 0x748c;
pub const HDMI0_GENERIC1_0: c_uint = 0x7490;
pub const HDMI0_GENERIC1_1: c_uint = 0x7494;
pub const HDMI0_GENERIC1_2: c_uint = 0x7498;
pub const HDMI0_GENERIC1_3: c_uint = 0x749c;
pub const HDMI0_GENERIC1_4: c_uint = 0x74a0;
pub const HDMI0_GENERIC1_5: c_uint = 0x74a4;
pub const HDMI0_GENERIC1_6: c_uint = 0x74a8;
pub const HDMI0_ACR_32_0: c_uint = 0x74ac;

pub const HDMI0_ACR_32_1: c_uint = 0x74b0;

pub const HDMI0_ACR_44_0: c_uint = 0x74b4;

pub const HDMI0_ACR_44_1: c_uint = 0x74b8;

pub const HDMI0_ACR_48_0: c_uint = 0x74bc;

pub const HDMI0_ACR_48_1: c_uint = 0x74c0;

pub const HDMI0_ACR_STATUS_0: c_uint = 0x74c4;
pub const HDMI0_ACR_STATUS_1: c_uint = 0x74c8;
pub const HDMI0_AUDIO_INFO0: c_uint = 0x74cc;

pub const HDMI0_AUDIO_INFO1: c_uint = 0x74d0;

pub const HDMI0_60958_0: c_uint = 0x74d4;

pub const HDMI0_60958_1: c_uint = 0x74d8;

pub const HDMI0_ACR_PACKET_CONTROL: c_uint = 0x74dc;

pub const DCE3_HDMI0_AUDIO_CRC_CONTROL: c_uint = 0x74dc;
pub const HDMI0_RAMP_CONTROL0: c_uint = 0x74e0;

pub const HDMI0_RAMP_CONTROL1: c_uint = 0x74e4;

pub const HDMI0_RAMP_CONTROL2: c_uint = 0x74e8;

pub const HDMI0_RAMP_CONTROL3: c_uint = 0x74ec;

// HDMI0_60958_2 is r7xx only
pub const HDMI0_60958_2: c_uint = 0x74f0;

// r6xx only; second instance starts at 0x7700
pub const HDMI1_CONTROL: c_uint = 0x7700;
pub const HDMI1_STATUS: c_uint = 0x7704;
pub const HDMI1_AUDIO_PACKET_CONTROL: c_uint = 0x7708;
// DCE3; second instance starts at 0x7800 NOT 0x7700
pub const DCE3_HDMI1_CONTROL: c_uint = 0x7800;
pub const DCE3_HDMI1_STATUS: c_uint = 0x7804;
pub const DCE3_HDMI1_AUDIO_PACKET_CONTROL: c_uint = 0x7808;
// DCE3.2 (for interrupts)
pub const AFMT_STATUS: c_uint = 0x7600;

pub const AFMT_AUDIO_PACKET_CONTROL: c_uint = 0x7604;

// DCE3 FMT blocks
pub const FMT_CONTROL: c_uint = 0x6700;

// 0 = RGB 4:4:4 or YCbCr 4:4:4, 1 = YCbCr 4:2:2
pub const FMT_BIT_DEPTH_CONTROL: c_uint = 0x6710;

pub const FMT_CLAMP_CONTROL: c_uint = 0x672c;

// Power management
pub const CG_SPLL_FUNC_CNTL: c_uint = 0x600;

pub const GENERAL_PWRMGT: c_uint = 0x618;

pub const CG_TPC: c_uint = 0x61c;

pub const SCLK_PWRMGT_CNTL: c_uint = 0x620;

pub const MCLK_PWRMGT_CNTL: c_uint = 0x624;

pub const MPLL_TIME: c_uint = 0x634;

pub const SCLK_FREQ_SETTING_STEP_0_PART1: c_uint = 0x648;

pub const SCLK_FREQ_SETTING_STEP_0_PART2: c_uint = 0x64c;

pub const VID_RT: c_uint = 0x6f8;

pub const CTXSW_PROFILE_INDEX: c_uint = 0x6fc;

pub const TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0x70c;

pub const LOWER_GPIO_ENABLE: c_uint = 0x710;
pub const UPPER_GPIO_ENABLE: c_uint = 0x714;
pub const CTXSW_VID_LOWER_GPIO_CNTL: c_uint = 0x718;
pub const VID_UPPER_GPIO_CNTL: c_uint = 0x740;
pub const CG_CTX_CGTT3D_R: c_uint = 0x744;

pub const CG_VDDC3D_OOR: c_uint = 0x748;

pub const CG_FTV: c_uint = 0x74c;
pub const CG_FFCT_0: c_uint = 0x750;

pub const CG_BSP: c_uint = 0x78c;

pub const CG_RT: c_uint = 0x790;

pub const CG_LT: c_uint = 0x794;

pub const CG_GIT: c_uint = 0x798;

pub const CG_SSP: c_uint = 0x7a8;

pub const CG_RLC_REQ_AND_RSP: c_uint = 0x7c4;

pub const CG_FC_T: c_uint = 0x7cc;

pub const GPIOPAD_MASK: c_uint = 0x1798;
pub const GPIOPAD_A: c_uint = 0x179c;
pub const GPIOPAD_EN: c_uint = 0x17a0;
pub const GRBM_PWR_CNTL: c_uint = 0x800c;

//
// UVD
//
pub const UVD_SEMA_ADDR_LOW: c_uint = 0xef00;
pub const UVD_SEMA_ADDR_HIGH: c_uint = 0xef04;
pub const UVD_SEMA_CMD: c_uint = 0xef08;
pub const UVD_GPCOM_VCPU_CMD: c_uint = 0xef0c;
pub const UVD_GPCOM_VCPU_DATA0: c_uint = 0xef10;
pub const UVD_GPCOM_VCPU_DATA1: c_uint = 0xef14;
pub const UVD_ENGINE_CNTL: c_uint = 0xef18;
pub const UVD_NO_OP: c_uint = 0xeffc;
pub const UVD_SEMA_CNTL: c_uint = 0xf400;
pub const UVD_RB_ARB_CTRL: c_uint = 0xf480;
pub const UVD_LMI_EXT40_ADDR: c_uint = 0xf498;
pub const UVD_CGC_GATE: c_uint = 0xf4a8;
pub const UVD_LMI_CTRL2: c_uint = 0xf4f4;
pub const UVD_MASTINT_EN: c_uint = 0xf500;
pub const UVD_FW_START: c_uint = 0xf51C;
pub const UVD_LMI_ADDR_EXT: c_uint = 0xf594;
pub const UVD_LMI_CTRL: c_uint = 0xf598;
pub const UVD_LMI_SWAP_CNTL: c_uint = 0xf5b4;
pub const UVD_MP_SWAP_CNTL: c_uint = 0xf5bC;
pub const UVD_MPC_CNTL: c_uint = 0xf5dC;
pub const UVD_MPC_SET_MUXA0: c_uint = 0xf5e4;
pub const UVD_MPC_SET_MUXA1: c_uint = 0xf5e8;
pub const UVD_MPC_SET_MUXB0: c_uint = 0xf5eC;
pub const UVD_MPC_SET_MUXB1: c_uint = 0xf5f0;
pub const UVD_MPC_SET_MUX: c_uint = 0xf5f4;
pub const UVD_MPC_SET_ALU: c_uint = 0xf5f8;
pub const UVD_VCPU_CACHE_OFFSET0: c_uint = 0xf608;
pub const UVD_VCPU_CACHE_SIZE0: c_uint = 0xf60c;
pub const UVD_VCPU_CACHE_OFFSET1: c_uint = 0xf610;
pub const UVD_VCPU_CACHE_SIZE1: c_uint = 0xf614;
pub const UVD_VCPU_CACHE_OFFSET2: c_uint = 0xf618;
pub const UVD_VCPU_CACHE_SIZE2: c_uint = 0xf61c;
pub const UVD_VCPU_CNTL: c_uint = 0xf660;
pub const UVD_SOFT_RESET: c_uint = 0xf680;

pub const UVD_RBC_IB_BASE: c_uint = 0xf684;
pub const UVD_RBC_IB_SIZE: c_uint = 0xf688;
pub const UVD_RBC_RB_BASE: c_uint = 0xf68c;
pub const UVD_RBC_RB_RPTR: c_uint = 0xf690;
pub const UVD_RBC_RB_WPTR: c_uint = 0xf694;
pub const UVD_RBC_RB_WPTR_CNTL: c_uint = 0xf698;
pub const UVD_STATUS: c_uint = 0xf6bc;
pub const UVD_SEMA_TIMEOUT_STATUS: c_uint = 0xf6c0;
pub const UVD_SEMA_WAIT_INCOMPLETE_TIMEOUT_CNTL: c_uint = 0xf6c4;
pub const UVD_SEMA_WAIT_FAULT_TIMEOUT_CNTL: c_uint = 0xf6c8;
pub const UVD_SEMA_SIGNAL_INCOMPLETE_TIMEOUT_CNTL: c_uint = 0xf6cc;
pub const UVD_RBC_RB_CNTL: c_uint = 0xf6a4;
pub const UVD_RBC_RB_RPTR_ADDR: c_uint = 0xf6a8;
pub const UVD_CONTEXT_ID: c_uint = 0xf6f4;
// rs780 only
pub const GFX_MACRO_BYPASS_CNTL: c_uint = 0x30c0;

pub const CG_UPLL_FUNC_CNTL: c_uint = 0x7e0;

pub const CG_UPLL_FUNC_CNTL_2: c_uint = 0x7e4;

//
// PM4
//

// Packet 3 types
pub const PACKET3_NOP: c_uint = 0x10;
pub const PACKET3_INDIRECT_BUFFER_END: c_uint = 0x17;
pub const PACKET3_SET_PREDICATION: c_uint = 0x20;
pub const PACKET3_REG_RMW: c_uint = 0x21;
pub const PACKET3_COND_EXEC: c_uint = 0x22;
pub const PACKET3_PRED_EXEC: c_uint = 0x23;
pub const PACKET3_START_3D_CMDBUF: c_uint = 0x24;
pub const PACKET3_DRAW_INDEX_2: c_uint = 0x27;
pub const PACKET3_CONTEXT_CONTROL: c_uint = 0x28;
pub const PACKET3_DRAW_INDEX_IMMD_BE: c_uint = 0x29;
pub const PACKET3_INDEX_TYPE: c_uint = 0x2A;
pub const PACKET3_DRAW_INDEX: c_uint = 0x2B;
pub const PACKET3_DRAW_INDEX_AUTO: c_uint = 0x2D;
pub const PACKET3_DRAW_INDEX_IMMD: c_uint = 0x2E;
pub const PACKET3_NUM_INSTANCES: c_uint = 0x2F;
pub const PACKET3_STRMOUT_BUFFER_UPDATE: c_uint = 0x34;
pub const PACKET3_INDIRECT_BUFFER_MP: c_uint = 0x38;
pub const PACKET3_MEM_SEMAPHORE: c_uint = 0x39;

pub const PACKET3_MPEG_INDEX: c_uint = 0x3A;
pub const PACKET3_COPY_DW: c_uint = 0x3B;
pub const PACKET3_WAIT_REG_MEM: c_uint = 0x3C;
pub const PACKET3_MEM_WRITE: c_uint = 0x3D;
pub const PACKET3_INDIRECT_BUFFER: c_uint = 0x32;
pub const PACKET3_CP_DMA: c_uint = 0x41;
// 1. header
// 2. SRC_ADDR_LO [31:0]
// 3. CP_SYNC [31] | SRC_ADDR_HI [7:0]
// 4. DST_ADDR_LO [31:0]
// 5. DST_ADDR_HI [7:0]
// 6. COMMAND [29:22] | BYTE_COUNT [20:0]
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

pub const PACKET3_PFP_SYNC_ME: c_uint = 0x42 /* r7xx+ only */;
pub const PACKET3_SURFACE_SYNC: c_uint = 0x43;

pub const PACKET3_ME_INITIALIZE: c_uint = 0x44;

pub const PACKET3_COND_WRITE: c_uint = 0x45;
pub const PACKET3_EVENT_WRITE: c_uint = 0x46;

// 0 - any non-TS event
// 1 - ZPASS_DONE
// 2 - SAMPLE_PIPELINESTAT
// 3 - SAMPLE_STREAMOUTSTAT
// 4 - *S_PARTIAL_FLUSH
// 5 - TS events
//
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
pub const PACKET3_ONE_REG_WRITE: c_uint = 0x57;
pub const PACKET3_SET_CONFIG_REG: c_uint = 0x68;
pub const PACKET3_SET_CONFIG_REG_OFFSET: c_uint = 0x00008000;
pub const PACKET3_SET_CONFIG_REG_END: c_uint = 0x0000ac00;
pub const PACKET3_SET_CONTEXT_REG: c_uint = 0x69;
pub const PACKET3_SET_CONTEXT_REG_OFFSET: c_uint = 0x00028000;
pub const PACKET3_SET_CONTEXT_REG_END: c_uint = 0x00029000;
pub const PACKET3_SET_ALU_CONST: c_uint = 0x6A;
pub const PACKET3_SET_ALU_CONST_OFFSET: c_uint = 0x00030000;
pub const PACKET3_SET_ALU_CONST_END: c_uint = 0x00032000;
pub const PACKET3_SET_BOOL_CONST: c_uint = 0x6B;
pub const PACKET3_SET_BOOL_CONST_OFFSET: c_uint = 0x0003e380;
pub const PACKET3_SET_BOOL_CONST_END: c_uint = 0x00040000;
pub const PACKET3_SET_LOOP_CONST: c_uint = 0x6C;
pub const PACKET3_SET_LOOP_CONST_OFFSET: c_uint = 0x0003e200;
pub const PACKET3_SET_LOOP_CONST_END: c_uint = 0x0003e380;
pub const PACKET3_SET_RESOURCE: c_uint = 0x6D;
pub const PACKET3_SET_RESOURCE_OFFSET: c_uint = 0x00038000;
pub const PACKET3_SET_RESOURCE_END: c_uint = 0x0003c000;
pub const PACKET3_SET_SAMPLER: c_uint = 0x6E;
pub const PACKET3_SET_SAMPLER_OFFSET: c_uint = 0x0003c000;
pub const PACKET3_SET_SAMPLER_END: c_uint = 0x0003cff0;
pub const PACKET3_SET_CTL_CONST: c_uint = 0x6F;
pub const PACKET3_SET_CTL_CONST_OFFSET: c_uint = 0x0003cff0;
pub const PACKET3_SET_CTL_CONST_END: c_uint = 0x0003e200;
pub const PACKET3_STRMOUT_BASE_UPDATE: c_uint = 0x72 /* r7xx */;
pub const PACKET3_SURFACE_BASE_UPDATE: c_uint = 0x73;
pub const R_000011_K8_FB_LOCATION: c_uint = 0x11;
pub const R_000012_MC_MISC_UMA_CNTL: c_uint = 0x12;

pub const R_0028F8_MC_INDEX: c_uint = 0x28F8;

pub const C_0028F8_MC_IND_ADDR: c_uint = 0xFFFFFE00;

pub const R_0028FC_MC_DATA: c_uint = 0x28FC;
pub const R_008020_GRBM_SOFT_RESET: c_uint = 0x8020;

pub const R_008010_GRBM_STATUS: c_uint = 0x8010;

pub const R_008014_GRBM_STATUS2: c_uint = 0x8014;

pub const R_000E50_SRBM_STATUS: c_uint = 0x0E50;

pub const R_000E60_SRBM_SOFT_RESET: c_uint = 0x0E60;

pub const R_005480_HDP_MEM_COHERENCY_FLUSH_CNTL: c_uint = 0x5480;
pub const R_028C04_PA_SC_AA_CONFIG: c_uint = 0x028C04;

pub const C_028C04_MSAA_NUM_SAMPLES: c_uint = 0xFFFFFFFC;

pub const C_028C04_AA_MASK_CENTROID_DTMN: c_uint = 0xFFFFFFEF;

pub const C_028C04_MAX_SAMPLE_DIST: c_uint = 0xFFFE1FFF;
pub const R_0280E0_CB_COLOR0_FRAG: c_uint = 0x0280E0;

pub const C_0280E0_BASE_256B: c_uint = 0x00000000;
pub const R_0280E4_CB_COLOR1_FRAG: c_uint = 0x0280E4;
pub const R_0280E8_CB_COLOR2_FRAG: c_uint = 0x0280E8;
pub const R_0280EC_CB_COLOR3_FRAG: c_uint = 0x0280EC;
pub const R_0280F0_CB_COLOR4_FRAG: c_uint = 0x0280F0;
pub const R_0280F4_CB_COLOR5_FRAG: c_uint = 0x0280F4;
pub const R_0280F8_CB_COLOR6_FRAG: c_uint = 0x0280F8;
pub const R_0280FC_CB_COLOR7_FRAG: c_uint = 0x0280FC;
pub const R_0280C0_CB_COLOR0_TILE: c_uint = 0x0280C0;

pub const C_0280C0_BASE_256B: c_uint = 0x00000000;
pub const R_0280C4_CB_COLOR1_TILE: c_uint = 0x0280C4;
pub const R_0280C8_CB_COLOR2_TILE: c_uint = 0x0280C8;
pub const R_0280CC_CB_COLOR3_TILE: c_uint = 0x0280CC;
pub const R_0280D0_CB_COLOR4_TILE: c_uint = 0x0280D0;
pub const R_0280D4_CB_COLOR5_TILE: c_uint = 0x0280D4;
pub const R_0280D8_CB_COLOR6_TILE: c_uint = 0x0280D8;
pub const R_0280DC_CB_COLOR7_TILE: c_uint = 0x0280DC;
pub const R_0280A0_CB_COLOR0_INFO: c_uint = 0x0280A0;

pub const C_0280A0_ENDIAN: c_uint = 0xFFFFFFFC;

pub const C_0280A0_FORMAT: c_uint = 0xFFFFFF03;
pub const V_0280A0_COLOR_INVALID: c_uint = 0x00000000;
pub const V_0280A0_COLOR_8: c_uint = 0x00000001;
pub const V_0280A0_COLOR_4_4: c_uint = 0x00000002;
pub const V_0280A0_COLOR_3_3_2: c_uint = 0x00000003;
pub const V_0280A0_COLOR_16: c_uint = 0x00000005;
pub const V_0280A0_COLOR_16_FLOAT: c_uint = 0x00000006;
pub const V_0280A0_COLOR_8_8: c_uint = 0x00000007;
pub const V_0280A0_COLOR_5_6_5: c_uint = 0x00000008;
pub const V_0280A0_COLOR_6_5_5: c_uint = 0x00000009;
pub const V_0280A0_COLOR_1_5_5_5: c_uint = 0x0000000A;
pub const V_0280A0_COLOR_4_4_4_4: c_uint = 0x0000000B;
pub const V_0280A0_COLOR_5_5_5_1: c_uint = 0x0000000C;
pub const V_0280A0_COLOR_32: c_uint = 0x0000000D;
pub const V_0280A0_COLOR_32_FLOAT: c_uint = 0x0000000E;
pub const V_0280A0_COLOR_16_16: c_uint = 0x0000000F;
pub const V_0280A0_COLOR_16_16_FLOAT: c_uint = 0x00000010;
pub const V_0280A0_COLOR_8_24: c_uint = 0x00000011;
pub const V_0280A0_COLOR_8_24_FLOAT: c_uint = 0x00000012;
pub const V_0280A0_COLOR_24_8: c_uint = 0x00000013;
pub const V_0280A0_COLOR_24_8_FLOAT: c_uint = 0x00000014;
pub const V_0280A0_COLOR_10_11_11: c_uint = 0x00000015;
pub const V_0280A0_COLOR_10_11_11_FLOAT: c_uint = 0x00000016;
pub const V_0280A0_COLOR_11_11_10: c_uint = 0x00000017;
pub const V_0280A0_COLOR_11_11_10_FLOAT: c_uint = 0x00000018;
pub const V_0280A0_COLOR_2_10_10_10: c_uint = 0x00000019;
pub const V_0280A0_COLOR_8_8_8_8: c_uint = 0x0000001A;
pub const V_0280A0_COLOR_10_10_10_2: c_uint = 0x0000001B;
pub const V_0280A0_COLOR_X24_8_32_FLOAT: c_uint = 0x0000001C;
pub const V_0280A0_COLOR_32_32: c_uint = 0x0000001D;
pub const V_0280A0_COLOR_32_32_FLOAT: c_uint = 0x0000001E;
pub const V_0280A0_COLOR_16_16_16_16: c_uint = 0x0000001F;
pub const V_0280A0_COLOR_16_16_16_16_FLOAT: c_uint = 0x00000020;
pub const V_0280A0_COLOR_32_32_32_32: c_uint = 0x00000022;
pub const V_0280A0_COLOR_32_32_32_32_FLOAT: c_uint = 0x00000023;

pub const C_0280A0_ARRAY_MODE: c_uint = 0xFFFFF0FF;
pub const V_0280A0_ARRAY_LINEAR_GENERAL: c_uint = 0x00000000;
pub const V_0280A0_ARRAY_LINEAR_ALIGNED: c_uint = 0x00000001;
pub const V_0280A0_ARRAY_1D_TILED_THIN1: c_uint = 0x00000002;
pub const V_0280A0_ARRAY_2D_TILED_THIN1: c_uint = 0x00000004;

pub const C_0280A0_NUMBER_TYPE: c_uint = 0xFFFF8FFF;

pub const C_0280A0_READ_SIZE: c_uint = 0xFFFF7FFF;

pub const C_0280A0_COMP_SWAP: c_uint = 0xFFFCFFFF;

pub const C_0280A0_TILE_MODE: c_uint = 0xFFF3FFFF;
pub const V_0280A0_TILE_DISABLE: c_int = 0;
pub const V_0280A0_CLEAR_ENABLE: c_int = 1;
pub const V_0280A0_FRAG_ENABLE: c_int = 2;

pub const C_0280A0_BLEND_CLAMP: c_uint = 0xFFEFFFFF;

pub const C_0280A0_CLEAR_COLOR: c_uint = 0xFFDFFFFF;

pub const C_0280A0_BLEND_BYPASS: c_uint = 0xFFBFFFFF;

pub const C_0280A0_BLEND_FLOAT32: c_uint = 0xFF7FFFFF;

pub const C_0280A0_SIMPLE_FLOAT: c_uint = 0xFEFFFFFF;

pub const C_0280A0_ROUND_MODE: c_uint = 0xFDFFFFFF;

pub const C_0280A0_TILE_COMPACT: c_uint = 0xFBFFFFFF;

pub const C_0280A0_SOURCE_FORMAT: c_uint = 0xF7FFFFFF;
pub const R_0280A4_CB_COLOR1_INFO: c_uint = 0x0280A4;
pub const R_0280A8_CB_COLOR2_INFO: c_uint = 0x0280A8;
pub const R_0280AC_CB_COLOR3_INFO: c_uint = 0x0280AC;
pub const R_0280B0_CB_COLOR4_INFO: c_uint = 0x0280B0;
pub const R_0280B4_CB_COLOR5_INFO: c_uint = 0x0280B4;
pub const R_0280B8_CB_COLOR6_INFO: c_uint = 0x0280B8;
pub const R_0280BC_CB_COLOR7_INFO: c_uint = 0x0280BC;
pub const R_028060_CB_COLOR0_SIZE: c_uint = 0x028060;

pub const C_028060_PITCH_TILE_MAX: c_uint = 0xFFFFFC00;

pub const C_028060_SLICE_TILE_MAX: c_uint = 0xC00003FF;
pub const R_028064_CB_COLOR1_SIZE: c_uint = 0x028064;
pub const R_028068_CB_COLOR2_SIZE: c_uint = 0x028068;
pub const R_02806C_CB_COLOR3_SIZE: c_uint = 0x02806C;
pub const R_028070_CB_COLOR4_SIZE: c_uint = 0x028070;
pub const R_028074_CB_COLOR5_SIZE: c_uint = 0x028074;
pub const R_028078_CB_COLOR6_SIZE: c_uint = 0x028078;
pub const R_02807C_CB_COLOR7_SIZE: c_uint = 0x02807C;
pub const R_028238_CB_TARGET_MASK: c_uint = 0x028238;

pub const C_028238_TARGET0_ENABLE: c_uint = 0xFFFFFFF0;

pub const C_028238_TARGET1_ENABLE: c_uint = 0xFFFFFF0F;

pub const C_028238_TARGET2_ENABLE: c_uint = 0xFFFFF0FF;

pub const C_028238_TARGET3_ENABLE: c_uint = 0xFFFF0FFF;

pub const C_028238_TARGET4_ENABLE: c_uint = 0xFFF0FFFF;

pub const C_028238_TARGET5_ENABLE: c_uint = 0xFF0FFFFF;

pub const C_028238_TARGET6_ENABLE: c_uint = 0xF0FFFFFF;

pub const C_028238_TARGET7_ENABLE: c_uint = 0x0FFFFFFF;
pub const R_02823C_CB_SHADER_MASK: c_uint = 0x02823C;

pub const C_02823C_OUTPUT0_ENABLE: c_uint = 0xFFFFFFF0;

pub const C_02823C_OUTPUT1_ENABLE: c_uint = 0xFFFFFF0F;

pub const C_02823C_OUTPUT2_ENABLE: c_uint = 0xFFFFF0FF;

pub const C_02823C_OUTPUT3_ENABLE: c_uint = 0xFFFF0FFF;

pub const C_02823C_OUTPUT4_ENABLE: c_uint = 0xFFF0FFFF;

pub const C_02823C_OUTPUT5_ENABLE: c_uint = 0xFF0FFFFF;

pub const C_02823C_OUTPUT6_ENABLE: c_uint = 0xF0FFFFFF;

pub const C_02823C_OUTPUT7_ENABLE: c_uint = 0x0FFFFFFF;
pub const R_028AB0_VGT_STRMOUT_EN: c_uint = 0x028AB0;

pub const C_028AB0_STREAMOUT: c_uint = 0xFFFFFFFE;
pub const R_028B20_VGT_STRMOUT_BUFFER_EN: c_uint = 0x028B20;

pub const C_028B20_BUFFER_0_EN: c_uint = 0xFFFFFFFE;

pub const C_028B20_BUFFER_1_EN: c_uint = 0xFFFFFFFD;

pub const C_028B20_BUFFER_2_EN: c_uint = 0xFFFFFFFB;

pub const C_028B20_BUFFER_3_EN: c_uint = 0xFFFFFFF7;

pub const C_028B20_SIZE: c_uint = 0x00000000;
pub const R_038000_SQ_TEX_RESOURCE_WORD0_0: c_uint = 0x038000;

pub const C_038000_DIM: c_uint = 0xFFFFFFF8;
pub const V_038000_SQ_TEX_DIM_1D: c_uint = 0x00000000;
pub const V_038000_SQ_TEX_DIM_2D: c_uint = 0x00000001;
pub const V_038000_SQ_TEX_DIM_3D: c_uint = 0x00000002;
pub const V_038000_SQ_TEX_DIM_CUBEMAP: c_uint = 0x00000003;
pub const V_038000_SQ_TEX_DIM_1D_ARRAY: c_uint = 0x00000004;
pub const V_038000_SQ_TEX_DIM_2D_ARRAY: c_uint = 0x00000005;
pub const V_038000_SQ_TEX_DIM_2D_MSAA: c_uint = 0x00000006;
pub const V_038000_SQ_TEX_DIM_2D_ARRAY_MSAA: c_uint = 0x00000007;

pub const C_038000_TILE_MODE: c_uint = 0xFFFFFF87;
pub const V_038000_ARRAY_LINEAR_GENERAL: c_uint = 0x00000000;
pub const V_038000_ARRAY_LINEAR_ALIGNED: c_uint = 0x00000001;
pub const V_038000_ARRAY_1D_TILED_THIN1: c_uint = 0x00000002;
pub const V_038000_ARRAY_2D_TILED_THIN1: c_uint = 0x00000004;

pub const C_038000_TILE_TYPE: c_uint = 0xFFFFFF7F;

pub const C_038000_PITCH: c_uint = 0xFFF800FF;

pub const C_038000_TEX_WIDTH: c_uint = 0x0007FFFF;
pub const R_038004_SQ_TEX_RESOURCE_WORD1_0: c_uint = 0x038004;

pub const C_038004_TEX_HEIGHT: c_uint = 0xFFFFE000;

pub const C_038004_TEX_DEPTH: c_uint = 0xFC001FFF;

pub const C_038004_DATA_FORMAT: c_uint = 0x03FFFFFF;
pub const V_038004_COLOR_INVALID: c_uint = 0x00000000;
pub const V_038004_COLOR_8: c_uint = 0x00000001;
pub const V_038004_COLOR_4_4: c_uint = 0x00000002;
pub const V_038004_COLOR_3_3_2: c_uint = 0x00000003;
pub const V_038004_COLOR_16: c_uint = 0x00000005;
pub const V_038004_COLOR_16_FLOAT: c_uint = 0x00000006;
pub const V_038004_COLOR_8_8: c_uint = 0x00000007;
pub const V_038004_COLOR_5_6_5: c_uint = 0x00000008;
pub const V_038004_COLOR_6_5_5: c_uint = 0x00000009;
pub const V_038004_COLOR_1_5_5_5: c_uint = 0x0000000A;
pub const V_038004_COLOR_4_4_4_4: c_uint = 0x0000000B;
pub const V_038004_COLOR_5_5_5_1: c_uint = 0x0000000C;
pub const V_038004_COLOR_32: c_uint = 0x0000000D;
pub const V_038004_COLOR_32_FLOAT: c_uint = 0x0000000E;
pub const V_038004_COLOR_16_16: c_uint = 0x0000000F;
pub const V_038004_COLOR_16_16_FLOAT: c_uint = 0x00000010;
pub const V_038004_COLOR_8_24: c_uint = 0x00000011;
pub const V_038004_COLOR_8_24_FLOAT: c_uint = 0x00000012;
pub const V_038004_COLOR_24_8: c_uint = 0x00000013;
pub const V_038004_COLOR_24_8_FLOAT: c_uint = 0x00000014;
pub const V_038004_COLOR_10_11_11: c_uint = 0x00000015;
pub const V_038004_COLOR_10_11_11_FLOAT: c_uint = 0x00000016;
pub const V_038004_COLOR_11_11_10: c_uint = 0x00000017;
pub const V_038004_COLOR_11_11_10_FLOAT: c_uint = 0x00000018;
pub const V_038004_COLOR_2_10_10_10: c_uint = 0x00000019;
pub const V_038004_COLOR_8_8_8_8: c_uint = 0x0000001A;
pub const V_038004_COLOR_10_10_10_2: c_uint = 0x0000001B;
pub const V_038004_COLOR_X24_8_32_FLOAT: c_uint = 0x0000001C;
pub const V_038004_COLOR_32_32: c_uint = 0x0000001D;
pub const V_038004_COLOR_32_32_FLOAT: c_uint = 0x0000001E;
pub const V_038004_COLOR_16_16_16_16: c_uint = 0x0000001F;
pub const V_038004_COLOR_16_16_16_16_FLOAT: c_uint = 0x00000020;
pub const V_038004_COLOR_32_32_32_32: c_uint = 0x00000022;
pub const V_038004_COLOR_32_32_32_32_FLOAT: c_uint = 0x00000023;
pub const V_038004_FMT_1: c_uint = 0x00000025;
pub const V_038004_FMT_GB_GR: c_uint = 0x00000027;
pub const V_038004_FMT_BG_RG: c_uint = 0x00000028;
pub const V_038004_FMT_32_AS_8: c_uint = 0x00000029;
pub const V_038004_FMT_32_AS_8_8: c_uint = 0x0000002A;
pub const V_038004_FMT_5_9_9_9_SHAREDEXP: c_uint = 0x0000002B;
pub const V_038004_FMT_8_8_8: c_uint = 0x0000002C;
pub const V_038004_FMT_16_16_16: c_uint = 0x0000002D;
pub const V_038004_FMT_16_16_16_FLOAT: c_uint = 0x0000002E;
pub const V_038004_FMT_32_32_32: c_uint = 0x0000002F;
pub const V_038004_FMT_32_32_32_FLOAT: c_uint = 0x00000030;
pub const V_038004_FMT_BC1: c_uint = 0x00000031;
pub const V_038004_FMT_BC2: c_uint = 0x00000032;
pub const V_038004_FMT_BC3: c_uint = 0x00000033;
pub const V_038004_FMT_BC4: c_uint = 0x00000034;
pub const V_038004_FMT_BC5: c_uint = 0x00000035;
pub const V_038004_FMT_BC6: c_uint = 0x00000036;
pub const V_038004_FMT_BC7: c_uint = 0x00000037;
pub const V_038004_FMT_32_AS_32_32_32_32: c_uint = 0x00000038;
pub const R_038010_SQ_TEX_RESOURCE_WORD4_0: c_uint = 0x038010;

pub const C_038010_FORMAT_COMP_X: c_uint = 0xFFFFFFFC;

pub const C_038010_FORMAT_COMP_Y: c_uint = 0xFFFFFFF3;

pub const C_038010_FORMAT_COMP_Z: c_uint = 0xFFFFFFCF;

pub const C_038010_FORMAT_COMP_W: c_uint = 0xFFFFFF3F;

pub const C_038010_NUM_FORMAT_ALL: c_uint = 0xFFFFFCFF;

pub const C_038010_SRF_MODE_ALL: c_uint = 0xFFFFFBFF;

pub const C_038010_FORCE_DEGAMMA: c_uint = 0xFFFFF7FF;

pub const C_038010_ENDIAN_SWAP: c_uint = 0xFFFFCFFF;

pub const C_038010_REQUEST_SIZE: c_uint = 0xFFFF3FFF;

pub const C_038010_DST_SEL_X: c_uint = 0xFFF8FFFF;

pub const C_038010_DST_SEL_Y: c_uint = 0xFFC7FFFF;

pub const C_038010_DST_SEL_Z: c_uint = 0xFE3FFFFF;

pub const C_038010_DST_SEL_W: c_uint = 0xF1FFFFFF;

pub const C_038010_BASE_LEVEL: c_uint = 0x0FFFFFFF;
pub const R_038014_SQ_TEX_RESOURCE_WORD5_0: c_uint = 0x038014;

pub const C_038014_LAST_LEVEL: c_uint = 0xFFFFFFF0;

pub const C_038014_BASE_ARRAY: c_uint = 0xFFFE000F;

pub const C_038014_LAST_ARRAY: c_uint = 0xC001FFFF;
pub const R_0288A8_SQ_ESGS_RING_ITEMSIZE: c_uint = 0x0288A8;

pub const C_0288A8_ITEMSIZE: c_uint = 0xFFFF8000;
pub const R_008C44_SQ_ESGS_RING_SIZE: c_uint = 0x008C44;

pub const C_008C44_MEM_SIZE: c_uint = 0x00000000;
pub const R_0288B0_SQ_ESTMP_RING_ITEMSIZE: c_uint = 0x0288B0;

pub const C_0288B0_ITEMSIZE: c_uint = 0xFFFF8000;
pub const R_008C54_SQ_ESTMP_RING_SIZE: c_uint = 0x008C54;

pub const C_008C54_MEM_SIZE: c_uint = 0x00000000;
pub const R_0288C0_SQ_FBUF_RING_ITEMSIZE: c_uint = 0x0288C0;

pub const C_0288C0_ITEMSIZE: c_uint = 0xFFFF8000;
pub const R_008C74_SQ_FBUF_RING_SIZE: c_uint = 0x008C74;

pub const C_008C74_MEM_SIZE: c_uint = 0x00000000;
pub const R_0288B4_SQ_GSTMP_RING_ITEMSIZE: c_uint = 0x0288B4;

pub const C_0288B4_ITEMSIZE: c_uint = 0xFFFF8000;
pub const R_008C5C_SQ_GSTMP_RING_SIZE: c_uint = 0x008C5C;

pub const C_008C5C_MEM_SIZE: c_uint = 0x00000000;
pub const R_0288AC_SQ_GSVS_RING_ITEMSIZE: c_uint = 0x0288AC;

pub const C_0288AC_ITEMSIZE: c_uint = 0xFFFF8000;
pub const R_008C4C_SQ_GSVS_RING_SIZE: c_uint = 0x008C4C;

pub const C_008C4C_MEM_SIZE: c_uint = 0x00000000;
pub const R_0288BC_SQ_PSTMP_RING_ITEMSIZE: c_uint = 0x0288BC;

pub const C_0288BC_ITEMSIZE: c_uint = 0xFFFF8000;
pub const R_008C6C_SQ_PSTMP_RING_SIZE: c_uint = 0x008C6C;

pub const C_008C6C_MEM_SIZE: c_uint = 0x00000000;
pub const R_0288C4_SQ_REDUC_RING_ITEMSIZE: c_uint = 0x0288C4;

pub const C_0288C4_ITEMSIZE: c_uint = 0xFFFF8000;
pub const R_008C7C_SQ_REDUC_RING_SIZE: c_uint = 0x008C7C;

pub const C_008C7C_MEM_SIZE: c_uint = 0x00000000;
pub const R_0288B8_SQ_VSTMP_RING_ITEMSIZE: c_uint = 0x0288B8;

pub const C_0288B8_ITEMSIZE: c_uint = 0xFFFF8000;
pub const R_008C64_SQ_VSTMP_RING_SIZE: c_uint = 0x008C64;

pub const C_008C64_MEM_SIZE: c_uint = 0x00000000;
pub const R_0288C8_SQ_GS_VERT_ITEMSIZE: c_uint = 0x0288C8;

pub const C_0288C8_ITEMSIZE: c_uint = 0xFFFF8000;
pub const R_028010_DB_DEPTH_INFO: c_uint = 0x028010;

pub const C_028010_FORMAT: c_uint = 0xFFFFFFF8;
pub const V_028010_DEPTH_INVALID: c_uint = 0x00000000;
pub const V_028010_DEPTH_16: c_uint = 0x00000001;
pub const V_028010_DEPTH_X8_24: c_uint = 0x00000002;
pub const V_028010_DEPTH_8_24: c_uint = 0x00000003;
pub const V_028010_DEPTH_X8_24_FLOAT: c_uint = 0x00000004;
pub const V_028010_DEPTH_8_24_FLOAT: c_uint = 0x00000005;
pub const V_028010_DEPTH_32_FLOAT: c_uint = 0x00000006;
pub const V_028010_DEPTH_X24_8_32_FLOAT: c_uint = 0x00000007;

pub const C_028010_READ_SIZE: c_uint = 0xFFFFFFF7;

pub const C_028010_ARRAY_MODE: c_uint = 0xFFF87FFF;
pub const V_028010_ARRAY_1D_TILED_THIN1: c_uint = 0x00000002;
pub const V_028010_ARRAY_2D_TILED_THIN1: c_uint = 0x00000004;

pub const C_028010_TILE_SURFACE_ENABLE: c_uint = 0xFDFFFFFF;

pub const C_028010_TILE_COMPACT: c_uint = 0xFBFFFFFF;

pub const C_028010_ZRANGE_PRECISION: c_uint = 0x7FFFFFFF;
pub const R_028000_DB_DEPTH_SIZE: c_uint = 0x028000;

pub const C_028000_PITCH_TILE_MAX: c_uint = 0xFFFFFC00;

pub const C_028000_SLICE_TILE_MAX: c_uint = 0xC00003FF;
pub const R_028004_DB_DEPTH_VIEW: c_uint = 0x028004;

pub const C_028004_SLICE_START: c_uint = 0xFFFFF800;

pub const C_028004_SLICE_MAX: c_uint = 0xFF001FFF;
pub const R_028800_DB_DEPTH_CONTROL: c_uint = 0x028800;

pub const C_028800_STENCIL_ENABLE: c_uint = 0xFFFFFFFE;

pub const C_028800_Z_ENABLE: c_uint = 0xFFFFFFFD;

pub const C_028800_Z_WRITE_ENABLE: c_uint = 0xFFFFFFFB;

pub const C_028800_ZFUNC: c_uint = 0xFFFFFF8F;

pub const C_028800_BACKFACE_ENABLE: c_uint = 0xFFFFFF7F;

pub const C_028800_STENCILFUNC: c_uint = 0xFFFFF8FF;

pub const C_028800_STENCILFAIL: c_uint = 0xFFFFC7FF;

pub const C_028800_STENCILZPASS: c_uint = 0xFFFE3FFF;

pub const C_028800_STENCILZFAIL: c_uint = 0xFFF1FFFF;

pub const C_028800_STENCILFUNC_BF: c_uint = 0xFF8FFFFF;

pub const C_028800_STENCILFAIL_BF: c_uint = 0xFC7FFFFF;

pub const C_028800_STENCILZPASS_BF: c_uint = 0xE3FFFFFF;

pub const C_028800_STENCILZFAIL_BF: c_uint = 0x1FFFFFFF;
