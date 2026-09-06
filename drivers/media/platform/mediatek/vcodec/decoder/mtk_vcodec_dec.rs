//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/decoder/mtk_vcodec_dec.h
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

pub const VCODEC_DEC_ALIGNED_64: c_int = 64;
pub const VCODEC_CAPABILITY_4K_DISABLED: c_uint = 0x10;

pub const MTK_VDEC_IRQ_STATUS_DEC_SUCCESS: c_uint = 0x10000;
//
// struct vdec_fb  - decoder frame buffer
// @base_y	: Y plane memory info
// @base_c	: C plane memory info
// @status      : frame buffer status (vdec_fb_status)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_fb {
    pub base_y: mtk_vcodec_mem,
    pub base_c: mtk_vcodec_mem,
    pub status: c_uint,
}

//
// struct mtk_video_dec_buf - Private data related to each VB2 buffer.
// @m2m_buf:	M2M buffer
// @used:	Capture buffer contain decoded frame data and keep in
// codec data structure
// @queued_in_vb2:	Capture buffer is queue in vb2
// @queued_in_v4l2:	Capture buffer is in v4l2 driver, but not in vb2
// queue yet
// @error:		An unrecoverable error occurs on this buffer.
// @frame_buffer:	Decode status, and buffer information of Capture buffer
// @bs_buffer:	Output buffer info
//
// Note : These status information help us track and debug buffer state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_video_dec_buf {
    pub m2m_buf: v4l2_m2m_buffer,
    pub used: bool,
    pub queued_in_vb2: bool,
    pub queued_in_v4l2: bool,
    pub error: bool,
    pub frame_buffer: vdec_fb,
    pub bs_buffer: mtk_vcodec_mem,
}

//
// mtk_vdec_lock/mtk_vdec_unlock are for ctx instance to
// get/release lock before/after access decoder hw.
// mtk_vdec_lock get decoder hw lock and set curr_ctx
// to ctx instance that get lock
//
extern "C" {
    pub fn mtk_vdec_unlock(ctx: *mut mtk_vcodec_dec_ctx);
}
extern "C" {
    pub fn mtk_vdec_lock(ctx: *mut mtk_vcodec_dec_ctx);
}
extern "C" {
    pub fn mtk_vcodec_dec_set_default_params(ctx: *mut mtk_vcodec_dec_ctx);
}
extern "C" {
    pub fn mtk_vcodec_dec_release(ctx: *mut mtk_vcodec_dec_ctx);
}
//
// VB2 ops
//
extern "C" {
    pub fn vb2ops_vdec_buf_prepare(vb: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn vb2ops_vdec_buf_finish(vb: *mut vb2_buffer);
}
extern "C" {
    pub fn vb2ops_vdec_buf_init(vb: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn vb2ops_vdec_start_streaming(q: *mut vb2_queue, count: c_uint) -> c_int;
}
extern "C" {
    pub fn vb2ops_vdec_stop_streaming(q: *mut vb2_queue);
}
