//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/kasan/init_32.c
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
// Macro flag: #define DISABLE_BRANCH_PROFILING

#[no_mangle]
unsafe extern "C" fn kasan_prot_ro() -> pgprot_t __init {
    static pgprot_t __init kasan_prot_ro(void)
    {
    if (early_mmu_has_feature(MMU_FTR_HPTE_TABLE))
    return PAGE_READONLY;
    return PAGE_KERNEL_RO;
    }
#[no_mangle]
unsafe extern "C" fn kasan_populate_pte(ptep: *mut pte_t, prot: pgprot_t) -> void __init {
    static void __init kasan_populate_pte(pte_t *ptep, pgprot_t prot)
    {
    let mut va: c_ulong = (unsigned long)kasan_early_shadow_page;
    let mut pa: phys_addr_t = __pa(kasan_early_shadow_page);
    int i;
    for (i = 0; i < PTRS_PER_PTE; i++, ptep++)
    __set_pte_at(&init_mm, va, ptep, pfn_pte(PHYS_PFN(pa), prot), 1);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_init_shadow_page_tables(k_start: c_ulong, k_end: c_ulong) -> int __init {
    int __init kasan_init_shadow_page_tables(unsigned long k_start, unsigned long k_end)
    {
    pmd_t *pmd;
    unsigned long k_cur, k_next;
    pmd = pmd_off_k(k_start);
    for (k_cur = k_start; k_cur != k_end; k_cur = k_next, pmd++) {
    pte_t *new;
    k_next = pgd_addr_end(k_cur, k_end);
    if ((void *)pmd_page_vaddr(*pmd) != kasan_early_shadow_pte)
    continue;
    new = memblock_alloc(PTE_FRAG_SIZE, PTE_FRAG_SIZE);
    if (!new)
    return -ENOMEM;
    kasan_populate_pte(new, PAGE_KERNEL);
    pmd_populate_kernel(&init_mm, pmd, new);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_init_region(start: *mut c_void, size: usize) -> int __init __weak {
    int __init __weak kasan_init_region(void *start, size_t size)
    {
    let mut k_start: c_ulong = (unsigned long)kasan_mem_to_shadow(start);
    let mut k_end: c_ulong = (unsigned long)kasan_mem_to_shadow(start + size);
    unsigned long k_cur;
    int ret;
    void *block;
    ret = kasan_init_shadow_page_tables(k_start, k_end);
    if (ret)
    return ret;
    k_start = k_start & PAGE_MASK;
    block = memblock_alloc(k_end - k_start, PAGE_SIZE);
    if (!block)
    return -ENOMEM;
    for (k_cur = k_start & PAGE_MASK; k_cur < k_end; k_cur += PAGE_SIZE) {
    pmd_t *pmd = pmd_off_k(k_cur);
    void *va = block + k_cur - k_start;
    let mut pte: pte_t = pfn_pte(PHYS_PFN(__pa(va)), PAGE_KERNEL);
    __set_pte_at(&init_mm, k_cur, pte_offset_kernel(pmd, k_cur), pte, 0);
    }
    flush_tlb_kernel_range(k_start, k_end);
    return 0;
    }
    void __init
    kasan_update_early_region(unsigned long k_start, unsigned long k_end, pte_t pte)
    {
    unsigned long k_cur;
    for (k_cur = k_start; k_cur != k_end; k_cur += PAGE_SIZE) {
    pmd_t *pmd = pmd_off_k(k_cur);
    pte_t *ptep = pte_offset_kernel(pmd, k_cur);
    if (pte_page(*ptep) != virt_to_page(lm_alias(kasan_early_shadow_page)))
    continue;
    __set_pte_at(&init_mm, k_cur, ptep, pte, 0);
    }
    flush_tlb_kernel_range(k_start, k_end);
    }
#[no_mangle]
unsafe extern "C" fn kasan_remap_early_shadow_ro() -> void __init {
    static void __init kasan_remap_early_shadow_ro(void)
    {
    let mut prot: pgprot_t = kasan_prot_ro();
    let mut pa: phys_addr_t = __pa(kasan_early_shadow_page);
    kasan_populate_pte(kasan_early_shadow_pte, prot);
    kasan_update_early_region(KASAN_SHADOW_START, KASAN_SHADOW_END,
    pfn_pte(PHYS_PFN(pa), prot));
    }
#[no_mangle]
unsafe extern "C" fn kasan_unmap_early_shadow_vmalloc() -> void __init {
    static void __init kasan_unmap_early_shadow_vmalloc(void)
    {
    let mut k_start: c_ulong = (unsigned long)kasan_mem_to_shadow((void *)VMALLOC_START);
    let mut k_end: c_ulong = (unsigned long)kasan_mem_to_shadow((void *)VMALLOC_END);
    kasan_update_early_region(k_start, k_end, __pte(0));

    k_start = (unsigned long)kasan_mem_to_shadow((void *)MODULES_VADDR);
    k_end = (unsigned long)kasan_mem_to_shadow((void *)MODULES_END);
    kasan_update_early_region(k_start, k_end, __pte(0));

    }
#[no_mangle]
pub unsafe extern "C" fn kasan_mmu_init() -> void __init {
    void __init kasan_mmu_init(void)
    {
    int ret;
    if (early_mmu_has_feature(MMU_FTR_HPTE_TABLE)) {
    ret = kasan_init_shadow_page_tables(KASAN_SHADOW_START, KASAN_SHADOW_END);
    if (ret)
    panic("kasan: kasan_init_shadow_page_tables() failed");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_init() -> void __init {
    void __init kasan_init(void)
    {
    phys_addr_t base, end;
    u64 i;
    int ret;
    for_each_mem_range(i, &base, &end) {
    let mut top: phys_addr_t = min(end, total_lowmem);
    if (base >= top)
    continue;
    ret = kasan_init_region(__va(base), top - base);
    if (ret)
    panic("kasan: kasan_init_region() failed");
    }
    if (IS_ENABLED(CONFIG_KASAN_VMALLOC)) {
    ret = kasan_init_shadow_page_tables(KASAN_SHADOW_START, KASAN_SHADOW_END);
    if (ret)
    panic("kasan: kasan_init_shadow_page_tables() failed");
    }
    kasan_remap_early_shadow_ro();
    clear_page(kasan_early_shadow_page);
// At this point kasan is fully initialized. Enable error messages
    init_task.kasan_depth = 0;
    kasan_init_generic();
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_late_init() -> void __init {
    void __init kasan_late_init(void)
    {
    if (IS_ENABLED(CONFIG_KASAN_VMALLOC))
    kasan_unmap_early_shadow_vmalloc();
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_early_init() -> void __init {
    void __init kasan_early_init(void)
    {
    let mut addr: c_ulong = KASAN_SHADOW_START;
    let mut end: c_ulong = KASAN_SHADOW_END;
    unsigned long next;
    pmd_t *pmd = pmd_off_k(addr);
    BUILD_BUG_ON(KASAN_SHADOW_START & ~PGDIR_MASK);
    kasan_populate_pte(kasan_early_shadow_pte, PAGE_KERNEL);
    do {
    next = pgd_addr_end(addr, end);
    pmd_populate_kernel(&init_mm, pmd, kasan_early_shadow_pte);
    } while (pmd++, addr = next, addr != end);
    }
