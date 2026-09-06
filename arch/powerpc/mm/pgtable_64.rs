//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/pgtable_64.c
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
// This file contains pgtable related functions for 64-bit machines.
//
// Derived from arch/ppc64/mm/init.c
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Modifications by Paul Mackerras (PowerMac) (paulus@samba.org)
// and Cort Dougan (PReP) (cort@cs.nmt.edu)
// Copyright (C) 1996 Paul Mackerras
//
// Derived from "arch/i386/mm/init.c"
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//
// Dave Engebretsen <engebret@us.ibm.com>
// Rework for PPC64 port.
//

//
// partition table and process table for ISA 3.0
//
    struct prtb_entry *process_tb;
    struct patb_entry *partition_tb;
//
// page table size
//
    unsigned long __pte_index_size;
    EXPORT_SYMBOL(__pte_index_size);
    unsigned long __pmd_index_size;
    EXPORT_SYMBOL(__pmd_index_size);
    unsigned long __pud_index_size;
    EXPORT_SYMBOL(__pud_index_size);
    unsigned long __pgd_index_size;
    EXPORT_SYMBOL(__pgd_index_size);
    unsigned long __pud_cache_index;
    EXPORT_SYMBOL(__pud_cache_index);
    unsigned long __pte_table_size;
    EXPORT_SYMBOL(__pte_table_size);
    unsigned long __pmd_table_size;
    EXPORT_SYMBOL(__pmd_table_size);
    unsigned long __pud_table_size;
    EXPORT_SYMBOL(__pud_table_size);
    unsigned long __pgd_table_size;
    EXPORT_SYMBOL(__pgd_table_size);
    unsigned long __pmd_val_bits;
    EXPORT_SYMBOL(__pmd_val_bits);
    unsigned long __pud_val_bits;
    EXPORT_SYMBOL(__pud_val_bits);
    unsigned long __pgd_val_bits;
    EXPORT_SYMBOL(__pgd_val_bits);
    unsigned long __kernel_virt_start;
    EXPORT_SYMBOL(__kernel_virt_start);
    unsigned long __vmalloc_start;
    EXPORT_SYMBOL(__vmalloc_start);
    unsigned long __vmalloc_end;
    EXPORT_SYMBOL(__vmalloc_end);
    unsigned long __kernel_io_start;
    EXPORT_SYMBOL(__kernel_io_start);
    unsigned long __kernel_io_end;
    struct page *vmemmap;
    EXPORT_SYMBOL(vmemmap);
    unsigned long __pte_frag_nr;
    EXPORT_SYMBOL(__pte_frag_nr);
    unsigned long __pte_frag_size_shift;
    EXPORT_SYMBOL(__pte_frag_size_shift);

// 4 level page table
    struct page *p4d_page(p4d_t p4d)
    {
    if (p4d_leaf(p4d)) {
    if (!IS_ENABLED(CONFIG_HAVE_ARCH_HUGE_VMAP))
    VM_WARN_ON(!p4d_leaf(p4d));
    return pte_page(p4d_pte(p4d));
    }
    return virt_to_page(p4d_pgtable(p4d));
    }

    struct page *pud_page(pud_t pud)
    {
    if (pud_leaf(pud)) {
    if (!IS_ENABLED(CONFIG_HAVE_ARCH_HUGE_VMAP))
    VM_WARN_ON(!pud_leaf(pud));
    return pte_page(pud_pte(pud));
    }
    return virt_to_page(pud_pgtable(pud));
    }
//
// For hugepage we have pfn in the pmd, we use PTE_RPN_SHIFT bits for flags
// For PTE page, we have a PTE_FRAG_SIZE (4K) aligned virtual address.
//
    struct page *pmd_page(pmd_t pmd)
    {
    if (pmd_leaf(pmd)) {
//
// vmalloc_to_page may be called on any vmap address (not only
// vmalloc), and it uses pmd_page() etc., when huge vmap is
// enabled so these checks can't be used.
//
    if (!IS_ENABLED(CONFIG_HAVE_ARCH_HUGE_VMAP))
    VM_WARN_ON(!pmd_leaf(pmd));
    return pte_page(pmd_pte(pmd));
    }
    return virt_to_page(pmd_page_vaddr(pmd));
    }

#[no_mangle]
pub unsafe extern "C" fn mark_rodata_ro() {
    void mark_rodata_ro(void)
    {
    if (!mmu_has_feature(MMU_FTR_KERNEL_RO)) {
    pr_warn("Warning: Unable to mark rodata read only on this CPU.\n");
    return;
    }
    if (radix_enabled())
    radix__mark_rodata_ro();
    else
    hash__mark_rodata_ro();
    }
#[no_mangle]
pub unsafe extern "C" fn mark_initmem_nx() {
    void mark_initmem_nx(void)
    {
    if (radix_enabled())
    radix__mark_initmem_nx();
    else
    hash__mark_initmem_nx();
    }
