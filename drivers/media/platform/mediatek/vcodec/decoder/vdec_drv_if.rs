//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/decoder/vdec_drv_if.h
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
// Author: PC Chen <pc.chen@mediatek.com>
// Tiffany Lin <tiffany.lin@mediatek.com>
//

//
// enum vdec_fb_status  - decoder frame buffer status
// @FB_ST_NORMAL: initial state
// @FB_ST_DISPLAY: frame buffer is ready to be displayed
// @FB_ST_FREE: frame buffer is not used by decoder any more
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdec_fb_status {
    FB_ST_NORMAL		= 0,
    FB_ST_DISPLAY		= (1 << 0),
    FB_ST_FREE		= (1 << 1)
}

// For GET_PARAM_DISP_FRAME_BUFFER and GET_PARAM_FREE_FRAME_BUFFER,
// the caller does not own the returned buffer. The buffer will not be
// released before vdec_if_deinit.
// GET_PARAM_DISP_FRAME_BUFFER	: get next displayable frame buffer,
// struct vdec_fb
// GET_PARAM_FREE_FRAME_BUFFER	: get non-referenced framebuffer, vdec_fb
// GET_PARAM_PIC_INFO		: get picture info, struct vdec_pic_info
// GET_PARAM_CROP_INFO		: get crop info, struct v4l2_crop
// GET_PARAM_DPB_SIZE		: get dpb size, unsigned int
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdec_get_param_type {
    GET_PARAM_DISP_FRAME_BUFFER,
    GET_PARAM_FREE_FRAME_BUFFER,
    GET_PARAM_PIC_INFO,
    GET_PARAM_CROP_INFO,
    GET_PARAM_DPB_SIZE
}

//
// struct vdec_fb_node  - decoder frame buffer node
// @list	: list to hold this node
// @fb	: point to frame buffer (vdec_fb), fb could point to frame buffer and
// working buffer this is for maintain buffers in different state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_fb_node {
    pub list: list_head,
    pub fb: *mut vdec_fb,
}

//
// vdec_if_init() - initialize decode driver
// @ctx	: [in] v4l2 context
// @fourcc	: [in] video format fourcc, V4L2_PIX_FMT_H264/VP8/VP9..
//
extern "C" {
    pub fn vdec_if_init(ctx: *mut mtk_vcodec_dec_ctx, fourcc: c_uint) -> c_int;
}
//
// vdec_if_deinit() - deinitialize decode driver
// @ctx	: [in] v4l2 context
//
extern "C" {
    pub fn vdec_if_deinit(ctx: *mut mtk_vcodec_dec_ctx);
}
//
// vdec_if_decode() - trigger decode
// @ctx	: [in] v4l2 context
// @bs	: [in] input bitstream
// @fb	: [in] frame buffer to store decoded frame, when null means parse
// header only
// @res_chg	: [out] resolution change happens if current bs have different
// picture width/height
// Note: To flush the decoder when reaching EOF, set input bitstream as NULL.
//
// Return: 0 on success. -EIO on unrecoverable error.
//
// vdec_if_get_param() - get driver's parameter
// @ctx	: [in] v4l2 context
// @type	: [in] input parameter type
// @out	: [out] buffer to store query result
//
