//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/kasan/init_book3e_64.c
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
// KASAN for 64-bit Book3e powerpc
//
// Copyright 2022, Christophe Leroy, CS GROUP France
//
// Macro flag: #define DISABLE_BRANCH_PROFILING

#[no_mangle]
pub unsafe extern "C" fn kasan_pud_table(p4d: p4d_t) -> bool {
    static inline bool kasan_pud_table(p4d_t p4d)
    {
    return p4d_page(p4d) == virt_to_page(lm_alias(kasan_early_shadow_pud));
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_pmd_table(pud: pud_t) -> bool {
    static inline bool kasan_pmd_table(pud_t pud)
    {
    return pud_page(pud) == virt_to_page(lm_alias(kasan_early_shadow_pmd));
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_pte_table(pmd: pmd_t) -> bool {
    static inline bool kasan_pte_table(pmd_t pmd)
    {
    return pmd_page(pmd) == virt_to_page(lm_alias(kasan_early_shadow_pte));
    }
#[no_mangle]
unsafe extern "C" fn kasan_map_kernel_page(ea: c_ulong, pa: c_ulong, prot: pgprot_t) -> int __init {
    static int __init kasan_map_kernel_page(unsigned long ea, unsigned long pa, pgprot_t prot)
    {
    pgd_t *pgdp;
    p4d_t *p4dp;
    pud_t *pudp;
    pmd_t *pmdp;
    pte_t *ptep;
    pgdp = pgd_offset_k(ea);
    p4dp = p4d_offset(pgdp, ea);
    if (kasan_pud_table(*p4dp)) {
    pudp = memblock_alloc_or_panic(PUD_TABLE_SIZE, PUD_TABLE_SIZE);
    memcpy(pudp, kasan_early_shadow_pud, PUD_TABLE_SIZE);
    p4d_populate(&init_mm, p4dp, pudp);
    }
    pudp = pud_offset(p4dp, ea);
    if (kasan_pmd_table(*pudp)) {
    pmdp = memblock_alloc_or_panic(PMD_TABLE_SIZE, PMD_TABLE_SIZE);
    memcpy(pmdp, kasan_early_shadow_pmd, PMD_TABLE_SIZE);
    pud_populate(&init_mm, pudp, pmdp);
    }
    pmdp = pmd_offset(pudp, ea);
    if (kasan_pte_table(*pmdp)) {
    ptep = memblock_alloc_or_panic(PTE_TABLE_SIZE, PTE_TABLE_SIZE);
    memcpy(ptep, kasan_early_shadow_pte, PTE_TABLE_SIZE);
    pmd_populate_kernel(&init_mm, pmdp, ptep);
    }
    ptep = pte_offset_kernel(pmdp, ea);
    __set_pte_at(&init_mm, ea, ptep, pfn_pte(pa >> PAGE_SHIFT, prot), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kasan_init_phys_region(start: *mut c_void, end: *mut c_void) -> void __init {
    static void __init kasan_init_phys_region(void *start, void *end)
    {
    unsigned long k_start, k_end, k_cur;
    void *va;
    k_start = ALIGN_DOWN((unsigned long)kasan_mem_to_shadow(start), PAGE_SIZE);
    k_end = ALIGN((unsigned long)kasan_mem_to_shadow(end), PAGE_SIZE);
    va = memblock_alloc_or_panic(k_end - k_start, PAGE_SIZE);
    for (k_cur = k_start; k_cur < k_end; k_cur += PAGE_SIZE, va += PAGE_SIZE)
    kasan_map_kernel_page(k_cur, __pa(va), PAGE_KERNEL);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_early_init() -> void __init {
    void __init kasan_early_init(void)
    {
    int i;
    unsigned long addr;
    pgd_t *pgd = pgd_offset_k(KASAN_SHADOW_START);
    let mut zero_pte: pte_t = pfn_pte(virt_to_pfn(kasan_early_shadow_page), PAGE_KERNEL);
    BUILD_BUG_ON(!IS_ALIGNED(KASAN_SHADOW_START, PGDIR_SIZE));
    BUILD_BUG_ON(!IS_ALIGNED(KASAN_SHADOW_END, PGDIR_SIZE));
    for (i = 0; i < PTRS_PER_PTE; i++)
    __set_pte_at(&init_mm, (unsigned long)kasan_early_shadow_page,
    &kasan_early_shadow_pte[i], zero_pte, 0);
    for (i = 0; i < PTRS_PER_PMD; i++)
    pmd_populate_kernel(&init_mm, &kasan_early_shadow_pmd[i],
    kasan_early_shadow_pte);
    for (i = 0; i < PTRS_PER_PUD; i++)
    pud_populate(&init_mm, &kasan_early_shadow_pud[i],
    kasan_early_shadow_pmd);
    for (addr = KASAN_SHADOW_START; addr != KASAN_SHADOW_END; addr += PGDIR_SIZE)
    p4d_populate(&init_mm, p4d_offset(pgd++, addr), kasan_early_shadow_pud);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_init() -> void __init {
    void __init kasan_init(void)
    {
    phys_addr_t start, end;
    u64 i;
    let mut zero_pte: pte_t = pfn_pte(virt_to_pfn(kasan_early_shadow_page), PAGE_KERNEL_RO);
    for_each_mem_range(i, &start, &end)
    kasan_init_phys_region(phys_to_virt(start), phys_to_virt(end));
    if (IS_ENABLED(CONFIG_KASAN_VMALLOC))
    kasan_remove_zero_shadow((void *)VMALLOC_START, VMALLOC_SIZE);
    for (i = 0; i < PTRS_PER_PTE; i++)
    __set_pte_at(&init_mm, (unsigned long)kasan_early_shadow_page,
    &kasan_early_shadow_pte[i], zero_pte, 0);
    flush_tlb_kernel_range(KASAN_SHADOW_START, KASAN_SHADOW_END);
    memset(kasan_early_shadow_page, 0, PAGE_SIZE);
// Enable error messages
    init_task.kasan_depth = 0;
    kasan_init_generic();
    }
    void __init kasan_late_init(void) { }
