//! Automatically rewritten from C to Rust
//! Source: lib/iomem_copy.c
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
// Copyright 2024 Kalray, Inc.  All Rights Reserved.
//

//
// memset_io() - Set a range of I/O memory to a constant value
// @addr: The beginning of the I/O-memory range to set
// @val: The value to set the memory to
// @count: The number of bytes to set
//
// Set a range of I/O memory to a given value.
//
#[no_mangle]
pub unsafe extern "C" fn memset_io(addr: *mut volatile void __iomem, val: c_int, count: usize) {
    void memset_io(volatile void __iomem *addr, int val, size_t count)
    {
    let mut qc: c_long = (u8)val;
    qc *= ~0UL / 0xff;
    while (count && !IS_ALIGNED((long)addr, sizeof(long))) {
    __raw_writeb(val, addr);
    addr++;
    count--;
    }
    while (count >= sizeof(long)) {

    __raw_writeq(qc, addr);

    __raw_writel(qc, addr);

    addr += sizeof(long);
    count -= sizeof(long);
    }
    while (count) {
    __raw_writeb(val, addr);
    addr++;
    count--;
    }
    }
    EXPORT_SYMBOL(memset_io);

//
// memcpy_fromio() - Copy a block of data from I/O memory
// @dst: The (RAM) destination for the copy
// @src: The (I/O memory) source for the data
// @count: The number of bytes to copy
//
// Copy a block of data from I/O memory.
//
#[no_mangle]
pub unsafe extern "C" fn memcpy_fromio(dst: *mut c_void, src: *const volatile void __iomem, count: usize) {
    void memcpy_fromio(void *dst, const volatile void __iomem *src, size_t count)
    {
    while (count && !IS_ALIGNED((long)src, sizeof(long))) {
// (u8 *)dst = __raw_readb(src);
    src++;
    dst++;
    count--;
    }
    while (count >= sizeof(long)) {

    let mut val: c_long = __raw_readq(src);

    let mut val: c_long = __raw_readl(src);

    put_unaligned(val, (long *)dst);
    src += sizeof(long);
    dst += sizeof(long);
    count -= sizeof(long);
    }
    while (count) {
// (u8 *)dst = __raw_readb(src);
    src++;
    dst++;
    count--;
    }
    }
    EXPORT_SYMBOL(memcpy_fromio);

//
// memcpy_toio() -Copy a block of data into I/O memory
// @dst: The (I/O memory) destination for the copy
// @src: The (RAM) source for the data
// @count: The number of bytes to copy
//
// Copy a block of data to I/O memory.
//
#[no_mangle]
pub unsafe extern "C" fn memcpy_toio(dst: *mut volatile void __iomem, src: *const c_void, count: usize) {
    void memcpy_toio(volatile void __iomem *dst, const void *src, size_t count)
    {
    while (count && !IS_ALIGNED((long)dst, sizeof(long))) {
    __raw_writeb(*(u8 *)src, dst);
    src++;
    dst++;
    count--;
    }
    while (count >= sizeof(long)) {
    let mut val: c_long = get_unaligned((long *)src);

    __raw_writeq(val, dst);

    __raw_writel(val, dst);

    src += sizeof(long);
    dst += sizeof(long);
    count -= sizeof(long);
    }
    while (count) {
    __raw_writeb(*(u8 *)src, dst);
    src++;
    dst++;
    count--;
    }
    }
    EXPORT_SYMBOL(memcpy_toio);
