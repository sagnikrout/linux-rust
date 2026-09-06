//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/matrox/matroxfb_crtc2.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matroxfb_dh_fb_info {
    pub fbcon: fb_info,
    pub fbcon_registered: c_int,
    pub initialized: c_int,
    pub primary_dev: *mut *mut matrox_fb_info,
    pub /: *mut *mut unsigned long base; / physical,
    pub /: *mut *mut vaddr_t vbase; / virtual,
    pub len: c_uint,
    pub len_usable: c_uint,
    pub len_maximum: c_uint,
    pub offbase: c_uint,
    pub borrowed: c_uint,
    pub video: },
    pub base: c_ulong,
    pub vbase: vaddr_t,
    pub len: c_uint,
    pub mmio: },
    pub interlaced:1: c_uint,
    pub cmap: [u_int32_t; 16],
}
