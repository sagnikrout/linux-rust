//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/omapvrfb.h
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
// VRFB Rotation Engine
//
// Copyright (C) 2009 Nokia Corporation
// Author: Tomi Valkeinen <tomi.valkeinen@nokia.com>
//
pub const OMAP_VRFB_LINE_LEN: c_int = 2048;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vrfb {
    pub context: u8,
    pub vaddr: [*mut void __iomem; 4],
    pub paddr: [c_ulong; 4],
    pub xres: u16,
    pub yres: u16,
    pub xoffset: u16,
    pub yoffset: u16,
    pub bytespp: u8,
    pub yuv_mode: bool,
}

extern "C" {
    pub fn omap_vrfb_supported() -> bool;
}
extern "C" {
    pub fn omap_vrfb_request_ctx(vrfb: *mut vrfb) -> c_int;
}
extern "C" {
    pub fn omap_vrfb_release_ctx(vrfb: *mut vrfb);
}
extern "C" {
    pub fn omap_vrfb_min_phys_size(width: u16, height: u16, bytespp: u8) -> u32;
}
extern "C" {
    pub fn omap_vrfb_max_height(phys_size: u32, width: u16, bytespp: u8) -> u16;
}
extern "C" {
    pub fn omap_vrfb_map_angle(vrfb: *mut vrfb, height: u16, rot: u8) -> c_int;
}
extern "C" {
    pub fn omap_vrfb_restore_context();
}

