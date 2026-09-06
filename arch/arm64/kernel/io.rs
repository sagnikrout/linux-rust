//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/io.c
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
// Based on arch/arm/kernel/io.c
//
// Copyright (C) 2012 ARM Ltd.
//

//
// This generates a memcpy that works on a from/to address which is aligned to
// bits. Count is in terms of the number of bits sized quantities to copy. It
// optimizes to use the STR groupings when possible so that it is WC friendly.
//

    ({                                                                \
    volatile u##bits __iomem *_to = to;                       \
    const u##bits *_from = from;                              \
    size_t _count = count;                                    \
    const u##bits *_end_from = _from + ALIGN_DOWN(_count, 8); \
    \
    for (; _from < _end_from; _from += 8, _to += 8)           \
    __const_memcpy_toio_aligned##bits(_to, _from, 8); \
    if ((_count % 8) >= 4) {                                  \
    __const_memcpy_toio_aligned##bits(_to, _from, 4); \
    _from += 4;                                       \
    _to += 4;                                         \
    }                                                         \
    if ((_count % 4) >= 2) {                                  \
    __const_memcpy_toio_aligned##bits(_to, _from, 2); \
    _from += 2;                                       \
    _to += 2;                                         \
    }                                                         \
    if (_count % 2)                                           \
    __const_memcpy_toio_aligned##bits(_to, _from, 1); \
    })
#[no_mangle]
pub unsafe extern "C" fn __iowrite64_copy_full(to: *mut void __iomem, from: *const c_void, count: usize) {
    void __iowrite64_copy_full(void __iomem *to, const void *from, size_t count)
    {
    memcpy_toio_aligned(to, from, count, 64);
    dgh();
    }
    EXPORT_SYMBOL(__iowrite64_copy_full);
#[no_mangle]
pub unsafe extern "C" fn __iowrite32_copy_full(to: *mut void __iomem, from: *const c_void, count: usize) {
    void __iowrite32_copy_full(void __iomem *to, const void *from, size_t count)
    {
    memcpy_toio_aligned(to, from, count, 32);
    dgh();
    }
    EXPORT_SYMBOL(__iowrite32_copy_full);
