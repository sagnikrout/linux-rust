//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/permedia2.h
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
// Permedia2 framebuffer driver definitions.
// Copyright (c) 1998-2000 Ilario Nardinocchi (nardinoc@CS.UniBO.IT)
// --------------------------------------------------------------------------
// $Id: pm2fb.h,v 1.26 2000/09/19 00:11:53 illo Exp $
// --------------------------------------------------------------------------
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

pub const PM2_REGS_SIZE: c_uint = 0x10000;

//
// Permedia2 registers used in the framebuffer
//
pub const PM2R_RESET_STATUS: c_uint = 0x0000;
pub const PM2R_IN_FIFO_SPACE: c_uint = 0x0018;
pub const PM2R_OUT_FIFO_WORDS: c_uint = 0x0020;
pub const PM2R_APERTURE_ONE: c_uint = 0x0050;
pub const PM2R_APERTURE_TWO: c_uint = 0x0058;
pub const PM2R_FIFO_DISCON: c_uint = 0x0068;
pub const PM2R_CHIP_CONFIG: c_uint = 0x0070;
pub const PM2R_REBOOT: c_uint = 0x1000;
pub const PM2R_MEM_CONTROL: c_uint = 0x1040;
pub const PM2R_BOOT_ADDRESS: c_uint = 0x1080;
pub const PM2R_MEM_CONFIG: c_uint = 0x10c0;
pub const PM2R_BYPASS_WRITE_MASK: c_uint = 0x1100;
pub const PM2R_FRAMEBUFFER_WRITE_MASK: c_uint = 0x1140;
pub const PM2R_OUT_FIFO: c_uint = 0x2000;
pub const PM2R_SCREEN_BASE: c_uint = 0x3000;
pub const PM2R_SCREEN_STRIDE: c_uint = 0x3008;
pub const PM2R_H_TOTAL: c_uint = 0x3010;
pub const PM2R_HG_END: c_uint = 0x3018;
pub const PM2R_HB_END: c_uint = 0x3020;
pub const PM2R_HS_START: c_uint = 0x3028;
pub const PM2R_HS_END: c_uint = 0x3030;
pub const PM2R_V_TOTAL: c_uint = 0x3038;
pub const PM2R_VB_END: c_uint = 0x3040;
pub const PM2R_VS_START: c_uint = 0x3048;
pub const PM2R_VS_END: c_uint = 0x3050;
pub const PM2R_VIDEO_CONTROL: c_uint = 0x3058;
pub const PM2R_LINE_COUNT: c_uint = 0x3070;
pub const PM2R_FIFO_CONTROL: c_uint = 0x3078;
pub const PM2R_RD_PALETTE_WRITE_ADDRESS: c_uint = 0x4000;
pub const PM2R_RD_PALETTE_DATA: c_uint = 0x4008;
pub const PM2R_RD_PIXEL_MASK: c_uint = 0x4010;
pub const PM2R_RD_PALETTE_READ_ADDRESS: c_uint = 0x4018;
pub const PM2R_RD_CURSOR_COLOR_ADDRESS: c_uint = 0x4020;
pub const PM2R_RD_CURSOR_COLOR_DATA: c_uint = 0x4028;
pub const PM2R_RD_INDEXED_DATA: c_uint = 0x4050;
pub const PM2R_RD_CURSOR_DATA: c_uint = 0x4058;
pub const PM2R_RD_CURSOR_X_LSB: c_uint = 0x4060;
pub const PM2R_RD_CURSOR_X_MSB: c_uint = 0x4068;
pub const PM2R_RD_CURSOR_Y_LSB: c_uint = 0x4070;
pub const PM2R_RD_CURSOR_Y_MSB: c_uint = 0x4078;
pub const PM2R_START_X_DOM: c_uint = 0x8000;
pub const PM2R_D_X_DOM: c_uint = 0x8008;
pub const PM2R_START_X_SUB: c_uint = 0x8010;
pub const PM2R_D_X_SUB: c_uint = 0x8018;
pub const PM2R_START_Y: c_uint = 0x8020;
pub const PM2R_D_Y: c_uint = 0x8028;
pub const PM2R_COUNT: c_uint = 0x8030;
pub const PM2R_RENDER: c_uint = 0x8038;
pub const PM2R_BIT_MASK_PATTERN: c_uint = 0x8068;
pub const PM2R_RASTERIZER_MODE: c_uint = 0x80a0;
pub const PM2R_RECTANGLE_ORIGIN: c_uint = 0x80d0;
pub const PM2R_RECTANGLE_SIZE: c_uint = 0x80d8;
pub const PM2R_PACKED_DATA_LIMITS: c_uint = 0x8150;
pub const PM2R_SCISSOR_MODE: c_uint = 0x8180;
pub const PM2R_SCISSOR_MIN_XY: c_uint = 0x8188;
pub const PM2R_SCISSOR_MAX_XY: c_uint = 0x8190;
pub const PM2R_SCREEN_SIZE: c_uint = 0x8198;
pub const PM2R_AREA_STIPPLE_MODE: c_uint = 0x81a0;
pub const PM2R_WINDOW_ORIGIN: c_uint = 0x81c8;
pub const PM2R_TEXTURE_ADDRESS_MODE: c_uint = 0x8380;
pub const PM2R_TEXTURE_MAP_FORMAT: c_uint = 0x8588;
pub const PM2R_TEXTURE_DATA_FORMAT: c_uint = 0x8590;
pub const PM2R_TEXTURE_READ_MODE: c_uint = 0x8670;
pub const PM2R_TEXEL_LUT_MODE: c_uint = 0x8678;
pub const PM2R_TEXTURE_COLOR_MODE: c_uint = 0x8680;
pub const PM2R_FOG_MODE: c_uint = 0x8690;
pub const PM2R_TEXEL0: c_uint = 0x8760;
pub const PM2R_COLOR_DDA_MODE: c_uint = 0x87e0;
pub const PM2R_CONSTANT_COLOR: c_uint = 0x87e8;
pub const PM2R_ALPHA_BLEND_MODE: c_uint = 0x8810;
pub const PM2R_DITHER_MODE: c_uint = 0x8818;
pub const PM2R_FB_SOFT_WRITE_MASK: c_uint = 0x8820;
pub const PM2R_LOGICAL_OP_MODE: c_uint = 0x8828;
pub const PM2R_LB_READ_MODE: c_uint = 0x8880;
pub const PM2R_LB_READ_FORMAT: c_uint = 0x8888;
pub const PM2R_LB_SOURCE_OFFSET: c_uint = 0x8890;
pub const PM2R_LB_WINDOW_BASE: c_uint = 0x88b8;
pub const PM2R_LB_WRITE_FORMAT: c_uint = 0x88c8;
pub const PM2R_STENCIL_MODE: c_uint = 0x8988;
pub const PM2R_DEPTH_MODE: c_uint = 0x89a0;
pub const PM2R_FB_READ_MODE: c_uint = 0x8a80;
pub const PM2R_FB_SOURCE_OFFSET: c_uint = 0x8a88;
pub const PM2R_FB_PIXEL_OFFSET: c_uint = 0x8a90;
pub const PM2R_FB_WINDOW_BASE: c_uint = 0x8ab0;
pub const PM2R_FB_WRITE_MODE: c_uint = 0x8ab8;
pub const PM2R_FB_HARD_WRITE_MASK: c_uint = 0x8ac0;
pub const PM2R_FB_BLOCK_COLOR: c_uint = 0x8ac8;
pub const PM2R_FB_READ_PIXEL: c_uint = 0x8ad0;
pub const PM2R_FILTER_MODE: c_uint = 0x8c00;
pub const PM2R_SYNC: c_uint = 0x8c40;
pub const PM2R_YUV_MODE: c_uint = 0x8f00;
pub const PM2R_STATISTICS_MODE: c_uint = 0x8c08;
pub const PM2R_FB_SOURCE_DELTA: c_uint = 0x8d88;
pub const PM2R_CONFIG: c_uint = 0x8d90;
pub const PM2R_DELTA_MODE: c_uint = 0x9300;
// Permedia2v
pub const PM2VR_RD_INDEX_LOW: c_uint = 0x4020;
pub const PM2VR_RD_INDEX_HIGH: c_uint = 0x4028;
pub const PM2VR_RD_INDEXED_DATA: c_uint = 0x4030;
// Permedia2 RAMDAC indexed registers
pub const PM2I_RD_CURSOR_CONTROL: c_uint = 0x06;
pub const PM2I_RD_COLOR_MODE: c_uint = 0x18;
pub const PM2I_RD_MODE_CONTROL: c_uint = 0x19;
pub const PM2I_RD_MISC_CONTROL: c_uint = 0x1e;
pub const PM2I_RD_PIXEL_CLOCK_A1: c_uint = 0x20;
pub const PM2I_RD_PIXEL_CLOCK_A2: c_uint = 0x21;
pub const PM2I_RD_PIXEL_CLOCK_A3: c_uint = 0x22;
pub const PM2I_RD_PIXEL_CLOCK_STATUS: c_uint = 0x29;
pub const PM2I_RD_MEMORY_CLOCK_1: c_uint = 0x30;
pub const PM2I_RD_MEMORY_CLOCK_2: c_uint = 0x31;
pub const PM2I_RD_MEMORY_CLOCK_3: c_uint = 0x32;
pub const PM2I_RD_MEMORY_CLOCK_STATUS: c_uint = 0x33;
pub const PM2I_RD_COLOR_KEY_CONTROL: c_uint = 0x40;
pub const PM2I_RD_OVERLAY_KEY: c_uint = 0x41;
pub const PM2I_RD_RED_KEY: c_uint = 0x42;
pub const PM2I_RD_GREEN_KEY: c_uint = 0x43;
pub const PM2I_RD_BLUE_KEY: c_uint = 0x44;
// Permedia2v extensions
pub const PM2VI_RD_MISC_CONTROL: c_uint = 0x000;
pub const PM2VI_RD_SYNC_CONTROL: c_uint = 0x001;
pub const PM2VI_RD_DAC_CONTROL: c_uint = 0x002;
pub const PM2VI_RD_PIXEL_SIZE: c_uint = 0x003;
pub const PM2VI_RD_COLOR_FORMAT: c_uint = 0x004;
pub const PM2VI_RD_CURSOR_MODE: c_uint = 0x005;
pub const PM2VI_RD_CURSOR_X_LOW: c_uint = 0x007;
pub const PM2VI_RD_CURSOR_X_HIGH: c_uint = 0x008;
pub const PM2VI_RD_CURSOR_Y_LOW: c_uint = 0x009;
pub const PM2VI_RD_CURSOR_Y_HIGH: c_uint = 0x00A;
pub const PM2VI_RD_CURSOR_X_HOT: c_uint = 0x00B;
pub const PM2VI_RD_CURSOR_Y_HOT: c_uint = 0x00C;
pub const PM2VI_RD_OVERLAY_KEY: c_uint = 0x00D;
pub const PM2VI_RD_CLK0_PRESCALE: c_uint = 0x201;
pub const PM2VI_RD_CLK0_FEEDBACK: c_uint = 0x202;
pub const PM2VI_RD_CLK0_POSTSCALE: c_uint = 0x203;
pub const PM2VI_RD_CLK1_PRESCALE: c_uint = 0x204;
pub const PM2VI_RD_CLK1_FEEDBACK: c_uint = 0x205;
pub const PM2VI_RD_CLK1_POSTSCALE: c_uint = 0x206;
pub const PM2VI_RD_MCLK_CONTROL: c_uint = 0x20D;
pub const PM2VI_RD_MCLK_PRESCALE: c_uint = 0x20E;
pub const PM2VI_RD_MCLK_FEEDBACK: c_uint = 0x20F;
pub const PM2VI_RD_MCLK_POSTSCALE: c_uint = 0x210;
pub const PM2VI_RD_CURSOR_PALETTE: c_uint = 0x303;
pub const PM2VI_RD_CURSOR_PATTERN: c_uint = 0x400;
// Fields and flags

pub const PM2F_RENDER_LINE: c_int = 0;

pub const PM2F_PLL_LOCKED: c_uint = 0x10;

pub const PM2F_DATATYPE_COLOR: c_uint = 0x8000;
pub const PM2F_VGA_ENABLE: c_uint = 0x02;
pub const PM2F_VGA_FIXED: c_uint = 0x04;
pub const PM2F_FB_WRITE_ENABLE: c_uint = 0x01;
pub const PM2F_FB_READ_SOURCE_ENABLE: c_uint = 0x0200;
pub const PM2F_RD_PALETTE_WIDTH_8: c_uint = 0x02;
pub const PM2F_PART_PROD_MASK: c_uint = 0x01ff;
pub const PM2F_SCREEN_SCISSOR_ENABLE: c_uint = 0x02;
pub const PM2F_DATA_64_ENABLE: c_uint = 0x00010000;
pub const PM2F_BLANK_LOW: c_uint = 0x02;
pub const PM2F_HSYNC_MASK: c_uint = 0x18;
pub const PM2F_VSYNC_MASK: c_uint = 0x60;
pub const PM2F_HSYNC_ACT_HIGH: c_uint = 0x08;
pub const PM2F_HSYNC_FORCED_LOW: c_uint = 0x10;
pub const PM2F_HSYNC_ACT_LOW: c_uint = 0x18;
pub const PM2F_VSYNC_ACT_HIGH: c_uint = 0x20;
pub const PM2F_VSYNC_FORCED_LOW: c_uint = 0x40;
pub const PM2F_VSYNC_ACT_LOW: c_uint = 0x60;
pub const PM2F_LINE_DOUBLE: c_uint = 0x04;
pub const PM2F_VIDEO_ENABLE: c_uint = 0x01;
pub const PM2F_RD_PIXELFORMAT_SVGA: c_uint = 0x01;
pub const PM2F_RD_PIXELFORMAT_RGB232OFFSET: c_uint = 0x02;
pub const PM2F_RD_PIXELFORMAT_RGBA2321: c_uint = 0x03;
pub const PM2F_RD_PIXELFORMAT_RGBA5551: c_uint = 0x04;
pub const PM2F_RD_PIXELFORMAT_RGBA4444: c_uint = 0x05;
pub const PM2F_RD_PIXELFORMAT_RGB565: c_uint = 0x06;
pub const PM2F_RD_PIXELFORMAT_RGBA8888: c_uint = 0x08;
pub const PM2F_RD_PIXELFORMAT_RGB888: c_uint = 0x09;
pub const PM2F_RD_GUI_ACTIVE: c_uint = 0x10;
pub const PM2F_RD_COLOR_MODE_RGB: c_uint = 0x20;

pub const PM2F_RD_TRUECOLOR: c_uint = 0x80;
pub const PM2F_NO_ALPHA_BUFFER: c_uint = 0x10;
pub const PM2F_TEXTEL_SIZE_16: c_uint = 0x00080000;
pub const PM2F_TEXTEL_SIZE_32: c_uint = 0x00100000;
pub const PM2F_TEXTEL_SIZE_4: c_uint = 0x00180000;
pub const PM2F_TEXTEL_SIZE_24: c_uint = 0x00200000;

pub const PM2F_APERTURE_STANDARD: c_int = 0;
pub const PM2F_APERTURE_BYTESWAP: c_int = 1;
pub const PM2F_APERTURE_HALFWORDSWAP: c_int = 2;

//
// That's all folks!
//
