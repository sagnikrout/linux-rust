//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/mm/pgtable.c
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

    struct page *dmw_virt_to_page(unsigned long kaddr)
    {
    return phys_to_page(__pa(kaddr));
    }
    EXPORT_SYMBOL(dmw_virt_to_page);
    struct page *tlb_virt_to_page(unsigned long kaddr)
    {
    return phys_to_page(pfn_to_phys(pte_pfn(*virt_to_kpte(kaddr))));
    }
    EXPORT_SYMBOL(tlb_virt_to_page);
    pgd_t *pgd_alloc(struct mm_struct *mm)
    {
    pgd_t *init, *ret;
    ret = __pgd_alloc(mm, 0);
    if (ret) {
    init = pgd_offset(&init_mm, 0UL);
    pgd_init(ret);
    memcpy(ret + USER_PTRS_PER_PGD, init + USER_PTRS_PER_PGD,
    (PTRS_PER_PGD - USER_PTRS_PER_PGD) * sizeof(pgd_t));
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(pgd_alloc);
#[no_mangle]
pub unsafe extern "C" fn pgd_init(addr: *mut c_void) {
    void pgd_init(void *addr)
    {
    unsigned long *p, *end;
    unsigned long entry;

    entry = (unsigned long)invalid_pud_table;

    entry = (unsigned long)invalid_pmd_table;

    entry = (unsigned long)invalid_pte_table;

    p = (unsigned long *)addr;
    end = p + PTRS_PER_PGD;
    do {
    p[0] = entry;
    p[1] = entry;
    p[2] = entry;
    p[3] = entry;
    p[4] = entry;
    p += 8;
    p[-3] = entry;
    p[-2] = entry;
    p[-1] = entry;
    } while (p != end);
    }
    EXPORT_SYMBOL_GPL(pgd_init);

#[no_mangle]
pub unsafe extern "C" fn pmd_init(addr: *mut c_void) {
    void pmd_init(void *addr)
    {
    unsigned long *p, *end;
    let mut pagetable: c_ulong = (unsigned long)invalid_pte_table;
    p = (unsigned long *)addr;
    end = p + PTRS_PER_PMD;
    do {
    p[0] = pagetable;
    p[1] = pagetable;
    p[2] = pagetable;
    p[3] = pagetable;
    p[4] = pagetable;
    p += 8;
    p[-3] = pagetable;
    p[-2] = pagetable;
    p[-1] = pagetable;
    } while (p != end);
    }
    EXPORT_SYMBOL_GPL(pmd_init);

#[no_mangle]
pub unsafe extern "C" fn pud_init(addr: *mut c_void) {
    void pud_init(void *addr)
    {
    unsigned long *p, *end;
    let mut pagetable: c_ulong = (unsigned long)invalid_pmd_table;
    p = (unsigned long *)addr;
    end = p + PTRS_PER_PUD;
    do {
    p[0] = pagetable;
    p[1] = pagetable;
    p[2] = pagetable;
    p[3] = pagetable;
    p[4] = pagetable;
    p += 8;
    p[-3] = pagetable;
    p[-2] = pagetable;
    p[-1] = pagetable;
    } while (p != end);
    }
    EXPORT_SYMBOL_GPL(pud_init);

#[no_mangle]
pub unsafe extern "C" fn kernel_pte_init(addr: *mut c_void) {
    void kernel_pte_init(void *addr)
    {
    unsigned long *p, *end;
    p = (unsigned long *)addr;
    end = p + PTRS_PER_PTE;
    do {
    p[0] = _PAGE_GLOBAL;
    p[1] = _PAGE_GLOBAL;
    p[2] = _PAGE_GLOBAL;
    p[3] = _PAGE_GLOBAL;
    p[4] = _PAGE_GLOBAL;
    p += 8;
    p[-3] = _PAGE_GLOBAL;
    p[-2] = _PAGE_GLOBAL;
    p[-1] = _PAGE_GLOBAL;
    } while (p != end);
    }
    void set_pmd_at(struct mm_struct *mm, unsigned long addr,
    pmd_t *pmdp, pmd_t pmd)
    {
    WRITE_ONCE(*pmdp, pmd);
    flush_tlb_all();
    }
#[no_mangle]
pub unsafe extern "C" fn pagetable_init() -> void __init {
    void __init pagetable_init(void)
    {

    unsigned long vaddr;
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pte_t *pte;

// Initialize the entire pgd.
    pgd_init(swapper_pg_dir);
    pgd_init(invalid_pg_dir);

    pud_init(invalid_pud_table);

    pmd_init(invalid_pmd_table);

// Permanent kmaps
    vaddr = PKMAP_BASE;
    fixrange_init(vaddr & PMD_MASK, vaddr + PAGE_SIZE * LAST_PKMAP, swapper_pg_dir);
    pgd = swapper_pg_dir + pgd_index(vaddr);
    p4d = p4d_offset(pgd, vaddr);
    pud = pud_offset(p4d, vaddr);
    pmd = pmd_offset(pud, vaddr);
    pte = pte_offset_kernel(pmd, vaddr);
    pkmap_page_table = pte;
// Fixed mappings
    vaddr = __fix_to_virt(__end_of_fixed_addresses - 1);
    fixrange_init(vaddr & PMD_MASK, vaddr + FIXADDR_SIZE, swapper_pg_dir);

    }
