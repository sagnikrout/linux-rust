//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-jpeg/jpeg-hw-exynos4.h
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
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Jacek Anaszewski <j.anaszewski@samsung.com>
//
// Header file of the register interface for JPEG driver on Exynos4x12.
//
extern "C" {
    pub fn exynos4_jpeg_sw_reset(base: *mut void __iomem);
}
extern "C" {
    pub fn exynos4_jpeg_set_enc_dec_mode(base: *mut void __iomem, mode: c_uint);
}
extern "C" {
    pub fn exynos4_jpeg_set_enc_tbl(base: *mut void __iomem);
}
extern "C" {
    pub fn exynos4_jpeg_set_interrupt(base: *mut void __iomem, version: c_uint);
}
extern "C" {
    pub fn exynos4_jpeg_get_int_status(base: *mut void __iomem) -> c_uint;
}
extern "C" {
    pub fn exynos4_jpeg_set_huf_table_enable(base: *mut void __iomem, value: c_int);
}
extern "C" {
    pub fn exynos4_jpeg_set_sys_int_enable(base: *mut void __iomem, value: c_int);
}
extern "C" {
    pub fn exynos4_jpeg_set_dec_components(base: *mut void __iomem, n: c_int);
}
extern "C" {
    pub fn exynos4_jpeg_select_dec_q_tbl(base: *mut void __iomem, c: c_char, x: c_char);
}
extern "C" {
    pub fn exynos4_jpeg_select_dec_h_tbl(base: *mut void __iomem, c: c_char, x: c_char);
}
extern "C" {
    pub fn exynos4_jpeg_set_encode_hoff_cnt(base: *mut void __iomem, fmt: c_uint);
}
extern "C" {
    pub fn exynos4_jpeg_set_dec_bitstream_size(base: *mut void __iomem, size: c_uint);
}
extern "C" {
    pub fn exynos4_jpeg_get_stream_size(base: *mut void __iomem) -> c_uint;
}
extern "C" {
    pub fn exynos4_jpeg_get_frame_fmt(base: *mut void __iomem) -> c_uint;
}
