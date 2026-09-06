//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fb_sys_fops.c
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
// linux/drivers/video/fb_sys_read.c - Generic file operations where
// framebuffer is in system RAM
//
// Copyright (C) 2007 Antonino Daplas <adaplas@pol.net>
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

    ssize_t fb_sys_read(struct fb_info *info, char __user *buf, size_t count,
    loff_t *ppos)
    {
    let mut p: c_ulong = *ppos;
    void *src;
    let mut err: c_int = 0;
    unsigned long total_size, c;
    ssize_t ret;
    if (!(info.flags & FBINFO_VIRTFB))
    fb_warn_once(info, "Framebuffer is not in virtual address space.");
    if (!info.screen_buffer)
    return -ENODEV;
    total_size = info.screen_size;
    if (total_size == 0)
    total_size = info.fix.smem_len;
//
// Security Hardening: Defend against buggy legacy drivers that may
// calculate a malformed screen_size. Clamp total_size to the actual
// hardware mapped memory limit (smem_len) to prevent OOB access.
//
    if (info.fix.smem_len && total_size > info.fix.smem_len)
    total_size = info.fix.smem_len;
    if (p >= total_size)
    return 0;
    if (count >= total_size)
    count = total_size;
    if (count + p > total_size)
    count = total_size - p;
    src = info.screen_buffer + p;
    if (info.fbops.fb_sync)
    info.fbops.fb_sync(info);
    c = copy_to_user(buf, src, count);
    if (c)
    err = -EFAULT;
    ret = count - c;
// ppos += ret;
    return ret ? ret : err;
    }
    EXPORT_SYMBOL_GPL(fb_sys_read);
    ssize_t fb_sys_write(struct fb_info *info, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    let mut p: c_ulong = *ppos;
    void *dst;
    let mut err: c_int = 0;
    unsigned long total_size, c;
    size_t ret;
    if (!(info.flags & FBINFO_VIRTFB))
    fb_warn_once(info, "Framebuffer is not in virtual address space.");
    if (!info.screen_buffer)
    return -ENODEV;
    total_size = info.screen_size;
    if (total_size == 0)
    total_size = info.fix.smem_len;
//
// Security Hardening: Defend against buggy legacy drivers that may
// calculate a malformed screen_size. Clamp total_size to the actual
// hardware mapped memory limit (smem_len) to prevent OOB access.
//
    if (info.fix.smem_len && total_size > info.fix.smem_len)
    total_size = info.fix.smem_len;
    if (p > total_size)
    return -EFBIG;
    if (count > total_size) {
    err = -EFBIG;
    count = total_size;
    }
    if (count + p > total_size) {
    if (!err)
    err = -ENOSPC;
    count = total_size - p;
    }
    dst = info.screen_buffer + p;
    if (info.fbops.fb_sync)
    info.fbops.fb_sync(info);
    c = copy_from_user(dst, buf, count);
    if (c)
    err = -EFAULT;
    ret = count - c;
// ppos += ret;
    return ret ? ret : err;
    }
    EXPORT_SYMBOL_GPL(fb_sys_write);
    MODULE_AUTHOR("Antonino Daplas <adaplas@pol.net>");
    MODULE_DESCRIPTION("Generic file read (fb in system RAM)");
    MODULE_LICENSE("GPL");
