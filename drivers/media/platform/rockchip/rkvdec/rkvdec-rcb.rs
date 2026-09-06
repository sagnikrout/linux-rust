//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/rockchip/rkvdec/rkvdec-rcb.h
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
// Rockchip video decoder Rows and Cols Buffers manager
//
// Copyright (C) 2025 Collabora, Ltd.
// Detlev Casanova <detlev.casanova@collabora.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rcb_axis {
    PIC_WIDTH = 0,
    PIC_HEIGHT = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcb_size_info {
    pub multiplier: u8,
    pub axis: rcb_axis,
}

extern "C" {
    pub fn rkvdec_rcb_buf_dma_addr(ctx: *mut rkvdec_ctx, id: c_int) -> dma_addr_t;
}
extern "C" {
    pub fn rkvdec_rcb_buf_size(ctx: *mut rkvdec_ctx, id: c_int) -> usize;
}
extern "C" {
    pub fn rkvdec_rcb_buf_count(ctx: *mut rkvdec_ctx) -> c_int;
}
extern "C" {
    pub fn rkvdec_free_rcb(ctx: *mut rkvdec_ctx);
}
