//! Automatically rewritten from C to Rust
//! Source: sound/core/memory.c
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//
// Misc memory accessors
//

//
// copy_to_user_fromio - copy data from mmio-space to user-space
// @dst: the destination pointer on user-space
// @src: the source pointer on mmio
// @count: the data size to copy in bytes
//
// Copies the data from mmio-space to user-space.
//
// Return: Zero if successful, or non-zero on failure.
//
#[no_mangle]
pub unsafe extern "C" fn copy_to_user_fromio(dst: *mut void __user, src: *const volatile void __iomem, count: usize) -> c_int {
    int copy_to_user_fromio(void __user *dst, const volatile void __iomem *src, size_t count)
    {
    struct iov_iter iter;
    if (import_ubuf(ITER_DEST, dst, count, &iter))
    return -EFAULT;
    if (copy_to_iter_fromio((const void __iomem *)src, count, &iter) != count)
    return -EFAULT;
    return 0;
    }
    EXPORT_SYMBOL(copy_to_user_fromio);
//
// copy_to_iter_fromio - copy data from mmio-space to iov_iter
// @src: the source pointer on mmio
// @count: the data size to copy in bytes
// @dst: the destination iov_iter
//
// Copies the data from mmio-space to iov_iter.
//
// Return: number of bytes to be copied
//
    size_t copy_to_iter_fromio(const void __iomem *src, size_t count,
    struct iov_iter *dst)
    {

    return copy_to_iter((const void  *)src, count, dst);

    char buf[256];
    let mut res: usize = 0;
    while (count) {
    let mut c: usize = count;
    if (c > sizeof(buf))
    c = sizeof(buf);
    memcpy_fromio(buf, (void __iomem *)src, c);
    if (copy_to_iter(buf, c, dst) != c)
    return res;
    count -= c;
    src += c;
    res += c;
    }
    return res;

    }
    EXPORT_SYMBOL(copy_to_iter_fromio);
//
// copy_from_user_toio - copy data from user-space to mmio-space
// @dst: the destination pointer on mmio-space
// @src: the source pointer on user-space
// @count: the data size to copy in bytes
//
// Copies the data from user-space to mmio-space.
//
// Return: Zero if successful, or non-zero on failure.
//
#[no_mangle]
pub unsafe extern "C" fn copy_from_user_toio(dst: *mut volatile void __iomem, src: *const void __user, count: usize) -> c_int {
    int copy_from_user_toio(volatile void __iomem *dst, const void __user *src, size_t count)
    {
    struct iov_iter iter;
    if (import_ubuf(ITER_SOURCE, (void __user *)src, count, &iter))
    return -EFAULT;
    if (copy_from_iter_toio((void __iomem *)dst, count, &iter) != count)
    return -EFAULT;
    return 0;
    }
    EXPORT_SYMBOL(copy_from_user_toio);
//
// copy_from_iter_toio - copy data from iov_iter to mmio-space
// @dst: the destination pointer on mmio-space
// @count: the data size to copy in bytes
// @src: the source iov_iter
//
// Copies the data from iov_iter to mmio-space.
//
// Return: number of bytes to be copied
//
    size_t copy_from_iter_toio(void __iomem *dst, size_t count,
    struct iov_iter *src)
    {

    return copy_from_iter((void  *)dst, count, src);

    char buf[256];
    let mut res: usize = 0;
    while (count) {
    let mut c: usize = count;
    if (c > sizeof(buf))
    c = sizeof(buf);
    if (copy_from_iter(buf, c, src) != c)
    return res;
    memcpy_toio(dst, buf, c);
    count -= c;
    dst += c;
    res += c;
    }
    return res;

    }
    EXPORT_SYMBOL(copy_from_iter_toio);
