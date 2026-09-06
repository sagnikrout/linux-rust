//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fb_info.c
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
// framebuffer_alloc - creates a new frame buffer info structure
//
// @size: size of driver private data, can be zero
// @dev: pointer to the device for this fb, this can be NULL
//
// Creates a new frame buffer info structure. Also reserves @size bytes
// for driver private data (info->par). info->par (if any) will be
// aligned to sizeof(long). The new instances of struct fb_info and
// the driver private data are both cleared to zero.
//
// Returns the new structure, or NULL if an error occurred.
//
    struct fb_info *framebuffer_alloc(size_t size, struct device *dev)
    {

    let mut fb_info_size: c_int = sizeof(struct fb_info);
    struct fb_info *info;
    char *p;
    if (size)
    fb_info_size += PADDING;
    p = kzalloc(fb_info_size + size, GFP_KERNEL);
    if (!p)
    return core::ptr::null_mut();
    info = (struct fb_info *) p;
    if (size)
    info.par = p + fb_info_size;
    info.device = dev;
    info.fbcon_rotate_hint = -1;
    info.blank = FB_BLANK_UNBLANK;

    mutex_init(&info.bl_curve_mutex);

    return info;

    }
    EXPORT_SYMBOL(framebuffer_alloc);
//
// framebuffer_release - marks the structure available for freeing
//
// @info: frame buffer info structure
//
// Drop the reference count of the device embedded in the
// framebuffer info structure.
//
#[no_mangle]
pub unsafe extern "C" fn framebuffer_release(info: *mut fb_info) {
    void framebuffer_release(struct fb_info *info)
    {
    if (!info)
    return;
    if (WARN_ON(refcount_read(&info.count)))
    return;

    mutex_destroy(&info.bl_curve_mutex);

    kfree(info);
    }
    EXPORT_SYMBOL(framebuffer_release);
