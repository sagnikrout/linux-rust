//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/nxp/imx-jpeg/mxc-jpeg-hw.h
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
// i.MX8QXP/i.MX8QM JPEG encoder/decoder v4l2 driver
//
// Copyright 2018-2019 NXP
//

// JPEG Decoder/Encoder Wrapper Register Map
pub const GLB_CTRL: c_uint = 0x0;
pub const COM_STATUS: c_uint = 0x4;
pub const BUF_BASE0: c_uint = 0x14;
pub const BUF_BASE1: c_uint = 0x18;
pub const LINE_PITCH: c_uint = 0x1C;
pub const STM_BUFBASE: c_uint = 0x20;
pub const STM_BUFSIZE: c_uint = 0x24;
pub const IMGSIZE: c_uint = 0x28;
pub const STM_CTRL: c_uint = 0x2C;
// CAST JPEG-Decoder/Encoder Status Register Map (read-only)
pub const CAST_STATUS0: c_uint = 0x100;
pub const CAST_STATUS1: c_uint = 0x104;
pub const CAST_STATUS2: c_uint = 0x108;
pub const CAST_STATUS3: c_uint = 0x10c;
pub const CAST_STATUS4: c_uint = 0x110;
pub const CAST_STATUS5: c_uint = 0x114;
pub const CAST_STATUS6: c_uint = 0x118;
pub const CAST_STATUS7: c_uint = 0x11c;
pub const CAST_STATUS8: c_uint = 0x120;
pub const CAST_STATUS9: c_uint = 0x124;
pub const CAST_STATUS10: c_uint = 0x128;
pub const CAST_STATUS11: c_uint = 0x12c;
pub const CAST_STATUS12: c_uint = 0x130;
pub const CAST_STATUS13: c_uint = 0x134;
// the following are for encoder only
pub const CAST_STATUS14: c_uint = 0x138;
pub const CAST_STATUS15: c_uint = 0x13c;
pub const CAST_STATUS16: c_uint = 0x140;
pub const CAST_STATUS17: c_uint = 0x144;
pub const CAST_STATUS18: c_uint = 0x148;
pub const CAST_STATUS19: c_uint = 0x14c;
// CAST JPEG-Decoder Control Register Map (write-only)

// CAST JPEG-Encoder Control Register Map (write-only)

// JPEG-Decoder Wrapper Slot Registers 0..3
pub const SLOT_BASE: c_uint = 0x10000;
pub const SLOT_STATUS: c_uint = 0x0;
pub const SLOT_IRQ_EN: c_uint = 0x4;
pub const SLOT_BUF_PTR: c_uint = 0x8;
pub const SLOT_CUR_DESCPT_PTR: c_uint = 0xC;
pub const SLOT_NXT_DESCPT_PTR: c_uint = 0x10;

// GLB_CTRL fields
pub const GLB_CTRL_JPG_EN: c_uint = 0x1;

// COM_STAUS fields

// STM_CTRL fields

// SLOT_STATUS fields for slots 0..3

// SLOT_IRQ_EN fields TBD
pub const MXC_NXT_DESCPT_EN: c_uint = 0x1;
pub const MXC_DEC_EXIT_IDLE_MODE: c_uint = 0x4;
// JPEG-Decoder Wrapper - STM_CTRL Register Fields

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxc_jpeg_image_format {
    MXC_JPEG_INVALID = -1,
    MXC_JPEG_YUV420 = 0x0, /* 2 Plannar, Y=1st plane UV=2nd plane */
    MXC_JPEG_YUV422 = 0x1, /* 1 Plannar, YUYV sequence */
    MXC_JPEG_BGR	= 0x2, /* BGR packed format */
    MXC_JPEG_YUV444	= 0x3, /* 1 Plannar, YUVYUV sequence */
    MXC_JPEG_GRAY = 0x4, /* Y8 or Y12 or Single Component */
    MXC_JPEG_RESERVED = 0x5,
    MXC_JPEG_ABGR	= 0x6,
}

extern "C" {
    pub fn print_descriptor_info(dev: *mut device, desc: *mut mxc_jpeg_desc);
}
extern "C" {
    pub fn print_wrapper_info(dev: *mut device, reg: *mut void __iomem);
}
extern "C" {
    pub fn mxc_jpeg_sw_reset(reg: *mut void __iomem);
}
extern "C" {
    pub fn mxc_jpeg_enable(reg: *mut void __iomem) -> c_int;
}
extern "C" {
    pub fn mxc_jpeg_enc_mode_conf(dev: *mut device, reg: *mut void __iomem, extseq: u8);
}
extern "C" {
    pub fn mxc_jpeg_enc_mode_go(dev: *mut device, reg: *mut void __iomem, extseq: u8);
}
extern "C" {
    pub fn mxc_jpeg_enc_set_quality(dev: *mut device, reg: *mut void __iomem, quality: u8);
}
extern "C" {
    pub fn mxc_jpeg_dec_mode_go(dev: *mut device, reg: *mut void __iomem);
}
extern "C" {
    pub fn mxc_jpeg_enable_slot(reg: *mut void __iomem, slot: c_int);
}
extern "C" {
    pub fn mxc_jpeg_set_l_endian(reg: *mut void __iomem, le: c_int);
}
extern "C" {
    pub fn mxc_jpeg_enable_irq(reg: *mut void __iomem, slot: c_int);
}
extern "C" {
    pub fn mxc_jpeg_disable_irq(reg: *mut void __iomem, slot: c_int);
}
extern "C" {
    pub fn mxc_jpeg_set_bufsize(desc: *mut mxc_jpeg_desc, bufsize: u32);
}
extern "C" {
    pub fn mxc_jpeg_set_res(desc: *mut mxc_jpeg_desc, w: u16, h: u16);
}
extern "C" {
    pub fn mxc_jpeg_set_line_pitch(desc: *mut mxc_jpeg_desc, line_pitch: u32);
}
extern "C" {
    pub fn mxc_jpeg_set_desc(desc: u32, reg: *mut void __iomem, slot: c_int);
}
extern "C" {
    pub fn mxc_jpeg_clr_desc(reg: *mut void __iomem, slot: c_int);
}
