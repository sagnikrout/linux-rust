//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/mediatek/mtk_plane.h
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
// Copyright (c) 2015 MediaTek Inc.
// Author: CK Hu <ck.hu@mediatek.com>
//

pub const AFBC_DATA_BLOCK_WIDTH: c_int = 32;
pub const AFBC_DATA_BLOCK_HEIGHT: c_int = 8;
pub const AFBC_HEADER_BLOCK_SIZE: c_int = 16;
pub const AFBC_HEADER_ALIGNMENT: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_plane_pending_state {
    pub config: bool,
    pub enable: bool,
    pub addr: dma_addr_t,
    pub hdr_addr: dma_addr_t,
    pub pitch: c_uint,
    pub hdr_pitch: c_uint,
    pub format: c_uint,
    pub modifier: c_ulonglong,
    pub x: c_uint,
    pub y: c_uint,
    pub width: c_uint,
    pub height: c_uint,
    pub rotation: c_uint,
    pub dirty: bool,
    pub async_dirty: bool,
    pub async_config: bool,
    pub color_encoding: drm_color_encoding,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_plane_state {
    pub base: drm_plane_state,
    pub pending: mtk_plane_pending_state,
}

extern "C" {
    pub fn container_of(_arg: state, mtk_plane_state: struct, _arg: base) -> return;
}
