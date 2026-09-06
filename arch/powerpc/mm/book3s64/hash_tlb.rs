//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/book3s64/hash_tlb.c
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
// This file contains the routines for flushing entries from the
// TLB and MMU hash table.
//
// Derived from arch/ppc64/mm/init.c:
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
// and Cort Dougan (PReP) (cort@cs.nmt.edu)
// Copyright (C) 1996 Paul Mackerras
//
// Derived from "arch/i386/mm/init.c"
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//
// Dave Engebretsen <engebret@us.ibm.com>
// Rework for PPC64 port.
//

    DEFINE_PER_CPU(struct ppc64_tlb_batch, ppc64_tlb_batch);
    EXPORT_SYMBOL_IF_KUNIT(ppc64_tlb_batch);
//
// A linux PTE was changed and the corresponding hash table entry
// neesd to be flushed. This function will either perform the flush
// immediately or will batch it up if the current CPU has an active
// batch on it.
//
    void hpte_need_flush(struct mm_struct *mm, unsigned long addr,
    pte_t *ptep, unsigned long pte, int huge)
    {
    unsigned long vpn;
    struct ppc64_tlb_batch *batch = &get_cpu_var(ppc64_tlb_batch);
    unsigned long vsid;
    unsigned int psize;
    int ssize;
    real_pte_t rpte;
    int i, offset;
    i = batch.index;
//
// Get page size (maybe move back to caller).
//
// NOTE: when using special 64K mappings in 4K environment like
// for SPEs, we obtain the page size from the slice, which thus
// must still exist (and thus the VMA not reused) at the time
// of this call
//
    if (huge) {

    psize = get_slice_psize(mm, addr);
// Mask the address for the correct page size
    addr &= ~((1UL << mmu_psize_defs[psize].shift) - 1);
    if (unlikely(psize == MMU_PAGE_16G))
    offset = PTRS_PER_PUD;
    else
    offset = PTRS_PER_PMD;

    BUG();
    psize = pte_pagesize_index(mm, addr, pte); /* shutup gcc */

    } else {
    psize = pte_pagesize_index(mm, addr, pte);
//
// Mask the address for the standard page size.  If we
// have a 64k page kernel, but the hardware does not
// support 64k pages, this might be different from the
// hardware page size encoded in the slice table.
//
    addr &= PAGE_MASK;
    offset = PTRS_PER_PTE;
    }
// Build full vaddr
    if (!is_kernel_addr(addr)) {
    ssize = user_segment_size(addr);
    vsid = get_user_vsid(&mm.context, addr, ssize);
    } else {
    vsid = get_kernel_vsid(addr, mmu_kernel_ssize);
    ssize = mmu_kernel_ssize;
    }
    WARN_ON(vsid == 0);
    vpn = hpt_vpn(addr, vsid, ssize);
    rpte = __real_pte(__pte(pte), ptep, offset);
//
// Check if we have an active batch on this CPU. If not, just
// flush now and return.
//
    if (!is_lazy_mmu_mode_active()) {
    flush_hash_page(vpn, rpte, psize, ssize, mm_is_thread_local(mm));
    put_cpu_var(ppc64_tlb_batch);
    return;
    }
//
// This can happen when we are in the middle of a TLB batch and
// we encounter memory pressure (eg copy_page_range when it tries
// to allocate a new pte). If we have to reclaim memory and end
// up scanning and resetting referenced bits then our batch context
// will change mid stream.
//
// We also need to ensure only one page size is present in a given
// batch
//
    if (i != 0 && (mm != batch.mm || batch.psize != psize ||
    batch.ssize != ssize)) {
    __flush_tlb_pending(batch);
    i = 0;
    }
    if (i == 0) {
    batch.mm = mm;
    batch.psize = psize;
    batch.ssize = ssize;
    }
    batch.pte[i] = rpte;
    batch.vpn[i] = vpn;
    batch.index = ++i;
    if (i >= PPC64_TLB_BATCH_NR)
    __flush_tlb_pending(batch);
    put_cpu_var(ppc64_tlb_batch);
    }
//
// This function is called when terminating an mmu batch or when a batch
// is full. It will perform the flush of all the entries currently stored
// in a batch.
//
// Must be called from within some kind of spinlock/non-preempt region...
//
#[no_mangle]
pub unsafe extern "C" fn __flush_tlb_pending(batch: *mut ppc64_tlb_batch) {
    void __flush_tlb_pending(struct ppc64_tlb_batch *batch)
    {
    int i, local;
    i = batch.index;
    local = mm_is_thread_local(batch.mm);
    if (i == 1)
    flush_hash_page(batch.vpn[0], batch.pte[0],
    batch.psize, batch.ssize, local);
    else
    flush_hash_range(i, local);
    batch.index = 0;
    }
    EXPORT_SYMBOL_IF_KUNIT(__flush_tlb_pending);
#[no_mangle]
pub unsafe extern "C" fn hash__tlb_flush(tlb: *mut mmu_gather) {
    void hash__tlb_flush(struct mmu_gather *tlb)
    {
    struct ppc64_tlb_batch *tlbbatch = &get_cpu_var(ppc64_tlb_batch);
//
// If there's a TLB batch pending, then we must flush it because the
// pages are going to be freed and we really don't want to have a CPU
// access a freed page because it has a stale TLB
//
    if (tlbbatch.index)
    __flush_tlb_pending(tlbbatch);
    put_cpu_var(ppc64_tlb_batch);
    }
//
// __flush_hash_table_range - Flush all HPTEs for a given address range
// from the hash table (and the TLB). But keeps
// the linux PTEs intact.
//
// @start	: starting address
// @end         : ending address (not included in the flush)
//
// This function is mostly to be used by some IO hotplug code in order
// to remove all hash entries from a given address range used to map IO
// space on a removed PCI-PCI bidge without tearing down the full mapping
// since 64K pages may overlap with other bridges when using 64K pages
// with 4K HW pages on IO space.
//
// Because of that usage pattern, it is implemented for small size rather
// than speed.
//
#[no_mangle]
pub unsafe extern "C" fn __flush_hash_table_range(start: c_ulong, end: c_ulong) {
    void __flush_hash_table_range(unsigned long start, unsigned long end)
    {
    int hugepage_shift;
    unsigned long flags;
    start = ALIGN_DOWN(start, PAGE_SIZE);
    end = ALIGN(end, PAGE_SIZE);
//
// Note: Normally, we should only ever use a batch within a
// PTE locked section. This violates the rule, but will work
// since we don't actually modify the PTEs, we just flush the
// hash while leaving the PTEs intact (including their reference
// to being hashed). This is not the most performance oriented
// way to do things but is fine for our needs here.
//
    local_irq_save(flags);
    lazy_mmu_mode_enable();
    for (; start < end; start += PAGE_SIZE) {
    pte_t *ptep = find_init_mm_pte(start, &hugepage_shift);
    unsigned long pte;
    if (ptep == core::ptr::null_mut())
    continue;
    pte = pte_val(*ptep);
    if (!(pte & H_PAGE_HASHPTE))
    continue;
    hpte_need_flush(&init_mm, start, ptep, pte, hugepage_shift);
    }
    lazy_mmu_mode_disable();
    local_irq_restore(flags);
    }
#[no_mangle]
pub unsafe extern "C" fn flush_hash_table_pmd_range(mm: *mut mm_struct, pmd: *mut pmd_t, addr: c_ulong) {
    void flush_hash_table_pmd_range(struct mm_struct *mm, pmd_t *pmd, unsigned long addr)
    {
    pte_t *pte;
    pte_t *start_pte;
    unsigned long flags;
    addr = ALIGN_DOWN(addr, PMD_SIZE);
//
// Note: Normally, we should only ever use a batch within a
// PTE locked section. This violates the rule, but will work
// since we don't actually modify the PTEs, we just flush the
// hash while leaving the PTEs intact (including their reference
// to being hashed). This is not the most performance oriented
// way to do things but is fine for our needs here.
//
    local_irq_save(flags);
    lazy_mmu_mode_enable();
    start_pte = pte_offset_map(pmd, addr);
    if (!start_pte)
    goto out;
    for (pte = start_pte; pte < start_pte + PTRS_PER_PTE; pte++) {
    let mut pteval: c_ulong = pte_val(*pte);
    if (pteval & H_PAGE_HASHPTE)
    hpte_need_flush(mm, addr, pte, pteval, 0);
    addr += PAGE_SIZE;
    }
    pte_unmap(start_pte);
    out:
    lazy_mmu_mode_disable();
    local_irq_restore(flags);
    }
