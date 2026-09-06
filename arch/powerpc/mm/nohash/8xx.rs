//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/nohash/8xx.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// This file contains the routines for initializing the MMU
// on the 8xx series of chips.
// -- christophe
//
// Derived from arch/powerpc/mm/40x_mmu.c:
//

    static unsigned long block_mapped_ram;
//
// Return PA for this VA if it is in an area mapped with LTLBs or fixmap.
// Otherwise, returns 0
//
#[no_mangle]
pub unsafe extern "C" fn v_block_mapped(va: c_ulong) -> phys_addr_t {
    phys_addr_t v_block_mapped(unsigned long va)
    {
    let mut p: c_ulong = PHYS_IMMR_BASE;
    if (va >= VIRT_IMMR_BASE && va < VIRT_IMMR_BASE + IMMR_SIZE)
    return p + va - VIRT_IMMR_BASE;
    if (va >= PAGE_OFFSET && va < PAGE_OFFSET + block_mapped_ram)
    return __pa(va);
    return 0;
    }
//
// Return VA for a given PA mapped with LTLBs or fixmap
// Return 0 if not mapped
//
#[no_mangle]
pub unsafe extern "C" fn p_block_mapped(pa: phys_addr_t) -> c_ulong {
    unsigned long p_block_mapped(phys_addr_t pa)
    {
    let mut p: c_ulong = PHYS_IMMR_BASE;
    if (pa >= p && pa < p + IMMR_SIZE)
    return VIRT_IMMR_BASE + pa - p;
    if (pa < block_mapped_ram)
    return (unsigned long)__va(pa);
    return 0;
    }
    static int __ref __early_map_kernel_hugepage(unsigned long va, phys_addr_t pa,
    pgprot_t prot, int psize, bool new)
    {
    pmd_t *pmdp = pmd_off_k(va);
    pte_t *ptep;
    let mut shift: c_uint = mmu_psize_to_shift(psize);
    if (new) {
    if (WARN_ON(slab_is_available()))
    return -EINVAL;
    if (psize == MMU_PAGE_8M) {
    if (WARN_ON(!pmd_none(*pmdp) || !pmd_none(*(pmdp + 1))))
    return -EINVAL;
    ptep = early_alloc_pgtable(PTE_FRAG_SIZE);
    pmd_populate_kernel(&init_mm, pmdp, ptep);
    ptep = early_alloc_pgtable(PTE_FRAG_SIZE);
    pmd_populate_kernel(&init_mm, pmdp + 1, ptep);
    ptep = (pte_t *)pmdp;
    } else {
    ptep = early_pte_alloc_kernel(pmdp, va);
// The PTE should never be already present
    if (WARN_ON(pte_present(*ptep) && pgprot_val(prot)))
    return -EINVAL;
    }
    } else {
    if (psize == MMU_PAGE_8M)
    ptep = (pte_t *)pmdp;
    else
    ptep = pte_offset_kernel(pmdp, va);
    }
    if (WARN_ON(!ptep))
    return -ENOMEM;
    set_huge_pte_at(&init_mm, va, ptep,
    arch_make_huge_pte(pfn_pte(pa >> PAGE_SHIFT, prot), shift, 0),
    1UL << shift);
    return 0;
    }
//
// MMU_init_hw does the chip-specific initialization of the MMU hardware.
//
#[no_mangle]
pub unsafe extern "C" fn MMU_init_hw() -> void __init {
    void __init MMU_init_hw(void)
    {
    }
    static bool immr_is_mapped __initdata;
#[no_mangle]
pub unsafe extern "C" fn mmu_mapin_immr() -> void __init {
    void __init mmu_mapin_immr(void)
    {
    if (immr_is_mapped)
    return;
    immr_is_mapped = true;
    __early_map_kernel_hugepage(VIRT_IMMR_BASE, PHYS_IMMR_BASE,
    PAGE_KERNEL_NCG, MMU_PAGE_512K, true);
    }
    static int mmu_mapin_ram_chunk(unsigned long offset, unsigned long top,
    pgprot_t prot, bool new)
    {
    let mut v: c_ulong = PAGE_OFFSET + offset;
    let mut p: c_ulong = offset;
    let mut err: c_int = 0;
    WARN_ON(!IS_ALIGNED(offset, SZ_16K) || !IS_ALIGNED(top, SZ_16K));
    for (; p < ALIGN(p, SZ_512K) && p < top && !err; p += SZ_16K, v += SZ_16K)
    err = __early_map_kernel_hugepage(v, p, prot, MMU_PAGE_16K, new);
    for (; p < ALIGN(p, SZ_8M) && p < top && !err; p += SZ_512K, v += SZ_512K)
    err = __early_map_kernel_hugepage(v, p, prot, MMU_PAGE_512K, new);
    for (; p < ALIGN_DOWN(top, SZ_8M) && p < top && !err; p += SZ_8M, v += SZ_8M)
    err = __early_map_kernel_hugepage(v, p, prot, MMU_PAGE_8M, new);
    for (; p < ALIGN_DOWN(top, SZ_512K) && p < top && !err; p += SZ_512K, v += SZ_512K)
    err = __early_map_kernel_hugepage(v, p, prot, MMU_PAGE_512K, new);
    for (; p < ALIGN_DOWN(top, SZ_16K) && p < top && !err; p += SZ_16K, v += SZ_16K)
    err = __early_map_kernel_hugepage(v, p, prot, MMU_PAGE_16K, new);
    if (!new)
    flush_tlb_kernel_range(PAGE_OFFSET + v, PAGE_OFFSET + top);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mmu_mapin_ram(base: c_ulong, top: c_ulong) -> unsigned long __init {
    unsigned long __init mmu_mapin_ram(unsigned long base, unsigned long top)
    {
    let mut etext8: c_ulong = ALIGN(__pa(_etext), SZ_8M);
    let mut sinittext: c_ulong = __pa(_sinittext);
    let mut strict_boundary: bool = strict_kernel_rwx_enabled() || debug_pagealloc_enabled_or_kfence();
    let mut boundary: c_ulong = strict_boundary ? sinittext : etext8;
    let mut einittext8: c_ulong = ALIGN(__pa(_einittext), SZ_8M);
    WARN_ON(top < einittext8);
    mmu_mapin_immr();
    mmu_mapin_ram_chunk(0, boundary, PAGE_KERNEL_X, true);
    if (debug_pagealloc_enabled_or_kfence()) {
    top = boundary;
    } else {
    mmu_mapin_ram_chunk(boundary, einittext8, PAGE_KERNEL_X, true);
    mmu_mapin_ram_chunk(einittext8, top, PAGE_KERNEL, true);
    }
    if (top > SZ_32M)
    memblock_set_current_limit(top);
    block_mapped_ram = top;
    return top;
    }
#[no_mangle]
pub unsafe extern "C" fn mmu_mark_initmem_nx() -> c_int {
    int mmu_mark_initmem_nx(void)
    {
    let mut etext8: c_ulong = ALIGN(__pa(_etext), SZ_8M);
    let mut sinittext: c_ulong = __pa(_sinittext);
    let mut boundary: c_ulong = strict_kernel_rwx_enabled() ? sinittext : etext8;
    let mut einittext8: c_ulong = ALIGN(__pa(_einittext), SZ_8M);
    let mut err: c_int = 0;
    if (!debug_pagealloc_enabled_or_kfence())
    err = mmu_mapin_ram_chunk(boundary, einittext8, PAGE_KERNEL, false);
    if (IS_ENABLED(CONFIG_PIN_TLB_TEXT))
    mmu_pin_tlb(block_mapped_ram, false);
    return err;
    }

#[no_mangle]
pub unsafe extern "C" fn mmu_mark_rodata_ro() -> c_int {
    int mmu_mark_rodata_ro(void)
    {
    let mut sinittext: c_ulong = __pa(_sinittext);
    int err;
    err = mmu_mapin_ram_chunk(0, sinittext, PAGE_KERNEL_ROX, false);
    if (IS_ENABLED(CONFIG_PIN_TLB_DATA))
    mmu_pin_tlb(block_mapped_ram, true);
    return err;
    }

    void __init setup_initial_memory_limit(phys_addr_t first_memblock_base,
    phys_addr_t first_memblock_size)
    {
// We don't currently support the first MEMBLOCK not mapping 0
// physical on those processors
//
    BUG_ON(first_memblock_base != 0);
// 8xx can only access 32MB at the moment
    memblock_set_current_limit(min_t(u64, first_memblock_size, SZ_32M));
    }
#[no_mangle]
pub unsafe extern "C" fn pud_clear_huge(pud: *mut pud_t) -> c_int {
    int pud_clear_huge(pud_t *pud)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pmd_clear_huge(pmd: *mut pmd_t) -> c_int {
    int pmd_clear_huge(pmd_t *pmd)
    {
    return 0;
    }
