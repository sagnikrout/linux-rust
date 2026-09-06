//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/kasan/8xx.c
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

    static int __init
    kasan_init_shadow_8M(unsigned long k_start, unsigned long k_end, void *block)
    {
    pmd_t *pmd = pmd_off_k(k_start);
    unsigned long k_cur, k_next;
    for (k_cur = k_start; k_cur != k_end; k_cur = k_next, pmd++, block += SZ_4M) {
    pte_t *ptep;
    int i;
    k_next = pgd_addr_end(k_cur, k_end);
    if ((void *)pmd_page_vaddr(*pmd) != kasan_early_shadow_pte)
    continue;
    ptep = memblock_alloc(PTE_FRAG_SIZE, PTE_FRAG_SIZE);
    if (!ptep)
    return -ENOMEM;
    for (i = 0; i < PTRS_PER_PTE; i++) {
    let mut pte: pte_t = pte_mkhuge(pfn_pte(PHYS_PFN(__pa(block + i * PAGE_SIZE)), PAGE_KERNEL));
    __set_pte_at(&init_mm, k_cur, ptep + i, pte, 1);
    }
    pmd_populate_kernel(&init_mm, pmd, ptep);
// pmd = __pmd(pmd_val(*pmd) | _PMD_PAGE_8M);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_init_region(start: *mut c_void, size: usize) -> int __init {
    int __init kasan_init_region(void *start, size_t size)
    {
    let mut k_start: c_ulong = (unsigned long)kasan_mem_to_shadow(start);
    let mut k_end: c_ulong = (unsigned long)kasan_mem_to_shadow(start + size);
    unsigned long k_cur;
    int ret;
    void *block;
    block = memblock_alloc(k_end - k_start, SZ_8M);
    if (!block)
    return -ENOMEM;
    if (IS_ALIGNED(k_start, SZ_8M)) {
    kasan_init_shadow_8M(k_start, ALIGN_DOWN(k_end, SZ_8M), block);
    k_cur = ALIGN_DOWN(k_end, SZ_8M);
    if (k_cur == k_end)
    goto finish;
    } else {
    k_cur = k_start;
    }
    ret = kasan_init_shadow_page_tables(k_start, k_end);
    if (ret)
    return ret;
    for (; k_cur < k_end; k_cur += PAGE_SIZE) {
    pmd_t *pmd = pmd_off_k(k_cur);
    void *va = block + k_cur - k_start;
    let mut pte: pte_t = pfn_pte(PHYS_PFN(__pa(va)), PAGE_KERNEL);
    if (k_cur < ALIGN_DOWN(k_end, SZ_512K))
    pte = pte_mkhuge(pte);
    __set_pte_at(&init_mm, k_cur, pte_offset_kernel(pmd, k_cur), pte, 0);
    }
    finish:
    flush_tlb_kernel_range(k_start, k_end);
    return 0;
    }
