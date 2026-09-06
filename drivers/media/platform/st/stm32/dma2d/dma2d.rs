//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/stm32/dma2d/dma2d.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ST stm32 DMA2D - 2D Graphics Accelerator Driver
//
// Copyright (c) 2021 Dillon Min
// Dillon Min, <dillon.minfei@gmail.com>
//
// based on s5p-g2d
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// Kamil Debski, <k.debski@samsung.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma2d_op_mode {
    DMA2D_MODE_M2M,
    DMA2D_MODE_M2M_FPC,
    DMA2D_MODE_M2M_BLEND,
    DMA2D_MODE_R2M
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma2d_cmode {
// output pfc cmode from ARGB888 to ARGB4444
    DMA2D_CMODE_ARGB8888,
    DMA2D_CMODE_RGB888,
    DMA2D_CMODE_RGB565,
    DMA2D_CMODE_ARGB1555,
    DMA2D_CMODE_ARGB4444,
// bg or fg pfc cmode from L8 to A4
    DMA2D_CMODE_L8,
    DMA2D_CMODE_AL44,
    DMA2D_CMODE_AL88,
    DMA2D_CMODE_L4,
    DMA2D_CMODE_A8,
    DMA2D_CMODE_A4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma2d_alpha_mode {
    DMA2D_ALPHA_MODE_NO_MODIF,
    DMA2D_ALPHA_MODE_REPLACE,
    DMA2D_ALPHA_MODE_COMBINE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma2d_fmt {
    pub fourcc: u32,
    pub depth: c_int,
    pub cmode: dma2d_cmode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma2d_frame {
// Original dimensions
    pub width: u32,
    pub height: u32,
// Crop size
    pub c_width: u32,
    pub c_height: u32,
// Offset
    pub o_width: u32,
    pub o_height: u32,
    pub bottom: u32,
    pub right: u32,
    pub line_offset: u16,
// Image format
    pub fmt: *mut dma2d_fmt,
// [0]: blue
// [1]: green
// [2]: red
// [3]: alpha
//
    pub a_rgb: [u8; 4],
//
// AM[1:0] of DMA2D_FGPFCCR
//
    pub a_mode: dma2d_alpha_mode,
    pub size: u32,
    pub sequence: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma2d_ctx {
    pub fh: v4l2_fh,
    pub dev: *mut dma2d_dev,
    pub cap: dma2d_frame,
    pub out: dma2d_frame,
    pub bg: dma2d_frame,
//
// MODE[17:16] of DMA2D_CR
//
    pub op_mode: dma2d_op_mode,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub colorspace: v4l2_colorspace,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub xfer_func: v4l2_xfer_func,
    pub quant: v4l2_quantization,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma2d_dev {
    pub v4l2_dev: v4l2_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub vfd: *mut video_device,
// for device open/close etc
    pub mutex: mutex,
// to avoid the conflict with device running and user setting
// at the same time
//
    pub ctrl_lock: spinlock_t,
    pub num_inst: core::sync::atomic::AtomicI32,
    pub regs: *mut void __iomem,
    pub gate: *mut clk,
    pub curr: *mut dma2d_ctx,
    pub irq: c_int,
}

extern "C" {
    pub fn dma2d_start(d: *mut dma2d_dev);
}
extern "C" {
    pub fn dma2d_get_int(d: *mut dma2d_dev) -> u32;
}
extern "C" {
    pub fn dma2d_clear_int(d: *mut dma2d_dev);
}
