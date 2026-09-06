//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/pi/map_range.c
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
// Copyright 2023 Google LLC
// Author: Ard Biesheuvel <ardb@google.com>

//
// map_range - Map a contiguous range of physical pages into virtual memory
//
// @pte:		Address of physical pointer to array of pages to
// allocate page tables from
// @start:		Virtual address of the start of the range
// @end:		Virtual address of the end of the range (exclusive)
// @pa:			Physical address of the start of the range
// @prot:		Access permissions of the range
// @level:		Translation level for the mapping
// @tbl:		The level @level page table to create the mappings in
// @may_use_cont:	Whether the use of the contiguous attribute is allowed
// @va_offset:		Offset between a physical page and its current mapping
// in the VA space
//
    void __init map_range(phys_addr_t *pte, u64 start, u64 end, phys_addr_t pa,
    pgprot_t prot, int level, pte_t *tbl, bool may_use_cont,
    u64 va_offset)
    {
    let mut cmask: u64 = (level == 3) ? CONT_PTE_SIZE - 1 : U64_MAX;
    let mut protval: ptval_t = pgprot_val(prot) & ~PTE_TYPE_MASK;
    let mut lshift: c_int = (3 - level) * PTDESC_TABLE_SHIFT;
    let mut lmask: u64 = (PAGE_SIZE << lshift) - 1;
    start	&= PAGE_MASK;
    pa	&= PAGE_MASK;
// Advance tbl to the entry that covers start
    tbl += (start >> (lshift + PAGE_SHIFT)) % PTRS_PER_PTE;
//
// Set the right block/page bits for this level unless we are
// clearing the mapping
//
    if (protval)
    protval |= (level == 2) ? PMD_TYPE_SECT : PTE_TYPE_PAGE;
    while (start < end) {
    let mut next: u64 = min((start | lmask) + 1, PAGE_ALIGN(end));
    if (level < 2 || (level == 2 && (start | next | pa) & lmask)) {
//
// This chunk needs a finer grained mapping. Create a
// table mapping if necessary and recurse.
//
    if (pte_none(*tbl)) {
// tbl = __pte(__phys_to_pte_val(*pte) |
    PMD_TYPE_TABLE | PMD_TABLE_UXN);
// pte += PTRS_PER_PTE * sizeof(pte_t);
    }
    map_range(pte, start, next, pa, prot, level + 1,
    (pte_t *)(__pte_to_phys(*tbl) + va_offset),
    may_use_cont, va_offset);
    } else {
//
// Start a contiguous range if start and pa are
// suitably aligned
//
    if (((start | pa) & cmask) == 0 && may_use_cont)
    protval |= PTE_CONT;
//
// Clear the contiguous attribute if the remaining
// range does not cover a contiguous block
//
    if ((end & ~cmask) <= start)
    protval &= ~PTE_CONT;
// Put down a block or page mapping
// tbl = __pte(__phys_to_pte_val(pa) | protval);
    }
    pa += next - start;
    start = next;
    tbl++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn create_init_idmap(pg_dir: *mut pgd_t, clrmask: ptval_t) -> asmlinkage phys_addr_t __init {
    asmlinkage phys_addr_t __init create_init_idmap(pgd_t *pg_dir, ptval_t clrmask)
    {
    phys_addr_t ptep = (phys_addr_t)pg_dir + PAGE_SIZE; /* MMU is off */
    let mut text_prot: pgprot_t = PAGE_KERNEL_ROX;
    let mut data_prot: pgprot_t = PAGE_KERNEL;
    pgprot_val(text_prot) &= ~clrmask;
    pgprot_val(data_prot) &= ~clrmask;
// MMU is off; pointer casts to phys_addr_t are safe
    map_range(&ptep, (u64)_stext, (u64)__initdata_begin,
    (phys_addr_t)_stext, text_prot, IDMAP_ROOT_LEVEL,
    (pte_t *)pg_dir, false, 0);
    map_range(&ptep, (u64)__initdata_begin, (u64)_end,
    (phys_addr_t)__initdata_begin, data_prot, IDMAP_ROOT_LEVEL,
    (pte_t *)pg_dir, false, 0);
    return ptep;
    }
