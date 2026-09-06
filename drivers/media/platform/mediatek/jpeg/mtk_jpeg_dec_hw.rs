//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/jpeg/mtk_jpeg_dec_hw.h
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
// Copyright (c) 2016 MediaTek Inc.
// Author: Ming Hsiu Tsai <minghsiu.tsai@mediatek.com>
// Rick Chang <rick.chang@mediatek.com>
// Xia Jiang <xia.jiang@mediatek.com>
//

pub const MTK_JPEG_COMP_MAX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_dec_param {
    pub pic_w: u32,
    pub pic_h: u32,
    pub dec_w: u32,
    pub dec_h: u32,
    pub src_color: u32,
    pub dst_fourcc: u32,
    pub mcu_w: u32,
    pub mcu_h: u32,
    pub total_mcu: u32,
    pub unit_num: u32,
    pub comp_num: u32,
    pub comp_id: [u32; MTK_JPEG_COMP_MAX],
    pub sampling_w: [u32; MTK_JPEG_COMP_MAX],
    pub sampling_h: [u32; MTK_JPEG_COMP_MAX],
    pub qtbl_num: [u32; MTK_JPEG_COMP_MAX],
    pub blk_num: u32,
    pub blk_comp: [u32; MTK_JPEG_COMP_MAX],
    pub membership: u32,
    pub dma_mcu: u32,
    pub dma_group: u32,
    pub dma_last_mcu: u32,
    pub img_stride: [u32; MTK_JPEG_COMP_MAX],
    pub mem_stride: [u32; MTK_JPEG_COMP_MAX],
    pub comp_w: [u32; MTK_JPEG_COMP_MAX],
    pub comp_size: [u32; MTK_JPEG_COMP_MAX],
    pub y_size: u32,
    pub uv_size: u32,
    pub dec_size: u32,
    pub uv_brz_w: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_bs {
    pub str_addr: dma_addr_t,
    pub end_addr: dma_addr_t,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_jpeg_fb {
    pub plane_addr: [dma_addr_t; MTK_JPEG_COMP_MAX],
    pub size: usize,
}

extern "C" {
    pub fn mtk_jpeg_dec_fill_param(param: *mut mtk_jpeg_dec_param) -> c_int;
}
extern "C" {
    pub fn mtk_jpeg_dec_get_int_status(dec_reg_base: *mut void __iomem) -> u32;
}
extern "C" {
    pub fn mtk_jpeg_dec_enum_result(irq_result: u32) -> u32;
}
extern "C" {
    pub fn mtk_jpeg_dec_reset(dec_reg_base: *mut void __iomem);
}
extern "C" {
    pub fn mtk_jpeg_dec_start(dec_reg_base: *mut void __iomem);
}
