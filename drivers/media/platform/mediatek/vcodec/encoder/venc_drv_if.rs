//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/encoder/venc_drv_if.h
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
// Copyright (c) 2016 MediaTek Inc.
// Author: Daniel Hsiao <daniel.hsiao@mediatek.com>
// Jungchang Tsao <jungchang.tsao@mediatek.com>
// Tiffany Lin <tiffany.lin@mediatek.com>
//

//
// enum venc_yuv_fmt - The type of input yuv format
// (VPU related: If you change the order, you must also update the VPU codes.)
// @VENC_YUV_FORMAT_I420: I420 YUV format
// @VENC_YUV_FORMAT_YV12: YV12 YUV format
// @VENC_YUV_FORMAT_NV12: NV12 YUV format
// @VENC_YUV_FORMAT_NV21: NV21 YUV format
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum venc_yuv_fmt {
    VENC_YUV_FORMAT_I420 = 3,
    VENC_YUV_FORMAT_YV12 = 5,
    VENC_YUV_FORMAT_NV12 = 6,
    VENC_YUV_FORMAT_NV21 = 7,
}

//
// enum venc_start_opt - encode frame option used in venc_if_encode()
// @VENC_START_OPT_ENCODE_SEQUENCE_HEADER: encode SPS/PPS for H264
// @VENC_START_OPT_ENCODE_FRAME: encode normal frame
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum venc_start_opt {
    VENC_START_OPT_ENCODE_SEQUENCE_HEADER,
    VENC_START_OPT_ENCODE_FRAME,
}

//
// enum venc_set_param_type - The type of set parameter used in
// venc_if_set_param()
// (VPU related: If you change the order, you must also update the VPU codes.)
// @VENC_SET_PARAM_ENC: set encoder parameters
// @VENC_SET_PARAM_FORCE_INTRA: force an intra frame
// @VENC_SET_PARAM_ADJUST_BITRATE: adjust bitrate (in bps)
// @VENC_SET_PARAM_ADJUST_FRAMERATE: set frame rate
// @VENC_SET_PARAM_GOP_SIZE: set IDR interval
// @VENC_SET_PARAM_INTRA_PERIOD: set I frame interval
// @VENC_SET_PARAM_SKIP_FRAME: set H264 skip one frame
// @VENC_SET_PARAM_PREPEND_HEADER: set H264 prepend SPS/PPS before IDR
// @VENC_SET_PARAM_TS_MODE: set VP8 temporal scalability mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum venc_set_param_type {
    VENC_SET_PARAM_ENC,
    VENC_SET_PARAM_FORCE_INTRA,
    VENC_SET_PARAM_ADJUST_BITRATE,
    VENC_SET_PARAM_ADJUST_FRAMERATE,
    VENC_SET_PARAM_GOP_SIZE,
    VENC_SET_PARAM_INTRA_PERIOD,
    VENC_SET_PARAM_SKIP_FRAME,
    VENC_SET_PARAM_PREPEND_HEADER,
    VENC_SET_PARAM_TS_MODE,
}

//
// struct venc_enc_prm - encoder settings for VENC_SET_PARAM_ENC used in
// venc_if_set_param()
// @input_fourcc: input yuv format
// @h264_profile: V4L2 defined H.264 profile
// @h264_level: V4L2 defined H.264 level
// @width: image width
// @height: image height
// @buf_width: buffer width
// @buf_height: buffer height
// @frm_rate: frame rate in fps
// @intra_period: intra frame period
// @bitrate: target bitrate in bps
// @gop_size: group of picture size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_enc_param {
    pub input_yuv_fmt: venc_yuv_fmt,
    pub h264_profile: c_uint,
    pub h264_level: c_uint,
    pub width: c_uint,
    pub height: c_uint,
    pub buf_width: c_uint,
    pub buf_height: c_uint,
    pub frm_rate: c_uint,
    pub intra_period: c_uint,
    pub bitrate: c_uint,
    pub gop_size: c_uint,
}

//
// struct venc_frame_info - per-frame information to pass to the firmware.
//
// @frm_count:		sequential number for this frame
// @skip_frm_count:	number of frames skipped so far while decoding
// @frm_type:		type of the frame, from enum venc_h264_frame_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_frame_info {
    pub /: *mut *mut unsigned int frm_count; / per frame update,
    pub /: *mut *mut unsigned int skip_frm_count; / per frame update,
    pub /: *mut *mut unsigned int frm_type; / per frame update,
}

//
// struct venc_frm_buf - frame buffer information used in venc_if_encode()
// @fb_addr: plane frame buffer addresses
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_frm_buf {
    pub fb_addr: [mtk_vcodec_fb; MTK_VCODEC_MAX_PLANES],
}

//
// struct venc_done_result - This is return information used in venc_if_encode()
// @bs_size: output bitstream size
// @is_key_frm: output is key frame or not
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_done_result {
    pub bs_size: c_uint,
    pub is_key_frm: bool,
}

//
// venc_if_init - Create the driver handle
// @ctx: device context
// @fourcc: encoder input format
// Return: 0 if creating handle successfully, otherwise it is failed.
//
extern "C" {
    pub fn venc_if_init(ctx: *mut mtk_vcodec_enc_ctx, fourcc: c_uint) -> c_int;
}
//
// venc_if_deinit - Release the driver handle
// @ctx: device context
// Return: 0 if releasing handle successfully, otherwise it is failed.
//
extern "C" {
    pub fn venc_if_deinit(ctx: *mut mtk_vcodec_enc_ctx) -> c_int;
}
//
// venc_if_set_param - Set parameter to driver
// @ctx: device context
// @type: parameter type
// @in: input parameter
// Return: 0 if setting param successfully, otherwise it is failed.
//
// venc_if_encode - Encode one frame
// @ctx: device context
// @opt: encode frame option
// @frm_buf: input frame buffer information
// @bs_buf: output bitstream buffer information
// @result: encode result
// Return: 0 if encoding frame successfully, otherwise it is failed.
//
