//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun8i-di/sun8i-di.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Allwinner Deinterlace driver
//
// Copyright (C) 2019 Jernej Skrabec <jernej.skrabec@siol.net>
//

pub const DEINTERLACE_MOD_ENABLE: c_uint = 0x00;

pub const DEINTERLACE_FRM_CTRL: c_uint = 0x04;

pub const DEINTERLACE_BYPASS: c_uint = 0x08;

pub const DEINTERLACE_AGTH_SEL: c_uint = 0x0c;

pub const DEINTERLACE_LINT_CTRL: c_uint = 0x10;
pub const DEINTERLACE_TRD_PRELUMA: c_uint = 0x1c;
pub const DEINTERLACE_BUF_ADDR0: c_uint = 0x20;
pub const DEINTERLACE_BUF_ADDR1: c_uint = 0x24;
pub const DEINTERLACE_BUF_ADDR2: c_uint = 0x28;
pub const DEINTERLACE_FIELD_CTRL: c_uint = 0x2c;

pub const DEINTERLACE_TB_OFFSET0: c_uint = 0x30;
pub const DEINTERLACE_TB_OFFSET1: c_uint = 0x34;
pub const DEINTERLACE_TB_OFFSET2: c_uint = 0x38;
pub const DEINTERLACE_TRD_PRECHROMA: c_uint = 0x3c;
pub const DEINTERLACE_LINE_STRIDE0: c_uint = 0x40;
pub const DEINTERLACE_LINE_STRIDE1: c_uint = 0x44;
pub const DEINTERLACE_LINE_STRIDE2: c_uint = 0x48;
pub const DEINTERLACE_IN_FMT: c_uint = 0x4c;

pub const DEINTERLACE_WB_ADDR0: c_uint = 0x50;
pub const DEINTERLACE_WB_ADDR1: c_uint = 0x54;
pub const DEINTERLACE_WB_ADDR2: c_uint = 0x58;
pub const DEINTERLACE_OUT_FMT: c_uint = 0x5c;

pub const DEINTERLACE_INT_ENABLE: c_uint = 0x60;

pub const DEINTERLACE_INT_STATUS: c_uint = 0x64;

pub const DEINTERLACE_STATUS: c_uint = 0x68;

pub const DEINTERLACE_CSC_COEF: c_uint = 0x70 /* 12 registers */;
pub const DEINTERLACE_CTRL: c_uint = 0xa0;

pub const DEINTERLACE_DIAG_INTP: c_uint = 0xa4;

pub const DEINTERLACE_TEMP_DIFF: c_uint = 0xa8;

pub const DEINTERLACE_LUMA_TH: c_uint = 0xac;

pub const DEINTERLACE_SPAT_COMP: c_uint = 0xb0;

pub const DEINTERLACE_CHROMA_DIFF: c_uint = 0xb4;

pub const DEINTERLACE_PRELUMA: c_uint = 0xb8;
pub const DEINTERLACE_PRECHROMA: c_uint = 0xbc;
pub const DEINTERLACE_TILE_FLAG0: c_uint = 0xc0;
pub const DEINTERLACE_TILE_FLAG1: c_uint = 0xc4;
pub const DEINTERLACE_FLAG_LINE_STRIDE: c_uint = 0xc8;
pub const DEINTERLACE_FLAG_SEQ: c_uint = 0xcc;
pub const DEINTERLACE_WB_LINE_STRIDE_CTRL: c_uint = 0xd0;

pub const DEINTERLACE_WB_LINE_STRIDE0: c_uint = 0xd4;
pub const DEINTERLACE_WB_LINE_STRIDE1: c_uint = 0xd8;
pub const DEINTERLACE_WB_LINE_STRIDE2: c_uint = 0xdc;
pub const DEINTERLACE_TRD_CTRL: c_uint = 0xe0;
pub const DEINTERLACE_TRD_BUF_ADDR0: c_uint = 0xe4;
pub const DEINTERLACE_TRD_BUF_ADDR1: c_uint = 0xe8;
pub const DEINTERLACE_TRD_BUF_ADDR2: c_uint = 0xec;
pub const DEINTERLACE_TRD_TB_OFF0: c_uint = 0xf0;
pub const DEINTERLACE_TRD_TB_OFF1: c_uint = 0xf4;
pub const DEINTERLACE_TRD_TB_OFF2: c_uint = 0xf8;
pub const DEINTERLACE_TRD_WB_STRIDE: c_uint = 0xfc;
pub const DEINTERLACE_CH0_IN_SIZE: c_uint = 0x100;
pub const DEINTERLACE_CH0_OUT_SIZE: c_uint = 0x104;
pub const DEINTERLACE_CH0_HORZ_FACT: c_uint = 0x108;
pub const DEINTERLACE_CH0_VERT_FACT: c_uint = 0x10c;
pub const DEINTERLACE_CH0_HORZ_PHASE: c_uint = 0x110;
pub const DEINTERLACE_CH0_VERT_PHASE0: c_uint = 0x114;
pub const DEINTERLACE_CH0_VERT_PHASE1: c_uint = 0x118;
pub const DEINTERLACE_CH0_HORZ_TAP0: c_uint = 0x120;
pub const DEINTERLACE_CH0_HORZ_TAP1: c_uint = 0x124;
pub const DEINTERLACE_CH0_VERT_TAP: c_uint = 0x128;
pub const DEINTERLACE_CH1_IN_SIZE: c_uint = 0x200;
pub const DEINTERLACE_CH1_OUT_SIZE: c_uint = 0x204;
pub const DEINTERLACE_CH1_HORZ_FACT: c_uint = 0x208;
pub const DEINTERLACE_CH1_VERT_FACT: c_uint = 0x20c;
pub const DEINTERLACE_CH1_HORZ_PHASE: c_uint = 0x210;
pub const DEINTERLACE_CH1_VERT_PHASE0: c_uint = 0x214;
pub const DEINTERLACE_CH1_VERT_PHASE1: c_uint = 0x218;
pub const DEINTERLACE_CH1_HORZ_TAP0: c_uint = 0x220;
pub const DEINTERLACE_CH1_HORZ_TAP1: c_uint = 0x224;
pub const DEINTERLACE_CH1_VERT_TAP: c_uint = 0x228;
pub const DEINTERLACE_CH0_HORZ_COEF0: c_uint = 0x400 /* 32 registers */;
pub const DEINTERLACE_CH0_HORZ_COEF1: c_uint = 0x480 /* 32 registers */;
pub const DEINTERLACE_CH0_VERT_COEF: c_uint = 0x500 /* 32 registers */;
pub const DEINTERLACE_CH1_HORZ_COEF0: c_uint = 0x600 /* 32 registers */;
pub const DEINTERLACE_CH1_HORZ_COEF1: c_uint = 0x680 /* 32 registers */;
pub const DEINTERLACE_CH1_VERT_COEF: c_uint = 0x700 /* 32 registers */;
pub const DEINTERLACE_CH3_HORZ_COEF0: c_uint = 0x800 /* 32 registers */;
pub const DEINTERLACE_CH3_HORZ_COEF1: c_uint = 0x880 /* 32 registers */;
pub const DEINTERLACE_CH3_VERT_COEF: c_uint = 0x900 /* 32 registers */;

pub const DEINTERLACE_MODE_UV_COMBINED: c_int = 2;
pub const DEINTERLACE_IN_FMT_YUV420: c_int = 2;
pub const DEINTERLACE_OUT_FMT_YUV420SP: c_int = 13;
pub const DEINTERLACE_PS_UVUV: c_int = 0;
pub const DEINTERLACE_PS_VUVU: c_int = 1;
pub const DEINTERLACE_IDENTITY_COEF: c_uint = 0x4000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct deinterlace_ctx {
    pub fh: v4l2_fh,
    pub dev: *mut deinterlace_dev,
    pub src_fmt: v4l2_pix_format,
    pub dst_fmt: v4l2_pix_format,
    pub flag1_buf: *mut c_void,
    pub flag1_buf_dma: dma_addr_t,
    pub flag2_buf: *mut c_void,
    pub flag2_buf_dma: dma_addr_t,
    pub prev: *mut vb2_v4l2_buffer,
    pub first_field: c_uint,
    pub field: c_uint,
    pub aborting: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct deinterlace_dev {
    pub v4l2_dev: v4l2_device,
    pub vfd: video_device,
    pub dev: *mut device,
    pub m2m_dev: *mut v4l2_m2m_dev,
// Device file mutex
    pub dev_mutex: mutex,
    pub base: *mut void __iomem,
    pub bus_clk: *mut clk,
    pub mod_clk: *mut clk,
    pub ram_clk: *mut clk,
    pub rstc: *mut reset_control,
}
