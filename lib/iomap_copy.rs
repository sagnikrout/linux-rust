//! Automatically rewritten from C to Rust
//! Source: lib/iomap_copy.c
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
// Copyright 2006 PathScale, Inc.  All Rights Reserved.
//

//
// __iowrite32_copy - copy data to MMIO space, in 32-bit units
// @to: destination, in MMIO space (must be 32-bit aligned)
// @from: source (must be 32-bit aligned)
// @count: number of 32-bit quantities to copy
//
// Copy data from kernel space to MMIO space, in units of 32 bits at a
// time.  Order of access is not guaranteed, nor is a memory barrier
// performed afterwards.
//

#[no_mangle]
pub unsafe extern "C" fn __iowrite32_copy(to: *mut void __iomem, from: *const c_void, count: usize) {
    void __iowrite32_copy(void __iomem *to, const void *from, size_t count)
    {
    u32 __iomem *dst = to;
    const u32 *src = from;
    const u32 *end = src + count;
    while (src < end)
    __raw_writel(*src++, dst++);
    }
    EXPORT_SYMBOL_GPL(__iowrite32_copy);

//
// __ioread32_copy - copy data from MMIO space, in 32-bit units
// @to: destination (must be 32-bit aligned)
// @from: source, in MMIO space (must be 32-bit aligned)
// @count: number of 32-bit quantities to copy
//
// Copy data from MMIO space to kernel space, in units of 32 bits at a
// time.  Order of access is not guaranteed, nor is a memory barrier
// performed afterwards.
//
#[no_mangle]
pub unsafe extern "C" fn __ioread32_copy(to: *mut c_void, from: *const void __iomem, count: usize) {
    void __ioread32_copy(void *to, const void __iomem *from, size_t count)
    {
    u32 *dst = to;
    const u32 __iomem *src = from;
    const u32 __iomem *end = src + count;
    while (src < end)
// dst++ = __raw_readl(src++);
    }
    EXPORT_SYMBOL_GPL(__ioread32_copy);
//
// __iowrite64_copy - copy data to MMIO space, in 64-bit or 32-bit units
// @to: destination, in MMIO space (must be 64-bit aligned)
// @from: source (must be 64-bit aligned)
// @count: number of 64-bit quantities to copy
//
// Copy data from kernel space to MMIO space, in units of 32 or 64 bits at a
// time.  Order of access is not guaranteed, nor is a memory barrier
// performed afterwards.
//

#[no_mangle]
pub unsafe extern "C" fn __iowrite64_copy(to: *mut void __iomem, from: *const c_void, count: usize) {
    void __iowrite64_copy(void __iomem *to, const void *from, size_t count)
    {

    u64 __iomem *dst = to;
    const u64 *src = from;
    const u64 *end = src + count;
    while (src < end)
    __raw_writeq(*src++, dst++);

    __iowrite32_copy(to, from, count * 2);

    }
    EXPORT_SYMBOL_GPL(__iowrite64_copy);
