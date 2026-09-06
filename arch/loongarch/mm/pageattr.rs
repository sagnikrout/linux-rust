//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/mm/pageattr.c
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
// Copyright (C) 2024 Loongson Technology Corporation Limited
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pageattr_masks {
    pub set_mask: pgprot_t,
    pub clear_mask: pgprot_t,
}

#[no_mangle]
unsafe extern "C" fn set_pageattr_masks(val: c_ulong, walk: *mut mm_walk) -> c_ulong {
    static unsigned long set_pageattr_masks(unsigned long val, struct mm_walk *walk)
    {
    let mut new_val: c_ulong = val;
    struct pageattr_masks *masks = walk.private;
    new_val &= ~(pgprot_val(masks.clear_mask));
    new_val |= (pgprot_val(masks.set_mask));
    return new_val;
    }
    static int pageattr_pgd_entry(pgd_t *pgd, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    let mut val: pgd_t = pgdp_get(pgd);
    if (pgd_leaf(val)) {
    val = __pgd(set_pageattr_masks(pgd_val(val), walk));
    set_pgd(pgd, val);
    }
    return 0;
    }
    static int pageattr_p4d_entry(p4d_t *p4d, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    let mut val: p4d_t = p4dp_get(p4d);
    if (p4d_leaf(val)) {
    val = __p4d(set_pageattr_masks(p4d_val(val), walk));
    set_p4d(p4d, val);
    }
    return 0;
    }
    static int pageattr_pud_entry(pud_t *pud, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    let mut val: pud_t = pudp_get(pud);
    if (pud_leaf(val)) {
    val = __pud(set_pageattr_masks(pud_val(val), walk));
    set_pud(pud, val);
    }
    return 0;
    }
    static int pageattr_pmd_entry(pmd_t *pmd, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    let mut val: pmd_t = pmdp_get(pmd);
    if (pmd_leaf(val)) {
    val = __pmd(set_pageattr_masks(pmd_val(val), walk));
    set_pmd(pmd, val);
    }
    return 0;
    }
    static int pageattr_pte_entry(pte_t *pte, unsigned long addr,
    unsigned long next, struct mm_walk *walk)
    {
    let mut val: pte_t = ptep_get(pte);
    val = __pte(set_pageattr_masks(pte_val(val), walk));
    set_pte(pte, val);
    return 0;
    }
    static int pageattr_pte_hole(unsigned long addr, unsigned long next,
    int depth, struct mm_walk *walk)
    {
    return 0;
    }
    static const struct mm_walk_ops pageattr_ops = {
    .pgd_entry = pageattr_pgd_entry,
    .p4d_entry = pageattr_p4d_entry,
    .pud_entry = pageattr_pud_entry,
    .pmd_entry = pageattr_pmd_entry,
    .pte_entry = pageattr_pte_entry,
    .pte_hole = pageattr_pte_hole,
    .walk_lock = PGWALK_RDLOCK,
    };
#[no_mangle]
unsafe extern "C" fn __set_memory(addr: c_ulong, numpages: c_int, set_mask: pgprot_t, clear_mask: pgprot_t) -> c_int {
    static int __set_memory(unsigned long addr, int numpages, pgprot_t set_mask, pgprot_t clear_mask)
    {
    int ret;
    let mut start: c_ulong = addr;
    let mut end: c_ulong = start + PAGE_SIZE * numpages;
    struct pageattr_masks masks = {
    .set_mask = set_mask,
    .clear_mask = clear_mask
    };
    if (!numpages)
    return 0;
    mmap_write_lock(&init_mm);
    ret = walk_kernel_page_table_range(start, end, &pageattr_ops, core::ptr::null_mut(), &masks);
    mmap_write_unlock(&init_mm);
    flush_tlb_kernel_range(start, end);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_x(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_x(unsigned long addr, int numpages)
    {
    if (addr < vm_map_base)
    return 0;
    return __set_memory(addr, numpages, __pgprot(0), __pgprot(_PAGE_NO_EXEC));
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_nx(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_nx(unsigned long addr, int numpages)
    {
    if (addr < vm_map_base)
    return 0;
    return __set_memory(addr, numpages, __pgprot(_PAGE_NO_EXEC), __pgprot(0));
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_ro(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_ro(unsigned long addr, int numpages)
    {
    if (addr < vm_map_base)
    return 0;
    return __set_memory(addr, numpages, __pgprot(0), __pgprot(_PAGE_WRITE | _PAGE_DIRTY));
    }
#[no_mangle]
pub unsafe extern "C" fn set_memory_rw(addr: c_ulong, numpages: c_int) -> c_int {
    int set_memory_rw(unsigned long addr, int numpages)
    {
    if (addr < vm_map_base)
    return 0;
    return __set_memory(addr, numpages, __pgprot(_PAGE_WRITE | _PAGE_DIRTY), __pgprot(0));
    }
#[no_mangle]
pub unsafe extern "C" fn kernel_page_present(page: *mut page) -> bool {
    bool kernel_page_present(struct page *page)
    {
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pte_t *pte;
    let mut addr: c_ulong = (unsigned long)page_address(page);
    if (addr < vm_map_base)
    return memblock_is_memory(__pa(addr));
    pgd = pgd_offset_k(addr);
    if (pgd_none(pgdp_get(pgd)))
    return false;
    if (pgd_leaf(pgdp_get(pgd)))
    return true;
    p4d = p4d_offset(pgd, addr);
    if (p4d_none(p4dp_get(p4d)))
    return false;
    if (p4d_leaf(p4dp_get(p4d)))
    return true;
    pud = pud_offset(p4d, addr);
    if (pud_none(pudp_get(pud)))
    return false;
    if (pud_leaf(pudp_get(pud)))
    return true;
    pmd = pmd_offset(pud, addr);
    if (pmd_none(pmdp_get(pmd)))
    return false;
    if (pmd_leaf(pmdp_get(pmd)))
    return true;
    pte = pte_offset_kernel(pmd, addr);
    return pte_present(ptep_get(pte));
    }
#[no_mangle]
pub unsafe extern "C" fn set_direct_map_default_noflush(page: *mut page) -> c_int {
    int set_direct_map_default_noflush(struct page *page)
    {
    let mut addr: c_ulong = (unsigned long)page_address(page);
    if (addr < vm_map_base)
    return 0;
    return __set_memory(addr, 1, PAGE_KERNEL, __pgprot(0));
    }
#[no_mangle]
pub unsafe extern "C" fn set_direct_map_invalid_noflush(page: *mut page) -> c_int {
    int set_direct_map_invalid_noflush(struct page *page)
    {
    let mut addr: c_ulong = (unsigned long)page_address(page);
    if (addr < vm_map_base)
    return 0;
    return __set_memory(addr, 1, __pgprot(0), __pgprot(_PAGE_PRESENT | _PAGE_VALID));
    }
#[no_mangle]
pub unsafe extern "C" fn set_direct_map_valid_noflush(page: *mut page, nr: unsigned, valid: bool) -> c_int {
    int set_direct_map_valid_noflush(struct page *page, unsigned nr, bool valid)
    {
    let mut addr: c_ulong = (unsigned long)page_address(page);
    pgprot_t set, clear;
    if (addr < vm_map_base)
    return 0;
    if (valid) {
    set = PAGE_KERNEL;
    clear = __pgprot(0);
    } else {
    set = __pgprot(0);
    clear = __pgprot(_PAGE_PRESENT | _PAGE_VALID);
    }
    return __set_memory(addr, nr, set, clear);
    }
