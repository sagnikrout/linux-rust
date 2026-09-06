//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/sunxi/sun8i-rotate/sun8i_formats.c
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
// Copyright (C) 2020 Jernej Skrabec <jernej.skrabec@siol.net>

//
// Formats not included in array:
// ROTATE_FORMAT_BGR565
// ROTATE_FORMAT_VYUV
//
    static const struct rotate_format rotate_formats[] = {
    {
    .fourcc = V4L2_PIX_FMT_ARGB32,
    .hw_format = ROTATE_FORMAT_ARGB32,
    .planes = 1,
    .bpp = { 4, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_ABGR32,
    .hw_format = ROTATE_FORMAT_ABGR32,
    .planes = 1,
    .bpp = { 4, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_RGBA32,
    .hw_format = ROTATE_FORMAT_RGBA32,
    .planes = 1,
    .bpp = { 4, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_BGRA32,
    .hw_format = ROTATE_FORMAT_BGRA32,
    .planes = 1,
    .bpp = { 4, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_XRGB32,
    .hw_format = ROTATE_FORMAT_XRGB32,
    .planes = 1,
    .bpp = { 4, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_XBGR32,
    .hw_format = ROTATE_FORMAT_XBGR32,
    .planes = 1,
    .bpp = { 4, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_RGB32,
    .hw_format = ROTATE_FORMAT_RGBX32,
    .planes = 1,
    .bpp = { 4, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_BGR32,
    .hw_format = ROTATE_FORMAT_BGRX32,
    .planes = 1,
    .bpp = { 4, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_RGB24,
    .hw_format = ROTATE_FORMAT_RGB24,
    .planes = 1,
    .bpp = { 3, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_BGR24,
    .hw_format = ROTATE_FORMAT_BGR24,
    .planes = 1,
    .bpp = { 3, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_RGB565,
    .hw_format = ROTATE_FORMAT_RGB565,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_ARGB444,
    .hw_format = ROTATE_FORMAT_ARGB4444,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_ABGR444,
    .hw_format = ROTATE_FORMAT_ABGR4444,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_RGBA444,
    .hw_format = ROTATE_FORMAT_RGBA4444,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_BGRA444,
    .hw_format = ROTATE_FORMAT_BGRA4444,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_ARGB555,
    .hw_format = ROTATE_FORMAT_ARGB1555,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_ABGR555,
    .hw_format = ROTATE_FORMAT_ABGR1555,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_RGBA555,
    .hw_format = ROTATE_FORMAT_RGBA5551,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_BGRA555,
    .hw_format = ROTATE_FORMAT_BGRA5551,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 1,
    .vsub = 1,
    .flags = ROTATE_FLAG_OUTPUT
    }, {
    .fourcc = V4L2_PIX_FMT_YVYU,
    .hw_format = ROTATE_FORMAT_YVYU,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 2,
    .vsub = 1,
    .flags = ROTATE_FLAG_YUV
    }, {
    .fourcc = V4L2_PIX_FMT_UYVY,
    .hw_format = ROTATE_FORMAT_UYVY,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 2,
    .vsub = 1,
    .flags = ROTATE_FLAG_YUV
    }, {
    .fourcc = V4L2_PIX_FMT_YUYV,
    .hw_format = ROTATE_FORMAT_YUYV,
    .planes = 1,
    .bpp = { 2, 0, 0 },
    .hsub = 2,
    .vsub = 1,
    .flags = ROTATE_FLAG_YUV
    }, {
    .fourcc = V4L2_PIX_FMT_NV61,
    .hw_format = ROTATE_FORMAT_NV61,
    .planes = 2,
    .bpp = { 1, 2, 0 },
    .hsub = 2,
    .vsub = 1,
    .flags = ROTATE_FLAG_YUV
    }, {
    .fourcc = V4L2_PIX_FMT_NV16,
    .hw_format = ROTATE_FORMAT_NV16,
    .planes = 2,
    .bpp = { 1, 2, 0 },
    .hsub = 2,
    .vsub = 1,
    .flags = ROTATE_FLAG_YUV
    }, {
    .fourcc = V4L2_PIX_FMT_YUV422P,
    .hw_format = ROTATE_FORMAT_YUV422P,
    .planes = 3,
    .bpp = { 1, 1, 1 },
    .hsub = 2,
    .vsub = 1,
    .flags = ROTATE_FLAG_YUV
    }, {
    .fourcc = V4L2_PIX_FMT_NV21,
    .hw_format = ROTATE_FORMAT_NV21,
    .planes = 2,
    .bpp = { 1, 2, 0 },
    .hsub = 2,
    .vsub = 2,
    .flags = ROTATE_FLAG_YUV
    }, {
    .fourcc = V4L2_PIX_FMT_NV12,
    .hw_format = ROTATE_FORMAT_NV12,
    .planes = 2,
    .bpp = { 1, 2, 0 },
    .hsub = 2,
    .vsub = 2,
    .flags = ROTATE_FLAG_YUV
    }, {
    .fourcc = V4L2_PIX_FMT_YUV420,
    .hw_format = ROTATE_FORMAT_YUV420P,
    .planes = 3,
    .bpp = { 1, 1, 1 },
    .hsub = 2,
    .vsub = 2,
    .flags = ROTATE_FLAG_YUV | ROTATE_FLAG_OUTPUT
    },
    };
    const struct rotate_format *rotate_find_format(u32 pixelformat)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(rotate_formats); i++)
    if (rotate_formats[i].fourcc == pixelformat)
    return &rotate_formats[i];
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn rotate_enum_fmt(f: *mut v4l2_fmtdesc, dst: bool) -> c_int {
    int rotate_enum_fmt(struct v4l2_fmtdesc *f, bool dst)
    {
    int i, index;
    index = 0;
    for (i = 0; i < ARRAY_SIZE(rotate_formats); i++) {
// not all formats can be used for capture buffers
    if (dst && !(rotate_formats[i].flags & ROTATE_FLAG_OUTPUT))
    continue;
    if (index == f.index) {
    f.pixelformat = rotate_formats[i].fourcc;
    return 0;
    }
    index++;
    }
    return -EINVAL;
    }
