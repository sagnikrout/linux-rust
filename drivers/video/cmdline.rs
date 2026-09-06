//! Automatically rewritten from C to Rust
//! Source: drivers/video/cmdline.c
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
// Based on the fbdev code in drivers/video/fbdev/core/fb_cmdline:
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
// FB_MAX is the maximum number of framebuffer devices and also
// the maximum number of video= parameters. Although not directly
// related to each other, it makes sense to keep it that way.
//
    static const char *video_options[FB_MAX] __read_mostly;
    static const char *video_option __read_mostly;
    static int video_of_only __read_mostly;
    static const char *__video_get_option_string(const char *name)
    {
    const char *options = core::ptr::null_mut();
    let mut name_len: usize = 0;
    if (name)
    name_len = strlen(name);
    if (name_len) {
    unsigned int i;
    const char *opt;
    for (i = 0; i < ARRAY_SIZE(video_options); ++i) {
    if (!video_options[i])
    continue;
    if (video_options[i][0] == '\0')
    continue;
    opt = video_options[i];
    if (!strncmp(opt, name, name_len) && opt[name_len] == ':')
    options = opt + name_len + 1;
    }
    }
// No match, return global options
    if (!options)
    options = video_option;
    return options;
    }
//
// video_get_options - get kernel boot parameters
// @name:	name of the output as it would appear in the boot parameter
// line (video=<name>:<options>)
//
// Looks up the video= options for the given name. Names are connector
// names with DRM, or driver names with fbdev. If no video option for
// the name has been specified, the function returns the global video=
// setting. A @name of NULL always returns the global video setting.
//
// Returns:
// The string of video options for the given name, or NULL if no video
// option has been specified.
//
    const char *video_get_options(const char *name)
    {
    return __video_get_option_string(name);
    }
    EXPORT_SYMBOL(video_get_options);

#[no_mangle]
pub unsafe extern "C" fn __video_get_options(name: *const c_char, options: *const c_char, is_of: bool) -> bool {
    bool __video_get_options(const char *name, const char **options, bool is_of)
    {
    let mut enabled: bool = true;
    const char *opt = core::ptr::null_mut();
    if (video_of_only && !is_of)
    enabled = false;
    opt = __video_get_option_string(name);
    if (options)
// options = opt;
    return enabled;
    }
    EXPORT_SYMBOL(__video_get_options);

//
// Process command line options for video adapters. This function is
// a __setup and __init function. It only stores the options. Drivers
// have to call video_get_options() as necessary.
//
#[no_mangle]
unsafe extern "C" fn video_setup(options: *mut c_char) -> int __init {
    static int __init video_setup(char *options)
    {
    if (!options || !*options)
    goto out;
    if (!strncmp(options, "ofonly", 6)) {
    video_of_only = true;
    goto out;
    }
    if (strchr(options, ':')) {
// named
    size_t i;
    for (i = 0; i < ARRAY_SIZE(video_options); i++) {
    if (!video_options[i]) {
    video_options[i] = options;
    break;
    }
    }
    } else {
// global
    video_option = options;
    }
    out:
    return 1;
    }
    __setup("video=", video_setup);
