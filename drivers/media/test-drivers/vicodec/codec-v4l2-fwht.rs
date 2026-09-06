//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vicodec/codec-v4l2-fwht.h
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


// SPDX-License-Identifier: LGPL-2.1
//
// Copyright 2018 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_fwht_pixfmt_info {
    pub id: u32,
    pub bytesperline_mult: c_uint,
    pub sizeimage_mult: c_uint,
    pub sizeimage_div: c_uint,
    pub luma_alpha_step: c_uint,
    pub chroma_step: c_uint,
// Chroma plane subsampling
    pub width_div: c_uint,
    pub height_div: c_uint,
    pub components_num: c_uint,
    pub planes_num: c_uint,
    pub pixenc: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_fwht_state {
    pub info: *const v4l2_fwht_pixfmt_info,
    pub visible_width: c_uint,
    pub visible_height: c_uint,
    pub coded_width: c_uint,
    pub coded_height: c_uint,
    pub stride: c_uint,
    pub ref_stride: c_uint,
    pub gop_size: c_uint,
    pub gop_cnt: c_uint,
    pub i_frame_qp: u16,
    pub p_frame_qp: u16,
    pub colorspace: v4l2_colorspace,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub xfer_func: v4l2_xfer_func,
    pub quantization: v4l2_quantization,
    pub ref_frame: fwht_raw_frame,
    pub header: fwht_cframe_hdr,
    pub compressed_frame: *mut u8,
    pub ref_frame_ts: u64,
}

extern "C" {
    pub fn v4l2_fwht_encode(state: *mut v4l2_fwht_state, p_in: *mut u8, p_out: *mut u8) -> c_int;
}
extern "C" {
    pub fn v4l2_fwht_decode(state: *mut v4l2_fwht_state, p_in: *mut u8, p_out: *mut u8) -> c_int;
}
