//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/core/fb_io_fops.c
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

#[no_mangle]
pub unsafe extern "C" fn fb_io_read(info: *mut fb_info, buf: *mut char __user, count: usize, ppos: *mut loff_t) -> isize {
    ssize_t fb_io_read(struct fb_info *info, char __user *buf, size_t count, loff_t *ppos)
    {
    let mut p: c_ulong = *ppos;
    u8 *buffer, *dst;
    u8 __iomem *src;
    int c, cnt = 0, err = 0;
    unsigned long total_size, trailing;
    if (info.flags & FBINFO_VIRTFB)
    fb_warn_once(info, "Framebuffer is not in I/O address space.");
    if (!info.screen_base)
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
    buffer = kmalloc((count > PAGE_SIZE) ? PAGE_SIZE : count,
    GFP_KERNEL);
    if (!buffer)
    return -ENOMEM;
    src = (u8 __iomem *) (info.screen_base + p);
    if (info.fbops.fb_sync)
    info.fbops.fb_sync(info);
    while (count) {
    c  = (count > PAGE_SIZE) ? PAGE_SIZE : count;
    dst = buffer;
    fb_memcpy_fromio(dst, src, c);
    dst += c;
    src += c;
    trailing = copy_to_user(buf, buffer, c);
    if (trailing == c) {
    err = -EFAULT;
    break;
    }
    c -= trailing;
// ppos += c;
    buf += c;
    cnt += c;
    count -= c;
//
// If there was a partial copy, the user buffer is faulty.
// Break out to avoid over-advancing the src pointer and
// reading out of bounds in the next iteration.
//
    if (trailing)
    break;
    }
    kfree(buffer);
    return cnt ? cnt : err;
    }
    EXPORT_SYMBOL(fb_io_read);
#[no_mangle]
pub unsafe extern "C" fn fb_io_write(info: *mut fb_info, buf: *const char __user, count: usize, ppos: *mut loff_t) -> isize {
    ssize_t fb_io_write(struct fb_info *info, const char __user *buf, size_t count, loff_t *ppos)
    {
    let mut p: c_ulong = *ppos;
    u8 *buffer, *src;
    u8 __iomem *dst;
    int c, cnt = 0, err = 0;
    unsigned long total_size, trailing;
    if (info.flags & FBINFO_VIRTFB)
    fb_warn_once(info, "Framebuffer is not in I/O address space.");
    if (!info.screen_base)
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
    buffer = kmalloc((count > PAGE_SIZE) ? PAGE_SIZE : count,
    GFP_KERNEL);
    if (!buffer)
    return -ENOMEM;
    dst = (u8 __iomem *) (info.screen_base + p);
    if (info.fbops.fb_sync)
    info.fbops.fb_sync(info);
    while (count) {
    c = (count > PAGE_SIZE) ? PAGE_SIZE : count;
    src = buffer;
    trailing = copy_from_user(src, buf, c);
    if (trailing == c) {
    err = -EFAULT;
    break;
    }
    c -= trailing;
    fb_memcpy_toio(dst, src, c);
    dst += c;
    src += c;
// ppos += c;
    buf += c;
    cnt += c;
    count -= c;
    }
    kfree(buffer);
    return (cnt) ? cnt : err;
    }
    EXPORT_SYMBOL(fb_io_write);
#[no_mangle]
pub unsafe extern "C" fn fb_io_mmap(info: *mut fb_info, vma: *mut vm_area_struct) -> c_int {
    int fb_io_mmap(struct fb_info *info, struct vm_area_struct *vma)
    {
    let mut start: c_ulong = info.fix.smem_start;
    let mut len: u32 = info.fix.smem_len;
    let mut mmio_pgoff: c_ulong = PAGE_ALIGN((start & ~PAGE_MASK) + len) >> PAGE_SHIFT;
    if (info.flags & FBINFO_VIRTFB)
    fb_warn_once(info, "Framebuffer is not in I/O address space.");
//
// This can be either the framebuffer mapping, or if pgoff points
// past it, the mmio mapping.
//
    if (vma.vm_pgoff >= mmio_pgoff) {
    if (info.var.accel_flags)
    return -EINVAL;
    vma.vm_pgoff -= mmio_pgoff;
    start = info.fix.mmio_start;
    len = info.fix.mmio_len;
    }
    vma.vm_page_prot = vma_get_page_prot(vma);
    vma.vm_page_prot = pgprot_framebuffer(vma.vm_page_prot, vma.vm_start,
    vma.vm_end, start);
    return vm_iomap_memory(vma, start, len);
    }
    EXPORT_SYMBOL(fb_io_mmap);
    MODULE_DESCRIPTION("Fbdev helpers for framebuffers in I/O memory");
    MODULE_LICENSE("GPL");
