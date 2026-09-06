//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/dma-swiotlb.c
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
// Contains routines needed to support swiotlb for ppc.
//
// Copyright (C) 2009-2010 Freescale Semiconductor, Inc.
// Author: Becky Bruce
//

    unsigned int ppc_swiotlb_enable;
    unsigned int ppc_swiotlb_flags;
#[no_mangle]
pub unsafe extern "C" fn swiotlb_detect_4g() -> void __init {
    void __init swiotlb_detect_4g(void)
    {
    if ((memblock_end_of_DRAM() - 1) > 0xffffffff)
    ppc_swiotlb_enable = 1;
    }
#[no_mangle]
unsafe extern "C" fn check_swiotlb_enabled() -> int __init {
    static int __init check_swiotlb_enabled(void)
    {
    if (ppc_swiotlb_enable)
    swiotlb_print_info();
    else
    swiotlb_exit();
    return 0;
    }
    subsys_initcall(check_swiotlb_enabled);
