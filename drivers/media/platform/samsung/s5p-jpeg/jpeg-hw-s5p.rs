//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-jpeg/jpeg-hw-s5p.h
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


// SPDX-License-Identifier: GPL-2.0-only
// linux/drivers/media/platform/samsung/s5p-jpeg/jpeg-hw.h
//
// Copyright (c) 2011 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

pub const S5P_JPEG_MIN_WIDTH: c_int = 32;
pub const S5P_JPEG_MIN_HEIGHT: c_int = 32;
pub const S5P_JPEG_MAX_WIDTH: c_int = 8192;
pub const S5P_JPEG_MAX_HEIGHT: c_int = 8192;
pub const S5P_JPEG_RAW_IN_565: c_int = 0;
pub const S5P_JPEG_RAW_IN_422: c_int = 1;
pub const S5P_JPEG_RAW_OUT_422: c_int = 0;
pub const S5P_JPEG_RAW_OUT_420: c_int = 1;
extern "C" {
    pub fn s5p_jpeg_reset(regs: *mut void __iomem);
}
extern "C" {
    pub fn s5p_jpeg_poweron(regs: *mut void __iomem);
}
extern "C" {
    pub fn s5p_jpeg_input_raw_mode(regs: *mut void __iomem, mode: c_ulong);
}
extern "C" {
    pub fn s5p_jpeg_proc_mode(regs: *mut void __iomem, mode: c_ulong);
}
extern "C" {
    pub fn s5p_jpeg_subsampling_mode(regs: *mut void __iomem, mode: c_uint);
}
extern "C" {
    pub fn s5p_jpeg_get_subsampling_mode(regs: *mut void __iomem) -> c_uint;
}
extern "C" {
    pub fn s5p_jpeg_dri(regs: *mut void __iomem, dri: c_uint);
}
extern "C" {
    pub fn s5p_jpeg_qtbl(regs: *mut void __iomem, t: c_uint, n: c_uint);
}
extern "C" {
    pub fn s5p_jpeg_htbl_ac(regs: *mut void __iomem, t: c_uint);
}
extern "C" {
    pub fn s5p_jpeg_htbl_dc(regs: *mut void __iomem, t: c_uint);
}
extern "C" {
    pub fn s5p_jpeg_y(regs: *mut void __iomem, y: c_uint);
}
extern "C" {
    pub fn s5p_jpeg_x(regs: *mut void __iomem, x: c_uint);
}
extern "C" {
    pub fn s5p_jpeg_rst_int_enable(regs: *mut void __iomem, enable: bool);
}
extern "C" {
    pub fn s5p_jpeg_data_num_int_enable(regs: *mut void __iomem, enable: bool);
}
extern "C" {
    pub fn s5p_jpeg_final_mcu_num_int_enable(regs: *mut void __iomem, enbl: bool);
}
extern "C" {
    pub fn s5p_jpeg_timer_stat(regs: *mut void __iomem) -> c_int;
}
extern "C" {
    pub fn s5p_jpeg_clear_timer_stat(regs: *mut void __iomem);
}
extern "C" {
    pub fn s5p_jpeg_enc_stream_int(regs: *mut void __iomem, size: c_ulong);
}
extern "C" {
    pub fn s5p_jpeg_enc_stream_stat(regs: *mut void __iomem) -> c_int;
}
extern "C" {
    pub fn s5p_jpeg_clear_enc_stream_stat(regs: *mut void __iomem);
}
extern "C" {
    pub fn s5p_jpeg_outform_raw(regs: *mut void __iomem, format: c_ulong);
}
extern "C" {
    pub fn s5p_jpeg_jpgadr(regs: *mut void __iomem, addr: c_ulong);
}
extern "C" {
    pub fn s5p_jpeg_imgadr(regs: *mut void __iomem, addr: c_ulong);
}
extern "C" {
    pub fn s5p_jpeg_start(regs: *mut void __iomem);
}
extern "C" {
    pub fn s5p_jpeg_result_stat_ok(regs: *mut void __iomem) -> c_int;
}
extern "C" {
    pub fn s5p_jpeg_stream_stat_ok(regs: *mut void __iomem) -> c_int;
}
extern "C" {
    pub fn s5p_jpeg_clear_int(regs: *mut void __iomem);
}
extern "C" {
    pub fn s5p_jpeg_compressed_size(regs: *mut void __iomem) -> c_uint;
}
