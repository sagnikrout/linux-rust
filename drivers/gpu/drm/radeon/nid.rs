//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/nid.h
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
pub const CAYMAN_MAX_SH_GPRS: c_int = 256;
pub const CAYMAN_MAX_TEMP_GPRS: c_int = 16;
pub const CAYMAN_MAX_SH_THREADS: c_int = 256;
pub const CAYMAN_MAX_SH_STACK_ENTRIES: c_int = 4096;
pub const CAYMAN_MAX_FRC_EOV_CNT: c_int = 16384;
pub const CAYMAN_MAX_BACKENDS: c_int = 8;
pub const CAYMAN_MAX_BACKENDS_MASK: c_uint = 0xFF;
pub const CAYMAN_MAX_BACKENDS_PER_SE_MASK: c_uint = 0xF;
pub const CAYMAN_MAX_SIMDS: c_int = 16;
pub const CAYMAN_MAX_SIMDS_MASK: c_uint = 0xFFFF;
pub const CAYMAN_MAX_SIMDS_PER_SE_MASK: c_uint = 0xFFF;
pub const CAYMAN_MAX_PIPES: c_int = 8;
pub const CAYMAN_MAX_PIPES_MASK: c_uint = 0xFF;
pub const CAYMAN_MAX_LDS_NUM: c_uint = 0xFFFF;
pub const CAYMAN_MAX_TCC: c_int = 16;
pub const CAYMAN_MAX_TCC_MASK: c_uint = 0xFF;
pub const CAYMAN_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x02011003;
pub const ARUBA_GB_ADDR_CONFIG_GOLDEN: c_uint = 0x12010001;
pub const DMIF_ADDR_CONFIG: c_uint = 0xBD4;
// fusion vce clocks
pub const CG_ECLK_CNTL: c_uint = 0x620;

pub const CG_ECLK_STATUS: c_uint = 0x624;

// DCE6 only
pub const DMIF_ADDR_CALC: c_uint = 0xC00;
pub const SRBM_GFX_CNTL: c_uint = 0x0E44;

pub const SRBM_STATUS: c_uint = 0x0E50;

pub const SRBM_SOFT_RESET: c_uint = 0x0E60;

pub const SRBM_READ_ERROR: c_uint = 0xE98;
pub const SRBM_INT_CNTL: c_uint = 0xEA0;
pub const SRBM_INT_ACK: c_uint = 0xEA8;
pub const SRBM_STATUS2: c_uint = 0x0EC4;

pub const VM_CONTEXT0_REQUEST_RESPONSE: c_uint = 0x1470;

pub const RESPONSE_TYPE_MASK: c_uint = 0x000000F0;
pub const RESPONSE_TYPE_SHIFT: c_int = 4;
pub const VM_L2_CNTL: c_uint = 0x1400;

// CONTEXT1_IDENTITY_ACCESS_MODE
// 0 physical = logical
// 1 logical via context1 page table
// 2 inside identity aperture use translation, outside physical = logical
// 3 inside identity aperture physical = logical, outside use translation
//
pub const VM_L2_CNTL2: c_uint = 0x1404;

pub const VM_L2_CNTL3: c_uint = 0x1408;

pub const VM_L2_STATUS: c_uint = 0x140C;

pub const VM_CONTEXT0_CNTL: c_uint = 0x1410;

pub const VM_CONTEXT1_CNTL: c_uint = 0x1414;
pub const VM_CONTEXT0_CNTL2: c_uint = 0x1430;
pub const VM_CONTEXT1_CNTL2: c_uint = 0x1434;
pub const VM_INVALIDATE_REQUEST: c_uint = 0x1478;
pub const VM_INVALIDATE_RESPONSE: c_uint = 0x147c;
pub const VM_CONTEXT1_PROTECTION_FAULT_ADDR: c_uint = 0x14FC;
pub const VM_CONTEXT1_PROTECTION_FAULT_STATUS: c_uint = 0x14DC;

pub const PROTECTIONS_SHIFT: c_int = 0;
// bit 0: range
// bit 2: pde0
// bit 3: valid
// bit 4: read
// bit 5: write
//

pub const MEMORY_CLIENT_ID_SHIFT: c_int = 12;

pub const MEMORY_CLIENT_RW_SHIFT: c_int = 24;

pub const FAULT_VMID_SHIFT: c_int = 25;
pub const VM_CONTEXT0_PROTECTION_FAULT_DEFAULT_ADDR: c_uint = 0x1518;
pub const VM_CONTEXT1_PROTECTION_FAULT_DEFAULT_ADDR: c_uint = 0x151c;
pub const VM_CONTEXT0_PAGE_TABLE_BASE_ADDR: c_uint = 0x153C;
pub const VM_CONTEXT0_PAGE_TABLE_START_ADDR: c_uint = 0x155C;
pub const VM_CONTEXT0_PAGE_TABLE_END_ADDR: c_uint = 0x157C;
pub const MC_SHARED_CHMAP: c_uint = 0x2004;
pub const NOOFCHAN_SHIFT: c_int = 12;
pub const NOOFCHAN_MASK: c_uint = 0x00003000;
pub const MC_SHARED_CHREMAP: c_uint = 0x2008;
pub const MC_VM_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x2034;
pub const MC_VM_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x2038;
pub const MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR: c_uint = 0x203C;
pub const MC_VM_MX_L1_TLB_CNTL: c_uint = 0x2064;

pub const FUS_MC_VM_FB_OFFSET: c_uint = 0x2068;
pub const MC_SHARED_BLACKOUT_CNTL: c_uint = 0x20ac;
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

pub const MC_SEQ_SUP_CNTL: c_uint = 0x28c8;

pub const MC_SEQ_SUP_PGM: c_uint = 0x28cc;
pub const MC_IO_PAD_CNTL_D0: c_uint = 0x29d0;

pub const MC_SEQ_MISC0: c_uint = 0x2a00;
pub const MC_SEQ_MISC0_GDDR5_SHIFT: c_int = 28;
pub const MC_SEQ_MISC0_GDDR5_MASK: c_uint = 0xf0000000;
pub const MC_SEQ_MISC0_GDDR5_VALUE: c_int = 5;
pub const MC_SEQ_IO_DEBUG_INDEX: c_uint = 0x2a44;
pub const MC_SEQ_IO_DEBUG_DATA: c_uint = 0x2a48;
pub const HDP_HOST_PATH_CNTL: c_uint = 0x2C00;
pub const HDP_NONSURFACE_BASE: c_uint = 0x2C04;
pub const HDP_NONSURFACE_INFO: c_uint = 0x2C08;
pub const HDP_NONSURFACE_SIZE: c_uint = 0x2C0C;
pub const HDP_ADDR_CONFIG: c_uint = 0x2F48;
pub const HDP_MISC_CNTL: c_uint = 0x2F4C;

pub const CC_SYS_RB_BACKEND_DISABLE: c_uint = 0x3F88;
pub const GC_USER_SYS_RB_BACKEND_DISABLE: c_uint = 0x3F8C;
pub const CGTS_SYS_TCC_DISABLE: c_uint = 0x3F90;
pub const CGTS_USER_SYS_TCC_DISABLE: c_uint = 0x3F94;
pub const RLC_GFX_INDEX: c_uint = 0x3FC4;
pub const CONFIG_MEMSIZE: c_uint = 0x5428;
pub const HDP_MEM_COHERENCY_FLUSH_CNTL: c_uint = 0x5480;
pub const HDP_REG_COHERENCY_FLUSH_CNTL: c_uint = 0x54A0;
pub const GRBM_CNTL: c_uint = 0x8000;

pub const GRBM_STATUS: c_uint = 0x8010;
pub const CMDFIFO_AVAIL_MASK: c_uint = 0x0000000F;

pub const GRBM_STATUS_SE0: c_uint = 0x8014;
pub const GRBM_STATUS_SE1: c_uint = 0x8018;

pub const GRBM_SOFT_RESET: c_uint = 0x8020;

pub const GRBM_GFX_INDEX: c_uint = 0x802C;

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
pub const CP_COHER_CNTL2: c_uint = 0x85E8;
pub const CP_STALLED_STAT1: c_uint = 0x8674;
pub const CP_STALLED_STAT2: c_uint = 0x8678;
pub const CP_BUSY_STAT: c_uint = 0x867C;
pub const CP_STAT: c_uint = 0x8680;
pub const CP_ME_CNTL: c_uint = 0x86D8;

pub const CP_RB2_RPTR: c_uint = 0x86f8;
pub const CP_RB1_RPTR: c_uint = 0x86fc;
pub const CP_RB0_RPTR: c_uint = 0x8700;
pub const CP_RB_WPTR_DELAY: c_uint = 0x8704;
pub const CP_MEQ_THRESHOLDS: c_uint = 0x8764;

pub const CP_PERFMON_CNTL: c_uint = 0x87FC;
pub const VGT_CACHE_INVALIDATION: c_uint = 0x88C4;

pub const VC_ONLY: c_int = 0;
pub const TC_ONLY: c_int = 1;
pub const VC_AND_TC: c_int = 2;

pub const NO_AUTO: c_int = 0;
pub const ES_AUTO: c_int = 1;
pub const GS_AUTO: c_int = 2;
pub const ES_AND_GS_AUTO: c_int = 3;
pub const VGT_GS_VERTEX_REUSE: c_uint = 0x88D4;
pub const CC_GC_SHADER_PIPE_CONFIG: c_uint = 0x8950;
pub const GC_USER_SHADER_PIPE_CONFIG: c_uint = 0x8954;

pub const INACTIVE_QD_PIPES_MASK: c_uint = 0x0000FF00;
pub const INACTIVE_QD_PIPES_SHIFT: c_int = 8;

pub const INACTIVE_SIMDS_MASK: c_uint = 0xFFFF0000;
pub const INACTIVE_SIMDS_SHIFT: c_int = 16;
pub const VGT_PRIMITIVE_TYPE: c_uint = 0x8958;
pub const VGT_NUM_INSTANCES: c_uint = 0x8974;
pub const VGT_TF_RING_SIZE: c_uint = 0x8988;
pub const VGT_OFFCHIP_LDS_BASE: c_uint = 0x89b4;
pub const PA_SC_LINE_STIPPLE_STATE: c_uint = 0x8B10;
pub const PA_CL_ENHANCE: c_uint = 0x8A14;

pub const PA_SC_FIFO_SIZE: c_uint = 0x8BCC;

pub const PA_SC_FORCE_EOV_MAX_CNTS: c_uint = 0x8B24;

pub const SQ_CONFIG: c_uint = 0x8C00;

pub const SQ_GPR_RESOURCE_MGMT_1: c_uint = 0x8C04;

pub const SQ_ESGS_RING_SIZE: c_uint = 0x8c44;
pub const SQ_GSVS_RING_SIZE: c_uint = 0x8c4c;
pub const SQ_ESTMP_RING_BASE: c_uint = 0x8c50;
pub const SQ_ESTMP_RING_SIZE: c_uint = 0x8c54;
pub const SQ_GSTMP_RING_BASE: c_uint = 0x8c58;
pub const SQ_GSTMP_RING_SIZE: c_uint = 0x8c5c;
pub const SQ_VSTMP_RING_BASE: c_uint = 0x8c60;
pub const SQ_VSTMP_RING_SIZE: c_uint = 0x8c64;
pub const SQ_PSTMP_RING_BASE: c_uint = 0x8c68;
pub const SQ_PSTMP_RING_SIZE: c_uint = 0x8c6c;
pub const SQ_MS_FIFO_SIZES: c_uint = 0x8CF0;

pub const SQ_LSTMP_RING_BASE: c_uint = 0x8e10;
pub const SQ_LSTMP_RING_SIZE: c_uint = 0x8e14;
pub const SQ_HSTMP_RING_BASE: c_uint = 0x8e18;
pub const SQ_HSTMP_RING_SIZE: c_uint = 0x8e1c;
pub const SQ_DYN_GPR_CNTL_PS_FLUSH_REQ: c_uint = 0x8D8C;

pub const SQ_CONST_MEM_BASE: c_uint = 0x8df8;
pub const SX_EXPORT_BUFFER_SIZES: c_uint = 0x900C;

pub const SX_DEBUG_1: c_uint = 0x9058;

pub const SPI_CONFIG_CNTL: c_uint = 0x9100;

pub const SPI_CONFIG_CNTL_1: c_uint = 0x913C;

pub const CGTS_TCC_DISABLE: c_uint = 0x9148;
pub const CGTS_USER_TCC_DISABLE: c_uint = 0x914C;
pub const TCC_DISABLE_MASK: c_uint = 0xFFFF0000;
pub const TCC_DISABLE_SHIFT: c_int = 16;
pub const CGTS_SM_CTRL_REG: c_uint = 0x9150;

pub const TA_CNTL_AUX: c_uint = 0x9508;

pub const TCP_CHAN_STEER_LO: c_uint = 0x960c;
pub const TCP_CHAN_STEER_HI: c_uint = 0x9610;
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

pub const NUM_LOWER_PIPES_MASK: c_uint = 0x40000000;
pub const NUM_LOWER_PIPES_SHIFT: c_int = 30;
pub const GB_BACKEND_MAP: c_uint = 0x98FC;
pub const CB_PERF_CTR0_SEL_0: c_uint = 0x9A20;
pub const CB_PERF_CTR0_SEL_1: c_uint = 0x9A24;
pub const CB_PERF_CTR1_SEL_0: c_uint = 0x9A28;
pub const CB_PERF_CTR1_SEL_1: c_uint = 0x9A2C;
pub const CB_PERF_CTR2_SEL_0: c_uint = 0x9A30;
pub const CB_PERF_CTR2_SEL_1: c_uint = 0x9A34;
pub const CB_PERF_CTR3_SEL_0: c_uint = 0x9A38;
pub const CB_PERF_CTR3_SEL_1: c_uint = 0x9A3C;
pub const GC_USER_RB_BACKEND_DISABLE: c_uint = 0x9B7C;
pub const BACKEND_DISABLE_MASK: c_uint = 0x00FF0000;
pub const BACKEND_DISABLE_SHIFT: c_int = 16;
pub const SMX_DC_CTL0: c_uint = 0xA020;

pub const SMX_EVENT_CTL: c_uint = 0xA02C;

pub const CP_RB0_BASE: c_uint = 0xC100;
pub const CP_RB0_CNTL: c_uint = 0xC104;

pub const CP_RB0_RPTR_ADDR: c_uint = 0xC10C;
pub const CP_RB0_RPTR_ADDR_HI: c_uint = 0xC110;
pub const CP_RB0_WPTR: c_uint = 0xC114;
pub const CP_INT_CNTL: c_uint = 0xC124;

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
pub const CP_PFP_UCODE_ADDR: c_uint = 0xC150;
pub const CP_PFP_UCODE_DATA: c_uint = 0xC154;
pub const CP_ME_RAM_RADDR: c_uint = 0xC158;
pub const CP_ME_RAM_WADDR: c_uint = 0xC15C;
pub const CP_ME_RAM_DATA: c_uint = 0xC160;
pub const CP_DEBUG: c_uint = 0xC1FC;
pub const VGT_EVENT_INITIATOR: c_uint = 0x28a90;

// TN SMU registers
pub const TN_CURRENT_GNB_TEMP: c_uint = 0x1F390;
// pm registers
pub const SMC_MSG: c_uint = 0x20c;

pub const HOST_SMC_MSG_SHIFT: c_int = 0;

pub const HOST_SMC_RESP_SHIFT: c_int = 8;

pub const SMC_HOST_MSG_SHIFT: c_int = 16;

pub const SMC_HOST_RESP_SHIFT: c_int = 24;
pub const CG_SPLL_FUNC_CNTL: c_uint = 0x600;

pub const SPLL_PDIV_A_SHIFT: c_int = 20;
pub const CG_SPLL_FUNC_CNTL_2: c_uint = 0x604;

pub const CG_SPLL_FUNC_CNTL_3: c_uint = 0x608;

pub const SPLL_FB_DIV_SHIFT: c_int = 0;

pub const MPLL_CNTL_MODE: c_uint = 0x61c;

pub const MPLL_AD_FUNC_CNTL: c_uint = 0x624;

pub const MPLL_AD_FUNC_CNTL_2: c_uint = 0x628;

pub const MPLL_DQ_FUNC_CNTL: c_uint = 0x62c;
pub const MPLL_DQ_FUNC_CNTL_2: c_uint = 0x630;
pub const GENERAL_PWRMGT: c_uint = 0x63c;

pub const SCLK_PWRMGT_CNTL: c_uint = 0x644;

pub const MCLK_PWRMGT_CNTL: c_uint = 0x648;

pub const DLL_CNTL: c_uint = 0x64c;

pub const TARGET_AND_CURRENT_PROFILE_INDEX: c_uint = 0x66c;

pub const CG_AT: c_uint = 0x6d4;

pub const CG_BIF_REQ_AND_RSP: c_uint = 0x7f4;

pub const CG_CLIENT_REQ_SHIFT: c_int = 0;

pub const CG_CLIENT_RESP_SHIFT: c_int = 8;

pub const CLIENT_CG_REQ_SHIFT: c_int = 16;

pub const CLIENT_CG_RESP_SHIFT: c_int = 24;
pub const CG_SPLL_SPREAD_SPECTRUM: c_uint = 0x790;

pub const CLK_S_SHIFT: c_int = 4;
pub const CG_SPLL_SPREAD_SPECTRUM_2: c_uint = 0x794;

pub const CLK_V_SHIFT: c_int = 0;
pub const SMC_SCRATCH0: c_uint = 0x81c;
pub const CG_SPLL_FUNC_CNTL_4: c_uint = 0x850;
pub const MPLL_SS1: c_uint = 0x85c;

pub const MPLL_SS2: c_uint = 0x860;

pub const CG_CAC_CTRL: c_uint = 0x88c;

pub const CG_IND_ADDR: c_uint = 0x8f8;
pub const CG_IND_DATA: c_uint = 0x8fc;
// CGIND regs
pub const CG_CGTT_LOCAL_0: c_uint = 0x00;
pub const CG_CGTT_LOCAL_1: c_uint = 0x01;
pub const MC_CG_CONFIG: c_uint = 0x25bc;

pub const INDEX_SHIFT: c_int = 6;
pub const MC_ARB_CAC_CNTL: c_uint = 0x2750;

pub const READ_WEIGHT_SHIFT: c_int = 1;

pub const WRITE_WEIGHT_SHIFT: c_int = 7;

pub const MC_ARB_DRAM_TIMING: c_uint = 0x2774;
pub const MC_ARB_DRAM_TIMING2: c_uint = 0x2778;
pub const MC_ARB_RFSH_RATE: c_uint = 0x27b0;

pub const POWERMODE0_SHIFT: c_int = 0;

pub const POWERMODE1_SHIFT: c_int = 8;

pub const POWERMODE2_SHIFT: c_int = 16;

pub const POWERMODE3_SHIFT: c_int = 24;
pub const MC_ARB_CG: c_uint = 0x27e8;

pub const CG_ARB_REQ_SHIFT: c_int = 0;

pub const CG_ARB_RESP_SHIFT: c_int = 8;

pub const ARB_CG_REQ_SHIFT: c_int = 16;

pub const ARB_CG_RESP_SHIFT: c_int = 24;
pub const MC_ARB_DRAM_TIMING_1: c_uint = 0x27f0;
pub const MC_ARB_DRAM_TIMING_2: c_uint = 0x27f4;
pub const MC_ARB_DRAM_TIMING_3: c_uint = 0x27f8;
pub const MC_ARB_DRAM_TIMING2_1: c_uint = 0x27fc;
pub const MC_ARB_DRAM_TIMING2_2: c_uint = 0x2800;
pub const MC_ARB_DRAM_TIMING2_3: c_uint = 0x2804;
pub const MC_ARB_BURST_TIME: c_uint = 0x2808;

pub const STATE0_SHIFT: c_int = 0;

pub const STATE1_SHIFT: c_int = 5;

pub const STATE2_SHIFT: c_int = 10;

pub const STATE3_SHIFT: c_int = 15;
pub const MC_CG_DATAPORT: c_uint = 0x2884;
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
pub const MC_SEQ_MISC0_GDDR5_SHIFT: c_int = 28;
pub const MC_SEQ_MISC0_GDDR5_MASK: c_uint = 0xf0000000;
pub const MC_SEQ_MISC0_GDDR5_VALUE: c_int = 5;
pub const MC_SEQ_MISC1: c_uint = 0x2a04;
pub const MC_SEQ_RESERVE_M: c_uint = 0x2a08;
pub const MC_PMG_CMD_EMRS: c_uint = 0x2a0c;
pub const MC_SEQ_MISC3: c_uint = 0x2a2c;
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
pub const MC_PMG_CMD_MRS2: c_uint = 0x2b5c;
pub const MC_SEQ_PMG_CMD_MRS2_LP: c_uint = 0x2b60;
pub const AUX_CONTROL: c_uint = 0x6200;

pub const AUX_SW_CONTROL: c_uint = 0x6204;

pub const AUX_SW_INTERRUPT_CONTROL: c_uint = 0x620c;

pub const AUX_SW_STATUS: c_uint = 0x6210;

pub const AUX_SW_DATA: c_uint = 0x6218;

pub const LB_SYNC_RESET_SEL: c_uint = 0x6b28;

pub const LB_SYNC_RESET_SEL_SHIFT: c_int = 0;
pub const DC_STUTTER_CNTL: c_uint = 0x6b30;

pub const SQ_CAC_THRESHOLD: c_uint = 0x8e4c;

pub const VSP_SHIFT: c_int = 0;

pub const VSP0_SHIFT: c_int = 8;

pub const GPR_SHIFT: c_int = 16;
pub const SQ_POWER_THROTTLE: c_uint = 0x8e58;

pub const MIN_POWER_SHIFT: c_int = 0;

pub const MAX_POWER_SHIFT: c_int = 0;
pub const SQ_POWER_THROTTLE2: c_uint = 0x8e5c;

pub const MAX_POWER_DELTA_SHIFT: c_int = 0;

pub const STI_SIZE_SHIFT: c_int = 16;

pub const LTI_RATIO_SHIFT: c_int = 27;
// CG indirect registers
pub const CG_CAC_REGION_1_WEIGHT_0: c_uint = 0x83;

pub const WEIGHT_TCP_SIG0_SHIFT: c_int = 0;

pub const WEIGHT_TCP_SIG1_SHIFT: c_int = 6;

pub const WEIGHT_TA_SIG_SHIFT: c_int = 12;
pub const CG_CAC_REGION_1_WEIGHT_1: c_uint = 0x84;

pub const WEIGHT_TCC_EN0_SHIFT: c_int = 0;

pub const WEIGHT_TCC_EN1_SHIFT: c_int = 6;

pub const WEIGHT_TCC_EN2_SHIFT: c_int = 12;

pub const WEIGHT_TCC_EN3_SHIFT: c_int = 18;
pub const CG_CAC_REGION_2_WEIGHT_0: c_uint = 0x85;

pub const WEIGHT_CB_EN0_SHIFT: c_int = 0;

pub const WEIGHT_CB_EN1_SHIFT: c_int = 6;

pub const WEIGHT_CB_EN2_SHIFT: c_int = 12;

pub const WEIGHT_CB_EN3_SHIFT: c_int = 18;
pub const CG_CAC_REGION_2_WEIGHT_1: c_uint = 0x86;

pub const WEIGHT_DB_SIG0_SHIFT: c_int = 0;

pub const WEIGHT_DB_SIG1_SHIFT: c_int = 6;

pub const WEIGHT_DB_SIG2_SHIFT: c_int = 12;

pub const WEIGHT_DB_SIG3_SHIFT: c_int = 18;
pub const CG_CAC_REGION_2_WEIGHT_2: c_uint = 0x87;

pub const WEIGHT_SXM_SIG0_SHIFT: c_int = 0;

pub const WEIGHT_SXM_SIG1_SHIFT: c_int = 6;

pub const WEIGHT_SXM_SIG2_SHIFT: c_int = 12;

pub const WEIGHT_SXS_SIG0_SHIFT: c_int = 18;

pub const WEIGHT_SXS_SIG1_SHIFT: c_int = 24;
pub const CG_CAC_REGION_3_WEIGHT_0: c_uint = 0x88;

pub const WEIGHT_XBR_0_SHIFT: c_int = 0;

pub const WEIGHT_XBR_1_SHIFT: c_int = 6;

pub const WEIGHT_XBR_2_SHIFT: c_int = 12;

pub const WEIGHT_SPI_SIG0_SHIFT: c_int = 18;
pub const CG_CAC_REGION_3_WEIGHT_1: c_uint = 0x89;

pub const WEIGHT_SPI_SIG1_SHIFT: c_int = 0;

pub const WEIGHT_SPI_SIG2_SHIFT: c_int = 6;

pub const WEIGHT_SPI_SIG3_SHIFT: c_int = 12;

pub const WEIGHT_SPI_SIG4_SHIFT: c_int = 18;

pub const WEIGHT_SPI_SIG5_SHIFT: c_int = 24;
pub const CG_CAC_REGION_4_WEIGHT_0: c_uint = 0x8a;

pub const WEIGHT_LDS_SIG0_SHIFT: c_int = 0;

pub const WEIGHT_LDS_SIG1_SHIFT: c_int = 6;

pub const WEIGHT_SC_SHIFT: c_int = 24;
pub const CG_CAC_REGION_4_WEIGHT_1: c_uint = 0x8b;

pub const WEIGHT_BIF_SHIFT: c_int = 0;

pub const WEIGHT_CP_SHIFT: c_int = 6;

pub const WEIGHT_PA_SIG0_SHIFT: c_int = 12;

pub const WEIGHT_PA_SIG1_SHIFT: c_int = 18;

pub const WEIGHT_VGT_SIG0_SHIFT: c_int = 24;
pub const CG_CAC_REGION_4_WEIGHT_2: c_uint = 0x8c;

pub const WEIGHT_VGT_SIG1_SHIFT: c_int = 0;

pub const WEIGHT_VGT_SIG2_SHIFT: c_int = 6;

pub const WEIGHT_DC_SIG0_SHIFT: c_int = 12;

pub const WEIGHT_DC_SIG1_SHIFT: c_int = 18;

pub const WEIGHT_DC_SIG2_SHIFT: c_int = 24;
pub const CG_CAC_REGION_4_WEIGHT_3: c_uint = 0x8d;

pub const WEIGHT_DC_SIG3_SHIFT: c_int = 0;

pub const WEIGHT_UVD_SIG0_SHIFT: c_int = 6;

pub const WEIGHT_UVD_SIG1_SHIFT: c_int = 12;

pub const WEIGHT_SPARE0_SHIFT: c_int = 18;

pub const WEIGHT_SPARE1_SHIFT: c_int = 24;
pub const CG_CAC_REGION_5_WEIGHT_0: c_uint = 0x8e;

pub const WEIGHT_SQ_VSP_SHIFT: c_int = 0;

pub const WEIGHT_SQ_VSP0_SHIFT: c_int = 14;
pub const CG_CAC_REGION_4_OVERRIDE_4: c_uint = 0xab;

pub const OVR_MODE_SPARE_0_SHIFT: c_int = 16;

pub const OVR_VAL_SPARE_0_SHIFT: c_int = 17;

pub const OVR_MODE_SPARE_1_SHIFT: c_int = 18;

pub const OVR_VAL_SPARE_1_SHIFT: c_int = 19;
pub const CG_CAC_REGION_5_WEIGHT_1: c_uint = 0xb7;

pub const WEIGHT_SQ_GPR_SHIFT: c_int = 0;

pub const WEIGHT_SQ_LDS_SHIFT: c_int = 14;
// PCIE link stuff
pub const PCIE_LC_TRAINING_CNTL: c_uint = 0xa1 /* PCIE_P */;
pub const PCIE_LC_LINK_WIDTH_CNTL: c_uint = 0xa2 /* PCIE_P */;

pub const PCIE_LC_SPEED_CNTL: c_uint = 0xa4 /* PCIE_P */;

pub const MM_CFGREGS_CNTL: c_uint = 0x544c;

pub const LINK_CNTL2: c_uint = 0x88 /* F0 */;

//
// UVD
//
pub const UVD_SEMA_ADDR_LOW: c_uint = 0xEF00;
pub const UVD_SEMA_ADDR_HIGH: c_uint = 0xEF04;
pub const UVD_SEMA_CMD: c_uint = 0xEF08;
pub const UVD_UDEC_ADDR_CONFIG: c_uint = 0xEF4C;
pub const UVD_UDEC_DB_ADDR_CONFIG: c_uint = 0xEF50;
pub const UVD_UDEC_DBW_ADDR_CONFIG: c_uint = 0xEF54;
pub const UVD_NO_OP: c_uint = 0xEFFC;
pub const UVD_RBC_RB_RPTR: c_uint = 0xF690;
pub const UVD_RBC_RB_WPTR: c_uint = 0xF694;
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
pub const PACKET3_DEALLOC_STATE: c_uint = 0x14;
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
pub const PACKET3_INDIRECT_BUFFER: c_uint = 0x32;
pub const PACKET3_STRMOUT_BUFFER_UPDATE: c_uint = 0x34;
pub const PACKET3_DRAW_INDEX_OFFSET_2: c_uint = 0x35;
pub const PACKET3_DRAW_INDEX_MULTI_ELEMENT: c_uint = 0x36;
pub const PACKET3_WRITE_DATA: c_uint = 0x37;
pub const PACKET3_MEM_SEMAPHORE: c_uint = 0x39;
pub const PACKET3_MPEG_INDEX: c_uint = 0x3A;
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
pub const PACKET3_EVENT_WRITE_EOS: c_uint = 0x48;
pub const PACKET3_PREAMBLE_CNTL: c_uint = 0x4A;

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
pub const PACKET3_ME_WRITE: c_uint = 0x7A;
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

pub const DMA_SEM_INCOMPLETE_TIMER_CNTL: c_uint = 0xd044;
pub const DMA_SEM_WAIT_FAIL_TIMER_CNTL: c_uint = 0xd048;
pub const DMA_TILING_CONFIG: c_uint = 0xd0b8;
pub const DMA_MODE: c_uint = 0xd0bc;

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
