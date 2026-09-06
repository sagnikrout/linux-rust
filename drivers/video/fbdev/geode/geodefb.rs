//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/geode/geodefb.h
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
// drivers/video/geode/geodefb.h
// -- Geode framebuffer driver
//
// Copyright (C) 2005 Arcom Control Systems Ltd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geode_dc_ops {
    pub ): *mut *mut void (set_mode)(struct fb_info,
    pub unsigned): *mut *mut *mut void (set_palette_reg)(struct fb_info , unsigned, unsigned, unsigned,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geode_vid_ops {
    pub ): *mut *mut void (set_dclk)(struct fb_info,
    pub ): *mut *mut void (configure_display)(struct fb_info,
    pub blank_mode): *mut *mut *mut int (blank_display)(struct fb_info , int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct geodefb_par {
    pub enable_crt: c_int,
    pub /: *mut *mut int panel_x; / dimensions of an attached flat panel, non-zero => enable panel,
    pub panel_y: c_int,
    pub dc_regs: *mut void __iomem,
    pub vid_regs: *mut void __iomem,
    pub dc_ops: *const geode_dc_ops,
    pub vid_ops: *const geode_vid_ops,
}
