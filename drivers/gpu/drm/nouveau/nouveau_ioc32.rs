//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nouveau_ioc32.c
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
// \file mga_ioc32.c
//
// 32-bit ioctl compatibility routines for the MGA DRM.
//
// \author Dave Airlie <airlied@linux.ie> with code from patches by Egbert Eich
//
// Copyright (C) Paul Mackerras 2005
// Copyright (C) Egbert Eich 2003,2004
// Copyright (C) Dave Airlie 2005
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHOR BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

//
// Called whenever a 32-bit process running under a 64-bit kernel
// performs an ioctl on /dev/dri/card<n>.
//
// \param filp file pointer.
// \param cmd command.
// \param arg user argument.
// \return zero on success or negative number on failure.
//
    long nouveau_compat_ioctl(struct file *filp, unsigned int cmd,
    unsigned long arg)
    {
    let mut nr: c_uint = DRM_IOCTL_NR(cmd);
    drm_ioctl_compat_t *fn = core::ptr::null_mut();
    int ret;
    if (nr < DRM_COMMAND_BASE)
    return drm_compat_ioctl(filp, cmd, arg);

    if (nr < DRM_COMMAND_BASE + ARRAY_SIZE(mga_compat_ioctls))
    fn = nouveau_compat_ioctls[nr - DRM_COMMAND_BASE];

    if (fn != core::ptr::null_mut())
    ret = (*fn)(filp, cmd, arg);
    else
    ret = nouveau_drm_ioctl(filp, cmd, arg);
    return ret;
    }
