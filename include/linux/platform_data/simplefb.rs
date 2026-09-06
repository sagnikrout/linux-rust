//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/simplefb.h
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
// simplefb.h - Simple Framebuffer Device
//
// Copyright (C) 2013 David Herrmann <dh.herrmann@gmail.com>
//

// format array, use it to initialize a "struct simplefb_format" array

//
// Data-Format for Simple-Framebuffers
// @name: unique 0-terminated name that can be used to identify the mode
// @red,green,blue: Offsets and sizes of the single RGB parts
// @transp: Offset and size of the alpha bits. length=0 means no alpha
// @fourcc: 32bit DRM four-CC code (see drm_fourcc.h)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simplefb_format {
    pub name: *const c_char,
    pub bits_per_pixel: u32,
    pub red: fb_bitfield,
    pub green: fb_bitfield,
    pub blue: fb_bitfield,
    pub transp: fb_bitfield,
    pub fourcc: u32,
}

//
// Simple-Framebuffer description
// If the arch-boot code creates simple-framebuffers without DT support, it
// can pass the width, height, stride and format via this platform-data object.
// The framebuffer location must be given as IORESOURCE_MEM resource.
// @format must be a format as described in "struct simplefb_format" above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simplefb_platform_data {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: *const c_char,
}
