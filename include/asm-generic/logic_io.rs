//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/logic_io.h
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
// Copyright (C) 2021 Intel Corporation
// Author: johannes@sipsolutions.net
//

// include this file into asm/io.h

//
// If you want emulated IO memory to fall back to 'normal' IO memory
// if a region wasn't registered as emulated, then you need to have
// all of the real_* functions implemented.
//

extern "C" {
    pub fn iounmap(addr: *mut void volatile __iomem);
}

extern "C" {
    pub fn __raw_readb(addr: *const volatile void __iomem) -> u8;
}

extern "C" {
    pub fn __raw_readw(addr: *const volatile void __iomem) -> u16;
}

extern "C" {
    pub fn __raw_readl(addr: *const volatile void __iomem) -> u32;
}

extern "C" {
    pub fn __raw_readq(addr: *const volatile void __iomem) -> u64;
}

extern "C" {
    pub fn __raw_writeb(value: u8, addr: *mut volatile void __iomem);
}

extern "C" {
    pub fn __raw_writew(value: u16, addr: *mut volatile void __iomem);
}

extern "C" {
    pub fn __raw_writel(value: u32, addr: *mut volatile void __iomem);
}

extern "C" {
    pub fn __raw_writeq(value: u64, addr: *mut volatile void __iomem);
}

extern "C" {
    pub fn memset_io(addr: *mut volatile void __iomem, value: c_int, size: usize);
}

extern "C" {
    pub fn memcpy_toio(addr: *mut volatile void __iomem, buffer: *const c_void, size: usize);
}

