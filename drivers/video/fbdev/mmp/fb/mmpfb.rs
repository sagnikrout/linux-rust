//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/mmp/fb/mmpfb.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// linux/drivers/video/mmp/fb/mmpfb.h
// Framebuffer driver for Marvell Display controller.
//
// Copyright (C) 2012 Marvell Technology Group Ltd.
// Authors: Zhou Zhu <zzhu3@marvell.com>
//

// LCD controller private state.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmpfb_info {
    pub dev: *mut device,
    pub id: c_int,
    pub name: *const c_char,
    pub fb_info: *mut fb_info,
// basicaly videomode is for output
    pub mode: fb_videomode,
    pub pix_fmt: c_int,
    pub fb_start: *mut c_void,
    pub fb_size: c_int,
    pub fb_start_dma: dma_addr_t,
    pub overlay: *mut mmp_overlay,
    pub path: *mut mmp_path,
    pub access_ok: mutex,
    pub pseudo_palette: [c_uint; 16],
    pub output_fmt: c_int,
}

