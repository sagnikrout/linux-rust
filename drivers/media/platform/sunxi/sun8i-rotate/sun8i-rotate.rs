//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/sunxi/sun8i-rotate/sun8i-rotate.h
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
// Allwinner DE2 rotation driver
//
// Copyright (C) 2020 Jernej Skrabec <jernej.skrabec@siol.net>
//

pub const ROTATE_GLB_CTL: c_uint = 0x00;

pub const ROTATE_INT: c_uint = 0x04;

pub const ROTATE_IN_FMT: c_uint = 0x20;

pub const ROTATE_IN_SIZE: c_uint = 0x24;
pub const ROTATE_IN_PITCH0: c_uint = 0x30;
pub const ROTATE_IN_PITCH1: c_uint = 0x34;
pub const ROTATE_IN_PITCH2: c_uint = 0x38;
pub const ROTATE_IN_ADDRL0: c_uint = 0x40;
pub const ROTATE_IN_ADDRH0: c_uint = 0x44;
pub const ROTATE_IN_ADDRL1: c_uint = 0x48;
pub const ROTATE_IN_ADDRH1: c_uint = 0x4c;
pub const ROTATE_IN_ADDRL2: c_uint = 0x50;
pub const ROTATE_IN_ADDRH2: c_uint = 0x54;
pub const ROTATE_OUT_SIZE: c_uint = 0x84;
pub const ROTATE_OUT_PITCH0: c_uint = 0x90;
pub const ROTATE_OUT_PITCH1: c_uint = 0x94;
pub const ROTATE_OUT_PITCH2: c_uint = 0x98;
pub const ROTATE_OUT_ADDRL0: c_uint = 0xA0;
pub const ROTATE_OUT_ADDRH0: c_uint = 0xA4;
pub const ROTATE_OUT_ADDRL1: c_uint = 0xA8;
pub const ROTATE_OUT_ADDRH1: c_uint = 0xAC;
pub const ROTATE_OUT_ADDRL2: c_uint = 0xB0;
pub const ROTATE_OUT_ADDRH2: c_uint = 0xB4;
pub const ROTATE_BURST_8: c_uint = 0x07;
pub const ROTATE_BURST_16: c_uint = 0x0f;
pub const ROTATE_BURST_32: c_uint = 0x1f;
pub const ROTATE_BURST_64: c_uint = 0x3f;
pub const ROTATE_MODE_COPY_ROTATE: c_uint = 0x01;
pub const ROTATE_FORMAT_ARGB32: c_uint = 0x00;
pub const ROTATE_FORMAT_ABGR32: c_uint = 0x01;
pub const ROTATE_FORMAT_RGBA32: c_uint = 0x02;
pub const ROTATE_FORMAT_BGRA32: c_uint = 0x03;
pub const ROTATE_FORMAT_XRGB32: c_uint = 0x04;
pub const ROTATE_FORMAT_XBGR32: c_uint = 0x05;
pub const ROTATE_FORMAT_RGBX32: c_uint = 0x06;
pub const ROTATE_FORMAT_BGRX32: c_uint = 0x07;
pub const ROTATE_FORMAT_RGB24: c_uint = 0x08;
pub const ROTATE_FORMAT_BGR24: c_uint = 0x09;
pub const ROTATE_FORMAT_RGB565: c_uint = 0x0a;
pub const ROTATE_FORMAT_BGR565: c_uint = 0x0b;
pub const ROTATE_FORMAT_ARGB4444: c_uint = 0x0c;
pub const ROTATE_FORMAT_ABGR4444: c_uint = 0x0d;
pub const ROTATE_FORMAT_RGBA4444: c_uint = 0x0e;
pub const ROTATE_FORMAT_BGRA4444: c_uint = 0x0f;
pub const ROTATE_FORMAT_ARGB1555: c_uint = 0x10;
pub const ROTATE_FORMAT_ABGR1555: c_uint = 0x11;
pub const ROTATE_FORMAT_RGBA5551: c_uint = 0x12;
pub const ROTATE_FORMAT_BGRA5551: c_uint = 0x13;
pub const ROTATE_FORMAT_YUYV: c_uint = 0x20;
pub const ROTATE_FORMAT_UYVY: c_uint = 0x21;
pub const ROTATE_FORMAT_YVYU: c_uint = 0x22;
pub const ROTATE_FORMAT_VYUV: c_uint = 0x23;
pub const ROTATE_FORMAT_NV61: c_uint = 0x24;
pub const ROTATE_FORMAT_NV16: c_uint = 0x25;
pub const ROTATE_FORMAT_YUV422P: c_uint = 0x26;
pub const ROTATE_FORMAT_NV21: c_uint = 0x28;
pub const ROTATE_FORMAT_NV12: c_uint = 0x29;
pub const ROTATE_FORMAT_YUV420P: c_uint = 0x2A;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rotate_ctx {
    pub fh: v4l2_fh,
    pub dev: *mut rotate_dev,
    pub src_fmt: v4l2_pix_format,
    pub dst_fmt: v4l2_pix_format,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub hflip: u32,
    pub vflip: u32,
    pub rotate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rotate_dev {
    pub v4l2_dev: v4l2_device,
    pub vfd: video_device,
    pub dev: *mut device,
    pub m2m_dev: *mut v4l2_m2m_dev,
// Device file mutex
    pub dev_mutex: mutex,
    pub base: *mut void __iomem,
    pub bus_clk: *mut clk,
    pub mod_clk: *mut clk,
    pub rstc: *mut reset_control,
}
