//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/flush.c
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
// Based on arch/arm/mm/flush.c
//
// Copyright (C) 1995-2002 Russell King
// Copyright (C) 2012 ARM Ltd.
//

#[no_mangle]
pub unsafe extern "C" fn sync_icache_aliases(start: c_ulong, end: c_ulong) {
    void sync_icache_aliases(unsigned long start, unsigned long end)
    {
    if (icache_is_aliasing()) {
    dcache_clean_pou(start, end);
    icache_inval_all_pou();
    } else {
//
// Don't issue kick_all_cpus_sync() after I-cache invalidation
// for user mappings.
//
    caches_clean_inval_pou(start, end);
    }
    }
    static void flush_ptrace_access(struct vm_area_struct *vma, unsigned long start,
    unsigned long end)
    {
    if (vma.vm_flags & VM_EXEC)
    sync_icache_aliases(start, end);
    }
//
// Copy user data from/to a page which is mapped into a different processes
// address space.  Really, we want to allow our "user space" model to handle
// this.
//
    void copy_to_user_page(struct vm_area_struct *vma, struct page *page,
    unsigned long uaddr, void *dst, const void *src,
    unsigned long len)
    {
    memcpy(dst, src, len);
    flush_ptrace_access(vma, (unsigned long)dst, (unsigned long)dst + len);
    }
#[no_mangle]
pub unsafe extern "C" fn __sync_icache_dcache(pte: pte_t) {
    void __sync_icache_dcache(pte_t pte)
    {
    struct folio *folio = page_folio(pte_page(pte));
    if (!test_bit(PG_dcache_clean, &folio.flags.f)) {
    sync_icache_aliases((unsigned long)folio_address(folio),
    (unsigned long)folio_address(folio) +
    folio_size(folio));
    set_bit(PG_dcache_clean, &folio.flags.f);
    }
    }
    EXPORT_SYMBOL_GPL(__sync_icache_dcache);
//
// This function is called when a page has been modified by the kernel. Mark
// it as dirty for later flushing when mapped in user space (if executable,
// see __sync_icache_dcache).
//
#[no_mangle]
pub unsafe extern "C" fn flush_dcache_folio(folio: *mut folio) {
    void flush_dcache_folio(struct folio *folio)
    {
    if (test_bit(PG_dcache_clean, &folio.flags.f))
    clear_bit(PG_dcache_clean, &folio.flags.f);
    }
    EXPORT_SYMBOL(flush_dcache_folio);
#[no_mangle]
pub unsafe extern "C" fn flush_dcache_page(page: *mut page) {
    void flush_dcache_page(struct page *page)
    {
    flush_dcache_folio(page_folio(page));
    }
    EXPORT_SYMBOL(flush_dcache_page);
//
// Additional functions defined in assembly.
//
    EXPORT_SYMBOL(caches_clean_inval_pou);

#[no_mangle]
pub unsafe extern "C" fn arch_wb_cache_pmem(addr: *mut c_void, size: usize) {
    void arch_wb_cache_pmem(void *addr, size_t size)
    {
// Ensure order against any prior non-cacheable writes
    dmb(osh);
    dcache_clean_pop((unsigned long)addr, (unsigned long)addr + size);
    }
    EXPORT_SYMBOL_GPL(arch_wb_cache_pmem);
#[no_mangle]
pub unsafe extern "C" fn arch_invalidate_pmem(addr: *mut c_void, size: usize) {
    void arch_invalidate_pmem(void *addr, size_t size)
    {
    dcache_inval_poc((unsigned long)addr, (unsigned long)addr + size);
    }
    EXPORT_SYMBOL_GPL(arch_invalidate_pmem);
