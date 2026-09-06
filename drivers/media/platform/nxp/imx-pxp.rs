//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/nxp/imx-pxp.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Freescale PXP Register Definitions
//
// based on pxp_dma_v3.h, Xml Revision: 1.77, Template Revision: 1.3
//
// Copyright 2014-2015 Freescale Semiconductor, Inc. All Rights Reserved.
//

pub const BM_PXP_CTRL_SFTRST: c_uint = 0x80000000;

pub const BM_PXP_CTRL_CLKGATE: c_uint = 0x40000000;

pub const BM_PXP_CTRL_RSVD4: c_uint = 0x20000000;

pub const BM_PXP_CTRL_EN_REPEAT: c_uint = 0x10000000;

pub const BM_PXP_CTRL_ENABLE_ROTATE1: c_uint = 0x08000000;

pub const BM_PXP_CTRL_ENABLE_ROTATE0: c_uint = 0x04000000;

pub const BM_PXP_CTRL_ENABLE_LUT: c_uint = 0x02000000;

pub const BM_PXP_CTRL_ENABLE_CSC2: c_uint = 0x01000000;

pub const BM_PXP_CTRL_BLOCK_SIZE: c_uint = 0x00800000;

pub const BV_PXP_CTRL_BLOCK_SIZE__8X8: c_uint = 0x0;
pub const BV_PXP_CTRL_BLOCK_SIZE__16X16: c_uint = 0x1;
pub const BM_PXP_CTRL_RSVD1: c_uint = 0x00400000;

pub const BM_PXP_CTRL_ENABLE_ALPHA_B: c_uint = 0x00200000;

pub const BM_PXP_CTRL_ENABLE_INPUT_FETCH_STORE: c_uint = 0x00100000;

pub const BM_PXP_CTRL_ENABLE_WFE_B: c_uint = 0x00080000;

pub const BM_PXP_CTRL_ENABLE_WFE_A: c_uint = 0x00040000;

pub const BM_PXP_CTRL_ENABLE_DITHER: c_uint = 0x00020000;

pub const BM_PXP_CTRL_ENABLE_PS_AS_OUT: c_uint = 0x00010000;

pub const BM_PXP_CTRL_VFLIP1: c_uint = 0x00008000;

pub const BM_PXP_CTRL_HFLIP1: c_uint = 0x00004000;

pub const BP_PXP_CTRL_ROTATE1: c_int = 12;
pub const BM_PXP_CTRL_ROTATE1: c_uint = 0x00003000;

pub const BV_PXP_CTRL_ROTATE1__ROT_0: c_uint = 0x0;
pub const BV_PXP_CTRL_ROTATE1__ROT_90: c_uint = 0x1;
pub const BV_PXP_CTRL_ROTATE1__ROT_180: c_uint = 0x2;
pub const BV_PXP_CTRL_ROTATE1__ROT_270: c_uint = 0x3;
pub const BM_PXP_CTRL_VFLIP0: c_uint = 0x00000800;

pub const BM_PXP_CTRL_HFLIP0: c_uint = 0x00000400;

pub const BP_PXP_CTRL_ROTATE0: c_int = 8;
pub const BM_PXP_CTRL_ROTATE0: c_uint = 0x00000300;

pub const BV_PXP_CTRL_ROTATE0__ROT_0: c_uint = 0x0;
pub const BV_PXP_CTRL_ROTATE0__ROT_90: c_uint = 0x1;
pub const BV_PXP_CTRL_ROTATE0__ROT_180: c_uint = 0x2;
pub const BV_PXP_CTRL_ROTATE0__ROT_270: c_uint = 0x3;
pub const BP_PXP_CTRL_RSVD0: c_int = 6;
pub const BM_PXP_CTRL_RSVD0: c_uint = 0x000000C0;

pub const BM_PXP_CTRL_HANDSHAKE_ABORT_SKIP: c_uint = 0x00000020;

pub const BM_PXP_CTRL_ENABLE_LCD0_HANDSHAKE: c_uint = 0x00000010;

pub const BM_PXP_CTRL_LUT_DMA_IRQ_ENABLE: c_uint = 0x00000008;

pub const BM_PXP_CTRL_NEXT_IRQ_ENABLE: c_uint = 0x00000004;

pub const BM_PXP_CTRL_IRQ_ENABLE: c_uint = 0x00000002;

pub const BM_PXP_CTRL_ENABLE: c_uint = 0x00000001;

pub const BP_PXP_STAT_BLOCKX: c_int = 24;
pub const BM_PXP_STAT_BLOCKX: c_uint = 0xFF000000;

pub const BP_PXP_STAT_BLOCKY: c_int = 16;
pub const BM_PXP_STAT_BLOCKY: c_uint = 0x00FF0000;

pub const BP_PXP_STAT_AXI_ERROR_ID_1: c_int = 12;
pub const BM_PXP_STAT_AXI_ERROR_ID_1: c_uint = 0x0000F000;

pub const BM_PXP_STAT_RSVD2: c_uint = 0x00000800;

pub const BM_PXP_STAT_AXI_READ_ERROR_1: c_uint = 0x00000400;

pub const BM_PXP_STAT_AXI_WRITE_ERROR_1: c_uint = 0x00000200;

pub const BM_PXP_STAT_LUT_DMA_LOAD_DONE_IRQ: c_uint = 0x00000100;

pub const BP_PXP_STAT_AXI_ERROR_ID_0: c_int = 4;
pub const BM_PXP_STAT_AXI_ERROR_ID_0: c_uint = 0x000000F0;

pub const BM_PXP_STAT_NEXT_IRQ: c_uint = 0x00000008;

pub const BM_PXP_STAT_AXI_READ_ERROR_0: c_uint = 0x00000004;

pub const BM_PXP_STAT_AXI_WRITE_ERROR_0: c_uint = 0x00000002;

pub const BM_PXP_STAT_IRQ0: c_uint = 0x00000001;

pub const BP_PXP_OUT_CTRL_ALPHA: c_int = 24;
pub const BM_PXP_OUT_CTRL_ALPHA: c_uint = 0xFF000000;

pub const BM_PXP_OUT_CTRL_ALPHA_OUTPUT: c_uint = 0x00800000;

pub const BP_PXP_OUT_CTRL_RSVD1: c_int = 10;
pub const BM_PXP_OUT_CTRL_RSVD1: c_uint = 0x007FFC00;

pub const BP_PXP_OUT_CTRL_INTERLACED_OUTPUT: c_int = 8;
pub const BM_PXP_OUT_CTRL_INTERLACED_OUTPUT: c_uint = 0x00000300;

pub const BV_PXP_OUT_CTRL_INTERLACED_OUTPUT__PROGRESSIVE: c_uint = 0x0;
pub const BV_PXP_OUT_CTRL_INTERLACED_OUTPUT__FIELD0: c_uint = 0x1;
pub const BV_PXP_OUT_CTRL_INTERLACED_OUTPUT__FIELD1: c_uint = 0x2;
pub const BV_PXP_OUT_CTRL_INTERLACED_OUTPUT__INTERLACED: c_uint = 0x3;
pub const BP_PXP_OUT_CTRL_RSVD0: c_int = 5;
pub const BM_PXP_OUT_CTRL_RSVD0: c_uint = 0x000000E0;

pub const BP_PXP_OUT_CTRL_FORMAT: c_int = 0;
pub const BM_PXP_OUT_CTRL_FORMAT: c_uint = 0x0000001F;

pub const BV_PXP_OUT_CTRL_FORMAT__ARGB8888: c_uint = 0x0;
pub const BV_PXP_OUT_CTRL_FORMAT__RGB888: c_uint = 0x4;
pub const BV_PXP_OUT_CTRL_FORMAT__RGB888P: c_uint = 0x5;
pub const BV_PXP_OUT_CTRL_FORMAT__ARGB1555: c_uint = 0x8;
pub const BV_PXP_OUT_CTRL_FORMAT__ARGB4444: c_uint = 0x9;
pub const BV_PXP_OUT_CTRL_FORMAT__RGB555: c_uint = 0xC;
pub const BV_PXP_OUT_CTRL_FORMAT__RGB444: c_uint = 0xD;
pub const BV_PXP_OUT_CTRL_FORMAT__RGB565: c_uint = 0xE;
pub const BV_PXP_OUT_CTRL_FORMAT__YUV1P444: c_uint = 0x10;
pub const BV_PXP_OUT_CTRL_FORMAT__UYVY1P422: c_uint = 0x12;
pub const BV_PXP_OUT_CTRL_FORMAT__VYUY1P422: c_uint = 0x13;
pub const BV_PXP_OUT_CTRL_FORMAT__Y8: c_uint = 0x14;
pub const BV_PXP_OUT_CTRL_FORMAT__Y4: c_uint = 0x15;
pub const BV_PXP_OUT_CTRL_FORMAT__YUV2P422: c_uint = 0x18;
pub const BV_PXP_OUT_CTRL_FORMAT__YUV2P420: c_uint = 0x19;
pub const BV_PXP_OUT_CTRL_FORMAT__YVU2P422: c_uint = 0x1A;
pub const BV_PXP_OUT_CTRL_FORMAT__YVU2P420: c_uint = 0x1B;

pub const BP_PXP_OUT_BUF_ADDR: c_int = 0;
pub const BM_PXP_OUT_BUF_ADDR: c_uint = 0xFFFFFFFF;

pub const BP_PXP_OUT_BUF2_ADDR: c_int = 0;
pub const BM_PXP_OUT_BUF2_ADDR: c_uint = 0xFFFFFFFF;

pub const BP_PXP_OUT_PITCH_RSVD: c_int = 16;
pub const BM_PXP_OUT_PITCH_RSVD: c_uint = 0xFFFF0000;

pub const BP_PXP_OUT_PITCH_PITCH: c_int = 0;
pub const BM_PXP_OUT_PITCH_PITCH: c_uint = 0x0000FFFF;

pub const BP_PXP_OUT_LRC_RSVD1: c_int = 30;
pub const BM_PXP_OUT_LRC_RSVD1: c_uint = 0xC0000000;

pub const BP_PXP_OUT_LRC_X: c_int = 16;
pub const BM_PXP_OUT_LRC_X: c_uint = 0x3FFF0000;

pub const BP_PXP_OUT_LRC_RSVD0: c_int = 14;
pub const BM_PXP_OUT_LRC_RSVD0: c_uint = 0x0000C000;

pub const BP_PXP_OUT_LRC_Y: c_int = 0;
pub const BM_PXP_OUT_LRC_Y: c_uint = 0x00003FFF;

pub const BP_PXP_OUT_PS_ULC_RSVD1: c_int = 30;
pub const BM_PXP_OUT_PS_ULC_RSVD1: c_uint = 0xC0000000;

pub const BP_PXP_OUT_PS_ULC_X: c_int = 16;
pub const BM_PXP_OUT_PS_ULC_X: c_uint = 0x3FFF0000;

pub const BP_PXP_OUT_PS_ULC_RSVD0: c_int = 14;
pub const BM_PXP_OUT_PS_ULC_RSVD0: c_uint = 0x0000C000;

pub const BP_PXP_OUT_PS_ULC_Y: c_int = 0;
pub const BM_PXP_OUT_PS_ULC_Y: c_uint = 0x00003FFF;

pub const BP_PXP_OUT_PS_LRC_RSVD1: c_int = 30;
pub const BM_PXP_OUT_PS_LRC_RSVD1: c_uint = 0xC0000000;

pub const BP_PXP_OUT_PS_LRC_X: c_int = 16;
pub const BM_PXP_OUT_PS_LRC_X: c_uint = 0x3FFF0000;

pub const BP_PXP_OUT_PS_LRC_RSVD0: c_int = 14;
pub const BM_PXP_OUT_PS_LRC_RSVD0: c_uint = 0x0000C000;

pub const BP_PXP_OUT_PS_LRC_Y: c_int = 0;
pub const BM_PXP_OUT_PS_LRC_Y: c_uint = 0x00003FFF;

pub const BP_PXP_OUT_AS_ULC_RSVD1: c_int = 30;
pub const BM_PXP_OUT_AS_ULC_RSVD1: c_uint = 0xC0000000;

pub const BP_PXP_OUT_AS_ULC_X: c_int = 16;
pub const BM_PXP_OUT_AS_ULC_X: c_uint = 0x3FFF0000;

pub const BP_PXP_OUT_AS_ULC_RSVD0: c_int = 14;
pub const BM_PXP_OUT_AS_ULC_RSVD0: c_uint = 0x0000C000;

pub const BP_PXP_OUT_AS_ULC_Y: c_int = 0;
pub const BM_PXP_OUT_AS_ULC_Y: c_uint = 0x00003FFF;

pub const BP_PXP_OUT_AS_LRC_RSVD1: c_int = 30;
pub const BM_PXP_OUT_AS_LRC_RSVD1: c_uint = 0xC0000000;

pub const BP_PXP_OUT_AS_LRC_X: c_int = 16;
pub const BM_PXP_OUT_AS_LRC_X: c_uint = 0x3FFF0000;

pub const BP_PXP_OUT_AS_LRC_RSVD0: c_int = 14;
pub const BM_PXP_OUT_AS_LRC_RSVD0: c_uint = 0x0000C000;

pub const BP_PXP_OUT_AS_LRC_Y: c_int = 0;
pub const BM_PXP_OUT_AS_LRC_Y: c_uint = 0x00003FFF;

pub const BP_PXP_PS_CTRL_RSVD1: c_int = 12;
pub const BM_PXP_PS_CTRL_RSVD1: c_uint = 0xFFFFF000;

pub const BP_PXP_PS_CTRL_DECX: c_int = 10;
pub const BM_PXP_PS_CTRL_DECX: c_uint = 0x00000C00;

pub const BV_PXP_PS_CTRL_DECX__DISABLE: c_uint = 0x0;
pub const BV_PXP_PS_CTRL_DECX__DECX2: c_uint = 0x1;
pub const BV_PXP_PS_CTRL_DECX__DECX4: c_uint = 0x2;
pub const BV_PXP_PS_CTRL_DECX__DECX8: c_uint = 0x3;
pub const BP_PXP_PS_CTRL_DECY: c_int = 8;
pub const BM_PXP_PS_CTRL_DECY: c_uint = 0x00000300;

pub const BV_PXP_PS_CTRL_DECY__DISABLE: c_uint = 0x0;
pub const BV_PXP_PS_CTRL_DECY__DECY2: c_uint = 0x1;
pub const BV_PXP_PS_CTRL_DECY__DECY4: c_uint = 0x2;
pub const BV_PXP_PS_CTRL_DECY__DECY8: c_uint = 0x3;
pub const BM_PXP_PS_CTRL_RSVD0: c_uint = 0x00000080;

pub const BM_PXP_PS_CTRL_WB_SWAP: c_uint = 0x00000040;

pub const BP_PXP_PS_CTRL_FORMAT: c_int = 0;
pub const BM_PXP_PS_CTRL_FORMAT: c_uint = 0x0000003F;

pub const BV_PXP_PS_CTRL_FORMAT__RGB888: c_uint = 0x4;
pub const BV_PXP_PS_CTRL_FORMAT__RGB555: c_uint = 0xC;
pub const BV_PXP_PS_CTRL_FORMAT__RGB444: c_uint = 0xD;
pub const BV_PXP_PS_CTRL_FORMAT__RGB565: c_uint = 0xE;
pub const BV_PXP_PS_CTRL_FORMAT__YUV1P444: c_uint = 0x10;
pub const BV_PXP_PS_CTRL_FORMAT__UYVY1P422: c_uint = 0x12;
pub const BV_PXP_PS_CTRL_FORMAT__VYUY1P422: c_uint = 0x13;
pub const BV_PXP_PS_CTRL_FORMAT__Y8: c_uint = 0x14;
pub const BV_PXP_PS_CTRL_FORMAT__Y4: c_uint = 0x15;
pub const BV_PXP_PS_CTRL_FORMAT__YUV2P422: c_uint = 0x18;
pub const BV_PXP_PS_CTRL_FORMAT__YUV2P420: c_uint = 0x19;
pub const BV_PXP_PS_CTRL_FORMAT__YVU2P422: c_uint = 0x1A;
pub const BV_PXP_PS_CTRL_FORMAT__YVU2P420: c_uint = 0x1B;
pub const BV_PXP_PS_CTRL_FORMAT__YUV422: c_uint = 0x1E;
pub const BV_PXP_PS_CTRL_FORMAT__YUV420: c_uint = 0x1F;

pub const BP_PXP_PS_BUF_ADDR: c_int = 0;
pub const BM_PXP_PS_BUF_ADDR: c_uint = 0xFFFFFFFF;

pub const BP_PXP_PS_UBUF_ADDR: c_int = 0;
pub const BM_PXP_PS_UBUF_ADDR: c_uint = 0xFFFFFFFF;

pub const BP_PXP_PS_VBUF_ADDR: c_int = 0;
pub const BM_PXP_PS_VBUF_ADDR: c_uint = 0xFFFFFFFF;

pub const BP_PXP_PS_PITCH_RSVD: c_int = 16;
pub const BM_PXP_PS_PITCH_RSVD: c_uint = 0xFFFF0000;

pub const BP_PXP_PS_PITCH_PITCH: c_int = 0;
pub const BM_PXP_PS_PITCH_PITCH: c_uint = 0x0000FFFF;

pub const BP_PXP_PS_BACKGROUND_0_RSVD: c_int = 24;
pub const BM_PXP_PS_BACKGROUND_0_RSVD: c_uint = 0xFF000000;

pub const BP_PXP_PS_BACKGROUND_0_COLOR: c_int = 0;
pub const BM_PXP_PS_BACKGROUND_0_COLOR: c_uint = 0x00FFFFFF;

pub const BM_PXP_PS_SCALE_RSVD2: c_uint = 0x80000000;

pub const BP_PXP_PS_SCALE_YSCALE: c_int = 16;
pub const BM_PXP_PS_SCALE_YSCALE: c_uint = 0x7FFF0000;

pub const BM_PXP_PS_SCALE_RSVD1: c_uint = 0x00008000;

pub const BP_PXP_PS_SCALE_XSCALE: c_int = 0;
pub const BM_PXP_PS_SCALE_XSCALE: c_uint = 0x00007FFF;

pub const BP_PXP_PS_OFFSET_RSVD2: c_int = 28;
pub const BM_PXP_PS_OFFSET_RSVD2: c_uint = 0xF0000000;

pub const BP_PXP_PS_OFFSET_YOFFSET: c_int = 16;
pub const BM_PXP_PS_OFFSET_YOFFSET: c_uint = 0x0FFF0000;

pub const BP_PXP_PS_OFFSET_RSVD1: c_int = 12;
pub const BM_PXP_PS_OFFSET_RSVD1: c_uint = 0x0000F000;

pub const BP_PXP_PS_OFFSET_XOFFSET: c_int = 0;
pub const BM_PXP_PS_OFFSET_XOFFSET: c_uint = 0x00000FFF;

pub const BP_PXP_PS_CLRKEYLOW_0_RSVD1: c_int = 24;
pub const BM_PXP_PS_CLRKEYLOW_0_RSVD1: c_uint = 0xFF000000;

pub const BP_PXP_PS_CLRKEYLOW_0_PIXEL: c_int = 0;
pub const BM_PXP_PS_CLRKEYLOW_0_PIXEL: c_uint = 0x00FFFFFF;

pub const BP_PXP_PS_CLRKEYHIGH_0_RSVD1: c_int = 24;
pub const BM_PXP_PS_CLRKEYHIGH_0_RSVD1: c_uint = 0xFF000000;

pub const BP_PXP_PS_CLRKEYHIGH_0_PIXEL: c_int = 0;
pub const BM_PXP_PS_CLRKEYHIGH_0_PIXEL: c_uint = 0x00FFFFFF;

pub const BP_PXP_AS_CTRL_RSVD1: c_int = 22;
pub const BM_PXP_AS_CTRL_RSVD1: c_uint = 0xFFC00000;

pub const BM_PXP_AS_CTRL_ALPHA1_INVERT: c_uint = 0x00200000;

pub const BM_PXP_AS_CTRL_ALPHA0_INVERT: c_uint = 0x00100000;

pub const BP_PXP_AS_CTRL_ROP: c_int = 16;
pub const BM_PXP_AS_CTRL_ROP: c_uint = 0x000F0000;

pub const BV_PXP_AS_CTRL_ROP__MASKAS: c_uint = 0x0;
pub const BV_PXP_AS_CTRL_ROP__MASKNOTAS: c_uint = 0x1;
pub const BV_PXP_AS_CTRL_ROP__MASKASNOT: c_uint = 0x2;
pub const BV_PXP_AS_CTRL_ROP__MERGEAS: c_uint = 0x3;
pub const BV_PXP_AS_CTRL_ROP__MERGENOTAS: c_uint = 0x4;
pub const BV_PXP_AS_CTRL_ROP__MERGEASNOT: c_uint = 0x5;
pub const BV_PXP_AS_CTRL_ROP__NOTCOPYAS: c_uint = 0x6;
pub const BV_PXP_AS_CTRL_ROP__NOT: c_uint = 0x7;
pub const BV_PXP_AS_CTRL_ROP__NOTMASKAS: c_uint = 0x8;
pub const BV_PXP_AS_CTRL_ROP__NOTMERGEAS: c_uint = 0x9;
pub const BV_PXP_AS_CTRL_ROP__XORAS: c_uint = 0xA;
pub const BV_PXP_AS_CTRL_ROP__NOTXORAS: c_uint = 0xB;
pub const BP_PXP_AS_CTRL_ALPHA: c_int = 8;
pub const BM_PXP_AS_CTRL_ALPHA: c_uint = 0x0000FF00;

pub const BP_PXP_AS_CTRL_FORMAT: c_int = 4;
pub const BM_PXP_AS_CTRL_FORMAT: c_uint = 0x000000F0;

pub const BV_PXP_AS_CTRL_FORMAT__ARGB8888: c_uint = 0x0;
pub const BV_PXP_AS_CTRL_FORMAT__RGBA8888: c_uint = 0x1;
pub const BV_PXP_AS_CTRL_FORMAT__RGB888: c_uint = 0x4;
pub const BV_PXP_AS_CTRL_FORMAT__ARGB1555: c_uint = 0x8;
pub const BV_PXP_AS_CTRL_FORMAT__ARGB4444: c_uint = 0x9;
pub const BV_PXP_AS_CTRL_FORMAT__RGB555: c_uint = 0xC;
pub const BV_PXP_AS_CTRL_FORMAT__RGB444: c_uint = 0xD;
pub const BV_PXP_AS_CTRL_FORMAT__RGB565: c_uint = 0xE;
pub const BM_PXP_AS_CTRL_ENABLE_COLORKEY: c_uint = 0x00000008;

pub const BP_PXP_AS_CTRL_ALPHA_CTRL: c_int = 1;
pub const BM_PXP_AS_CTRL_ALPHA_CTRL: c_uint = 0x00000006;

pub const BV_PXP_AS_CTRL_ALPHA_CTRL__Embedded: c_uint = 0x0;
pub const BV_PXP_AS_CTRL_ALPHA_CTRL__Override: c_uint = 0x1;
pub const BV_PXP_AS_CTRL_ALPHA_CTRL__Multiply: c_uint = 0x2;
pub const BV_PXP_AS_CTRL_ALPHA_CTRL__ROPs: c_uint = 0x3;
pub const BM_PXP_AS_CTRL_RSVD0: c_uint = 0x00000001;

pub const BP_PXP_AS_BUF_ADDR: c_int = 0;
pub const BM_PXP_AS_BUF_ADDR: c_uint = 0xFFFFFFFF;

pub const BP_PXP_AS_PITCH_RSVD: c_int = 16;
pub const BM_PXP_AS_PITCH_RSVD: c_uint = 0xFFFF0000;

pub const BP_PXP_AS_PITCH_PITCH: c_int = 0;
pub const BM_PXP_AS_PITCH_PITCH: c_uint = 0x0000FFFF;

pub const BP_PXP_AS_CLRKEYLOW_0_RSVD1: c_int = 24;
pub const BM_PXP_AS_CLRKEYLOW_0_RSVD1: c_uint = 0xFF000000;

pub const BP_PXP_AS_CLRKEYLOW_0_PIXEL: c_int = 0;
pub const BM_PXP_AS_CLRKEYLOW_0_PIXEL: c_uint = 0x00FFFFFF;

pub const BP_PXP_AS_CLRKEYHIGH_0_RSVD1: c_int = 24;
pub const BM_PXP_AS_CLRKEYHIGH_0_RSVD1: c_uint = 0xFF000000;

pub const BP_PXP_AS_CLRKEYHIGH_0_PIXEL: c_int = 0;
pub const BM_PXP_AS_CLRKEYHIGH_0_PIXEL: c_uint = 0x00FFFFFF;

pub const BM_PXP_CSC1_COEF0_YCBCR_MODE: c_uint = 0x80000000;

pub const BM_PXP_CSC1_COEF0_BYPASS: c_uint = 0x40000000;

pub const BM_PXP_CSC1_COEF0_RSVD1: c_uint = 0x20000000;

pub const BP_PXP_CSC1_COEF0_C0: c_int = 18;
pub const BM_PXP_CSC1_COEF0_C0: c_uint = 0x1FFC0000;

pub const BP_PXP_CSC1_COEF0_UV_OFFSET: c_int = 9;
pub const BM_PXP_CSC1_COEF0_UV_OFFSET: c_uint = 0x0003FE00;
//
// We use v * (1 << 9) instead of v << 9, to workaround a gcc5 bug.
// The compiler cannot understand that the expression is constant.
//

pub const BP_PXP_CSC1_COEF0_Y_OFFSET: c_int = 0;
pub const BM_PXP_CSC1_COEF0_Y_OFFSET: c_uint = 0x000001FF;

pub const BP_PXP_CSC1_COEF1_RSVD1: c_int = 27;
pub const BM_PXP_CSC1_COEF1_RSVD1: c_uint = 0xF8000000;

pub const BP_PXP_CSC1_COEF1_C1: c_int = 16;
pub const BM_PXP_CSC1_COEF1_C1: c_uint = 0x07FF0000;

pub const BP_PXP_CSC1_COEF1_RSVD0: c_int = 11;
pub const BM_PXP_CSC1_COEF1_RSVD0: c_uint = 0x0000F800;

pub const BP_PXP_CSC1_COEF1_C4: c_int = 0;
pub const BM_PXP_CSC1_COEF1_C4: c_uint = 0x000007FF;

pub const BP_PXP_CSC1_COEF2_RSVD1: c_int = 27;
pub const BM_PXP_CSC1_COEF2_RSVD1: c_uint = 0xF8000000;

pub const BP_PXP_CSC1_COEF2_C2: c_int = 16;
pub const BM_PXP_CSC1_COEF2_C2: c_uint = 0x07FF0000;

pub const BP_PXP_CSC1_COEF2_RSVD0: c_int = 11;
pub const BM_PXP_CSC1_COEF2_RSVD0: c_uint = 0x0000F800;

pub const BP_PXP_CSC1_COEF2_C3: c_int = 0;
pub const BM_PXP_CSC1_COEF2_C3: c_uint = 0x000007FF;

pub const BP_PXP_CSC2_CTRL_RSVD: c_int = 3;
pub const BM_PXP_CSC2_CTRL_RSVD: c_uint = 0xFFFFFFF8;

pub const BP_PXP_CSC2_CTRL_CSC_MODE: c_int = 1;
pub const BM_PXP_CSC2_CTRL_CSC_MODE: c_uint = 0x00000006;

pub const BV_PXP_CSC2_CTRL_CSC_MODE__YUV2RGB: c_uint = 0x0;
pub const BV_PXP_CSC2_CTRL_CSC_MODE__YCbCr2RGB: c_uint = 0x1;
pub const BV_PXP_CSC2_CTRL_CSC_MODE__RGB2YUV: c_uint = 0x2;
pub const BV_PXP_CSC2_CTRL_CSC_MODE__RGB2YCbCr: c_uint = 0x3;
pub const BM_PXP_CSC2_CTRL_BYPASS: c_uint = 0x00000001;

pub const BP_PXP_CSC2_COEF0_RSVD1: c_int = 27;
pub const BM_PXP_CSC2_COEF0_RSVD1: c_uint = 0xF8000000;

pub const BP_PXP_CSC2_COEF0_A2: c_int = 16;
pub const BM_PXP_CSC2_COEF0_A2: c_uint = 0x07FF0000;

pub const BP_PXP_CSC2_COEF0_RSVD0: c_int = 11;
pub const BM_PXP_CSC2_COEF0_RSVD0: c_uint = 0x0000F800;

pub const BP_PXP_CSC2_COEF0_A1: c_int = 0;
pub const BM_PXP_CSC2_COEF0_A1: c_uint = 0x000007FF;

pub const BP_PXP_CSC2_COEF1_RSVD1: c_int = 27;
pub const BM_PXP_CSC2_COEF1_RSVD1: c_uint = 0xF8000000;

pub const BP_PXP_CSC2_COEF1_B1: c_int = 16;
pub const BM_PXP_CSC2_COEF1_B1: c_uint = 0x07FF0000;

pub const BP_PXP_CSC2_COEF1_RSVD0: c_int = 11;
pub const BM_PXP_CSC2_COEF1_RSVD0: c_uint = 0x0000F800;

pub const BP_PXP_CSC2_COEF1_A3: c_int = 0;
pub const BM_PXP_CSC2_COEF1_A3: c_uint = 0x000007FF;

pub const BP_PXP_CSC2_COEF2_RSVD1: c_int = 27;
pub const BM_PXP_CSC2_COEF2_RSVD1: c_uint = 0xF8000000;

pub const BP_PXP_CSC2_COEF2_B3: c_int = 16;
pub const BM_PXP_CSC2_COEF2_B3: c_uint = 0x07FF0000;

pub const BP_PXP_CSC2_COEF2_RSVD0: c_int = 11;
pub const BM_PXP_CSC2_COEF2_RSVD0: c_uint = 0x0000F800;

pub const BP_PXP_CSC2_COEF2_B2: c_int = 0;
pub const BM_PXP_CSC2_COEF2_B2: c_uint = 0x000007FF;

pub const BP_PXP_CSC2_COEF3_RSVD1: c_int = 27;
pub const BM_PXP_CSC2_COEF3_RSVD1: c_uint = 0xF8000000;

pub const BP_PXP_CSC2_COEF3_C2: c_int = 16;
pub const BM_PXP_CSC2_COEF3_C2: c_uint = 0x07FF0000;

pub const BP_PXP_CSC2_COEF3_RSVD0: c_int = 11;
pub const BM_PXP_CSC2_COEF3_RSVD0: c_uint = 0x0000F800;

pub const BP_PXP_CSC2_COEF3_C1: c_int = 0;
pub const BM_PXP_CSC2_COEF3_C1: c_uint = 0x000007FF;

pub const BP_PXP_CSC2_COEF4_RSVD1: c_int = 25;
pub const BM_PXP_CSC2_COEF4_RSVD1: c_uint = 0xFE000000;

pub const BP_PXP_CSC2_COEF4_D1: c_int = 16;
pub const BM_PXP_CSC2_COEF4_D1: c_uint = 0x01FF0000;

pub const BP_PXP_CSC2_COEF4_RSVD0: c_int = 11;
pub const BM_PXP_CSC2_COEF4_RSVD0: c_uint = 0x0000F800;

pub const BP_PXP_CSC2_COEF4_C3: c_int = 0;
pub const BM_PXP_CSC2_COEF4_C3: c_uint = 0x000007FF;

pub const BP_PXP_CSC2_COEF5_RSVD1: c_int = 25;
pub const BM_PXP_CSC2_COEF5_RSVD1: c_uint = 0xFE000000;

pub const BP_PXP_CSC2_COEF5_D3: c_int = 16;
pub const BM_PXP_CSC2_COEF5_D3: c_uint = 0x01FF0000;

pub const BP_PXP_CSC2_COEF5_RSVD0: c_int = 9;
pub const BM_PXP_CSC2_COEF5_RSVD0: c_uint = 0x0000FE00;

pub const BP_PXP_CSC2_COEF5_D2: c_int = 0;
pub const BM_PXP_CSC2_COEF5_D2: c_uint = 0x000001FF;

pub const BM_PXP_LUT_CTRL_BYPASS: c_uint = 0x80000000;

pub const BP_PXP_LUT_CTRL_RSVD3: c_int = 26;
pub const BM_PXP_LUT_CTRL_RSVD3: c_uint = 0x7C000000;

pub const BP_PXP_LUT_CTRL_LOOKUP_MODE: c_int = 24;
pub const BM_PXP_LUT_CTRL_LOOKUP_MODE: c_uint = 0x03000000;

pub const BV_PXP_LUT_CTRL_LOOKUP_MODE__CACHE_RGB565: c_uint = 0x0;
pub const BV_PXP_LUT_CTRL_LOOKUP_MODE__DIRECT_Y8: c_uint = 0x1;
pub const BV_PXP_LUT_CTRL_LOOKUP_MODE__DIRECT_RGB444: c_uint = 0x2;
pub const BV_PXP_LUT_CTRL_LOOKUP_MODE__DIRECT_RGB454: c_uint = 0x3;
pub const BP_PXP_LUT_CTRL_RSVD2: c_int = 18;
pub const BM_PXP_LUT_CTRL_RSVD2: c_uint = 0x00FC0000;

pub const BP_PXP_LUT_CTRL_OUT_MODE: c_int = 16;
pub const BM_PXP_LUT_CTRL_OUT_MODE: c_uint = 0x00030000;

pub const BV_PXP_LUT_CTRL_OUT_MODE__RESERVED: c_uint = 0x0;
pub const BV_PXP_LUT_CTRL_OUT_MODE__Y8: c_uint = 0x1;
pub const BV_PXP_LUT_CTRL_OUT_MODE__RGBW4444CFA: c_uint = 0x2;
pub const BV_PXP_LUT_CTRL_OUT_MODE__RGB888: c_uint = 0x3;
pub const BP_PXP_LUT_CTRL_RSVD1: c_int = 11;
pub const BM_PXP_LUT_CTRL_RSVD1: c_uint = 0x0000F800;

pub const BM_PXP_LUT_CTRL_SEL_8KB: c_uint = 0x00000400;

pub const BM_PXP_LUT_CTRL_LRU_UPD: c_uint = 0x00000200;

pub const BM_PXP_LUT_CTRL_INVALID: c_uint = 0x00000100;

pub const BP_PXP_LUT_CTRL_RSVD0: c_int = 1;
pub const BM_PXP_LUT_CTRL_RSVD0: c_uint = 0x000000FE;

pub const BM_PXP_LUT_CTRL_DMA_START: c_uint = 0x00000001;

pub const BM_PXP_LUT_ADDR_RSVD2: c_uint = 0x80000000;

pub const BP_PXP_LUT_ADDR_NUM_BYTES: c_int = 16;
pub const BM_PXP_LUT_ADDR_NUM_BYTES: c_uint = 0x7FFF0000;

pub const BP_PXP_LUT_ADDR_RSVD1: c_int = 14;
pub const BM_PXP_LUT_ADDR_RSVD1: c_uint = 0x0000C000;

pub const BP_PXP_LUT_ADDR_ADDR: c_int = 0;
pub const BM_PXP_LUT_ADDR_ADDR: c_uint = 0x00003FFF;

pub const BP_PXP_LUT_DATA_DATA: c_int = 0;
pub const BM_PXP_LUT_DATA_DATA: c_uint = 0xFFFFFFFF;

pub const BP_PXP_LUT_EXTMEM_ADDR: c_int = 0;
pub const BM_PXP_LUT_EXTMEM_ADDR: c_uint = 0xFFFFFFFF;

pub const BP_PXP_CFA_DATA: c_int = 0;
pub const BM_PXP_CFA_DATA: c_uint = 0xFFFFFFFF;

pub const BP_PXP_ALPHA_A_CTRL_S1_GLOBAL_ALPHA: c_int = 24;
pub const BM_PXP_ALPHA_A_CTRL_S1_GLOBAL_ALPHA: c_uint = 0xFF000000;

pub const BP_PXP_ALPHA_A_CTRL_S0_GLOBAL_ALPHA: c_int = 16;
pub const BM_PXP_ALPHA_A_CTRL_S0_GLOBAL_ALPHA: c_uint = 0x00FF0000;

pub const BP_PXP_ALPHA_A_CTRL_RSVD0: c_int = 14;
pub const BM_PXP_ALPHA_A_CTRL_RSVD0: c_uint = 0x0000C000;

pub const BM_PXP_ALPHA_A_CTRL_S1_COLOR_MODE: c_uint = 0x00002000;

pub const BV_PXP_ALPHA_A_CTRL_S1_COLOR_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_S1_COLOR_MODE__1: c_uint = 0x1;
pub const BM_PXP_ALPHA_A_CTRL_S1_ALPHA_MODE: c_uint = 0x00001000;

pub const BV_PXP_ALPHA_A_CTRL_S1_ALPHA_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_S1_ALPHA_MODE__1: c_uint = 0x1;
pub const BP_PXP_ALPHA_A_CTRL_S1_GLOBAL_ALPHA_MODE: c_int = 10;
pub const BM_PXP_ALPHA_A_CTRL_S1_GLOBAL_ALPHA_MODE: c_uint = 0x00000C00;

pub const BV_PXP_ALPHA_A_CTRL_S1_GLOBAL_ALPHA_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_S1_GLOBAL_ALPHA_MODE__1: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_S1_GLOBAL_ALPHA_MODE__2: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_S1_GLOBAL_ALPHA_MODE__3: c_uint = 0x0;
pub const BP_PXP_ALPHA_A_CTRL_S1_S0_FACTOR_MODE: c_int = 8;
pub const BM_PXP_ALPHA_A_CTRL_S1_S0_FACTOR_MODE: c_uint = 0x00000300;

pub const BV_PXP_ALPHA_A_CTRL_S1_S0_FACTOR_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_S1_S0_FACTOR_MODE__1: c_uint = 0x1;
pub const BV_PXP_ALPHA_A_CTRL_S1_S0_FACTOR_MODE__2: c_uint = 0x2;
pub const BV_PXP_ALPHA_A_CTRL_S1_S0_FACTOR_MODE__3: c_uint = 0x3;
pub const BM_PXP_ALPHA_A_CTRL_RSVD1: c_uint = 0x00000080;

pub const BM_PXP_ALPHA_A_CTRL_S0_COLOR_MODE: c_uint = 0x00000040;

pub const BV_PXP_ALPHA_A_CTRL_S0_COLOR_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_S0_COLOR_MODE__1: c_uint = 0x1;
pub const BM_PXP_ALPHA_A_CTRL_S0_ALPHA_MODE: c_uint = 0x00000020;

pub const BV_PXP_ALPHA_A_CTRL_S0_ALPHA_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_S0_ALPHA_MODE__1: c_uint = 0x1;
pub const BP_PXP_ALPHA_A_CTRL_S0_GLOBAL_ALPHA_MODE: c_int = 3;
pub const BM_PXP_ALPHA_A_CTRL_S0_GLOBAL_ALPHA_MODE: c_uint = 0x00000018;

pub const BV_PXP_ALPHA_A_CTRL_S0_GLOBAL_ALPHA_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_S0_GLOBAL_ALPHA_MODE__1: c_uint = 0x1;
pub const BV_PXP_ALPHA_A_CTRL_S0_GLOBAL_ALPHA_MODE__2: c_uint = 0x2;
pub const BV_PXP_ALPHA_A_CTRL_S0_GLOBAL_ALPHA_MODE__3: c_uint = 0x3;
pub const BP_PXP_ALPHA_A_CTRL_S0_S1_FACTOR_MODE: c_int = 1;
pub const BM_PXP_ALPHA_A_CTRL_S0_S1_FACTOR_MODE: c_uint = 0x00000006;

pub const BV_PXP_ALPHA_A_CTRL_S0_S1_FACTOR_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_S0_S1_FACTOR_MODE__1: c_uint = 0x1;
pub const BV_PXP_ALPHA_A_CTRL_S0_S1_FACTOR_MODE__2: c_uint = 0x2;
pub const BV_PXP_ALPHA_A_CTRL_S0_S1_FACTOR_MODE__3: c_uint = 0x3;
pub const BM_PXP_ALPHA_A_CTRL_POTER_DUFF_ENABLE: c_uint = 0x00000001;

pub const BV_PXP_ALPHA_A_CTRL_POTER_DUFF_ENABLE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_A_CTRL_POTER_DUFF_ENABLE__1: c_uint = 0x1;

pub const BP_PXP_ALPHA_B_CTRL_S1_GLOBAL_ALPHA: c_int = 24;
pub const BM_PXP_ALPHA_B_CTRL_S1_GLOBAL_ALPHA: c_uint = 0xFF000000;

pub const BP_PXP_ALPHA_B_CTRL_S0_GLOBAL_ALPHA: c_int = 16;
pub const BM_PXP_ALPHA_B_CTRL_S0_GLOBAL_ALPHA: c_uint = 0x00FF0000;

pub const BP_PXP_ALPHA_B_CTRL_RSVD0: c_int = 14;
pub const BM_PXP_ALPHA_B_CTRL_RSVD0: c_uint = 0x0000C000;

pub const BM_PXP_ALPHA_B_CTRL_S1_COLOR_MODE: c_uint = 0x00002000;

pub const BV_PXP_ALPHA_B_CTRL_S1_COLOR_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_B_CTRL_S1_COLOR_MODE__1: c_uint = 0x1;
pub const BM_PXP_ALPHA_B_CTRL_S1_ALPHA_MODE: c_uint = 0x00001000;

pub const BV_PXP_ALPHA_B_CTRL_S1_ALPHA_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_B_CTRL_S1_ALPHA_MODE__1: c_uint = 0x1;
pub const BP_PXP_ALPHA_B_CTRL_S1_GLOBAL_ALPHA_MODE: c_int = 10;
pub const BM_PXP_ALPHA_B_CTRL_S1_GLOBAL_ALPHA_MODE: c_uint = 0x00000C00;

pub const BV_PXP_ALPHA_B_CTRL_S1_GLOBAL_ALPHA_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_B_CTRL_S1_GLOBAL_ALPHA_MODE__1: c_uint = 0x1;
pub const BV_PXP_ALPHA_B_CTRL_S1_GLOBAL_ALPHA_MODE__2: c_uint = 0x2;
pub const BV_PXP_ALPHA_B_CTRL_S1_GLOBAL_ALPHA_MODE__3: c_uint = 0x3;
pub const BP_PXP_ALPHA_B_CTRL_S1_S0_FACTOR_MODE: c_int = 8;
pub const BM_PXP_ALPHA_B_CTRL_S1_S0_FACTOR_MODE: c_uint = 0x00000300;

pub const BV_PXP_ALPHA_B_CTRL_S1_S0_FACTOR_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_B_CTRL_S1_S0_FACTOR_MODE__1: c_uint = 0x1;
pub const BV_PXP_ALPHA_B_CTRL_S1_S0_FACTOR_MODE__2: c_uint = 0x2;
pub const BV_PXP_ALPHA_B_CTRL_S1_S0_FACTOR_MODE__3: c_uint = 0x3;
pub const BM_PXP_ALPHA_B_CTRL_RSVD1: c_uint = 0x00000080;

pub const BM_PXP_ALPHA_B_CTRL_S0_COLOR_MODE: c_uint = 0x00000040;

pub const BV_PXP_ALPHA_B_CTRL_S0_COLOR_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_B_CTRL_S0_COLOR_MODE__1: c_uint = 0x1;
pub const BM_PXP_ALPHA_B_CTRL_S0_ALPHA_MODE: c_uint = 0x00000020;

pub const BV_PXP_ALPHA_B_CTRL_S0_ALPHA_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_B_CTRL_S0_ALPHA_MODE__1: c_uint = 0x1;
pub const BP_PXP_ALPHA_B_CTRL_S0_GLOBAL_ALPHA_MODE: c_int = 3;
pub const BM_PXP_ALPHA_B_CTRL_S0_GLOBAL_ALPHA_MODE: c_uint = 0x00000018;

pub const BV_PXP_ALPHA_B_CTRL_S0_GLOBAL_ALPHA_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_B_CTRL_S0_GLOBAL_ALPHA_MODE__1: c_uint = 0x1;
pub const BV_PXP_ALPHA_B_CTRL_S0_GLOBAL_ALPHA_MODE__2: c_uint = 0x2;
pub const BV_PXP_ALPHA_B_CTRL_S0_GLOBAL_ALPHA_MODE__3: c_uint = 0x3;
pub const BP_PXP_ALPHA_B_CTRL_S0_S1_FACTOR_MODE: c_int = 1;
pub const BM_PXP_ALPHA_B_CTRL_S0_S1_FACTOR_MODE: c_uint = 0x00000006;

pub const BV_PXP_ALPHA_B_CTRL_S0_S1_FACTOR_MODE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_B_CTRL_S0_S1_FACTOR_MODE__1: c_uint = 0x1;
pub const BV_PXP_ALPHA_B_CTRL_S0_S1_FACTOR_MODE__2: c_uint = 0x2;
pub const BV_PXP_ALPHA_B_CTRL_S0_S1_FACTOR_MODE__3: c_uint = 0x3;
pub const BM_PXP_ALPHA_B_CTRL_POTER_DUFF_ENABLE: c_uint = 0x00000001;

pub const BV_PXP_ALPHA_B_CTRL_POTER_DUFF_ENABLE__0: c_uint = 0x0;
pub const BV_PXP_ALPHA_B_CTRL_POTER_DUFF_ENABLE__1: c_uint = 0x1;

pub const BP_PXP_ALPHA_B_CTRL_1_RSVD0: c_int = 8;
pub const BM_PXP_ALPHA_B_CTRL_1_RSVD0: c_uint = 0xFFFFFF00;

pub const BP_PXP_ALPHA_B_CTRL_1_ROP: c_int = 4;
pub const BM_PXP_ALPHA_B_CTRL_1_ROP: c_uint = 0x000000F0;

pub const BV_PXP_ALPHA_B_CTRL_1_ROP__MASKAS: c_uint = 0x0;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__MASKNOTAS: c_uint = 0x1;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__MASKASNOT: c_uint = 0x2;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__MERGEAS: c_uint = 0x3;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__MERGENOTAS: c_uint = 0x4;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__MERGEASNOT: c_uint = 0x5;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__NOTCOPYAS: c_uint = 0x6;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__NOT: c_uint = 0x7;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__NOTMASKAS: c_uint = 0x8;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__NOTMERGEAS: c_uint = 0x9;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__XORAS: c_uint = 0xA;
pub const BV_PXP_ALPHA_B_CTRL_1_ROP__NOTXORAS: c_uint = 0xB;
pub const BP_PXP_ALPHA_B_CTRL_1_RSVD1: c_int = 2;
pub const BM_PXP_ALPHA_B_CTRL_1_RSVD1: c_uint = 0x0000000C;

pub const BM_PXP_ALPHA_B_CTRL_1_OL_CLRKEY_ENABLE: c_uint = 0x00000002;

pub const BM_PXP_ALPHA_B_CTRL_1_ROP_ENABLE: c_uint = 0x00000001;

pub const BP_PXP_PS_BACKGROUND_1_RSVD: c_int = 24;
pub const BM_PXP_PS_BACKGROUND_1_RSVD: c_uint = 0xFF000000;

pub const BP_PXP_PS_BACKGROUND_1_COLOR: c_int = 0;
pub const BM_PXP_PS_BACKGROUND_1_COLOR: c_uint = 0x00FFFFFF;

pub const BP_PXP_PS_CLRKEYLOW_1_RSVD1: c_int = 24;
pub const BM_PXP_PS_CLRKEYLOW_1_RSVD1: c_uint = 0xFF000000;

pub const BP_PXP_PS_CLRKEYLOW_1_PIXEL: c_int = 0;
pub const BM_PXP_PS_CLRKEYLOW_1_PIXEL: c_uint = 0x00FFFFFF;

pub const BP_PXP_PS_CLRKEYHIGH_1_RSVD1: c_int = 24;
pub const BM_PXP_PS_CLRKEYHIGH_1_RSVD1: c_uint = 0xFF000000;

pub const BP_PXP_PS_CLRKEYHIGH_1_PIXEL: c_int = 0;
pub const BM_PXP_PS_CLRKEYHIGH_1_PIXEL: c_uint = 0x00FFFFFF;

pub const BP_PXP_AS_CLRKEYLOW_1_RSVD1: c_int = 24;
pub const BM_PXP_AS_CLRKEYLOW_1_RSVD1: c_uint = 0xFF000000;

pub const BP_PXP_AS_CLRKEYLOW_1_PIXEL: c_int = 0;
pub const BM_PXP_AS_CLRKEYLOW_1_PIXEL: c_uint = 0x00FFFFFF;

pub const BP_PXP_AS_CLRKEYHIGH_1_RSVD1: c_int = 24;
pub const BM_PXP_AS_CLRKEYHIGH_1_RSVD1: c_uint = 0xFF000000;

pub const BP_PXP_AS_CLRKEYHIGH_1_PIXEL: c_int = 0;
pub const BM_PXP_AS_CLRKEYHIGH_1_PIXEL: c_uint = 0x00FFFFFF;

pub const BP_PXP_CTRL2_RSVD3: c_int = 28;
pub const BM_PXP_CTRL2_RSVD3: c_uint = 0xF0000000;

pub const BM_PXP_CTRL2_ENABLE_ROTATE1: c_uint = 0x08000000;

pub const BM_PXP_CTRL2_ENABLE_ROTATE0: c_uint = 0x04000000;

pub const BM_PXP_CTRL2_ENABLE_LUT: c_uint = 0x02000000;

pub const BM_PXP_CTRL2_ENABLE_CSC2: c_uint = 0x01000000;

pub const BM_PXP_CTRL2_BLOCK_SIZE: c_uint = 0x00800000;

pub const BV_PXP_CTRL2_BLOCK_SIZE__8X8: c_uint = 0x0;
pub const BV_PXP_CTRL2_BLOCK_SIZE__16X16: c_uint = 0x1;
pub const BM_PXP_CTRL2_RSVD2: c_uint = 0x00400000;

pub const BM_PXP_CTRL2_ENABLE_ALPHA_B: c_uint = 0x00200000;

pub const BM_PXP_CTRL2_ENABLE_INPUT_FETCH_STORE: c_uint = 0x00100000;

pub const BM_PXP_CTRL2_ENABLE_WFE_B: c_uint = 0x00080000;

pub const BM_PXP_CTRL2_ENABLE_WFE_A: c_uint = 0x00040000;

pub const BM_PXP_CTRL2_ENABLE_DITHER: c_uint = 0x00020000;

pub const BM_PXP_CTRL2_RSVD1: c_uint = 0x00010000;

pub const BM_PXP_CTRL2_VFLIP1: c_uint = 0x00008000;

pub const BM_PXP_CTRL2_HFLIP1: c_uint = 0x00004000;

pub const BP_PXP_CTRL2_ROTATE1: c_int = 12;
pub const BM_PXP_CTRL2_ROTATE1: c_uint = 0x00003000;

pub const BV_PXP_CTRL2_ROTATE1__ROT_0: c_uint = 0x0;
pub const BV_PXP_CTRL2_ROTATE1__ROT_90: c_uint = 0x1;
pub const BV_PXP_CTRL2_ROTATE1__ROT_180: c_uint = 0x2;
pub const BV_PXP_CTRL2_ROTATE1__ROT_270: c_uint = 0x3;
pub const BM_PXP_CTRL2_VFLIP0: c_uint = 0x00000800;

pub const BM_PXP_CTRL2_HFLIP0: c_uint = 0x00000400;

pub const BP_PXP_CTRL2_ROTATE0: c_int = 8;
pub const BM_PXP_CTRL2_ROTATE0: c_uint = 0x00000300;

pub const BV_PXP_CTRL2_ROTATE0__ROT_0: c_uint = 0x0;
pub const BV_PXP_CTRL2_ROTATE0__ROT_90: c_uint = 0x1;
pub const BV_PXP_CTRL2_ROTATE0__ROT_180: c_uint = 0x2;
pub const BV_PXP_CTRL2_ROTATE0__ROT_270: c_uint = 0x3;
pub const BP_PXP_CTRL2_RSVD0: c_int = 1;
pub const BM_PXP_CTRL2_RSVD0: c_uint = 0x000000FE;

pub const BM_PXP_CTRL2_ENABLE: c_uint = 0x00000001;

pub const BP_PXP_POWER_REG0_CTRL: c_int = 12;
pub const BM_PXP_POWER_REG0_CTRL: c_uint = 0xFFFFF000;

pub const BP_PXP_POWER_REG0_ROT0_MEM_LP_STATE: c_int = 9;
pub const BM_PXP_POWER_REG0_ROT0_MEM_LP_STATE: c_uint = 0x00000E00;

pub const BV_PXP_POWER_REG0_ROT0_MEM_LP_STATE__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG0_ROT0_MEM_LP_STATE__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG0_ROT0_MEM_LP_STATE__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG0_ROT0_MEM_LP_STATE__SD: c_uint = 0x4;
pub const BP_PXP_POWER_REG0_LUT_LP_STATE_WAY1_BANKN: c_int = 6;
pub const BM_PXP_POWER_REG0_LUT_LP_STATE_WAY1_BANKN: c_uint = 0x000001C0;

pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY1_BANKN__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY1_BANKN__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY1_BANKN__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY1_BANKN__SD: c_uint = 0x4;
pub const BP_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANKN: c_int = 3;
pub const BM_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANKN: c_uint = 0x00000038;

pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANKN__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANKN__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANKN__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANKN__SD: c_uint = 0x4;
pub const BP_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANK0: c_int = 0;
pub const BM_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANK0: c_uint = 0x00000007;

pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANK0__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANK0__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANK0__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG0_LUT_LP_STATE_WAY0_BANK0__SD: c_uint = 0x4;

pub const BP_PXP_POWER_REG1_RSVD0: c_int = 24;
pub const BM_PXP_POWER_REG1_RSVD0: c_uint = 0xFF000000;

pub const BP_PXP_POWER_REG1_ALU_B_MEM_LP_STATE: c_int = 21;
pub const BM_PXP_POWER_REG1_ALU_B_MEM_LP_STATE: c_uint = 0x00E00000;

pub const BV_PXP_POWER_REG1_ALU_B_MEM_LP_STATE__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG1_ALU_B_MEM_LP_STATE__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG1_ALU_B_MEM_LP_STATE__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG1_ALU_B_MEM_LP_STATE__SD: c_uint = 0x4;
pub const BP_PXP_POWER_REG1_ALU_A_MEM_LP_STATE: c_int = 18;
pub const BM_PXP_POWER_REG1_ALU_A_MEM_LP_STATE: c_uint = 0x001C0000;

pub const BV_PXP_POWER_REG1_ALU_A_MEM_LP_STATE__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG1_ALU_A_MEM_LP_STATE__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG1_ALU_A_MEM_LP_STATE__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG1_ALU_A_MEM_LP_STATE__SD: c_uint = 0x4;
pub const BP_PXP_POWER_REG1_DITH2_LUT_MEM_LP_STATE: c_int = 15;
pub const BM_PXP_POWER_REG1_DITH2_LUT_MEM_LP_STATE: c_uint = 0x00038000;

pub const BV_PXP_POWER_REG1_DITH2_LUT_MEM_LP_STATE__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG1_DITH2_LUT_MEM_LP_STATE__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG1_DITH2_LUT_MEM_LP_STATE__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG1_DITH2_LUT_MEM_LP_STATE__SD: c_uint = 0x4;
pub const BP_PXP_POWER_REG1_DITH1_LUT_MEM_LP_STATE: c_int = 12;
pub const BM_PXP_POWER_REG1_DITH1_LUT_MEM_LP_STATE: c_uint = 0x00007000;

pub const BV_PXP_POWER_REG1_DITH1_LUT_MEM_LP_STATE__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG1_DITH1_LUT_MEM_LP_STATE__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG1_DITH1_LUT_MEM_LP_STATE__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG1_DITH1_LUT_MEM_LP_STATE__SD: c_uint = 0x4;
pub const BP_PXP_POWER_REG1_DITH0_ERR1_MEM_LP_STATE: c_int = 9;
pub const BM_PXP_POWER_REG1_DITH0_ERR1_MEM_LP_STATE: c_uint = 0x00000E00;

pub const BV_PXP_POWER_REG1_DITH0_ERR1_MEM_LP_STATE__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG1_DITH0_ERR1_MEM_LP_STATE__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG1_DITH0_ERR1_MEM_LP_STATE__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG1_DITH0_ERR1_MEM_LP_STATE__SD: c_uint = 0x4;
pub const BP_PXP_POWER_REG1_DITH0_ERR0_MEM_LP_STATE: c_int = 6;
pub const BM_PXP_POWER_REG1_DITH0_ERR0_MEM_LP_STATE: c_uint = 0x000001C0;

pub const BV_PXP_POWER_REG1_DITH0_ERR0_MEM_LP_STATE__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG1_DITH0_ERR0_MEM_LP_STATE__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG1_DITH0_ERR0_MEM_LP_STATE__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG1_DITH0_ERR0_MEM_LP_STATE__SD: c_uint = 0x4;
pub const BP_PXP_POWER_REG1_DITH0_LUT_MEM_LP_STATE: c_int = 3;
pub const BM_PXP_POWER_REG1_DITH0_LUT_MEM_LP_STATE: c_uint = 0x00000038;

pub const BV_PXP_POWER_REG1_DITH0_LUT_MEM_LP_STATE__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG1_DITH0_LUT_MEM_LP_STATE__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG1_DITH0_LUT_MEM_LP_STATE__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG1_DITH0_LUT_MEM_LP_STATE__SD: c_uint = 0x4;
pub const BP_PXP_POWER_REG1_ROT1_MEM_LP_STATE: c_int = 0;
pub const BM_PXP_POWER_REG1_ROT1_MEM_LP_STATE: c_uint = 0x00000007;

pub const BV_PXP_POWER_REG1_ROT1_MEM_LP_STATE__NONE: c_uint = 0x0;
pub const BV_PXP_POWER_REG1_ROT1_MEM_LP_STATE__LS: c_uint = 0x1;
pub const BV_PXP_POWER_REG1_ROT1_MEM_LP_STATE__DS: c_uint = 0x2;
pub const BV_PXP_POWER_REG1_ROT1_MEM_LP_STATE__SD: c_uint = 0x4;

pub const BP_PXP_DATA_PATH_CTRL0_MUX15_SEL: c_int = 30;
pub const BM_PXP_DATA_PATH_CTRL0_MUX15_SEL: c_uint = 0xC0000000;

pub const BV_PXP_DATA_PATH_CTRL0_MUX15_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX15_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX15_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX15_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX14_SEL: c_int = 28;
pub const BM_PXP_DATA_PATH_CTRL0_MUX14_SEL: c_uint = 0x30000000;

pub const BV_PXP_DATA_PATH_CTRL0_MUX14_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX14_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX14_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX14_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX13_SEL: c_int = 26;
pub const BM_PXP_DATA_PATH_CTRL0_MUX13_SEL: c_uint = 0x0C000000;

pub const BV_PXP_DATA_PATH_CTRL0_MUX13_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX13_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX13_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX13_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX12_SEL: c_int = 24;
pub const BM_PXP_DATA_PATH_CTRL0_MUX12_SEL: c_uint = 0x03000000;

pub const BV_PXP_DATA_PATH_CTRL0_MUX12_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX12_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX12_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX12_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX11_SEL: c_int = 22;
pub const BM_PXP_DATA_PATH_CTRL0_MUX11_SEL: c_uint = 0x00C00000;

pub const BV_PXP_DATA_PATH_CTRL0_MUX11_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX11_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX11_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX11_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX10_SEL: c_int = 20;
pub const BM_PXP_DATA_PATH_CTRL0_MUX10_SEL: c_uint = 0x00300000;

pub const BV_PXP_DATA_PATH_CTRL0_MUX10_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX10_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX10_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX10_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX9_SEL: c_int = 18;
pub const BM_PXP_DATA_PATH_CTRL0_MUX9_SEL: c_uint = 0x000C0000;

pub const BV_PXP_DATA_PATH_CTRL0_MUX9_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX9_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX9_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX9_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX8_SEL: c_int = 16;
pub const BM_PXP_DATA_PATH_CTRL0_MUX8_SEL: c_uint = 0x00030000;

pub const BV_PXP_DATA_PATH_CTRL0_MUX8_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX8_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX8_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX8_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX7_SEL: c_int = 14;
pub const BM_PXP_DATA_PATH_CTRL0_MUX7_SEL: c_uint = 0x0000C000;

pub const BV_PXP_DATA_PATH_CTRL0_MUX7_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX7_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX7_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX7_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX6_SEL: c_int = 12;
pub const BM_PXP_DATA_PATH_CTRL0_MUX6_SEL: c_uint = 0x00003000;

pub const BV_PXP_DATA_PATH_CTRL0_MUX6_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX6_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX6_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX6_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX5_SEL: c_int = 10;
pub const BM_PXP_DATA_PATH_CTRL0_MUX5_SEL: c_uint = 0x00000C00;

pub const BV_PXP_DATA_PATH_CTRL0_MUX5_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX5_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX5_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX5_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX4_SEL: c_int = 8;
pub const BM_PXP_DATA_PATH_CTRL0_MUX4_SEL: c_uint = 0x00000300;

pub const BV_PXP_DATA_PATH_CTRL0_MUX4_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX4_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX4_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX4_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX3_SEL: c_int = 6;
pub const BM_PXP_DATA_PATH_CTRL0_MUX3_SEL: c_uint = 0x000000C0;

pub const BV_PXP_DATA_PATH_CTRL0_MUX3_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX3_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX3_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX3_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX2_SEL: c_int = 4;
pub const BM_PXP_DATA_PATH_CTRL0_MUX2_SEL: c_uint = 0x00000030;

pub const BV_PXP_DATA_PATH_CTRL0_MUX2_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX2_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX2_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX2_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX1_SEL: c_int = 2;
pub const BM_PXP_DATA_PATH_CTRL0_MUX1_SEL: c_uint = 0x0000000C;

pub const BV_PXP_DATA_PATH_CTRL0_MUX1_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX1_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX1_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX1_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL0_MUX0_SEL: c_int = 0;
pub const BM_PXP_DATA_PATH_CTRL0_MUX0_SEL: c_uint = 0x00000003;

pub const BV_PXP_DATA_PATH_CTRL0_MUX0_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL0_MUX0_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL0_MUX0_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL0_MUX0_SEL__3: c_uint = 0x3;

pub const BP_PXP_DATA_PATH_CTRL1_RSVD0: c_int = 4;
pub const BM_PXP_DATA_PATH_CTRL1_RSVD0: c_uint = 0xFFFFFFF0;

pub const BP_PXP_DATA_PATH_CTRL1_MUX17_SEL: c_int = 2;
pub const BM_PXP_DATA_PATH_CTRL1_MUX17_SEL: c_uint = 0x0000000C;

pub const BV_PXP_DATA_PATH_CTRL1_MUX17_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL1_MUX17_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL1_MUX17_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL1_MUX17_SEL__3: c_uint = 0x3;
pub const BP_PXP_DATA_PATH_CTRL1_MUX16_SEL: c_int = 0;
pub const BM_PXP_DATA_PATH_CTRL1_MUX16_SEL: c_uint = 0x00000003;

pub const BV_PXP_DATA_PATH_CTRL1_MUX16_SEL__0: c_uint = 0x0;
pub const BV_PXP_DATA_PATH_CTRL1_MUX16_SEL__1: c_uint = 0x1;
pub const BV_PXP_DATA_PATH_CTRL1_MUX16_SEL__2: c_uint = 0x2;
pub const BV_PXP_DATA_PATH_CTRL1_MUX16_SEL__3: c_uint = 0x3;

pub const BM_PXP_INIT_MEM_CTRL_START: c_uint = 0x80000000;

pub const BP_PXP_INIT_MEM_CTRL_SELECT: c_int = 27;
pub const BM_PXP_INIT_MEM_CTRL_SELECT: c_uint = 0x78000000;

pub const BV_PXP_INIT_MEM_CTRL_SELECT__DITHER0_LUT: c_uint = 0x0;
pub const BV_PXP_INIT_MEM_CTRL_SELECT__DITHER0_ERR0: c_uint = 0x1;
pub const BV_PXP_INIT_MEM_CTRL_SELECT__DITHER0_ERR1: c_uint = 0x2;
pub const BV_PXP_INIT_MEM_CTRL_SELECT__DITHER1_LUT: c_uint = 0x3;
pub const BV_PXP_INIT_MEM_CTRL_SELECT__DITHER2_LUT: c_uint = 0x4;
pub const BV_PXP_INIT_MEM_CTRL_SELECT__ALU_A: c_uint = 0x5;
pub const BV_PXP_INIT_MEM_CTRL_SELECT__ALU_B: c_uint = 0x6;
pub const BV_PXP_INIT_MEM_CTRL_SELECT__WFE_A_FETCH: c_uint = 0x7;
pub const BV_PXP_INIT_MEM_CTRL_SELECT__WFE_B_FETCH: c_uint = 0x8;
pub const BV_PXP_INIT_MEM_CTRL_SELECT__RESERVED: c_uint = 0x15;
pub const BP_PXP_INIT_MEM_CTRL_RSVD0: c_int = 16;
pub const BM_PXP_INIT_MEM_CTRL_RSVD0: c_uint = 0x07FF0000;

pub const BP_PXP_INIT_MEM_CTRL_ADDR: c_int = 0;
pub const BM_PXP_INIT_MEM_CTRL_ADDR: c_uint = 0x0000FFFF;

pub const BP_PXP_INIT_MEM_DATA_DATA: c_int = 0;
pub const BM_PXP_INIT_MEM_DATA_DATA: c_uint = 0xFFFFFFFF;

pub const BP_PXP_INIT_MEM_DATA_HIGH_DATA: c_int = 0;
pub const BM_PXP_INIT_MEM_DATA_HIGH_DATA: c_uint = 0xFFFFFFFF;

pub const BM_PXP_IRQ_MASK_COMPRESS_DONE_IRQ_EN: c_uint = 0x80000000;

pub const BP_PXP_IRQ_MASK_RSVD1: c_int = 16;
pub const BM_PXP_IRQ_MASK_RSVD1: c_uint = 0x7FFF0000;

pub const BM_PXP_IRQ_MASK_WFE_B_STORE_IRQ_EN: c_uint = 0x00008000;

pub const BM_PXP_IRQ_MASK_WFE_A_STORE_IRQ_EN: c_uint = 0x00004000;

pub const BM_PXP_IRQ_MASK_DITHER_STORE_IRQ_EN: c_uint = 0x00002000;

pub const BM_PXP_IRQ_MASK_FIRST_STORE_IRQ_EN: c_uint = 0x00001000;

pub const BM_PXP_IRQ_MASK_WFE_B_CH1_STORE_IRQ_EN: c_uint = 0x00000800;

pub const BM_PXP_IRQ_MASK_WFE_B_CH0_STORE_IRQ_EN: c_uint = 0x00000400;

pub const BM_PXP_IRQ_MASK_WFE_A_CH1_STORE_IRQ_EN: c_uint = 0x00000200;

pub const BM_PXP_IRQ_MASK_WFE_A_CH0_STORE_IRQ_EN: c_uint = 0x00000100;

pub const BM_PXP_IRQ_MASK_DITHER_CH1_STORE_IRQ_EN: c_uint = 0x00000080;

pub const BM_PXP_IRQ_MASK_DITHER_CH0_STORE_IRQ_EN: c_uint = 0x00000040;

pub const BM_PXP_IRQ_MASK_DITHER_CH1_PREFETCH_IRQ_EN: c_uint = 0x00000020;

pub const BM_PXP_IRQ_MASK_DITHER_CH0_PREFETCH_IRQ_EN: c_uint = 0x00000010;

pub const BM_PXP_IRQ_MASK_FIRST_CH1_STORE_IRQ_EN: c_uint = 0x00000008;

pub const BM_PXP_IRQ_MASK_FIRST_CH0_STORE_IRQ_EN: c_uint = 0x00000004;

pub const BM_PXP_IRQ_MASK_FIRST_CH1_PREFETCH_IRQ_EN: c_uint = 0x00000002;

pub const BM_PXP_IRQ_MASK_FIRST_CH0_PREFETCH_IRQ_EN: c_uint = 0x00000001;

pub const BM_PXP_IRQ_COMPRESS_DONE_IRQ: c_uint = 0x80000000;

pub const BP_PXP_IRQ_RSVD1: c_int = 16;
pub const BM_PXP_IRQ_RSVD1: c_uint = 0x7FFF0000;

pub const BM_PXP_IRQ_WFE_B_STORE_IRQ: c_uint = 0x00008000;

pub const BM_PXP_IRQ_WFE_A_STORE_IRQ: c_uint = 0x00004000;

pub const BM_PXP_IRQ_DITHER_STORE_IRQ: c_uint = 0x00002000;

pub const BM_PXP_IRQ_FIRST_STORE_IRQ: c_uint = 0x00001000;

pub const BM_PXP_IRQ_WFE_B_CH1_STORE_IRQ: c_uint = 0x00000800;

pub const BM_PXP_IRQ_WFE_B_CH0_STORE_IRQ: c_uint = 0x00000400;

pub const BM_PXP_IRQ_WFE_A_CH1_STORE_IRQ: c_uint = 0x00000200;

pub const BM_PXP_IRQ_WFE_A_CH0_STORE_IRQ: c_uint = 0x00000100;

pub const BM_PXP_IRQ_DITHER_CH1_STORE_IRQ: c_uint = 0x00000080;

pub const BM_PXP_IRQ_DITHER_CH0_STORE_IRQ: c_uint = 0x00000040;

pub const BM_PXP_IRQ_DITHER_CH1_PREFETCH_IRQ: c_uint = 0x00000020;

pub const BM_PXP_IRQ_DITHER_CH0_PREFETCH_IRQ: c_uint = 0x00000010;

pub const BM_PXP_IRQ_FIRST_CH1_STORE_IRQ: c_uint = 0x00000008;

pub const BM_PXP_IRQ_FIRST_CH0_STORE_IRQ: c_uint = 0x00000004;

pub const BM_PXP_IRQ_FIRST_CH1_PREFETCH_IRQ: c_uint = 0x00000002;

pub const BM_PXP_IRQ_FIRST_CH0_PREFETCH_IRQ: c_uint = 0x00000001;

pub const BP_PXP_NEXT_POINTER: c_int = 2;
pub const BM_PXP_NEXT_POINTER: c_uint = 0xFFFFFFFC;

pub const BM_PXP_NEXT_RSVD: c_uint = 0x00000002;

pub const BM_PXP_NEXT_ENABLED: c_uint = 0x00000001;

pub const BP_PXP_DEBUGCTRL_RSVD: c_int = 12;
pub const BM_PXP_DEBUGCTRL_RSVD: c_uint = 0xFFFFF000;

pub const BP_PXP_DEBUGCTRL_LUT_CLR_STAT_CNT: c_int = 8;
pub const BM_PXP_DEBUGCTRL_LUT_CLR_STAT_CNT: c_uint = 0x00000F00;

pub const BV_PXP_DEBUGCTRL_LUT_CLR_STAT_CNT__NONE: c_uint = 0x0;
pub const BV_PXP_DEBUGCTRL_LUT_CLR_STAT_CNT__MISS_CNT: c_uint = 0x1;
pub const BV_PXP_DEBUGCTRL_LUT_CLR_STAT_CNT__HIT_CNT: c_uint = 0x2;
pub const BV_PXP_DEBUGCTRL_LUT_CLR_STAT_CNT__LAT_CNT: c_uint = 0x4;
pub const BV_PXP_DEBUGCTRL_LUT_CLR_STAT_CNT__MAX_LAT: c_uint = 0x8;
pub const BP_PXP_DEBUGCTRL_SELECT: c_int = 0;
pub const BM_PXP_DEBUGCTRL_SELECT: c_uint = 0x000000FF;

pub const BV_PXP_DEBUGCTRL_SELECT__NONE: c_uint = 0x0;
pub const BV_PXP_DEBUGCTRL_SELECT__CTRL: c_uint = 0x1;
pub const BV_PXP_DEBUGCTRL_SELECT__PSBUF: c_uint = 0x2;
pub const BV_PXP_DEBUGCTRL_SELECT__PSBAX: c_uint = 0x3;
pub const BV_PXP_DEBUGCTRL_SELECT__PSBAY: c_uint = 0x4;
pub const BV_PXP_DEBUGCTRL_SELECT__ASBUF: c_uint = 0x5;
pub const BV_PXP_DEBUGCTRL_SELECT__ROTATION: c_uint = 0x6;
pub const BV_PXP_DEBUGCTRL_SELECT__OUTBUF0: c_uint = 0x7;
pub const BV_PXP_DEBUGCTRL_SELECT__OUTBUF1: c_uint = 0x8;
pub const BV_PXP_DEBUGCTRL_SELECT__OUTBUF2: c_uint = 0x9;
pub const BV_PXP_DEBUGCTRL_SELECT__LUT_STAT: c_uint = 0x10;
pub const BV_PXP_DEBUGCTRL_SELECT__LUT_MISS: c_uint = 0x11;
pub const BV_PXP_DEBUGCTRL_SELECT__LUT_HIT: c_uint = 0x12;
pub const BV_PXP_DEBUGCTRL_SELECT__LUT_LAT: c_uint = 0x13;
pub const BV_PXP_DEBUGCTRL_SELECT__LUT_MAX_LAT: c_uint = 0x14;

pub const BP_PXP_DEBUG_DATA: c_int = 0;
pub const BM_PXP_DEBUG_DATA: c_uint = 0xFFFFFFFF;

pub const BP_PXP_VERSION_MAJOR: c_int = 24;
pub const BM_PXP_VERSION_MAJOR: c_uint = 0xFF000000;

pub const BP_PXP_VERSION_MINOR: c_int = 16;
pub const BM_PXP_VERSION_MINOR: c_uint = 0x00FF0000;

pub const BP_PXP_VERSION_STEP: c_int = 0;
pub const BM_PXP_VERSION_STEP: c_uint = 0x0000FFFF;

