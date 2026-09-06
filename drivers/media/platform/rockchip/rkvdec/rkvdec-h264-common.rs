//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkvdec/rkvdec-h264-common.h
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
// Rockchip video decoder h264 common functions
//
// Copyright (C) 2025 Collabora, Ltd.
// Detlev Casanova <detlev.casanova@collabora.com>
//
// Copyright (C) 2019 Collabora, Ltd.
// Boris Brezillon <boris.brezillon@collabora.com>
//
// Copyright (C) 2016 Rockchip Electronics Co., Ltd.
// Jeffy Chen <jeffy.chen@rock-chips.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_h264_scaling_list {
    pub scaling_list_4x4: [u8; 6][16],
    pub scaling_list_8x8: [u8; 6][64],
    pub padding: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_h264_reflists {
    pub p: [v4l2_h264_reference; V4L2_H264_REF_LIST_LEN],
    pub b0: [v4l2_h264_reference; V4L2_H264_REF_LIST_LEN],
    pub b1: [v4l2_h264_reference; V4L2_H264_REF_LIST_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_h264_run {
    pub base: rkvdec_run,
    pub decode_params: *const v4l2_ctrl_h264_decode_params,
    pub sps: *const v4l2_ctrl_h264_sps,
    pub pps: *const v4l2_ctrl_h264_pps,
    pub scaling_matrix: *const v4l2_ctrl_h264_scaling_matrix,
    pub ref_buf: [*mut vb2_buffer; V4L2_H264_NUM_DPB_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_rps {
    pub 4]: u32 info[RKVDEC_H264_RPS_SIZE / 8 /,
}

extern "C" {
    pub fn lookup_ref_buf_idx(ctx: *mut rkvdec_ctx, run: *mut rkvdec_h264_run);
}
extern "C" {
    pub fn rkvdec_h264_adjust_fmt(ctx: *mut rkvdec_ctx, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn rkvdec_h264_get_image_fmt(ctx: *mut rkvdec_ctx, ctrl: *mut v4l2_ctrl) -> rkvdec_image_fmt;
}
extern "C" {
    pub fn rkvdec_h264_validate_sps(ctx: *mut rkvdec_ctx, sps: *const v4l2_ctrl_h264_sps) -> c_int;
}
extern "C" {
    pub fn rkvdec_h264_run_preamble(ctx: *mut rkvdec_ctx, run: *mut rkvdec_h264_run);
}
