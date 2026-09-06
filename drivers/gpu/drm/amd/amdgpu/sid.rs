//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/sid.h
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
pub const SI_MAX_CTLACKS_ASSERTION_WAIT: c_int = 100;
// CG IND registers are accessed via SMC indirect space + SMC_CG_IND_START
pub const SMC_CG_IND_START: c_uint = 0xc0030000;
pub const SMC_CG_IND_END: c_uint = 0xc0040000;
// SMC IND registers
pub const SMC_SYSCON_RESET_CNTL: c_uint = 0x80000000;

pub const SMC_SYSCON_CLOCK_CNTL_0: c_uint = 0x80000004;

pub const DCCG_DISP_SLOW_SELECT_REG: c_uint = 0x13F;

pub const DCCG_DISP1_SLOW_SELECT_SHIFT: c_int = 0;

pub const DCCG_DISP2_SLOW_SELECT_SHIFT: c_int = 4;
// discrete uvd clocks
pub const CG_UPLL_FUNC_CNTL: c_uint = 0x18d;

pub const CG_UPLL_FUNC_CNTL_2: c_uint = 0x18e;

pub const CG_UPLL_FUNC_CNTL_3: c_uint = 0x18f;

pub const CG_UPLL_FUNC_CNTL_4: c_uint = 0x191;

pub const CG_UPLL_FUNC_CNTL_5: c_uint = 0x192;

pub const CG_UPLL_SPREAD_SPECTRUM: c_uint = 0x194;

pub const VM_INVALIDATE_REQUEST: c_uint = 0x51E;
pub const VM_INVALIDATE_RESPONSE: c_uint = 0x51F;
pub const VM_L2_CG: c_uint = 0x570;

pub const MC_VM_FB_LOCATION: c_uint = 0x809;
pub const MC_VM_AGP_TOP: c_uint = 0x80A;
pub const MC_VM_AGP_BOT: c_uint = 0x80B;
pub const MC_VM_AGP_BASE: c_uint = 0x80C;
pub const MC_VM_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x80D;
pub const MC_VM_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x80E;
pub const MC_VM_SYSTEM_APERTURE_DEFAULT_ADDR: c_uint = 0x80F;
pub const MC_VM_MX_L1_TLB_CNTL: c_uint = 0x819;

pub const MC_SHARED_BLACKOUT_CNTL: c_uint = 0x82B;
pub const MC_HUB_MISC_HUB_CG: c_uint = 0x82E;
pub const MC_HUB_MISC_VM_CG: c_uint = 0x82F;
pub const MC_HUB_MISC_SIP_CG: c_uint = 0x830;
pub const MC_XPB_CLK_GAT: c_uint = 0x91E;
pub const MC_CITF_MISC_RD_CG: c_uint = 0x992;
pub const MC_CITF_MISC_WR_CG: c_uint = 0x993;
pub const MC_CITF_MISC_VM_CG: c_uint = 0x994;
pub const MC_ARB_DRAM_TIMING: c_uint = 0x9DD;
pub const MC_ARB_DRAM_TIMING2: c_uint = 0x9DE;
pub const MC_ARB_BURST_TIME: c_uint = 0xA02;

pub const STATE0_SHIFT: c_int = 0;

pub const STATE1_SHIFT: c_int = 5;

pub const STATE2_SHIFT: c_int = 10;

pub const STATE3_SHIFT: c_int = 15;
pub const MC_SEQ_TRAIN_WAKEUP_CNTL: c_uint = 0xA3A;

pub const MC_SEQ_SUP_CNTL: c_uint = 0xA32;

pub const MC_SEQ_SUP_PGM: c_uint = 0xA33;
pub const MC_PMG_AUTO_CMD: c_uint = 0xA34;
pub const MC_IO_PAD_CNTL_D0: c_uint = 0xA74;

pub const MC_SEQ_RAS_TIMING: c_uint = 0xA28;
pub const MC_SEQ_CAS_TIMING: c_uint = 0xA29;
pub const MC_SEQ_MISC_TIMING: c_uint = 0xA2A;
pub const MC_SEQ_MISC_TIMING2: c_uint = 0xA2B;
pub const MC_SEQ_PMG_TIMING: c_uint = 0xA2C;
pub const MC_SEQ_RD_CTL_D0: c_uint = 0xA2D;
pub const MC_SEQ_RD_CTL_D1: c_uint = 0xA2E;
pub const MC_SEQ_WR_CTL_D0: c_uint = 0xA2F;
pub const MC_SEQ_WR_CTL_D1: c_uint = 0xA30;
pub const MC_SEQ_MISC0: c_uint = 0xA80;
pub const MC_SEQ_MISC0_VEN_ID_SHIFT: c_int = 8;
pub const MC_SEQ_MISC0_VEN_ID_MASK: c_uint = 0x00000f00;
pub const MC_SEQ_MISC0_VEN_ID_VALUE: c_int = 3;
pub const MC_SEQ_MISC0_REV_ID_SHIFT: c_int = 12;
pub const MC_SEQ_MISC0_REV_ID_MASK: c_uint = 0x0000f000;
pub const MC_SEQ_MISC0_REV_ID_VALUE: c_int = 1;
pub const MC_SEQ_MISC0_GDDR5_SHIFT: c_int = 28;
pub const MC_SEQ_MISC0_GDDR5_MASK: c_uint = 0xf0000000;
pub const MC_SEQ_MISC0_GDDR5_VALUE: c_int = 5;
pub const MC_SEQ_MISC1: c_uint = 0xA81;
pub const MC_SEQ_RESERVE_M: c_uint = 0xA82;
pub const MC_PMG_CMD_EMRS: c_uint = 0xA83;
pub const MC_SEQ_IO_DEBUG_INDEX: c_uint = 0xA91;
pub const MC_SEQ_IO_DEBUG_DATA: c_uint = 0xA92;
pub const MC_SEQ_MISC5: c_uint = 0xA95;
pub const MC_SEQ_MISC6: c_uint = 0xA96;
pub const MC_SEQ_MISC7: c_uint = 0xA99;
pub const MC_SEQ_RAS_TIMING_LP: c_uint = 0xA9B;
pub const MC_SEQ_CAS_TIMING_LP: c_uint = 0xA9C;
pub const MC_SEQ_MISC_TIMING_LP: c_uint = 0xA9D;
pub const MC_SEQ_MISC_TIMING2_LP: c_uint = 0xA9E;
pub const MC_SEQ_WR_CTL_D0_LP: c_uint = 0xA9F;
pub const MC_SEQ_WR_CTL_D1_LP: c_uint = 0xAA0;
pub const MC_SEQ_PMG_CMD_EMRS_LP: c_uint = 0xAA1;
pub const MC_SEQ_PMG_CMD_MRS_LP: c_uint = 0xAA2;
pub const MC_PMG_CMD_MRS: c_uint = 0xAAB;
pub const MC_SEQ_RD_CTL_D0_LP: c_uint = 0xAC7;
pub const MC_SEQ_RD_CTL_D1_LP: c_uint = 0xAC8;
pub const MC_PMG_CMD_MRS1: c_uint = 0xAD1;
pub const MC_SEQ_PMG_CMD_MRS1_LP: c_uint = 0xAD2;
pub const MC_SEQ_PMG_TIMING_LP: c_uint = 0xAD3;
pub const MC_SEQ_WR_CTL_2: c_uint = 0xAD5;
pub const MC_SEQ_WR_CTL_2_LP: c_uint = 0xAD6;
pub const MC_PMG_CMD_MRS2: c_uint = 0xAD7;
pub const MC_SEQ_PMG_CMD_MRS2_LP: c_uint = 0xAD8;
pub const MCLK_PWRMGT_CNTL: c_uint = 0xAE8;

pub const DLL_CNTL: c_uint = 0xAE9;

pub const MPLL_CNTL_MODE: c_uint = 0xAEC;

pub const MPLL_FUNC_CNTL: c_uint = 0xAED;

pub const MPLL_FUNC_CNTL_1: c_uint = 0xAEE;

pub const MPLL_FUNC_CNTL_2: c_uint = 0xAEF;
pub const MPLL_AD_FUNC_CNTL: c_uint = 0xAF0;

pub const MPLL_DQ_FUNC_CNTL: c_uint = 0xAF1;

pub const MPLL_SS1: c_uint = 0xAF3;

pub const MPLL_SS2: c_uint = 0xAF4;

pub const ATC_MISC_CG: c_uint = 0xCD4;
pub const IH_RB_CNTL: c_uint = 0xF80;

pub const IH_RB_BASE: c_uint = 0xF81;
pub const IH_RB_RPTR: c_uint = 0xF82;
pub const IH_RB_WPTR: c_uint = 0xF83;

pub const IH_RB_WPTR_ADDR_HI: c_uint = 0xF84;
pub const IH_RB_WPTR_ADDR_LO: c_uint = 0xF85;
pub const IH_CNTL: c_uint = 0xF86;

pub const INTERRUPT_CNTL: c_uint = 0x151A;

pub const INTERRUPT_CNTL2: c_uint = 0x151B;
pub const VGT_VTX_VECT_EJECT_REG: c_uint = 0x222C;
pub const VGT_ESGS_RING_SIZE: c_uint = 0x2232;
pub const VGT_GSVS_RING_SIZE: c_uint = 0x2233;
pub const VGT_GS_VERTEX_REUSE: c_uint = 0x2235;
pub const VGT_PRIMITIVE_TYPE: c_uint = 0x2256;
pub const VGT_INDEX_TYPE: c_uint = 0x2257;
pub const VGT_NUM_INDICES: c_uint = 0x225C;
pub const VGT_NUM_INSTANCES: c_uint = 0x225D;
pub const VGT_TF_RING_SIZE: c_uint = 0x2262;
pub const VGT_HS_OFFCHIP_PARAM: c_uint = 0x226C;
pub const VGT_TF_MEMORY_BASE: c_uint = 0x226E;
pub const PA_SC_ENHANCE: c_uint = 0x22FC;
pub const TA_CNTL_AUX: c_uint = 0x2542;
// #define PA_SC_RASTER_CONFIG                             0xA0D4

// PCIE PORT registers idx/data 0x38/0x3c
// #define PCIE_LC_LINK_WIDTH_CNTL                           0xa2 /* PCIE_P

//
// PM4
//
pub const PACKET_TYPE0: c_int = 0;

pub const CP_PACKET2: c_uint = 0x80000000;
pub const PACKET2_PAD_SHIFT: c_int = 0;

pub const RADEON_PACKET_TYPE3: c_int = 3;

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
pub const PACKET3_INDIRECT_BUFFER: c_uint = 0x3F;
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
pub const PACKET3_SET_CONFIG_REG_START: c_uint = 0x00002000;
pub const PACKET3_SET_CONFIG_REG_END: c_uint = 0x00002c00;
pub const PACKET3_SET_CONTEXT_REG: c_uint = 0x69;
pub const PACKET3_SET_CONTEXT_REG_START: c_uint = 0x000a000;
pub const PACKET3_SET_CONTEXT_REG_END: c_uint = 0x000a400;
pub const PACKET3_SET_CONTEXT_REG_INDIRECT: c_uint = 0x73;
pub const PACKET3_SET_RESOURCE_INDIRECT: c_uint = 0x74;
pub const PACKET3_SET_SH_REG: c_uint = 0x76;
pub const PACKET3_SET_SH_REG_START: c_uint = 0x00002c00;
pub const PACKET3_SET_SH_REG_END: c_uint = 0x00003000;
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
pub const DMA1_REGISTER_OFFSET: c_uint = 0x200 /* not a register */;
pub const SDMA_MAX_INSTANCE: c_int = 2;

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
// VCE
pub const VCE_CMD_NO_OP: c_uint = 0x00000000;
pub const VCE_CMD_END: c_uint = 0x00000001;
pub const VCE_CMD_IB: c_uint = 0x00000002;
pub const VCE_CMD_FENCE: c_uint = 0x00000003;
pub const VCE_CMD_TRAP: c_uint = 0x00000004;
pub const VCE_CMD_IB_AUTO: c_uint = 0x00000005;
pub const VCE_CMD_SEMAPHORE: c_uint = 0x00000006;
// #dce stupp
// display controller offsets used for crtc/cur/lut/grph/viewport/etc.

// hpd instance offsets

// audio endpt instance offsets

pub const CURSOR_WIDTH: c_int = 64;
pub const CURSOR_HEIGHT: c_int = 64;
pub const R600_ROM_CNTL: c_uint = 0x580;

pub const GRPH_ARRAY_LINEAR_GENERAL: c_int = 0;
pub const GRPH_ARRAY_LINEAR_ALIGNED: c_int = 1;
pub const GRPH_ARRAY_1D_TILED_THIN1: c_int = 2;
pub const GRPH_ARRAY_2D_TILED_THIN1: c_int = 4;
pub const ES_AND_GS_AUTO: c_int = 3;

pub const GRPH_DEPTH_8BPP: c_int = 0;
pub const GRPH_DEPTH_16BPP: c_int = 1;
pub const GRPH_DEPTH_32BPP: c_int = 2;
// 8 BPP
pub const GRPH_FORMAT_INDEXED: c_int = 0;
// 16 BPP
pub const GRPH_FORMAT_ARGB1555: c_int = 0;
pub const GRPH_FORMAT_ARGB565: c_int = 1;
pub const GRPH_FORMAT_ARGB4444: c_int = 2;
pub const GRPH_FORMAT_AI88: c_int = 3;
pub const GRPH_FORMAT_MONO16: c_int = 4;
pub const GRPH_FORMAT_BGRA5551: c_int = 5;
// 32 BPP
pub const GRPH_FORMAT_ARGB8888: c_int = 0;
pub const GRPH_FORMAT_ARGB2101010: c_int = 1;
pub const GRPH_FORMAT_32BPP_DIG: c_int = 2;
pub const GRPH_FORMAT_8B_ARGB2101010: c_int = 3;
pub const GRPH_FORMAT_BGRA1010102: c_int = 4;
pub const GRPH_FORMAT_8B_BGRA1010102: c_int = 5;
pub const GRPH_FORMAT_RGB111110: c_int = 6;
pub const GRPH_FORMAT_BGR101111: c_int = 7;
pub const GRPH_ENDIAN_NONE: c_int = 0;
pub const GRPH_ENDIAN_8IN16: c_int = 1;
pub const GRPH_ENDIAN_8IN32: c_int = 2;
pub const GRPH_ENDIAN_8IN64: c_int = 3;
pub const GRPH_RED_SEL_R: c_int = 0;
pub const GRPH_RED_SEL_G: c_int = 1;
pub const GRPH_RED_SEL_B: c_int = 2;
pub const GRPH_RED_SEL_A: c_int = 3;
pub const GRPH_GREEN_SEL_G: c_int = 0;
pub const GRPH_GREEN_SEL_B: c_int = 1;
pub const GRPH_GREEN_SEL_A: c_int = 2;
pub const GRPH_GREEN_SEL_R: c_int = 3;
pub const GRPH_BLUE_SEL_B: c_int = 0;
pub const GRPH_BLUE_SEL_A: c_int = 1;
pub const GRPH_BLUE_SEL_R: c_int = 2;
pub const GRPH_BLUE_SEL_G: c_int = 3;
pub const GRPH_ALPHA_SEL_A: c_int = 0;
pub const GRPH_ALPHA_SEL_R: c_int = 1;
pub const GRPH_ALPHA_SEL_G: c_int = 2;
pub const GRPH_ALPHA_SEL_B: c_int = 3;
// CUR_CONTROL
pub const CURSOR_MONO: c_int = 0;
pub const CURSOR_24_1: c_int = 1;
pub const CURSOR_24_8_PRE_MULT: c_int = 2;
pub const CURSOR_24_8_UNPRE_MULT: c_int = 3;
pub const CURSOR_URGENT_ALWAYS: c_int = 0;
pub const CURSOR_URGENT_1_8: c_int = 1;
pub const CURSOR_URGENT_1_4: c_int = 2;
pub const CURSOR_URGENT_3_8: c_int = 3;
pub const CURSOR_URGENT_1_2: c_int = 4;
// INPUT_CSC_CONTROL

// OUTPUT_CSC_CONTROL

// DEGAMMA_CONTROL

// GAMUT_REMAP_CONTROL

// REGAMMA_CONTROL

// INPUT_GAMMA_CONTROL

pub const MC_SEQ_MISC0__MT__MASK: c_uint = 0xf0000000;
pub const MC_SEQ_MISC0__MT__GDDR1: c_uint = 0x10000000;
pub const MC_SEQ_MISC0__MT__DDR2: c_uint = 0x20000000;
pub const MC_SEQ_MISC0__MT__GDDR3: c_uint = 0x30000000;
pub const MC_SEQ_MISC0__MT__GDDR4: c_uint = 0x40000000;
pub const MC_SEQ_MISC0__MT__GDDR5: c_uint = 0x50000000;
pub const MC_SEQ_MISC0__MT__HBM: c_uint = 0x60000000;
pub const MC_SEQ_MISC0__MT__DDR3: c_uint = 0xB0000000;
pub const CP_INT_CNTL_RING__TIME_STAMP_INT_ENABLE_MASK: c_uint = 0x4000000;

pub const AMDGPU_PCIE_INDEX: c_uint = 0xc;
pub const AMDGPU_PCIE_DATA: c_uint = 0xd;
pub const PCIE_BUS_CLK: c_int = 10000;

pub const PCIE_PORT_INDEX: c_uint = 0xe;
pub const PCIE_PORT_DATA: c_uint = 0xf;
pub const EVERGREEN_PIF_PHY0_INDEX: c_uint = 0x8;
pub const EVERGREEN_PIF_PHY0_DATA: c_uint = 0xc;
pub const EVERGREEN_PIF_PHY1_INDEX: c_uint = 0x10;
pub const EVERGREEN_PIF_PHY1_DATA: c_uint = 0x14;
// Discrete VCE clocks
pub const CG_VCEPLL_FUNC_CNTL: c_uint = 0xc0030600;
pub const VCEPLL_RESET_MASK: c_uint = 0x00000001;
pub const VCEPLL_SLEEP_MASK: c_uint = 0x00000002;
pub const VCEPLL_BYPASS_EN_MASK: c_uint = 0x00000004;
pub const VCEPLL_CTLREQ_MASK: c_uint = 0x00000008;
pub const VCEPLL_VCO_MODE_MASK: c_uint = 0x00000600;
pub const VCEPLL_REF_DIV_MASK: c_uint = 0x003F0000;
pub const VCEPLL_CTLACK_MASK: c_uint = 0x40000000;
pub const VCEPLL_CTLACK2_MASK: c_uint = 0x80000000;
pub const CG_VCEPLL_FUNC_CNTL_2: c_uint = 0xc0030601;

pub const VCEPLL_PDIV_A_MASK: c_uint = 0x0000007F;

pub const VCEPLL_PDIV_B_MASK: c_uint = 0x00007F00;

pub const EVCLK_SRC_SEL_MASK: c_uint = 0x01F00000;

pub const ECCLK_SRC_SEL_MASK: c_uint = 0x3E000000;
pub const CG_VCEPLL_FUNC_CNTL_3: c_uint = 0xc0030602;

pub const VCEPLL_FB_DIV_MASK: c_uint = 0x01FFFFFF;
pub const CG_VCEPLL_FUNC_CNTL_4: c_uint = 0xc0030603;
pub const CG_VCEPLL_FUNC_CNTL_5: c_uint = 0xc0030604;
pub const CG_VCEPLL_SPREAD_SPECTRUM: c_uint = 0xc0030606;
pub const VCEPLL_SSEN_MASK: c_uint = 0x00000001;
