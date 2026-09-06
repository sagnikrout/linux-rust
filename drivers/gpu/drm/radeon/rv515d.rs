//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/rv515d.h
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
// Copyright 2008 Advanced Micro Devices, Inc.
// Copyright 2008 Red Hat Inc.
// Copyright 2009 Jerome Glisse.
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
// RV515 registers
//
pub const PCIE_INDEX: c_uint = 0x0030;
pub const PCIE_DATA: c_uint = 0x0034;
pub const MC_IND_INDEX: c_uint = 0x0070;

pub const MC_IND_DATA: c_uint = 0x0074;
pub const RBBM_SOFT_RESET: c_uint = 0x00F0;
pub const CONFIG_MEMSIZE: c_uint = 0x00F8;
pub const HDP_FB_LOCATION: c_uint = 0x0134;
pub const CP_CSQ_CNTL: c_uint = 0x0740;
pub const CP_CSQ_MODE: c_uint = 0x0744;
pub const CP_CSQ_ADDR: c_uint = 0x07F0;
pub const CP_CSQ_DATA: c_uint = 0x07F4;
pub const CP_CSQ_STAT: c_uint = 0x07F8;
pub const CP_CSQ2_STAT: c_uint = 0x07FC;
pub const RBBM_STATUS: c_uint = 0x0E40;
pub const DST_PIPE_CONFIG: c_uint = 0x170C;
pub const WAIT_UNTIL: c_uint = 0x1720;

pub const ISYNC_CNTL: c_uint = 0x1724;

pub const VAP_INDEX_OFFSET: c_uint = 0x208C;
pub const VAP_PVS_STATE_FLUSH_REG: c_uint = 0x2284;
pub const GB_ENABLE: c_uint = 0x4008;
pub const GB_MSPOS0: c_uint = 0x4010;
pub const MS_X0_SHIFT: c_int = 0;
pub const MS_Y0_SHIFT: c_int = 4;
pub const MS_X1_SHIFT: c_int = 8;
pub const MS_Y1_SHIFT: c_int = 12;
pub const MS_X2_SHIFT: c_int = 16;
pub const MS_Y2_SHIFT: c_int = 20;
pub const MSBD0_Y_SHIFT: c_int = 24;
pub const MSBD0_X_SHIFT: c_int = 28;
pub const GB_MSPOS1: c_uint = 0x4014;
pub const MS_X3_SHIFT: c_int = 0;
pub const MS_Y3_SHIFT: c_int = 4;
pub const MS_X4_SHIFT: c_int = 8;
pub const MS_Y4_SHIFT: c_int = 12;
pub const MS_X5_SHIFT: c_int = 16;
pub const MS_Y5_SHIFT: c_int = 20;
pub const MSBD1_SHIFT: c_int = 24;
pub const GB_TILE_CONFIG: c_uint = 0x4018;

pub const PIPE_COUNT_MASK: c_uint = 0x0000000E;
pub const PIPE_COUNT_SHIFT: c_int = 1;

pub const GB_SELECT: c_uint = 0x401C;
pub const GB_AA_CONFIG: c_uint = 0x4020;
pub const GB_PIPE_SELECT: c_uint = 0x402C;
pub const GA_ENHANCE: c_uint = 0x4274;

pub const GA_POLY_MODE: c_uint = 0x4288;

pub const GA_ROUND_MODE: c_uint = 0x428C;

pub const SU_REG_DEST: c_uint = 0x42C8;
pub const RB3D_DSTCACHE_CTLSTAT: c_uint = 0x4E4C;

pub const ZB_ZCACHE_CTLSTAT: c_uint = 0x4F18;

pub const DC_LB_MEMORY_SPLIT: c_uint = 0x6520;
pub const DC_LB_MEMORY_SPLIT_MASK: c_uint = 0x00000003;
pub const DC_LB_MEMORY_SPLIT_SHIFT: c_int = 0;
pub const DC_LB_MEMORY_SPLIT_D1HALF_D2HALF: c_int = 0;
pub const DC_LB_MEMORY_SPLIT_D1_3Q_D2_1Q: c_int = 1;
pub const DC_LB_MEMORY_SPLIT_D1_ONLY: c_int = 2;
pub const DC_LB_MEMORY_SPLIT_D1_1Q_D2_3Q: c_int = 3;

pub const DC_LB_DISP1_END_ADR_SHIFT: c_int = 4;
pub const DC_LB_DISP1_END_ADR_MASK: c_uint = 0x00007FF0;
pub const D1MODE_PRIORITY_A_CNT: c_uint = 0x6548;
pub const MODE_PRIORITY_MARK_MASK: c_uint = 0x00007FFF;

pub const D1MODE_PRIORITY_B_CNT: c_uint = 0x654C;
pub const LB_MAX_REQ_OUTSTANDING: c_uint = 0x6D58;
pub const LB_D1_MAX_REQ_OUTSTANDING_MASK: c_uint = 0x0000000F;
pub const LB_D1_MAX_REQ_OUTSTANDING_SHIFT: c_int = 0;
pub const LB_D2_MAX_REQ_OUTSTANDING_MASK: c_uint = 0x000F0000;
pub const LB_D2_MAX_REQ_OUTSTANDING_SHIFT: c_int = 16;
pub const D2MODE_PRIORITY_A_CNT: c_uint = 0x6D48;
pub const D2MODE_PRIORITY_B_CNT: c_uint = 0x6D4C;
// ix[MC] registers
pub const MC_FB_LOCATION: c_uint = 0x01;
pub const MC_FB_START_MASK: c_uint = 0x0000FFFF;
pub const MC_FB_START_SHIFT: c_int = 0;
pub const MC_FB_TOP_MASK: c_uint = 0xFFFF0000;
pub const MC_FB_TOP_SHIFT: c_int = 16;
pub const MC_AGP_LOCATION: c_uint = 0x02;
pub const MC_AGP_START_MASK: c_uint = 0x0000FFFF;
pub const MC_AGP_START_SHIFT: c_int = 0;
pub const MC_AGP_TOP_MASK: c_uint = 0xFFFF0000;
pub const MC_AGP_TOP_SHIFT: c_int = 16;
pub const MC_AGP_BASE: c_uint = 0x03;
pub const MC_AGP_BASE_2: c_uint = 0x04;
pub const MC_CNTL: c_uint = 0x5;
pub const MEM_NUM_CHANNELS_MASK: c_uint = 0x00000003;
pub const MC_STATUS: c_uint = 0x08;

pub const MC_MISC_LAT_TIMER: c_uint = 0x09;
pub const MC_CPR_INIT_LAT_MASK: c_uint = 0x0000000F;
pub const MC_VF_INIT_LAT_MASK: c_uint = 0x000000F0;
pub const MC_DISP0R_INIT_LAT_MASK: c_uint = 0x00000F00;
pub const MC_DISP0R_INIT_LAT_SHIFT: c_int = 8;
pub const MC_DISP1R_INIT_LAT_MASK: c_uint = 0x0000F000;
pub const MC_DISP1R_INIT_LAT_SHIFT: c_int = 12;
pub const MC_FIXED_INIT_LAT_MASK: c_uint = 0x000F0000;
pub const MC_E2R_INIT_LAT_MASK: c_uint = 0x00F00000;
pub const SAME_PAGE_PRIO_MASK: c_uint = 0x0F000000;
pub const MC_GLOBW_INIT_LAT_MASK: c_uint = 0xF0000000;
//
// PM4 packet
//
pub const CP_PACKET0: c_uint = 0x00000000;
pub const PACKET0_BASE_INDEX_SHIFT: c_int = 0;

pub const PACKET0_COUNT_SHIFT: c_int = 16;

pub const CP_PACKET1: c_uint = 0x40000000;
pub const CP_PACKET2: c_uint = 0x80000000;
pub const PACKET2_PAD_SHIFT: c_int = 0;

pub const CP_PACKET3: c_uint = 0xC0000000;
pub const PACKET3_IT_OPCODE_SHIFT: c_int = 8;

pub const PACKET3_COUNT_SHIFT: c_int = 16;

// PACKET3 op code
pub const PACKET3_NOP: c_uint = 0x10;
pub const PACKET3_3D_DRAW_VBUF: c_uint = 0x28;
pub const PACKET3_3D_DRAW_IMMD: c_uint = 0x29;
pub const PACKET3_3D_DRAW_INDX: c_uint = 0x2A;
pub const PACKET3_3D_LOAD_VBPNTR: c_uint = 0x2F;
pub const PACKET3_INDX_BUFFER: c_uint = 0x33;
pub const PACKET3_3D_DRAW_VBUF_2: c_uint = 0x34;
pub const PACKET3_3D_DRAW_IMMD_2: c_uint = 0x35;
pub const PACKET3_3D_DRAW_INDX_2: c_uint = 0x36;
pub const PACKET3_BITBLT_MULTI: c_uint = 0x9B;

// Registers
pub const R_0000F0_RBBM_SOFT_RESET: c_uint = 0x0000F0;

pub const C_0000F0_SOFT_RESET_CP: c_uint = 0xFFFFFFFE;

pub const C_0000F0_SOFT_RESET_HI: c_uint = 0xFFFFFFFD;

pub const C_0000F0_SOFT_RESET_VAP: c_uint = 0xFFFFFFFB;

pub const C_0000F0_SOFT_RESET_RE: c_uint = 0xFFFFFFF7;

pub const C_0000F0_SOFT_RESET_PP: c_uint = 0xFFFFFFEF;

pub const C_0000F0_SOFT_RESET_E2: c_uint = 0xFFFFFFDF;

pub const C_0000F0_SOFT_RESET_RB: c_uint = 0xFFFFFFBF;

pub const C_0000F0_SOFT_RESET_HDP: c_uint = 0xFFFFFF7F;

pub const C_0000F0_SOFT_RESET_MC: c_uint = 0xFFFFFEFF;

pub const C_0000F0_SOFT_RESET_AIC: c_uint = 0xFFFFFDFF;

pub const C_0000F0_SOFT_RESET_VIP: c_uint = 0xFFFFFBFF;

pub const C_0000F0_SOFT_RESET_DISP: c_uint = 0xFFFFF7FF;

pub const C_0000F0_SOFT_RESET_CG: c_uint = 0xFFFFEFFF;

pub const C_0000F0_SOFT_RESET_GA: c_uint = 0xFFFFDFFF;

pub const C_0000F0_SOFT_RESET_IDCT: c_uint = 0xFFFFBFFF;
pub const R_0000F8_CONFIG_MEMSIZE: c_uint = 0x0000F8;

pub const C_0000F8_CONFIG_MEMSIZE: c_uint = 0x00000000;
pub const R_000134_HDP_FB_LOCATION: c_uint = 0x000134;

pub const C_000134_HDP_FB_START: c_uint = 0xFFFF0000;
pub const R_000300_VGA_RENDER_CONTROL: c_uint = 0x000300;

pub const C_000300_VGA_BLINK_RATE: c_uint = 0xFFFFFFE0;

pub const C_000300_VGA_BLINK_MODE: c_uint = 0xFFFFFF9F;

pub const C_000300_VGA_CURSOR_BLINK_INVERT: c_uint = 0xFFFFFF7F;

pub const C_000300_VGA_EXTD_ADDR_COUNT_ENABLE: c_uint = 0xFFFFFEFF;

pub const C_000300_VGA_VSTATUS_CNTL: c_uint = 0xFFFCFFFF;

pub const C_000300_VGA_LOCK_8DOT: c_uint = 0xFEFFFFFF;

pub const C_000300_VGAREG_LINECMP_COMPATIBILITY_SEL: c_uint = 0xFDFFFFFF;
pub const R_000310_VGA_MEMORY_BASE_ADDRESS: c_uint = 0x000310;

pub const C_000310_VGA_MEMORY_BASE_ADDRESS: c_uint = 0x00000000;
pub const R_000328_VGA_HDP_CONTROL: c_uint = 0x000328;

pub const C_000328_VGA_MEM_PAGE_SELECT_EN: c_uint = 0xFFFFFFFE;

pub const C_000328_VGA_RBBM_LOCK_DISABLE: c_uint = 0xFFFFFEFF;

pub const C_000328_VGA_SOFT_RESET: c_uint = 0xFFFEFFFF;

pub const C_000328_VGA_TEST_RESET_CONTROL: c_uint = 0xFEFFFFFF;
pub const R_000330_D1VGA_CONTROL: c_uint = 0x000330;

pub const C_000330_D1VGA_MODE_ENABLE: c_uint = 0xFFFFFFFE;

pub const C_000330_D1VGA_TIMING_SELECT: c_uint = 0xFFFFFEFF;

pub const C_000330_D1VGA_SYNC_POLARITY_SELECT: c_uint = 0xFFFFFDFF;

pub const C_000330_D1VGA_OVERSCAN_TIMING_SELECT: c_uint = 0xFFFFFBFF;

pub const C_000330_D1VGA_OVERSCAN_COLOR_EN: c_uint = 0xFFFEFFFF;

pub const C_000330_D1VGA_ROTATE: c_uint = 0xFCFFFFFF;
pub const R_000338_D2VGA_CONTROL: c_uint = 0x000338;

pub const C_000338_D2VGA_MODE_ENABLE: c_uint = 0xFFFFFFFE;

pub const C_000338_D2VGA_TIMING_SELECT: c_uint = 0xFFFFFEFF;

pub const C_000338_D2VGA_SYNC_POLARITY_SELECT: c_uint = 0xFFFFFDFF;

pub const C_000338_D2VGA_OVERSCAN_TIMING_SELECT: c_uint = 0xFFFFFBFF;

pub const C_000338_D2VGA_OVERSCAN_COLOR_EN: c_uint = 0xFFFEFFFF;

pub const C_000338_D2VGA_ROTATE: c_uint = 0xFCFFFFFF;
pub const R_0007C0_CP_STAT: c_uint = 0x0007C0;

pub const C_0007C0_MRU_BUSY: c_uint = 0xFFFFFFFE;

pub const C_0007C0_MWU_BUSY: c_uint = 0xFFFFFFFD;

pub const C_0007C0_RSIU_BUSY: c_uint = 0xFFFFFFFB;

pub const C_0007C0_RCIU_BUSY: c_uint = 0xFFFFFFF7;

pub const C_0007C0_CSF_PRIMARY_BUSY: c_uint = 0xFFFFFDFF;

pub const C_0007C0_CSF_INDIRECT_BUSY: c_uint = 0xFFFFFBFF;

pub const C_0007C0_CSQ_PRIMARY_BUSY: c_uint = 0xFFFFF7FF;

pub const C_0007C0_CSQ_INDIRECT_BUSY: c_uint = 0xFFFFEFFF;

pub const C_0007C0_CSI_BUSY: c_uint = 0xFFFFDFFF;

pub const C_0007C0_CSF_INDIRECT2_BUSY: c_uint = 0xFFFFBFFF;

pub const C_0007C0_CSQ_INDIRECT2_BUSY: c_uint = 0xFFFF7FFF;

pub const C_0007C0_GUIDMA_BUSY: c_uint = 0xEFFFFFFF;

pub const C_0007C0_VIDDMA_BUSY: c_uint = 0xDFFFFFFF;

pub const C_0007C0_CMDSTRM_BUSY: c_uint = 0xBFFFFFFF;

pub const C_0007C0_CP_BUSY: c_uint = 0x7FFFFFFF;
pub const R_000E40_RBBM_STATUS: c_uint = 0x000E40;

pub const C_000E40_CMDFIFO_AVAIL: c_uint = 0xFFFFFF80;

pub const C_000E40_HIRQ_ON_RBB: c_uint = 0xFFFFFEFF;

pub const C_000E40_CPRQ_ON_RBB: c_uint = 0xFFFFFDFF;

pub const C_000E40_CFRQ_ON_RBB: c_uint = 0xFFFFFBFF;

pub const C_000E40_HIRQ_IN_RTBUF: c_uint = 0xFFFFF7FF;

pub const C_000E40_CPRQ_IN_RTBUF: c_uint = 0xFFFFEFFF;

pub const C_000E40_CFRQ_IN_RTBUF: c_uint = 0xFFFFDFFF;

pub const C_000E40_CF_PIPE_BUSY: c_uint = 0xFFFFBFFF;

pub const C_000E40_ENG_EV_BUSY: c_uint = 0xFFFF7FFF;

pub const C_000E40_CP_CMDSTRM_BUSY: c_uint = 0xFFFEFFFF;

pub const C_000E40_E2_BUSY: c_uint = 0xFFFDFFFF;

pub const C_000E40_RB2D_BUSY: c_uint = 0xFFFBFFFF;

pub const C_000E40_RB3D_BUSY: c_uint = 0xFFF7FFFF;

pub const C_000E40_VAP_BUSY: c_uint = 0xFFEFFFFF;

pub const C_000E40_RE_BUSY: c_uint = 0xFFDFFFFF;

pub const C_000E40_TAM_BUSY: c_uint = 0xFFBFFFFF;

pub const C_000E40_TDM_BUSY: c_uint = 0xFF7FFFFF;

pub const C_000E40_PB_BUSY: c_uint = 0xFEFFFFFF;

pub const C_000E40_TIM_BUSY: c_uint = 0xFDFFFFFF;

pub const C_000E40_GA_BUSY: c_uint = 0xFBFFFFFF;

pub const C_000E40_CBA2D_BUSY: c_uint = 0xF7FFFFFF;

pub const C_000E40_RBBM_HIBUSY: c_uint = 0xEFFFFFFF;

pub const C_000E40_SKID_CFBUSY: c_uint = 0xDFFFFFFF;

pub const C_000E40_VAP_VF_BUSY: c_uint = 0xBFFFFFFF;

pub const C_000E40_GUI_ACTIVE: c_uint = 0x7FFFFFFF;
pub const R_006080_D1CRTC_CONTROL: c_uint = 0x006080;

pub const C_006080_D1CRTC_MASTER_EN: c_uint = 0xFFFFFFFE;

pub const C_006080_D1CRTC_SYNC_RESET_SEL: c_uint = 0xFFFFFFEF;

pub const C_006080_D1CRTC_DISABLE_POINT_CNTL: c_uint = 0xFFFFFCFF;

pub const C_006080_D1CRTC_CURRENT_MASTER_EN_STATE: c_uint = 0xFFFEFFFF;

pub const C_006080_D1CRTC_DISP_READ_REQUEST_DISABLE: c_uint = 0xFEFFFFFF;
pub const R_0060E8_D1CRTC_UPDATE_LOCK: c_uint = 0x0060E8;

pub const C_0060E8_D1CRTC_UPDATE_LOCK: c_uint = 0xFFFFFFFE;
pub const R_006110_D1GRPH_PRIMARY_SURFACE_ADDRESS: c_uint = 0x006110;

pub const C_006110_D1GRPH_PRIMARY_SURFACE_ADDRESS: c_uint = 0x00000000;
pub const R_006118_D1GRPH_SECONDARY_SURFACE_ADDRESS: c_uint = 0x006118;

pub const C_006118_D1GRPH_SECONDARY_SURFACE_ADDRESS: c_uint = 0x00000000;
pub const R_006880_D2CRTC_CONTROL: c_uint = 0x006880;

pub const C_006880_D2CRTC_MASTER_EN: c_uint = 0xFFFFFFFE;

pub const C_006880_D2CRTC_SYNC_RESET_SEL: c_uint = 0xFFFFFFEF;

pub const C_006880_D2CRTC_DISABLE_POINT_CNTL: c_uint = 0xFFFFFCFF;

pub const C_006880_D2CRTC_CURRENT_MASTER_EN_STATE: c_uint = 0xFFFEFFFF;

pub const C_006880_D2CRTC_DISP_READ_REQUEST_DISABLE: c_uint = 0xFEFFFFFF;
pub const R_0068E8_D2CRTC_UPDATE_LOCK: c_uint = 0x0068E8;

pub const C_0068E8_D2CRTC_UPDATE_LOCK: c_uint = 0xFFFFFFFE;
pub const R_006910_D2GRPH_PRIMARY_SURFACE_ADDRESS: c_uint = 0x006910;

pub const C_006910_D2GRPH_PRIMARY_SURFACE_ADDRESS: c_uint = 0x00000000;
pub const R_006918_D2GRPH_SECONDARY_SURFACE_ADDRESS: c_uint = 0x006918;

pub const C_006918_D2GRPH_SECONDARY_SURFACE_ADDRESS: c_uint = 0x00000000;
pub const R_000001_MC_FB_LOCATION: c_uint = 0x000001;

pub const C_000001_MC_FB_START: c_uint = 0xFFFF0000;

pub const C_000001_MC_FB_TOP: c_uint = 0x0000FFFF;
pub const R_000002_MC_AGP_LOCATION: c_uint = 0x000002;

pub const C_000002_MC_AGP_START: c_uint = 0xFFFF0000;

pub const C_000002_MC_AGP_TOP: c_uint = 0x0000FFFF;
pub const R_000003_MC_AGP_BASE: c_uint = 0x000003;

pub const C_000003_AGP_BASE_ADDR: c_uint = 0x00000000;
pub const R_000004_MC_AGP_BASE_2: c_uint = 0x000004;

pub const C_000004_AGP_BASE_ADDR_2: c_uint = 0xFFFFFFF0;
pub const R_00000F_CP_DYN_CNTL: c_uint = 0x00000F;

pub const C_00000F_CP_FORCEON: c_uint = 0xFFFFFFFE;

pub const C_00000F_CP_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFFD;

pub const C_00000F_CP_CLOCK_STATUS: c_uint = 0xFFFFFFFB;

pub const C_00000F_CP_PROG_SHUTOFF: c_uint = 0xFFFFFFF7;

pub const C_00000F_CP_PROG_DELAY_VALUE: c_uint = 0xFFFFF00F;

pub const C_00000F_CP_LOWER_POWER_IDLE: c_uint = 0xFFF00FFF;

pub const C_00000F_CP_LOWER_POWER_IGNORE: c_uint = 0xFFEFFFFF;

pub const C_00000F_CP_NORMAL_POWER_IGNORE: c_uint = 0xFFDFFFFF;

pub const C_00000F_SPARE: c_uint = 0xFF3FFFFF;

pub const C_00000F_CP_NORMAL_POWER_BUSY: c_uint = 0x00FFFFFF;
pub const R_000011_E2_DYN_CNTL: c_uint = 0x000011;

pub const C_000011_E2_FORCEON: c_uint = 0xFFFFFFFE;

pub const C_000011_E2_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFFD;

pub const C_000011_E2_CLOCK_STATUS: c_uint = 0xFFFFFFFB;

pub const C_000011_E2_PROG_SHUTOFF: c_uint = 0xFFFFFFF7;

pub const C_000011_E2_PROG_DELAY_VALUE: c_uint = 0xFFFFF00F;

pub const C_000011_E2_LOWER_POWER_IDLE: c_uint = 0xFFF00FFF;

pub const C_000011_E2_LOWER_POWER_IGNORE: c_uint = 0xFFEFFFFF;

pub const C_000011_E2_NORMAL_POWER_IGNORE: c_uint = 0xFFDFFFFF;

pub const C_000011_SPARE: c_uint = 0xFF3FFFFF;

pub const C_000011_E2_NORMAL_POWER_BUSY: c_uint = 0x00FFFFFF;
pub const R_000013_IDCT_DYN_CNTL: c_uint = 0x000013;

pub const C_000013_IDCT_FORCEON: c_uint = 0xFFFFFFFE;

pub const C_000013_IDCT_MAX_DYN_STOP_LAT: c_uint = 0xFFFFFFFD;

pub const C_000013_IDCT_CLOCK_STATUS: c_uint = 0xFFFFFFFB;

pub const C_000013_IDCT_PROG_SHUTOFF: c_uint = 0xFFFFFFF7;

pub const C_000013_IDCT_PROG_DELAY_VALUE: c_uint = 0xFFFFF00F;

pub const C_000013_IDCT_LOWER_POWER_IDLE: c_uint = 0xFFF00FFF;

pub const C_000013_IDCT_LOWER_POWER_IGNORE: c_uint = 0xFFEFFFFF;

pub const C_000013_IDCT_NORMAL_POWER_IGNORE: c_uint = 0xFFDFFFFF;

pub const C_000013_SPARE: c_uint = 0xFF3FFFFF;

pub const C_000013_IDCT_NORMAL_POWER_BUSY: c_uint = 0x00FFFFFF;
