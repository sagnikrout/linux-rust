//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/mm/ioremap.c
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    void __init __iomem *early_ioremap(phys_addr_t phys_addr, unsigned long size)
    {
    return ((void __iomem *)TO_CACHE(phys_addr));
    }
#[no_mangle]
pub unsafe extern "C" fn early_iounmap(addr: *mut void __iomem, size: c_ulong) -> void __init {
    void __init early_iounmap(void __iomem *addr, unsigned long size)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn early_memremap_ro(phys_addr: resource_size_t, size: c_ulong) -> *mut void  __init {
    void * __init early_memremap_ro(resource_size_t phys_addr, unsigned long size)
    {
    return early_memremap(phys_addr, size);
    }
    void * __init early_memremap_prot(resource_size_t phys_addr, unsigned long size,
    unsigned long prot_val)
    {
    return early_memremap(phys_addr, size);
    }
