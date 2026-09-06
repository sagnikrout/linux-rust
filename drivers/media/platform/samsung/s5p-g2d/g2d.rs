//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-g2d/g2d.h
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
// Samsung S5P G2D - 2D Graphics Accelerator Driver
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// Kamil Debski, <k.debski@samsung.com>
//

pub const TYPE_G2D_3X: c_int = 3;
pub const TYPE_G2D_4X: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct g2d_dev {
    pub v4l2_dev: v4l2_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub vfd: *mut video_device,
    pub mutex: mutex,
    pub ctrl_lock: spinlock_t,
    pub num_inst: core::sync::atomic::AtomicI32,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub gate: *mut clk,
    pub curr: *mut g2d_ctx,
    pub variant: *mut g2d_variant,
    pub irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g2d_frame {
// Original dimensions
    pub width: u32,
    pub height: u32,
// Crop size
    pub c_width: u32,
    pub c_height: u32,
// Offset
    pub o_width: u32,
    pub o_height: u32,
// Image format
    pub fmt: *mut g2d_fmt,
// Variables that can calculated once and reused
    pub stride: u32,
    pub bottom: u32,
    pub right: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g2d_ctx {
    pub fh: v4l2_fh,
    pub dev: *mut g2d_dev,
    pub in: g2d_frame,
    pub out: g2d_frame,
    pub ctrl_hflip: *mut v4l2_ctrl,
    pub ctrl_vflip: *mut v4l2_ctrl,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub rop: u32,
    pub flip: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g2d_fmt {
    pub fourcc: u32,
    pub depth: c_int,
    pub hw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g2d_variant {
    pub hw_rev: c_ushort,
}

extern "C" {
    pub fn g2d_reset(d: *mut g2d_dev);
}
extern "C" {
    pub fn g2d_set_src_size(d: *mut g2d_dev, f: *mut g2d_frame);
}
extern "C" {
    pub fn g2d_set_src_addr(d: *mut g2d_dev, a: dma_addr_t);
}
extern "C" {
    pub fn g2d_set_dst_size(d: *mut g2d_dev, f: *mut g2d_frame);
}
extern "C" {
    pub fn g2d_set_dst_addr(d: *mut g2d_dev, a: dma_addr_t);
}
extern "C" {
    pub fn g2d_start(d: *mut g2d_dev);
}
extern "C" {
    pub fn g2d_clear_int(d: *mut g2d_dev);
}
extern "C" {
    pub fn g2d_set_rop4(d: *mut g2d_dev, r: u32);
}
extern "C" {
    pub fn g2d_set_flip(d: *mut g2d_dev, r: u32);
}
extern "C" {
    pub fn g2d_set_cmd(d: *mut g2d_dev, c: u32);
}
