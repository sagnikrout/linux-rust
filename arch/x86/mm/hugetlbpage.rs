//! Automatically rewritten from C to Rust
//! Source: arch/x86/mm/hugetlbpage.c
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
// IA-32 Huge TLB Page Support for Kernel.
//
// Copyright (C) 2002, Rohit Seth <rohit.seth@intel.com>
//

#[no_mangle]
pub unsafe extern "C" fn arch_hugetlb_valid_size(size: c_ulong) -> bool __init {
    bool __init arch_hugetlb_valid_size(unsigned long size)
    {
    if (size == PMD_SIZE)
    return true;
#[no_mangle]
pub unsafe extern "C" fn if(boot_cpu_has(X86_FEATURE_GBPAGES): size == PUD_SIZE &&) -> else {
    else if (size == PUD_SIZE && boot_cpu_has(X86_FEATURE_GBPAGES))
    return true;
    else
    return false;
    }

#[no_mangle]
unsafe extern "C" fn gigantic_pages_init() -> __init int {
    static __init int gigantic_pages_init(void)
    {
// With compaction or CMA we can allocate gigantic pages at runtime
    if (boot_cpu_has(X86_FEATURE_GBPAGES))
    hugetlb_add_hstate(PUD_SHIFT - PAGE_SHIFT);
    return 0;
    }
    arch_initcall(gigantic_pages_init);

#[no_mangle]
pub unsafe extern "C" fn arch_hugetlb_cma_order() -> unsigned int __init {
    unsigned int __init arch_hugetlb_cma_order(void)
    {
    if (boot_cpu_has(X86_FEATURE_GBPAGES))
    return PUD_SHIFT - PAGE_SHIFT;
    return 0;
    }
