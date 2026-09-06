//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/pgd.c
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
// PGD allocation/freeing
//
// Copyright (C) 2012 ARM Ltd.
// Author: Catalin Marinas <catalin.marinas@arm.com>
//

    static struct kmem_cache *pgd_cache __ro_after_init;
#[no_mangle]
unsafe extern "C" fn pgdir_is_page_size() -> bool {
    static bool pgdir_is_page_size(void)
    {
    if (PGD_SIZE == PAGE_SIZE)
    return true;
    if (CONFIG_PGTABLE_LEVELS == 4)
    return !pgtable_l4_enabled();
    if (CONFIG_PGTABLE_LEVELS == 5)
    return !pgtable_l5_enabled();
    return false;
    }
    pgd_t *pgd_alloc(struct mm_struct *mm)
    {
    let mut gfp: gfp_t = GFP_PGTABLE_USER;
    if (pgdir_is_page_size())
    return __pgd_alloc(mm, 0);
    else
    return kmem_cache_alloc(pgd_cache, gfp);
    }
#[no_mangle]
pub unsafe extern "C" fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) {
    void pgd_free(struct mm_struct *mm, pgd_t *pgd)
    {
    if (pgdir_is_page_size())
    __pgd_free(mm, pgd);
    else
    kmem_cache_free(pgd_cache, pgd);
    }
#[no_mangle]
pub unsafe extern "C" fn pgtable_cache_init() -> void __init {
    void __init pgtable_cache_init(void)
    {
    if (pgdir_is_page_size())
    return;

//
// With 52-bit physical addresses, the architecture requires the
// top-level table to be aligned to at least 64 bytes.
//
    BUILD_BUG_ON(!IS_ALIGNED(PGD_SIZE, 64));

//
// Naturally aligned pgds required by the architecture.
//
    pgd_cache = kmem_cache_create("pgd_cache", PGD_SIZE, PGD_SIZE,
    SLAB_PANIC, core::ptr::null_mut());
    }
