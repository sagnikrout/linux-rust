//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/display/komeda/komeda_framebuffer.h
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
// (C) COPYRIGHT 2018 ARM Limited. All rights reserved.
// Author: James.Qian.Wang <james.qian.wang@arm.com>
//

//
// struct komeda_fb - Entending drm_framebuffer with komeda attribute
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_fb {
// @base: &drm_framebuffer
    pub base: drm_framebuffer,
//
// @format_caps:
// extends drm_format_info for komeda specific information
//
    pub format_caps: *const komeda_format_caps,
// @is_va: if smmu is enabled, it will be true
    pub is_va: bool,
// @aligned_w: aligned frame buffer width
    pub aligned_w: u32,
// @aligned_h: aligned frame buffer height
    pub aligned_h: u32,
// @afbc_size: minimum size of afbc
    pub afbc_size: u32,
// @offset_payload: start of afbc body buffer
    pub offset_payload: u32,
}

