//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mtk-mdp3-regs.h
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//

//
// MDP native color code
// Plane count: 1, 2, 3
// H-subsample: 0, 1, 2
// V-subsample: 0, 1
// Color group: 0-RGB, 1-YUV, 2-raw
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp_color {
    MDP_COLOR_UNKNOWN	= 0,

// MDP_COLOR_FULLG8
    MDP_COLOR_FULLG8_RGGB	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0,  8, 2,  0, 21),
    MDP_COLOR_FULLG8_GRBG	= MDP_COLOR(0, 0, 0, 0, 1, 0, 1,  8, 2,  0, 21),
    MDP_COLOR_FULLG8_GBRG	= MDP_COLOR(0, 0, 0, 0, 1, 1, 0,  8, 2,  0, 21),
    MDP_COLOR_FULLG8_BGGR	= MDP_COLOR(0, 0, 0, 0, 1, 1, 1,  8, 2,  0, 21),
    MDP_COLOR_FULLG8	= MDP_COLOR_FULLG8_BGGR,

// MDP_COLOR_FULLG10
    MDP_COLOR_FULLG10_RGGB	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 10, 2,  0, 21),
    MDP_COLOR_FULLG10_GRBG	= MDP_COLOR(0, 0, 0, 0, 1, 0, 1, 10, 2,  0, 21),
    MDP_COLOR_FULLG10_GBRG	= MDP_COLOR(0, 0, 0, 0, 1, 1, 0, 10, 2,  0, 21),
    MDP_COLOR_FULLG10_BGGR	= MDP_COLOR(0, 0, 0, 0, 1, 1, 1, 10, 2,  0, 21),
    MDP_COLOR_FULLG10	= MDP_COLOR_FULLG10_BGGR,

// MDP_COLOR_FULLG12
    MDP_COLOR_FULLG12_RGGB	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 12, 2,  0, 21),
    MDP_COLOR_FULLG12_GRBG	= MDP_COLOR(0, 0, 0, 0, 1, 0, 1, 12, 2,  0, 21),
    MDP_COLOR_FULLG12_GBRG	= MDP_COLOR(0, 0, 0, 0, 1, 1, 0, 12, 2,  0, 21),
    MDP_COLOR_FULLG12_BGGR	= MDP_COLOR(0, 0, 0, 0, 1, 1, 1, 12, 2,  0, 21),
    MDP_COLOR_FULLG12	= MDP_COLOR_FULLG12_BGGR,

// MDP_COLOR_FULLG14
    MDP_COLOR_FULLG14_RGGB	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 14, 2,  0, 21),
    MDP_COLOR_FULLG14_GRBG	= MDP_COLOR(0, 0, 0, 0, 1, 0, 1, 14, 2,  0, 21),
    MDP_COLOR_FULLG14_GBRG	= MDP_COLOR(0, 0, 0, 0, 1, 1, 0, 14, 2,  0, 21),
    MDP_COLOR_FULLG14_BGGR	= MDP_COLOR(0, 0, 0, 0, 1, 1, 1, 14, 2,  0, 21),
    MDP_COLOR_FULLG14	= MDP_COLOR_FULLG14_BGGR,

    MDP_COLOR_UFO10		= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 10, 2,  0, 24),

// MDP_COLOR_BAYER8
    MDP_COLOR_BAYER8_RGGB	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0,  8, 2,  0, 20),
    MDP_COLOR_BAYER8_GRBG	= MDP_COLOR(0, 0, 0, 0, 1, 0, 1,  8, 2,  0, 20),
    MDP_COLOR_BAYER8_GBRG	= MDP_COLOR(0, 0, 0, 0, 1, 1, 0,  8, 2,  0, 20),
    MDP_COLOR_BAYER8_BGGR	= MDP_COLOR(0, 0, 0, 0, 1, 1, 1,  8, 2,  0, 20),
    MDP_COLOR_BAYER8	= MDP_COLOR_BAYER8_BGGR,

// MDP_COLOR_BAYER10
    MDP_COLOR_BAYER10_RGGB	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 10, 2,  0, 20),
    MDP_COLOR_BAYER10_GRBG	= MDP_COLOR(0, 0, 0, 0, 1, 0, 1, 10, 2,  0, 20),
    MDP_COLOR_BAYER10_GBRG	= MDP_COLOR(0, 0, 0, 0, 1, 1, 0, 10, 2,  0, 20),
    MDP_COLOR_BAYER10_BGGR	= MDP_COLOR(0, 0, 0, 0, 1, 1, 1, 10, 2,  0, 20),
    MDP_COLOR_BAYER10	= MDP_COLOR_BAYER10_BGGR,

// MDP_COLOR_BAYER12
    MDP_COLOR_BAYER12_RGGB	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 12, 2,  0, 20),
    MDP_COLOR_BAYER12_GRBG	= MDP_COLOR(0, 0, 0, 0, 1, 0, 1, 12, 2,  0, 20),
    MDP_COLOR_BAYER12_GBRG	= MDP_COLOR(0, 0, 0, 0, 1, 1, 0, 12, 2,  0, 20),
    MDP_COLOR_BAYER12_BGGR	= MDP_COLOR(0, 0, 0, 0, 1, 1, 1, 12, 2,  0, 20),
    MDP_COLOR_BAYER12	= MDP_COLOR_BAYER12_BGGR,

// MDP_COLOR_BAYER14
    MDP_COLOR_BAYER14_RGGB	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 14, 2,  0, 20),
    MDP_COLOR_BAYER14_GRBG	= MDP_COLOR(0, 0, 0, 0, 1, 0, 1, 14, 2,  0, 20),
    MDP_COLOR_BAYER14_GBRG	= MDP_COLOR(0, 0, 0, 0, 1, 1, 0, 14, 2,  0, 20),
    MDP_COLOR_BAYER14_BGGR	= MDP_COLOR(0, 0, 0, 0, 1, 1, 1, 14, 2,  0, 20),
    MDP_COLOR_BAYER14	= MDP_COLOR_BAYER14_BGGR,

    MDP_COLOR_RGB48		= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 48, 0,  0, 23),
// For bayer+mono raw-16
    MDP_COLOR_RGB565_RAW	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 16, 2,  0, 0),

    MDP_COLOR_BAYER8_UNPAK	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0,  8, 2,  0, 22),
    MDP_COLOR_BAYER10_UNPAK	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 10, 2,  0, 22),
    MDP_COLOR_BAYER12_UNPAK	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 12, 2,  0, 22),
    MDP_COLOR_BAYER14_UNPAK	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 14, 2,  0, 22),

// Unified formats
    MDP_COLOR_GREY		= MDP_COLOR(0, 0, 0, 0, 1, 0, 0,  8, 1,  0, 7),

    MDP_COLOR_RGB565	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 16, 0,  0, 0),
    MDP_COLOR_BGR565	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 16, 0,  1, 0),
    MDP_COLOR_RGB888	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 24, 0,  1, 1),
    MDP_COLOR_BGR888	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 24, 0,  0, 1),
    MDP_COLOR_RGBA8888	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 32, 0,  1, 2),
    MDP_COLOR_BGRA8888	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 32, 0,  0, 2),
    MDP_COLOR_ARGB8888	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 32, 0,  1, 3),
    MDP_COLOR_ABGR8888	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 32, 0,  0, 3),

    MDP_COLOR_UYVY		= MDP_COLOR(0, 0, 0, 0, 1, 1, 0, 16, 1,  0, 4),
    MDP_COLOR_VYUY		= MDP_COLOR(0, 0, 0, 0, 1, 1, 0, 16, 1,  1, 4),
    MDP_COLOR_YUYV		= MDP_COLOR(0, 0, 0, 0, 1, 1, 0, 16, 1,  0, 5),
    MDP_COLOR_YVYU		= MDP_COLOR(0, 0, 0, 0, 1, 1, 0, 16, 1,  1, 5),

    MDP_COLOR_I420		= MDP_COLOR(0, 0, 0, 0, 3, 1, 1,  8, 1,  0, 8),
    MDP_COLOR_YV12		= MDP_COLOR(0, 0, 0, 0, 3, 1, 1,  8, 1,  1, 8),
    MDP_COLOR_I422		= MDP_COLOR(0, 0, 0, 0, 3, 1, 0,  8, 1,  0, 9),
    MDP_COLOR_YV16		= MDP_COLOR(0, 0, 0, 0, 3, 1, 0,  8, 1,  1, 9),
    MDP_COLOR_I444		= MDP_COLOR(0, 0, 0, 0, 3, 0, 0,  8, 1,  0, 10),
    MDP_COLOR_YV24		= MDP_COLOR(0, 0, 0, 0, 3, 0, 0,  8, 1,  1, 10),

    MDP_COLOR_NV12		= MDP_COLOR(0, 0, 0, 0, 2, 1, 1,  8, 1,  0, 12),
    MDP_COLOR_NV21		= MDP_COLOR(0, 0, 0, 0, 2, 1, 1,  8, 1,  1, 12),
    MDP_COLOR_NV16		= MDP_COLOR(0, 0, 0, 0, 2, 1, 0,  8, 1,  0, 13),
    MDP_COLOR_NV61		= MDP_COLOR(0, 0, 0, 0, 2, 1, 0,  8, 1,  1, 13),
    MDP_COLOR_NV24		= MDP_COLOR(0, 0, 0, 0, 2, 0, 0,  8, 1,  0, 14),
    MDP_COLOR_NV42		= MDP_COLOR(0, 0, 0, 0, 2, 0, 0,  8, 1,  1, 14),

// MediaTek proprietary formats
// UFO encoded block mode
    MDP_COLOR_420_BLK_UFO	= MDP_COLOR(0, 0, 0, 5, 2, 1, 1, 256, 1, 0, 12),
// Block mode
    MDP_COLOR_420_BLK	= MDP_COLOR(0, 0, 0, 1, 2, 1, 1, 256, 1, 0, 12),
// Block mode + field mode
    MDP_COLOR_420_BLKI	= MDP_COLOR(0, 0, 0, 3, 2, 1, 1, 256, 1, 0, 12),
// Block mode
    MDP_COLOR_422_BLK	= MDP_COLOR(0, 0, 0, 1, 1, 1, 0, 512, 1, 0, 4),

    MDP_COLOR_IYU2		= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 24,  1, 0, 25),
    MDP_COLOR_YUV444	= MDP_COLOR(0, 0, 0, 0, 1, 0, 0, 24,  1, 0, 30),

// Packed 10-bit formats
    MDP_COLOR_RGBA1010102	= MDP_COLOR(0, 1, 0, 0, 1, 0, 0, 32,  0, 1, 2),
    MDP_COLOR_BGRA1010102	= MDP_COLOR(0, 1, 0, 0, 1, 0, 0, 32,  0, 0, 2),
// Packed 10-bit UYVY
    MDP_COLOR_UYVY_10P	= MDP_COLOR(0, 1, 0, 0, 1, 1, 0, 20,  1, 0, 4),
// Packed 10-bit NV21
    MDP_COLOR_NV21_10P	= MDP_COLOR(0, 1, 0, 0, 2, 1, 1, 10,  1, 1, 12),
// 10-bit block mode
    MDP_COLOR_420_BLK_10_H	= MDP_COLOR(0, 1, 0, 1, 2, 1, 1, 320, 1, 0, 12),
// 10-bit HEVC tile mode
    MDP_COLOR_420_BLK_10_V	= MDP_COLOR(0, 1, 1, 1, 2, 1, 1, 320, 1, 0, 12),
// UFO encoded 10-bit block mode
    MDP_COLOR_420_BLK_U10_H	= MDP_COLOR(0, 1, 0, 5, 2, 1, 1, 320, 1, 0, 12),
// UFO encoded 10-bit HEVC tile mode
    MDP_COLOR_420_BLK_U10_V	= MDP_COLOR(0, 1, 1, 5, 2, 1, 1, 320, 1, 0, 12),

// Loose 10-bit formats
    MDP_COLOR_UYVY_10L	= MDP_COLOR(0, 0, 1, 0, 1, 1, 0, 20,  1, 0, 4),
    MDP_COLOR_VYUY_10L	= MDP_COLOR(0, 0, 1, 0, 1, 1, 0, 20,  1, 1, 4),
    MDP_COLOR_YUYV_10L	= MDP_COLOR(0, 0, 1, 0, 1, 1, 0, 20,  1, 0, 5),
    MDP_COLOR_YVYU_10L	= MDP_COLOR(0, 0, 1, 0, 1, 1, 0, 20,  1, 1, 5),
    MDP_COLOR_NV12_10L	= MDP_COLOR(0, 0, 1, 0, 2, 1, 1, 10,  1, 0, 12),
    MDP_COLOR_NV21_10L	= MDP_COLOR(0, 0, 1, 0, 2, 1, 1, 10,  1, 1, 12),
    MDP_COLOR_NV16_10L	= MDP_COLOR(0, 0, 1, 0, 2, 1, 0, 10,  1, 0, 13),
    MDP_COLOR_NV61_10L	= MDP_COLOR(0, 0, 1, 0, 2, 1, 0, 10,  1, 1, 13),
    MDP_COLOR_YV12_10L	= MDP_COLOR(0, 0, 1, 0, 3, 1, 1, 10,  1, 1, 8),
    MDP_COLOR_I420_10L	= MDP_COLOR(0, 0, 1, 0, 3, 1, 1, 10,  1, 0, 8),
}

// Minimum Y stride that is accepted by MDP HW
// Minimum UV stride that is accepted by MDP HW
// Minimum Y plane size that is necessary in buffer
// Minimum UV plane size that is necessary in buffer
// Combine colorspace, xfer_func, ycbcr_encoding, and quantization
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp_ycbcr_profile {
// V4L2_YCBCR_ENC_601 and V4L2_QUANTIZATION_LIM_RANGE
    MDP_YCBCR_PROFILE_BT601,
// V4L2_YCBCR_ENC_709 and V4L2_QUANTIZATION_LIM_RANGE
    MDP_YCBCR_PROFILE_BT709,
// V4L2_YCBCR_ENC_601 and V4L2_QUANTIZATION_FULL_RANGE
    MDP_YCBCR_PROFILE_JPEG,
    MDP_YCBCR_PROFILE_FULL_BT601 = MDP_YCBCR_PROFILE_JPEG,

// Colorspaces not support for capture
// V4L2_YCBCR_ENC_BT2020 and V4L2_QUANTIZATION_LIM_RANGE
    MDP_YCBCR_PROFILE_BT2020,
// V4L2_YCBCR_ENC_709 and V4L2_QUANTIZATION_FULL_RANGE
    MDP_YCBCR_PROFILE_FULL_BT709,
// V4L2_YCBCR_ENC_BT2020 and V4L2_QUANTIZATION_FULL_RANGE
    MDP_YCBCR_PROFILE_FULL_BT2020,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_format {
    pub pixelformat: u32,
    pub mdp_color: u32,
    pub depth: [u8; VIDEO_MAX_PLANES],
    pub row_depth: [u8; VIDEO_MAX_PLANES],
    pub num_planes: u8,
    pub walign: u8,
    pub halign: u8,
    pub salign: u8,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_pix_limit {
    pub wmin: u32,
    pub hmin: u32,
    pub wmax: u32,
    pub hmax: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_limit {
    pub out_limit: mdp_pix_limit,
    pub cap_limit: mdp_pix_limit,
    pub h_scale_up_max: u32,
    pub v_scale_up_max: u32,
    pub h_scale_down_max: u32,
    pub v_scale_down_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp_stream_type {
    MDP_STREAM_TYPE_UNKNOWN,
    MDP_STREAM_TYPE_BITBLT,
    MDP_STREAM_TYPE_GPU_BITBLT,
    MDP_STREAM_TYPE_DUAL_BITBLT,
    MDP_STREAM_TYPE_2ND_BITBLT,
    MDP_STREAM_TYPE_ISP_IC,
    MDP_STREAM_TYPE_ISP_VR,
    MDP_STREAM_TYPE_ISP_ZSD,
    MDP_STREAM_TYPE_ISP_IP,
    MDP_STREAM_TYPE_ISP_VSS,
    MDP_STREAM_TYPE_ISP_ZSD_SLOW,
    MDP_STREAM_TYPE_WPE,
    MDP_STREAM_TYPE_WPE2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_crop {
    pub c: v4l2_rect,
    pub left_subpix: v4l2_fract,
    pub top_subpix: v4l2_fract,
    pub width_subpix: v4l2_fract,
    pub height_subpix: v4l2_fract,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_frame {
    pub format: v4l2_format,
    pub mdp_fmt: *const mdp_format,
    pub /: *mut *mut u32 ycbcr_prof; / enum mdp_ycbcr_profile,
    pub /: *mut *mut u32 usage; / enum mdp_buffer_usage,
    pub crop: mdp_crop,
    pub compose: v4l2_rect,
    pub rotation: i32,
    pub hflip:1: u32,
    pub vflip:1: u32,
    pub hdr:1: u32,
    pub dre:1: u32,
    pub sharpness:1: u32,
    pub dither:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_frameparam {
    pub list: list_head,
    pub ctx: *mut mdp_m2m_ctx,
    pub state: core::sync::atomic::AtomicI32,
    pub limit: *const mdp_limit,
    pub /: *mut *mut u32 type; / enum mdp_stream_type,
    pub frame_no: u32,
    pub output: mdp_frame,
    pub captures: [mdp_frame; MDP_MAX_CAPTURES],
    pub num_captures: u32,
    pub colorspace: v4l2_colorspace,
    pub ycbcr_enc: v4l2_ycbcr_encoding,
    pub xfer_func: v4l2_xfer_func,
    pub quant: v4l2_quantization,
}

extern "C" {
    pub fn mdp_enum_fmt_mplane(mdp: *mut mdp_dev, f: *mut v4l2_fmtdesc) -> c_int;
}
extern "C" {
    pub fn mdp_check_pp_enable(mdp: *mut mdp_dev, frame: *mut mdp_frame) -> bool;
}
extern "C" {
    pub fn mdp_frameparam_init(mdp: *mut mdp_dev, param: *mut mdp_frameparam) -> c_int;
}
