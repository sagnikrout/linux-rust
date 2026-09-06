//! Automatically rewritten from C to Rust
//! Source: arch/x86/power/hibernate_32.c
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
// Hibernation support specific for i386 - temporary page tables
//
// Copyright (c) 2006 Rafael J. Wysocki <rjw@sisk.pl>
//

// Pointer to the temporary resume page tables
    pgd_t *resume_pg_dir;
// The following three functions are based on the analogous code in
// arch/x86/mm/init_32.c
//
// Create a middle page table on a resume-safe page and put a pointer to it in
// the given global directory entry.  This only returns the gd entry
// in non-PAE compilation mode, since the middle layer is folded.
//
    static pmd_t *resume_one_md_table_init(pgd_t *pgd)
    {
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd_table;

    pmd_table = (pmd_t *)get_safe_page(GFP_ATOMIC);
    if (!pmd_table)
    return core::ptr::null_mut();
    set_pgd(pgd, __pgd(__pa(pmd_table) | _PAGE_PRESENT));
    p4d = p4d_offset(pgd, 0);
    pud = pud_offset(p4d, 0);
    BUG_ON(pmd_table != pmd_offset(pud, 0));

    p4d = p4d_offset(pgd, 0);
    pud = pud_offset(p4d, 0);
    pmd_table = pmd_offset(pud, 0);

    return pmd_table;
    }
//
// Create a page table on a resume-safe page and place a pointer to it in
// a middle page directory entry.
//
    static pte_t *resume_one_page_table_init(pmd_t *pmd)
    {
    if (pmd_none(*pmd)) {
    pte_t *page_table = (pte_t *)get_safe_page(GFP_ATOMIC);
    if (!page_table)
    return core::ptr::null_mut();
    set_pmd(pmd, __pmd(__pa(page_table) | _PAGE_TABLE));
    BUG_ON(page_table != pte_offset_kernel(pmd, 0));
    return page_table;
    }
    return pte_offset_kernel(pmd, 0);
    }
//
// This maps the physical memory to kernel virtual address space, a total
// of max_low_pfn pages, by creating page tables starting from address
// PAGE_OFFSET.  The page tables are allocated out of resume-safe pages.
//
#[no_mangle]
unsafe extern "C" fn resume_physical_mapping_init(pgd_base: *mut pgd_t) -> c_int {
    static int resume_physical_mapping_init(pgd_t *pgd_base)
    {
    unsigned long pfn;
    pgd_t *pgd;
    pmd_t *pmd;
    pte_t *pte;
    int pgd_idx, pmd_idx;
    pgd_idx = pgd_index(PAGE_OFFSET);
    pgd = pgd_base + pgd_idx;
    pfn = 0;
    for (; pgd_idx < PTRS_PER_PGD; pgd++, pgd_idx++) {
    pmd = resume_one_md_table_init(pgd);
    if (!pmd)
    return -ENOMEM;
    if (pfn >= max_low_pfn)
    continue;
    for (pmd_idx = 0; pmd_idx < PTRS_PER_PMD; pmd++, pmd_idx++) {
    if (pfn >= max_low_pfn)
    break;
// Map with big pages if possible, otherwise create
// normal page tables.
// NOTE: We can mark everything as executable here
//
    if (boot_cpu_has(X86_FEATURE_PSE)) {
    set_pmd(pmd, pfn_pmd(pfn, PAGE_KERNEL_LARGE_EXEC));
    pfn += PTRS_PER_PTE;
    } else {
    pte_t *max_pte;
    pte = resume_one_page_table_init(pmd);
    if (!pte)
    return -ENOMEM;
    max_pte = pte + PTRS_PER_PTE;
    for (; pte < max_pte; pte++, pfn++) {
    if (pfn >= max_low_pfn)
    break;
    set_pte(pte, pfn_pte(pfn, PAGE_KERNEL_EXEC));
    }
    }
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn resume_init_first_level_page_table(pg_dir: *mut pgd_t) {
    static inline void resume_init_first_level_page_table(pgd_t *pg_dir)
    {

    int i;
// Init entries of the first-level page table to the zero page
    for (i = 0; i < PTRS_PER_PGD; i++)
    set_pgd(pg_dir + i,
    __pgd(__pa(empty_zero_page) | _PAGE_PRESENT));

    }
#[no_mangle]
unsafe extern "C" fn set_up_temporary_text_mapping(pgd_base: *mut pgd_t) -> c_int {
    static int set_up_temporary_text_mapping(pgd_t *pgd_base)
    {
    pgd_t *pgd;
    pmd_t *pmd;
    pte_t *pte;
    pgd = pgd_base + pgd_index(restore_jump_address);
    pmd = resume_one_md_table_init(pgd);
    if (!pmd)
    return -ENOMEM;
    if (boot_cpu_has(X86_FEATURE_PSE)) {
    set_pmd(pmd + pmd_index(restore_jump_address),
    __pmd((jump_address_phys & PMD_MASK) | pgprot_val(PAGE_KERNEL_LARGE_EXEC)));
    } else {
    pte = resume_one_page_table_init(pmd);
    if (!pte)
    return -ENOMEM;
    set_pte(pte + pte_index(restore_jump_address),
    __pte((jump_address_phys & PAGE_MASK) | pgprot_val(PAGE_KERNEL_EXEC)));
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn swsusp_arch_resume() -> asmlinkage int {
    asmlinkage int swsusp_arch_resume(void)
    {
    int error;
    resume_pg_dir = (pgd_t *)get_safe_page(GFP_ATOMIC);
    if (!resume_pg_dir)
    return -ENOMEM;
    resume_init_first_level_page_table(resume_pg_dir);
    error = set_up_temporary_text_mapping(resume_pg_dir);
    if (error)
    return error;
    error = resume_physical_mapping_init(resume_pg_dir);
    if (error)
    return error;
    temp_pgt = __pa(resume_pg_dir);
    error = relocate_restore_code();
    if (error)
    return error;
// We have got enough memory and from now on we cannot recover
    restore_image();
    return 0;
    }
