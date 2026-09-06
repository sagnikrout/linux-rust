//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ivtv/ivtv-yuv.h
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
pub const IVTV_YUV_BUFFER_UV_OFFSET: c_uint = 0x65400	/* Offset to UV Buffer */;
// Offset to filter table in firmware
pub const IVTV_YUV_HORIZONTAL_FILTER_OFFSET: c_uint = 0x025d8;
pub const IVTV_YUV_VERTICAL_FILTER_OFFSET: c_uint = 0x03358;
pub const IVTV_YUV_UPDATE_HORIZONTAL: c_uint = 0x01;
pub const IVTV_YUV_UPDATE_VERTICAL: c_uint = 0x02;
pub const IVTV_YUV_UPDATE_INVALID: c_uint = 0x04;
extern "C" {
    pub fn ivtv_yuv_filter_check(itv: *mut ivtv) -> c_int;
}
extern "C" {
    pub fn ivtv_yuv_setup_stream_frame(itv: *mut ivtv);
}
extern "C" {
    pub fn ivtv_yuv_udma_stream_frame(itv: *mut ivtv, src: *mut void __user) -> c_int;
}
extern "C" {
    pub fn ivtv_yuv_frame_complete(itv: *mut ivtv);
}
extern "C" {
    pub fn ivtv_yuv_prep_frame(itv: *mut ivtv, args: *mut ivtv_dma_frame) -> c_int;
}
extern "C" {
    pub fn ivtv_yuv_close(itv: *mut ivtv);
}
extern "C" {
    pub fn ivtv_yuv_work_handler(itv: *mut ivtv);
}
