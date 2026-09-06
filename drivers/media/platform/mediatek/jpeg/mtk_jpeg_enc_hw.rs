//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/jpeg/mtk_jpeg_enc_hw.h
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
// Copyright (c) 2019 MediaTek Inc.
// Author: Xia Jiang <xia.jiang@mediatek.com>
//

pub const JPEG_ENC_INT_STATUS_MASK_ALLIRQ: c_uint = 0x13;

pub const JPEG_ENC_CTRL_YUV_FORMAT_MASK: c_uint = 0x18;

pub const JPEG_ENC_YUV_FORMAT_YUYV: c_int = 0;
pub const JPEG_ENC_YUV_FORMAT_YVYU: c_int = 1;
pub const JPEG_ENC_YUV_FORMAT_NV12: c_int = 2;
pub const JEPG_ENC_YUV_FORMAT_NV21: c_int = 3;
pub const JPEG_ENC_QUALITY_Q60: c_uint = 0x0;
pub const JPEG_ENC_QUALITY_Q80: c_uint = 0x1;
pub const JPEG_ENC_QUALITY_Q90: c_uint = 0x2;
pub const JPEG_ENC_QUALITY_Q95: c_uint = 0x3;
pub const JPEG_ENC_QUALITY_Q39: c_uint = 0x4;
pub const JPEG_ENC_QUALITY_Q68: c_uint = 0x5;
pub const JPEG_ENC_QUALITY_Q84: c_uint = 0x6;
pub const JPEG_ENC_QUALITY_Q92: c_uint = 0x7;
pub const JPEG_ENC_QUALITY_Q48: c_uint = 0x8;
pub const JPEG_ENC_QUALITY_Q74: c_uint = 0xa;
pub const JPEG_ENC_QUALITY_Q87: c_uint = 0xb;
pub const JPEG_ENC_QUALITY_Q34: c_uint = 0xc;
pub const JPEG_ENC_QUALITY_Q64: c_uint = 0xe;
pub const JPEG_ENC_QUALITY_Q82: c_uint = 0xf;
pub const JPEG_ENC_QUALITY_Q97: c_uint = 0x10;
pub const JPEG_ENC_RSTB: c_uint = 0x100;
pub const JPEG_ENC_CTRL: c_uint = 0x104;
pub const JPEG_ENC_QUALITY: c_uint = 0x108;
pub const JPEG_ENC_BLK_NUM: c_uint = 0x10C;
pub const JPEG_ENC_BLK_CNT: c_uint = 0x110;
pub const JPEG_ENC_INT_STS: c_uint = 0x11c;
pub const JPEG_ENC_DST_ADDR0: c_uint = 0x120;
pub const JPEG_ENC_DMA_ADDR0: c_uint = 0x124;
pub const JPEG_ENC_STALL_ADDR0: c_uint = 0x128;
pub const JPEG_ENC_OFFSET_ADDR: c_uint = 0x138;
pub const JPEG_ENC_RST_MCU_NUM: c_uint = 0x150;
pub const JPEG_ENC_IMG_SIZE: c_uint = 0x154;
pub const JPEG_ENC_DEBUG_INFO0: c_uint = 0x160;
pub const JPEG_ENC_DEBUG_INFO1: c_uint = 0x164;
pub const JPEG_ENC_TOTAL_CYCLE: c_uint = 0x168;
pub const JPEG_ENC_BYTE_OFFSET_MASK: c_uint = 0x16c;
pub const JPEG_ENC_SRC_LUMA_ADDR: c_uint = 0x170;
pub const JPEG_ENC_SRC_CHROMA_ADDR: c_uint = 0x174;
pub const JPEG_ENC_STRIDE: c_uint = 0x178;
pub const JPEG_ENC_IMG_STRIDE: c_uint = 0x17c;
pub const JPEG_ENC_DCM_CTRL: c_uint = 0x300;
pub const JPEG_ENC_CODEC_SEL: c_uint = 0x314;
pub const JPEG_ENC_ULTRA_THRES: c_uint = 0x318;
pub const JPEG_ENC_SRC_LUMA_ADDR_EXT: c_uint = 0x584;
pub const JPEG_ENC_SRC_CHRO_ADDR_EXT: c_uint = 0x588;
pub const JPEG_ENC_Q_TBL_ADDR_EXT: c_uint = 0x58C;
pub const JPEG_ENC_DEST_ADDR0_EXT: c_uint = 0x590;
pub const JPEG_ENC_STALL_ADDR0_EXT: c_uint = 0x594;
//
// struct mtk_jpeg_enc_qlt - JPEG encoder quality data
// @quality_param:	quality value
// @hardware_value:	hardware value of quality
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_enc_qlt {
    pub quality_param: u8,
    pub hardware_value: u8,
}

extern "C" {
    pub fn mtk_jpeg_enc_reset(base: *mut void __iomem);
}
extern "C" {
    pub fn mtk_jpeg_enc_get_file_size(base: *mut void __iomem, support_34bit: bool) -> u32;
}
extern "C" {
    pub fn mtk_jpeg_enc_start(enc_reg_base: *mut void __iomem);
}
extern "C" {
    pub fn mtk_jpeg_set_enc_params(ctx: *mut mtk_jpeg_ctx, base: *mut void __iomem);
}
