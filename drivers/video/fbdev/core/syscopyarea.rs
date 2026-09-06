//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/syscopyarea.c
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
// Copyright (C)  2025 Zsolt Kajtar (soci@c64.rulez.org)
//

// Macro flag: #define FB_REV_PIXELS_IN_BYTE

#[no_mangle]
pub unsafe extern "C" fn sys_copyarea(p: *mut fb_info, area: *const fb_copyarea) {
    void sys_copyarea(struct fb_info *p, const struct fb_copyarea *area)
    {
    if (!(p.flags & FBINFO_VIRTFB))
    fb_warn_once(p, "%s: framebuffer is not in virtual address space.\n", __func__);
    fb_copyarea(p, area);
    }
    EXPORT_SYMBOL(sys_copyarea);
    MODULE_AUTHOR("Zsolt Kajtar <soci@c64.rulez.org>");
    MODULE_DESCRIPTION("Virtual memory packed pixel framebuffer area copy");
    MODULE_LICENSE("GPL");
