//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/r500_reg.h
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
// pipe config regs
pub const R300_GA_POLY_MODE: c_uint = 0x4288;

pub const R300_GA_ROUND_MODE: c_uint = 0x428c;

pub const R300_GB_MSPOS0: c_uint = 0x4010;

pub const R300_GB_MSPOS1: c_uint = 0x4014;

pub const R300_GA_ENHANCE: c_uint = 0x4274;

pub const R300_RB3D_DSTCACHE_CTLSTAT: c_uint = 0x4e4c;

pub const R300_RB3D_ZCACHE_CTLSTAT: c_uint = 0x4f18;

pub const R400_GB_PIPE_SELECT: c_uint = 0x402c;
pub const R500_DYN_SCLK_PWMEM_PIPE: c_uint = 0x000d /* PLL */;
pub const R500_SU_REG_DEST: c_uint = 0x42c8;
pub const R300_GB_TILE_CONFIG: c_uint = 0x4018;

pub const R300_DST_PIPE_CONFIG: c_uint = 0x170c;

pub const R300_RB2D_DSTCACHE_MODE: c_uint = 0x3428;

pub const RADEON_CP_STAT: c_uint = 0x7C0;
pub const RADEON_RBBM_CMDFIFO_ADDR: c_uint = 0xE70;
pub const RADEON_RBBM_CMDFIFO_DATA: c_uint = 0xE74;
pub const RADEON_ISYNC_CNTL: c_uint = 0x1724;

pub const RS480_NB_MC_INDEX: c_uint = 0x168;

pub const RS480_NB_MC_DATA: c_uint = 0x16c;
//
// RS690
//
pub const RS690_MCCFG_FB_LOCATION: c_uint = 0x100;
pub const RS690_MC_FB_START_MASK: c_uint = 0x0000FFFF;
pub const RS690_MC_FB_START_SHIFT: c_int = 0;
pub const RS690_MC_FB_TOP_MASK: c_uint = 0xFFFF0000;
pub const RS690_MC_FB_TOP_SHIFT: c_int = 16;
pub const RS690_MCCFG_AGP_LOCATION: c_uint = 0x101;
pub const RS690_MC_AGP_START_MASK: c_uint = 0x0000FFFF;
pub const RS690_MC_AGP_START_SHIFT: c_int = 0;
pub const RS690_MC_AGP_TOP_MASK: c_uint = 0xFFFF0000;
pub const RS690_MC_AGP_TOP_SHIFT: c_int = 16;
pub const RS690_MCCFG_AGP_BASE: c_uint = 0x102;
pub const RS690_MCCFG_AGP_BASE_2: c_uint = 0x103;
pub const RS690_MC_INIT_MISC_LAT_TIMER: c_uint = 0x104;
pub const RS690_HDP_FB_LOCATION: c_uint = 0x0134;
pub const RS690_MC_INDEX: c_uint = 0x78;

pub const RS690_MC_DATA: c_uint = 0x7c;
pub const RS690_MC_STATUS: c_uint = 0x90;

pub const RS480_AGP_BASE_2: c_uint = 0x0164;
pub const RS480_MC_MISC_CNTL: c_uint = 0x18;

pub const RS480_GART_FEATURE_ID: c_uint = 0x2b;

pub const RS480_GART_BASE: c_uint = 0x2c;
pub const RS480_GART_CACHE_CNTRL: c_uint = 0x2e;

pub const RS480_AGP_ADDRESS_SPACE_SIZE: c_uint = 0x38;

pub const RS480_AGP_MODE_CNTL: c_uint = 0x39;

pub const RS690_AIC_CTRL_SCRATCH: c_uint = 0x3A;

//
// RS600
//
pub const RS600_MC_STATUS: c_uint = 0x0;

pub const RS600_MC_INDEX: c_uint = 0x70;

pub const RS600_MC_DATA: c_uint = 0x74;
pub const RS600_MC_STATUS: c_uint = 0x0;

pub const RS600_MC_FB_LOCATION: c_uint = 0x4;
pub const RS600_MC_FB_START_MASK: c_uint = 0x0000FFFF;
pub const RS600_MC_FB_START_SHIFT: c_int = 0;
pub const RS600_MC_FB_TOP_MASK: c_uint = 0xFFFF0000;
pub const RS600_MC_FB_TOP_SHIFT: c_int = 16;
pub const RS600_MC_AGP_LOCATION: c_uint = 0x5;
pub const RS600_MC_AGP_START_MASK: c_uint = 0x0000FFFF;
pub const RS600_MC_AGP_START_SHIFT: c_int = 0;
pub const RS600_MC_AGP_TOP_MASK: c_uint = 0xFFFF0000;
pub const RS600_MC_AGP_TOP_SHIFT: c_int = 16;
pub const RS600_MC_AGP_BASE: c_uint = 0x6;
pub const RS600_MC_AGP_BASE_2: c_uint = 0x7;
pub const RS600_MC_CNTL1: c_uint = 0x9;

pub const RS600_MC_PT0_CNTL: c_uint = 0x100;

pub const RS600_MC_PT0_CONTEXT0_CNTL: c_uint = 0x102;

pub const RS600_MC_PT0_SYSTEM_APERTURE_LOW_ADDR: c_uint = 0x112;
pub const RS600_MC_PT0_SYSTEM_APERTURE_HIGH_ADDR: c_uint = 0x114;
pub const RS600_MC_PT0_CONTEXT0_DEFAULT_READ_ADDR: c_uint = 0x11c;
pub const RS600_MC_PT0_CONTEXT0_FLAT_BASE_ADDR: c_uint = 0x12c;
pub const RS600_MC_PT0_CONTEXT0_FLAT_START_ADDR: c_uint = 0x13c;
pub const RS600_MC_PT0_CONTEXT0_FLAT_END_ADDR: c_uint = 0x14c;
pub const RS600_MC_PT0_CLIENT0_CNTL: c_uint = 0x16c;

// rs600/rs690/rs740

// see RS400_MSI_REARM in AIC_CNTL for rs480
pub const RV515_MC_FB_LOCATION: c_uint = 0x01;
pub const RV515_MC_FB_START_MASK: c_uint = 0x0000FFFF;
pub const RV515_MC_FB_START_SHIFT: c_int = 0;
pub const RV515_MC_FB_TOP_MASK: c_uint = 0xFFFF0000;
pub const RV515_MC_FB_TOP_SHIFT: c_int = 16;
pub const RV515_MC_AGP_LOCATION: c_uint = 0x02;
pub const RV515_MC_AGP_START_MASK: c_uint = 0x0000FFFF;
pub const RV515_MC_AGP_START_SHIFT: c_int = 0;
pub const RV515_MC_AGP_TOP_MASK: c_uint = 0xFFFF0000;
pub const RV515_MC_AGP_TOP_SHIFT: c_int = 16;
pub const RV515_MC_AGP_BASE: c_uint = 0x03;
pub const RV515_MC_AGP_BASE_2: c_uint = 0x04;
pub const R520_MC_FB_LOCATION: c_uint = 0x04;
pub const R520_MC_FB_START_MASK: c_uint = 0x0000FFFF;
pub const R520_MC_FB_START_SHIFT: c_int = 0;
pub const R520_MC_FB_TOP_MASK: c_uint = 0xFFFF0000;
pub const R520_MC_FB_TOP_SHIFT: c_int = 16;
pub const R520_MC_AGP_LOCATION: c_uint = 0x05;
pub const R520_MC_AGP_START_MASK: c_uint = 0x0000FFFF;
pub const R520_MC_AGP_START_SHIFT: c_int = 0;
pub const R520_MC_AGP_TOP_MASK: c_uint = 0xFFFF0000;
pub const R520_MC_AGP_TOP_SHIFT: c_int = 16;
pub const R520_MC_AGP_BASE: c_uint = 0x06;
pub const R520_MC_AGP_BASE_2: c_uint = 0x07;
pub const AVIVO_MC_INDEX: c_uint = 0x0070;
pub const R520_MC_STATUS: c_uint = 0x00;

pub const RV515_MC_STATUS: c_uint = 0x08;

pub const RV515_MC_INIT_MISC_LAT_TIMER: c_uint = 0x09;
pub const AVIVO_MC_DATA: c_uint = 0x0074;
pub const R520_MC_IND_INDEX: c_uint = 0x70;

pub const R520_MC_IND_DATA: c_uint = 0x74;
pub const RV515_MC_CNTL: c_uint = 0x5;

pub const R520_MC_CNTL0: c_uint = 0x8;

pub const AVIVO_CP_DYN_CNTL: c_uint = 0x000f /* PLL */;

pub const AVIVO_E2_DYN_CNTL: c_uint = 0x0011 /* PLL */;

pub const AVIVO_IDCT_DYN_CNTL: c_uint = 0x0013 /* PLL */;

pub const AVIVO_HDP_FB_LOCATION: c_uint = 0x134;
pub const AVIVO_VGA_RENDER_CONTROL: c_uint = 0x0300;

pub const AVIVO_D1VGA_CONTROL: c_uint = 0x0330;

pub const AVIVO_D2VGA_CONTROL: c_uint = 0x0338;
pub const AVIVO_EXT1_PPLL_REF_DIV_SRC: c_uint = 0x400;
pub const AVIVO_EXT1_PPLL_REF_DIV: c_uint = 0x404;
pub const AVIVO_EXT1_PPLL_UPDATE_LOCK: c_uint = 0x408;
pub const AVIVO_EXT1_PPLL_UPDATE_CNTL: c_uint = 0x40c;
pub const AVIVO_EXT2_PPLL_REF_DIV_SRC: c_uint = 0x410;
pub const AVIVO_EXT2_PPLL_REF_DIV: c_uint = 0x414;
pub const AVIVO_EXT2_PPLL_UPDATE_LOCK: c_uint = 0x418;
pub const AVIVO_EXT2_PPLL_UPDATE_CNTL: c_uint = 0x41c;
pub const AVIVO_EXT1_PPLL_FB_DIV: c_uint = 0x430;
pub const AVIVO_EXT2_PPLL_FB_DIV: c_uint = 0x434;
pub const AVIVO_EXT1_PPLL_POST_DIV_SRC: c_uint = 0x438;
pub const AVIVO_EXT1_PPLL_POST_DIV: c_uint = 0x43c;
pub const AVIVO_EXT2_PPLL_POST_DIV_SRC: c_uint = 0x440;
pub const AVIVO_EXT2_PPLL_POST_DIV: c_uint = 0x444;
pub const AVIVO_EXT1_PPLL_CNTL: c_uint = 0x448;
pub const AVIVO_EXT2_PPLL_CNTL: c_uint = 0x44c;
pub const AVIVO_P1PLL_CNTL: c_uint = 0x450;
pub const AVIVO_P2PLL_CNTL: c_uint = 0x454;
pub const AVIVO_P1PLL_INT_SS_CNTL: c_uint = 0x458;
pub const AVIVO_P2PLL_INT_SS_CNTL: c_uint = 0x45c;
pub const AVIVO_P1PLL_TMDSA_CNTL: c_uint = 0x460;
pub const AVIVO_P2PLL_LVTMA_CNTL: c_uint = 0x464;
pub const AVIVO_PCLK_CRTC1_CNTL: c_uint = 0x480;
pub const AVIVO_PCLK_CRTC2_CNTL: c_uint = 0x484;
pub const AVIVO_D1CRTC_H_TOTAL: c_uint = 0x6000;
pub const AVIVO_D1CRTC_H_BLANK_START_END: c_uint = 0x6004;
pub const AVIVO_D1CRTC_H_SYNC_A: c_uint = 0x6008;
pub const AVIVO_D1CRTC_H_SYNC_A_CNTL: c_uint = 0x600c;
pub const AVIVO_D1CRTC_H_SYNC_B: c_uint = 0x6010;
pub const AVIVO_D1CRTC_H_SYNC_B_CNTL: c_uint = 0x6014;
pub const AVIVO_D1CRTC_V_TOTAL: c_uint = 0x6020;
pub const AVIVO_D1CRTC_V_BLANK_START_END: c_uint = 0x6024;
pub const AVIVO_D1CRTC_V_SYNC_A: c_uint = 0x6028;
pub const AVIVO_D1CRTC_V_SYNC_A_CNTL: c_uint = 0x602c;
pub const AVIVO_D1CRTC_V_SYNC_B: c_uint = 0x6030;
pub const AVIVO_D1CRTC_V_SYNC_B_CNTL: c_uint = 0x6034;
pub const AVIVO_D1CRTC_CONTROL: c_uint = 0x6080;

pub const AVIVO_D1CRTC_BLANK_CONTROL: c_uint = 0x6084;
pub const AVIVO_D1CRTC_INTERLACE_CONTROL: c_uint = 0x6088;
pub const AVIVO_D1CRTC_INTERLACE_STATUS: c_uint = 0x608c;
pub const AVIVO_D1CRTC_STATUS: c_uint = 0x609c;

pub const AVIVO_D1CRTC_STATUS_POSITION: c_uint = 0x60a0;
pub const AVIVO_D1CRTC_FRAME_COUNT: c_uint = 0x60a4;
pub const AVIVO_D1CRTC_STATUS_HV_COUNT: c_uint = 0x60ac;
pub const AVIVO_D1CRTC_STEREO_CONTROL: c_uint = 0x60c4;
pub const AVIVO_D1MODE_MASTER_UPDATE_LOCK: c_uint = 0x60e0;
pub const AVIVO_D1MODE_MASTER_UPDATE_MODE: c_uint = 0x60e4;
pub const AVIVO_D1CRTC_UPDATE_LOCK: c_uint = 0x60e8;
// master controls
pub const AVIVO_DC_CRTC_MASTER_EN: c_uint = 0x60f8;
pub const AVIVO_DC_CRTC_TV_CONTROL: c_uint = 0x60fc;
pub const AVIVO_D1GRPH_ENABLE: c_uint = 0x6100;
pub const AVIVO_D1GRPH_CONTROL: c_uint = 0x6104;

// The R7xx *_HIGH surface regs are backwards; the D1 regs are in the D2
// block and vice versa.  This applies to GRPH, CUR, etc.
//
pub const AVIVO_D1GRPH_LUT_SEL: c_uint = 0x6108;

pub const AVIVO_D1GRPH_PRIMARY_SURFACE_ADDRESS: c_uint = 0x6110;
pub const R700_D1GRPH_PRIMARY_SURFACE_ADDRESS_HIGH: c_uint = 0x6914;
pub const R700_D2GRPH_PRIMARY_SURFACE_ADDRESS_HIGH: c_uint = 0x6114;
pub const AVIVO_D1GRPH_SECONDARY_SURFACE_ADDRESS: c_uint = 0x6118;
pub const R700_D1GRPH_SECONDARY_SURFACE_ADDRESS_HIGH: c_uint = 0x691c;
pub const R700_D2GRPH_SECONDARY_SURFACE_ADDRESS_HIGH: c_uint = 0x611c;
pub const AVIVO_D1GRPH_PITCH: c_uint = 0x6120;
pub const AVIVO_D1GRPH_SURFACE_OFFSET_X: c_uint = 0x6124;
pub const AVIVO_D1GRPH_SURFACE_OFFSET_Y: c_uint = 0x6128;
pub const AVIVO_D1GRPH_X_START: c_uint = 0x612c;
pub const AVIVO_D1GRPH_Y_START: c_uint = 0x6130;
pub const AVIVO_D1GRPH_X_END: c_uint = 0x6134;
pub const AVIVO_D1GRPH_Y_END: c_uint = 0x6138;
pub const AVIVO_D1GRPH_UPDATE: c_uint = 0x6144;

pub const AVIVO_D1GRPH_FLIP_CONTROL: c_uint = 0x6148;

pub const AVIVO_D1CUR_CONTROL: c_uint = 0x6400;

pub const AVIVO_D1CUR_SURFACE_ADDRESS: c_uint = 0x6408;
pub const R700_D1CUR_SURFACE_ADDRESS_HIGH: c_uint = 0x6c0c;
pub const R700_D2CUR_SURFACE_ADDRESS_HIGH: c_uint = 0x640c;
pub const AVIVO_D1CUR_SIZE: c_uint = 0x6410;
pub const AVIVO_D1CUR_POSITION: c_uint = 0x6414;
pub const AVIVO_D1CUR_HOT_SPOT: c_uint = 0x6418;
pub const AVIVO_D1CUR_UPDATE: c_uint = 0x6424;

pub const AVIVO_DC_LUT_RW_SELECT: c_uint = 0x6480;
pub const AVIVO_DC_LUT_RW_MODE: c_uint = 0x6484;
pub const AVIVO_DC_LUT_RW_INDEX: c_uint = 0x6488;
pub const AVIVO_DC_LUT_SEQ_COLOR: c_uint = 0x648c;
pub const AVIVO_DC_LUT_PWL_DATA: c_uint = 0x6490;
pub const AVIVO_DC_LUT_30_COLOR: c_uint = 0x6494;
pub const AVIVO_DC_LUT_READ_PIPE_SELECT: c_uint = 0x6498;
pub const AVIVO_DC_LUT_WRITE_EN_MASK: c_uint = 0x649c;
pub const AVIVO_DC_LUT_AUTOFILL: c_uint = 0x64a0;
pub const AVIVO_DC_LUTA_CONTROL: c_uint = 0x64c0;
pub const AVIVO_DC_LUTA_BLACK_OFFSET_BLUE: c_uint = 0x64c4;
pub const AVIVO_DC_LUTA_BLACK_OFFSET_GREEN: c_uint = 0x64c8;
pub const AVIVO_DC_LUTA_BLACK_OFFSET_RED: c_uint = 0x64cc;
pub const AVIVO_DC_LUTA_WHITE_OFFSET_BLUE: c_uint = 0x64d0;
pub const AVIVO_DC_LUTA_WHITE_OFFSET_GREEN: c_uint = 0x64d4;
pub const AVIVO_DC_LUTA_WHITE_OFFSET_RED: c_uint = 0x64d8;
pub const AVIVO_DC_LB_MEMORY_SPLIT: c_uint = 0x6520;

pub const AVIVO_D1MODE_DATA_FORMAT: c_uint = 0x6528;

pub const AVIVO_D1MODE_DESKTOP_HEIGHT: c_uint = 0x652C;
pub const AVIVO_D1MODE_VBLANK_STATUS: c_uint = 0x6534;

pub const AVIVO_D1MODE_VLINE_START_END: c_uint = 0x6538;
pub const AVIVO_D1MODE_VLINE_STATUS: c_uint = 0x653c;

pub const AVIVO_DxMODE_INT_MASK: c_uint = 0x6540;

pub const AVIVO_D1MODE_VIEWPORT_START: c_uint = 0x6580;
pub const AVIVO_D1MODE_VIEWPORT_SIZE: c_uint = 0x6584;
pub const AVIVO_D1MODE_EXT_OVERSCAN_LEFT_RIGHT: c_uint = 0x6588;
pub const AVIVO_D1MODE_EXT_OVERSCAN_TOP_BOTTOM: c_uint = 0x658c;
pub const AVIVO_D1SCL_SCALER_ENABLE: c_uint = 0x6590;
pub const AVIVO_D1SCL_SCALER_TAP_CONTROL: c_uint = 0x6594;
pub const AVIVO_D1SCL_UPDATE: c_uint = 0x65cc;

// second crtc
pub const AVIVO_D2CRTC_H_TOTAL: c_uint = 0x6800;
pub const AVIVO_D2CRTC_H_BLANK_START_END: c_uint = 0x6804;
pub const AVIVO_D2CRTC_H_SYNC_A: c_uint = 0x6808;
pub const AVIVO_D2CRTC_H_SYNC_A_CNTL: c_uint = 0x680c;
pub const AVIVO_D2CRTC_H_SYNC_B: c_uint = 0x6810;
pub const AVIVO_D2CRTC_H_SYNC_B_CNTL: c_uint = 0x6814;
pub const AVIVO_D2CRTC_V_TOTAL: c_uint = 0x6820;
pub const AVIVO_D2CRTC_V_BLANK_START_END: c_uint = 0x6824;
pub const AVIVO_D2CRTC_V_SYNC_A: c_uint = 0x6828;
pub const AVIVO_D2CRTC_V_SYNC_A_CNTL: c_uint = 0x682c;
pub const AVIVO_D2CRTC_V_SYNC_B: c_uint = 0x6830;
pub const AVIVO_D2CRTC_V_SYNC_B_CNTL: c_uint = 0x6834;
pub const AVIVO_D2CRTC_CONTROL: c_uint = 0x6880;
pub const AVIVO_D2CRTC_BLANK_CONTROL: c_uint = 0x6884;
pub const AVIVO_D2CRTC_INTERLACE_CONTROL: c_uint = 0x6888;
pub const AVIVO_D2CRTC_INTERLACE_STATUS: c_uint = 0x688c;
pub const AVIVO_D2CRTC_STATUS_POSITION: c_uint = 0x68a0;
pub const AVIVO_D2CRTC_FRAME_COUNT: c_uint = 0x68a4;
pub const AVIVO_D2CRTC_STEREO_CONTROL: c_uint = 0x68c4;
pub const AVIVO_D2GRPH_ENABLE: c_uint = 0x6900;
pub const AVIVO_D2GRPH_CONTROL: c_uint = 0x6904;
pub const AVIVO_D2GRPH_LUT_SEL: c_uint = 0x6908;
pub const AVIVO_D2GRPH_PRIMARY_SURFACE_ADDRESS: c_uint = 0x6910;
pub const AVIVO_D2GRPH_SECONDARY_SURFACE_ADDRESS: c_uint = 0x6918;
pub const AVIVO_D2GRPH_PITCH: c_uint = 0x6920;
pub const AVIVO_D2GRPH_SURFACE_OFFSET_X: c_uint = 0x6924;
pub const AVIVO_D2GRPH_SURFACE_OFFSET_Y: c_uint = 0x6928;
pub const AVIVO_D2GRPH_X_START: c_uint = 0x692c;
pub const AVIVO_D2GRPH_Y_START: c_uint = 0x6930;
pub const AVIVO_D2GRPH_X_END: c_uint = 0x6934;
pub const AVIVO_D2GRPH_Y_END: c_uint = 0x6938;
pub const AVIVO_D2GRPH_UPDATE: c_uint = 0x6944;
pub const AVIVO_D2GRPH_FLIP_CONTROL: c_uint = 0x6948;
pub const AVIVO_D2CUR_CONTROL: c_uint = 0x6c00;
pub const AVIVO_D2CUR_SURFACE_ADDRESS: c_uint = 0x6c08;
pub const AVIVO_D2CUR_SIZE: c_uint = 0x6c10;
pub const AVIVO_D2CUR_POSITION: c_uint = 0x6c14;
pub const AVIVO_D2MODE_VBLANK_STATUS: c_uint = 0x6d34;
pub const AVIVO_D2MODE_VLINE_START_END: c_uint = 0x6d38;
pub const AVIVO_D2MODE_VLINE_STATUS: c_uint = 0x6d3c;
pub const AVIVO_D2MODE_VIEWPORT_START: c_uint = 0x6d80;
pub const AVIVO_D2MODE_VIEWPORT_SIZE: c_uint = 0x6d84;
pub const AVIVO_D2MODE_EXT_OVERSCAN_LEFT_RIGHT: c_uint = 0x6d88;
pub const AVIVO_D2MODE_EXT_OVERSCAN_TOP_BOTTOM: c_uint = 0x6d8c;
pub const AVIVO_D2SCL_SCALER_ENABLE: c_uint = 0x6d90;
pub const AVIVO_D2SCL_SCALER_TAP_CONTROL: c_uint = 0x6d94;
pub const AVIVO_DDIA_BIT_DEPTH_CONTROL: c_uint = 0x7214;
pub const AVIVO_DACA_ENABLE: c_uint = 0x7800;

pub const AVIVO_DACA_SOURCE_SELECT: c_uint = 0x7804;

pub const AVIVO_DACA_FORCE_OUTPUT_CNTL: c_uint = 0x783c;

pub const AVIVO_DACA_POWERDOWN: c_uint = 0x7850;

pub const AVIVO_DACB_ENABLE: c_uint = 0x7a00;
pub const AVIVO_DACB_SOURCE_SELECT: c_uint = 0x7a04;
pub const AVIVO_DACB_FORCE_OUTPUT_CNTL: c_uint = 0x7a3c;

pub const AVIVO_DACB_POWERDOWN: c_uint = 0x7a50;

pub const AVIVO_TMDSA_CNTL: c_uint = 0x7880;

pub const AVIVO_TMDSA_SOURCE_SELECT: c_uint = 0x7884;
// 78a8 appears to be some kind of (reasonably tolerant) clock?
// 78d0 definitely hits the transmitter, definitely clock.
// MYSTERY1 This appears to control dithering?
pub const AVIVO_TMDSA_BIT_DEPTH_CONTROL: c_uint = 0x7894;

pub const AVIVO_TMDSA_DCBALANCER_CONTROL: c_uint = 0x78d0;

pub const AVIVO_TMDSA_DATA_SYNCHRONIZATION: c_uint = 0x78d8;

pub const AVIVO_TMDSA_CLOCK_ENABLE: c_uint = 0x7900;
pub const AVIVO_TMDSA_TRANSMITTER_ENABLE: c_uint = 0x7904;

pub const AVIVO_TMDSA_TRANSMITTER_CONTROL: c_uint = 0x7910;

pub const AVIVO_LVTMA_CNTL: c_uint = 0x7a80;

pub const AVIVO_LVTMA_SOURCE_SELECT: c_uint = 0x7a84;
pub const AVIVO_LVTMA_COLOR_FORMAT: c_uint = 0x7a88;
pub const AVIVO_LVTMA_BIT_DEPTH_CONTROL: c_uint = 0x7a94;

pub const AVIVO_LVTMA_DCBALANCER_CONTROL: c_uint = 0x7ad0;

pub const AVIVO_LVTMA_DATA_SYNCHRONIZATION: c_uint = 0x78d8;

pub const R500_LVTMA_CLOCK_ENABLE: c_uint = 0x7b00;
pub const R600_LVTMA_CLOCK_ENABLE: c_uint = 0x7b04;
pub const R500_LVTMA_TRANSMITTER_ENABLE: c_uint = 0x7b04;
pub const R600_LVTMA_TRANSMITTER_ENABLE: c_uint = 0x7b08;

pub const R500_LVTMA_TRANSMITTER_CONTROL: c_uint = 0x7b10;
pub const R600_LVTMA_TRANSMITTER_CONTROL: c_uint = 0x7b14;

pub const R500_LVTMA_PWRSEQ_CNTL: c_uint = 0x7af0;
pub const R600_LVTMA_PWRSEQ_CNTL: c_uint = 0x7af4;

pub const R500_LVTMA_PWRSEQ_STATE: c_uint = 0x7af4;
pub const R600_LVTMA_PWRSEQ_STATE: c_uint = 0x7af8;

pub const AVIVO_LVDS_BACKLIGHT_CNTL: c_uint = 0x7af8;

pub const AVIVO_DVOA_BIT_DEPTH_CONTROL: c_uint = 0x7988;
pub const AVIVO_DC_GPIO_HPD_A: c_uint = 0x7e94;
pub const AVIVO_DC_GPIO_HPD_Y: c_uint = 0x7e9c;
pub const AVIVO_DC_I2C_STATUS1: c_uint = 0x7d30;

pub const AVIVO_DC_I2C_RESET: c_uint = 0x7d34;

pub const AVIVO_DC_I2C_CONTROL1: c_uint = 0x7d38;

pub const AVIVO_DC_I2C_CONTROL2: c_uint = 0x7d3c;

pub const AVIVO_DC_I2C_CONTROL3: c_uint = 0x7d40;

pub const AVIVO_DC_I2C_DATA: c_uint = 0x7d44;
pub const AVIVO_DC_I2C_INTERRUPT_CONTROL: c_uint = 0x7d48;

pub const AVIVO_DC_I2C_ARBITRATION: c_uint = 0x7d50;

pub const AVIVO_DC_GPIO_DDC1_MASK: c_uint = 0x7e40;
pub const AVIVO_DC_GPIO_DDC1_A: c_uint = 0x7e44;
pub const AVIVO_DC_GPIO_DDC1_EN: c_uint = 0x7e48;
pub const AVIVO_DC_GPIO_DDC1_Y: c_uint = 0x7e4c;
pub const AVIVO_DC_GPIO_DDC2_MASK: c_uint = 0x7e50;
pub const AVIVO_DC_GPIO_DDC2_A: c_uint = 0x7e54;
pub const AVIVO_DC_GPIO_DDC2_EN: c_uint = 0x7e58;
pub const AVIVO_DC_GPIO_DDC2_Y: c_uint = 0x7e5c;
pub const AVIVO_DC_GPIO_DDC3_MASK: c_uint = 0x7e60;
pub const AVIVO_DC_GPIO_DDC3_A: c_uint = 0x7e64;
pub const AVIVO_DC_GPIO_DDC3_EN: c_uint = 0x7e68;
pub const AVIVO_DC_GPIO_DDC3_Y: c_uint = 0x7e6c;
pub const AVIVO_DISP_INTERRUPT_STATUS: c_uint = 0x7edc;

