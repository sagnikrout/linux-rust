//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/pgtable_32.c
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
// This file contains the routines setting up the linux page tables.
// -- paulus
//
// Derived from arch/ppc/mm/init.c:
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
// and Cort Dougan (PReP) (cort@cs.nmt.edu)
// Copyright (C) 1996 Paul Mackerras
//
// Derived from "arch/i386/mm/init.c"
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//

    static u8 early_fixmap_pagetable[FIXMAP_PTE_SIZE] __page_aligned_data;
#[no_mangle]
pub unsafe extern "C" fn early_ioremap_init() -> notrace void __init {
    notrace void __init early_ioremap_init(void)
    {
    let mut addr: c_ulong = ALIGN_DOWN(FIXADDR_START, PGDIR_SIZE);
    pte_t *ptep = (pte_t *)early_fixmap_pagetable;
    pmd_t *pmdp = pmd_off_k(addr);
    for (; (s32)(FIXADDR_TOP - addr) > 0;
    addr += PGDIR_SIZE, ptep += PTRS_PER_PTE, pmdp++)
    pmd_populate_kernel(&init_mm, pmdp, ptep);
    early_ioremap_setup();
    }
    void __init *early_alloc_pgtable(unsigned long size)
    {
    return memblock_alloc_or_panic(size, size);
    }
    pte_t __init *early_pte_alloc_kernel(pmd_t *pmdp, unsigned long va)
    {
    if (pmd_none(*pmdp)) {
    pte_t *ptep = early_alloc_pgtable(PTE_FRAG_SIZE);
    pmd_populate_kernel(&init_mm, pmdp, ptep);
    }
    return pte_offset_kernel(pmdp, va);
    }
#[no_mangle]
pub unsafe extern "C" fn map_kernel_page(va: c_ulong, pa: phys_addr_t, prot: pgprot_t) -> int __ref {
    int __ref map_kernel_page(unsigned long va, phys_addr_t pa, pgprot_t prot)
    {
    pmd_t *pd;
    pte_t *pg;
    let mut err: c_int = -ENOMEM;
// Use upper 10 bits of VA to index the first level map
    pd = pmd_off_k(va);
// Use middle 10 bits of VA to index the second-level map
    if (likely(slab_is_available()))
    pg = pte_alloc_kernel(pd, va);
    else
    pg = early_pte_alloc_kernel(pd, va);
    if (pg) {
    err = 0;
// The PTE should never be already set nor present in the
// hash table
//
    BUG_ON((pte_present(*pg) | pte_hashpte(*pg)) && pgprot_val(prot));
    set_pte_at(&init_mm, va, pg, pfn_pte(pa >> PAGE_SHIFT, prot));
    }
    smp_wmb();
    return err;
    }
//
// Map in a chunk of physical memory starting at start.
//
#[no_mangle]
unsafe extern "C" fn __mapin_ram_chunk(offset: c_ulong, top: c_ulong) -> void __init {
    static void __init __mapin_ram_chunk(unsigned long offset, unsigned long top)
    {
    unsigned long v, s;
    phys_addr_t p;
    bool ktext;
    s = offset;
    v = PAGE_OFFSET + s;
    p = memstart_addr + s;
    for (; s < top; s += PAGE_SIZE) {
    ktext = core_kernel_text(v);
    map_kernel_page(v, p, ktext ? PAGE_KERNEL_X : PAGE_KERNEL);
    v += PAGE_SIZE;
    p += PAGE_SIZE;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mapin_ram() -> void __init {
    void __init mapin_ram(void)
    {
    phys_addr_t base, end;
    u64 i;
    for_each_mem_range(i, &base, &end) {
    let mut top: phys_addr_t = min(end, total_lowmem);
    if (base >= top)
    continue;
    base = mmu_mapin_ram(base, top);
    __mapin_ram_chunk(base, top);
    }
    }
#[no_mangle]
unsafe extern "C" fn __mark_initmem_nx() -> c_int {
    static int __mark_initmem_nx(void)
    {
    unsigned long numpages = PFN_UP((unsigned long)_einittext) -
    PFN_DOWN((unsigned long)_sinittext);
    int err;
    err = mmu_mark_initmem_nx();
    if (!v_block_mapped((unsigned long)_sinittext)) {
    err = set_memory_nx((unsigned long)_sinittext, numpages);
    if (err)
    return err;
    err = set_memory_rw((unsigned long)_sinittext, numpages);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mark_initmem_nx() {
    void mark_initmem_nx(void)
    {
    let mut err: c_int = __mark_initmem_nx();
    if (err)
    panic("%s() failed, err = %d\n", __func__, err);
    }

#[no_mangle]
unsafe extern "C" fn __mark_rodata_ro() -> c_int {
    static int __mark_rodata_ro(void)
    {
    unsigned long numpages;
    if (IS_ENABLED(CONFIG_STRICT_MODULE_RWX) && mmu_has_feature(MMU_FTR_HPTE_TABLE))
    pr_warn("This platform has HASH MMU, STRICT_MODULE_RWX won't work\n");
    if (v_block_mapped((unsigned long)_stext + 1))
    return mmu_mark_rodata_ro();
//
// mark text and rodata as read only. __end_rodata is set by
// powerpc's linker script and includes tables and data
// requiring relocation which are not put in RO_DATA.
//
    numpages = PFN_UP((unsigned long)__end_rodata) -
    PFN_DOWN((unsigned long)_stext);
    return set_memory_ro((unsigned long)_stext, numpages);
    }
#[no_mangle]
pub unsafe extern "C" fn mark_rodata_ro() {
    void mark_rodata_ro(void)
    {
    let mut err: c_int = __mark_rodata_ro();
    if (err)
    panic("%s() failed, err = %d\n", __func__, err);
    }
