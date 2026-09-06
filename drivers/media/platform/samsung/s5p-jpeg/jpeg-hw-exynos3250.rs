//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-jpeg/jpeg-hw-exynos3250.h
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
// linux/drivers/media/platform/samsung/s5p-jpeg/jpeg-hw-exynos3250.h
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Jacek Anaszewski <j.anaszewski@samsung.com>
//

extern "C" {
    pub fn exynos3250_jpeg_reset(regs: *mut void __iomem);
}
extern "C" {
    pub fn exynos3250_jpeg_poweron(regs: *mut void __iomem);
}
extern "C" {
    pub fn exynos3250_jpeg_set_dma_num(regs: *mut void __iomem);
}
extern "C" {
    pub fn exynos3250_jpeg_clk_set(base: *mut void __iomem);
}
extern "C" {
    pub fn exynos3250_jpeg_input_raw_fmt(regs: *mut void __iomem, fmt: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_output_raw_fmt(regs: *mut void __iomem, fmt: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_set_y16(regs: *mut void __iomem, y16: bool);
}
extern "C" {
    pub fn exynos3250_jpeg_proc_mode(regs: *mut void __iomem, mode: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_subsampling_mode(regs: *mut void __iomem, mode: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_get_subsampling_mode(regs: *mut void __iomem) -> c_uint;
}
extern "C" {
    pub fn exynos3250_jpeg_dri(regs: *mut void __iomem, dri: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_qtbl(regs: *mut void __iomem, t: c_uint, n: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_htbl_ac(regs: *mut void __iomem, t: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_htbl_dc(regs: *mut void __iomem, t: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_set_y(regs: *mut void __iomem, y: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_set_x(regs: *mut void __iomem, x: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_interrupts_enable(regs: *mut void __iomem);
}
extern "C" {
    pub fn exynos3250_jpeg_enc_stream_bound(regs: *mut void __iomem, size: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_outform_raw(regs: *mut void __iomem, format: c_ulong);
}
extern "C" {
    pub fn exynos3250_jpeg_jpgadr(regs: *mut void __iomem, addr: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_imgadr(regs: *mut void __iomem, img_addr: *mut s5p_jpeg_addr);
}
extern "C" {
    pub fn exynos3250_jpeg_coef(base: *mut void __iomem, mode: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_start(regs: *mut void __iomem);
}
extern "C" {
    pub fn exynos3250_jpeg_rstart(regs: *mut void __iomem);
}
extern "C" {
    pub fn exynos3250_jpeg_get_int_status(regs: *mut void __iomem) -> c_uint;
}
extern "C" {
    pub fn exynos3250_jpeg_compressed_size(regs: *mut void __iomem) -> c_uint;
}
extern "C" {
    pub fn exynos3250_jpeg_dec_stream_size(regs: *mut void __iomem, size: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_dec_scaling_ratio(regs: *mut void __iomem, sratio: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_set_timer(regs: *mut void __iomem, time_value: c_uint);
}
extern "C" {
    pub fn exynos3250_jpeg_get_timer_status(regs: *mut void __iomem) -> c_uint;
}
extern "C" {
    pub fn exynos3250_jpeg_set_timer_status(regs: *mut void __iomem);
}
extern "C" {
    pub fn exynos3250_jpeg_clear_timer_status(regs: *mut void __iomem);
}
