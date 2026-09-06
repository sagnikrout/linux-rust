//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/vcodec/common/mtk_vcodec_util.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_mem {
    pub size: usize,
    pub va: *mut c_void,
    pub dma_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_vcodec_fb {
    pub size: usize,
    pub dma_addr: dma_addr_t,
}

extern "C" {
    pub fn mtk_vcodec_write_vdecsys(ctx: *mut mtk_vcodec_dec_ctx, reg: c_uint, val: c_uint) -> c_int;
}
extern "C" {
    pub fn mtk_vcodec_mem_alloc(priv: *mut c_void, mem: *mut mtk_vcodec_mem) -> c_int;
}
extern "C" {
    pub fn mtk_vcodec_mem_free(priv: *mut c_void, mem: *mut mtk_vcodec_mem);
}
