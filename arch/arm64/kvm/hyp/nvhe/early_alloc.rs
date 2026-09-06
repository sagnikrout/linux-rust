//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/hyp/nvhe/early_alloc.c
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
// Copyright (C) 2020 Google LLC
// Author: Quentin Perret <qperret@google.com>
//

    struct kvm_pgtable_mm_ops hyp_early_alloc_mm_ops;
    s64 __ro_after_init hyp_physvirt_offset;
    static unsigned long base;
    static unsigned long end;
    static unsigned long cur;
#[no_mangle]
pub unsafe extern "C" fn hyp_early_alloc_nr_used_pages() -> c_ulong {
    unsigned long hyp_early_alloc_nr_used_pages(void)
    {
    return (cur - base) >> PAGE_SHIFT;
    }
    void *hyp_early_alloc_contig(unsigned int nr_pages)
    {
    let mut size: c_ulong = (nr_pages << PAGE_SHIFT);
    void *ret = (void *)cur;
    if (!nr_pages)
    return core::ptr::null_mut();
    if (end - cur < size)
    return core::ptr::null_mut();
    cur += size;
    memset(ret, 0, size);
    return ret;
    }
    void *hyp_early_alloc_page(void *arg)
    {
    return hyp_early_alloc_contig(1);
    }
    static void hyp_early_alloc_get_page(void *addr) { }
    static void hyp_early_alloc_put_page(void *addr) { }
#[no_mangle]
pub unsafe extern "C" fn hyp_early_alloc_init(virt: *mut c_void, size: c_ulong) {
    void hyp_early_alloc_init(void *virt, unsigned long size)
    {
    base = cur = (unsigned long)virt;
    end = base + size;
    hyp_early_alloc_mm_ops.zalloc_page = hyp_early_alloc_page;
    hyp_early_alloc_mm_ops.phys_to_virt = hyp_phys_to_virt;
    hyp_early_alloc_mm_ops.virt_to_phys = hyp_virt_to_phys;
    hyp_early_alloc_mm_ops.get_page = hyp_early_alloc_get_page;
    hyp_early_alloc_mm_ops.put_page = hyp_early_alloc_put_page;
    }
