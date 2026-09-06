//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/omapdrm/omap_dmm_tiler.h
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
// Copyright (C) 2011 Texas Instruments Incorporated - https://www.ti.com
// Author: Rob Clark <rob@ti.com>
// Andy Gross <andy.gross@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tiler_fmt {
    TILFMT_8BIT = 0,
    TILFMT_16BIT,
    TILFMT_32BIT,
    TILFMT_PAGE,
    TILFMT_NFORMATS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pat_area {
    pub x0:8: u32,
    pub y0:8: u32,
    pub x1:8: u32,
    pub y1:8: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tiler_block {
    pub /: *mut *mut list_head alloc_node; / node for global block list,
    pub /: *mut *mut tcm_area area; / area,
    pub /: *mut *mut tiler_fmt fmt; / format,
}

// bits representing the same slot in DMM-TILER hw-block
pub const SLOT_WIDTH_BITS: c_int = 6;
pub const SLOT_HEIGHT_BITS: c_int = 6;
// bits reserved to describe coordinates in DMM-TILER hw-block
pub const CONT_WIDTH_BITS: c_int = 14;
pub const CONT_HEIGHT_BITS: c_int = 13;
// calculated constants

//

pub const SHIFT_ACC_MODE: c_int = 27;
pub const MASK_ACC_MODE: c_int = 3;

pub const TILVIEW_8BIT: c_uint = 0x60000000u;

// create tsptr by adding view orientation and access mode

extern "C" {
    pub fn tiler_map_show(s: *mut seq_file, arg: *mut c_void) -> c_int;
}

// pin/unpin
extern "C" {
    pub fn tiler_unpin(block: *mut tiler_block) -> c_int;
}
// reserve/release
extern "C" {
    pub fn tiler_release(block: *mut tiler_block) -> c_int;
}
// utilities
extern "C" {
    pub fn tiler_ssptr(block: *mut tiler_block) -> dma_addr_t;
}
extern "C" {
    pub fn tiler_stride(fmt: tiler_fmt, orient: u32) -> u32;
}
extern "C" {
    pub fn tiler_size(fmt: tiler_fmt, w: u16, h: u16) -> usize;
}
extern "C" {
    pub fn tiler_vsize(fmt: tiler_fmt, w: u16, h: u16) -> usize;
}
extern "C" {
    pub fn tiler_align(fmt: tiler_fmt, w: *mut u16, h: *mut u16);
}
extern "C" {
    pub fn tiler_get_cpu_cache_flags() -> u32;
}
extern "C" {
    pub fn dmm_is_available() -> bool;
}
// GEM bo flags -> tiler fmt
