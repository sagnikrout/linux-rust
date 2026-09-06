//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/mdp_format.h
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
//
// Copyright (c) 2016-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msm_format_flags {
    MSM_FORMAT_FLAG_YUV_BIT,
    MSM_FORMAT_FLAG_DX_BIT,
    MSM_FORMAT_FLAG_COMPRESSED_BIT,
    MSM_FORMAT_FLAG_UNPACK_TIGHT_BIT,
    MSM_FORMAT_FLAG_UNPACK_ALIGN_MSB_BIT,
}

//
// DPU HW,Component order color map
//
// struct msm_format: defines the format configuration
// @pixel_format: format fourcc
// @bpc_g_y: element bit widths: BPC for G or Y
// @bpc_b_cb: element bit widths: BPC for B or Cb
// @bpc_r_cr: element bit widths: BPC for R or Cr
// @bpc_a: element bit widths: BPC for the alpha channel
// @element: element color ordering
// @fetch_type: how the color components are packed in pixel format
// @chroma_sample: chroma sub-samplng type
// @alpha_enable: whether the format has an alpha channel
// @unpack_count: number of the components to unpack
// @bpp: bytes per pixel
// @flags: usage bit flags
// @num_planes: number of planes (including meta data planes)
// @fetch_mode: linear, tiled, or ubwc hw fetch behavior
// @tile_height: format tile height
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_format {
    pub pixel_format: u32,
    pub bpc_r_cr: mdp_bpc bpc_g_y, bpc_b_cb,,
    pub bpc_a: mdp_bpc_alpha,
    pub element: [u8; 4],
    pub fetch_type: mdp_fetch_type,
    pub chroma_sample: mdp_chroma_samp_type,
    pub alpha_enable: bool,
    pub unpack_count: u8,
    pub bpp: u8,
    pub flags: c_ulong,
    pub num_planes: u8,
    pub fetch_mode: mdp_fetch_mode,
    pub tile_height: u16,
}

