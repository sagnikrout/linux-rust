//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fb_cmdline.c
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


//
// linux/drivers/video/fb_cmdline.c
//
// Copyright (C) 2014 Intel Corp
// Copyright (C) 1994 Martin Schaller
//
// 2001 - Documented with DocBook
// - Brad Douglas <brad@neruo.com>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//
// Authors:
// Daniel Vetter <daniel.vetter@ffwll.ch>
//

//
// fb_get_options - get kernel boot parameters
// @name:   framebuffer name as it would appear in
// the boot parameter line
// (video=<name>:<options>)
// @option: the option will be stored here
//
// The caller owns the string returned in @option and is
// responsible for releasing the memory.
//
// NOTE: Needed to maintain backwards compatibility
//
#[no_mangle]
pub unsafe extern "C" fn fb_get_options(name: *const c_char, option: *mut c_char) -> c_int {
    int fb_get_options(const char *name, char **option)
    {
    const char *options = core::ptr::null_mut();
    let mut is_of: bool = false;
    bool enabled;
    if (name)
    is_of = !strncmp(name, "offb", 4);
    enabled = __video_get_options(name, &options, is_of);
    if (options) {
    if (!strncmp(options, "off", 3))
    enabled = false;
    }
    if (option) {
    if (options)
// option = kstrdup(options, GFP_KERNEL);
    else
// option = NULL;
    }
    return enabled ? 0 : 1; // 0 on success, 1 otherwise
    }
    EXPORT_SYMBOL(fb_get_options);
