//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkvdec/rkvdec-hevc-common.h
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
// Rockchip video decoder hevc common functions
//
// Copyright (C) 2025 Collabora, Ltd.
// Detlev Casanova <detlev.casanova@collabora.com>
//
// Copyright (C) 2023 Collabora, Ltd.
// Sebastian Fricke <sebastian.fricke@collabora.com>
//
// Copyright (C) 2019 Collabora, Ltd.
// Boris Brezillon <boris.brezillon@collabora.com>
//
// Copyright (C) 2016 Rockchip Electronics Co., Ltd.
// Jeffy Chen <jeffy.chen@rock-chips.com>
//

// i: 0-63, j: 0-14

// i: 0-63, j: 0-14

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_rps {
    pub 4]: u32 info[RKVDEC_RPS_HEVC_SIZE / 8 /,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rkvdec_hevc_run {
    pub base: rkvdec_run,
    pub slices_params: *const v4l2_ctrl_hevc_slice_params,
    pub decode_params: *const v4l2_ctrl_hevc_decode_params,
    pub sps: *const v4l2_ctrl_hevc_sps,
    pub pps: *const v4l2_ctrl_hevc_pps,
    pub scaling_matrix: *const v4l2_ctrl_hevc_scaling_matrix,
    pub ext_sps_st_rps: *const v4l2_ctrl_hevc_ext_sps_st_rps,
    pub ext_sps_lt_rps: *const v4l2_ctrl_hevc_ext_sps_lt_rps,
    pub num_slices: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scaling_factor {
    pub scalingfactor0: [u8; 1248],
    pub 16X4*/: *mut *mut u8 scalingfactor1[96]; /4X4 TU Rotate, total,
    pub Meeting*/: *mut *mut u8 scalingdc[12]; /N1005 Vienna,
    pub align*/: *mut *mut u8 reserved[4]; /16Bytes,
}

extern "C" {
    pub fn rkvdec_hevc_adjust_fmt(ctx: *mut rkvdec_ctx, f: *mut v4l2_format) -> c_int;
}
extern "C" {
    pub fn rkvdec_hevc_get_image_fmt(ctx: *mut rkvdec_ctx, ctrl: *mut v4l2_ctrl) -> rkvdec_image_fmt;
}
extern "C" {
    pub fn rkvdec_hevc_run_preamble(ctx: *mut rkvdec_ctx, run: *mut rkvdec_hevc_run);
}
