//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/encoder/mtk_vcodec_enc.h
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

pub const MTK_VENC_IRQ_STATUS_SPS: c_uint = 0x1;
pub const MTK_VENC_IRQ_STATUS_PPS: c_uint = 0x2;
pub const MTK_VENC_IRQ_STATUS_FRM: c_uint = 0x4;
pub const MTK_VENC_IRQ_STATUS_DRAM: c_uint = 0x8;
pub const MTK_VENC_IRQ_STATUS_PAUSE: c_uint = 0x10;
pub const MTK_VENC_IRQ_STATUS_SWITCH: c_uint = 0x20;
pub const MTK_VENC_IRQ_STATUS_OFFSET: c_uint = 0x05C;
pub const MTK_VENC_IRQ_ACK_OFFSET: c_uint = 0x060;
//
// struct mtk_video_enc_buf - Private data related to each VB2 buffer.
// @m2m_buf:	M2M buffer
// @param_change: Types of encode parameter change before encoding this
// buffer
// @enc_params: Encode parameters changed before encode this buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_video_enc_buf {
    pub m2m_buf: v4l2_m2m_buffer,
    pub param_change: u32,
    pub enc_params: mtk_enc_params,
}

extern "C" {
    pub fn mtk_venc_unlock(ctx: *mut mtk_vcodec_enc_ctx) -> c_int;
}
extern "C" {
    pub fn mtk_venc_lock(ctx: *mut mtk_vcodec_enc_ctx) -> c_int;
}
extern "C" {
    pub fn mtk_vcodec_enc_release(ctx: *mut mtk_vcodec_enc_ctx);
}
extern "C" {
    pub fn mtk_vcodec_enc_ctrls_setup(ctx: *mut mtk_vcodec_enc_ctx) -> c_int;
}
extern "C" {
    pub fn mtk_vcodec_enc_set_default_params(ctx: *mut mtk_vcodec_enc_ctx);
}
