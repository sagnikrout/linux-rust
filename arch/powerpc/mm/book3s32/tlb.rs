//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/book3s32/tlb.c
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
// This file contains the routines for TLB flushing.
// On machines where the MMU uses a hash table to store virtual to
// physical translations, these routines flush entries from the
// hash table also.
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

//
// TLB flushing:
//
// - flush_tlb_mm(mm) flushes the specified mm context TLB's
// - flush_tlb_page(vma, vmaddr) flushes one page
// - flush_tlb_range(vma, start, end) flushes a range of pages
// - flush_tlb_kernel_range(start, end) flushes kernel pages
//
// since the hardware hash table functions as an extension of the
// tlb as far as the linux tables are concerned, flush it too.
// -- Cort
//
// For each address in the range, find the pte for the address
// and check _PAGE_HASHPTE bit; if it is set, find and destroy
// the corresponding HPTE.
//
#[no_mangle]
pub unsafe extern "C" fn hash__flush_range(mm: *mut mm_struct, start: c_ulong, end: c_ulong) {
    void hash__flush_range(struct mm_struct *mm, unsigned long start, unsigned long end)
    {
    pmd_t *pmd;
    unsigned long pmd_end;
    int count;
    let mut ctx: c_uint = mm.context.id;
    start &= PAGE_MASK;
    if (start >= end)
    return;
    end = (end - 1) | ~PAGE_MASK;
    pmd = pmd_off(mm, start);
    for (;;) {
    pmd_end = ((start + PGDIR_SIZE) & PGDIR_MASK) - 1;
    if (pmd_end > end)
    pmd_end = end;
    if (!pmd_none(*pmd)) {
    count = ((pmd_end - start) >> PAGE_SHIFT) + 1;
    flush_hash_pages(ctx, start, pmd_val(*pmd), count);
    }
    if (pmd_end == end)
    break;
    start = pmd_end + 1;
    ++pmd;
    }
    }
    EXPORT_SYMBOL(hash__flush_range);
//
// Flush all the (user) entries for the address space described by mm.
//
#[no_mangle]
pub unsafe extern "C" fn hash__flush_tlb_mm(mm: *mut mm_struct) {
    void hash__flush_tlb_mm(struct mm_struct *mm)
    {
    struct vm_area_struct *mp;
    VMA_ITERATOR(vmi, mm, 0);
//
// It is safe to iterate the vmas when called from dup_mmap,
// holding mmap_lock.  It would also be safe from unmap_region
// or exit_mmap, but not from vmtruncate on SMP - but it seems
// dup_mmap is the only SMP case which gets here.
//
    for_each_vma(vmi, mp)
    hash__flush_range(mp.vm_mm, mp.vm_start, mp.vm_end);
    }
    EXPORT_SYMBOL(hash__flush_tlb_mm);
#[no_mangle]
pub unsafe extern "C" fn hash__flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    void hash__flush_tlb_page(struct vm_area_struct *vma, unsigned long vmaddr)
    {
    struct mm_struct *mm;
    pmd_t *pmd;
    mm = (vmaddr < TASK_SIZE)? vma.vm_mm: &init_mm;
    pmd = pmd_off(mm, vmaddr);
    if (!pmd_none(*pmd))
    flush_hash_pages(mm.context.id, vmaddr, pmd_val(*pmd), 1);
    }
    EXPORT_SYMBOL(hash__flush_tlb_page);
#[no_mangle]
pub unsafe extern "C" fn hash__flush_gather(tlb: *mut mmu_gather) {
    void hash__flush_gather(struct mmu_gather *tlb)
    {
    if (tlb.fullmm || tlb.need_flush_all)
    hash__flush_tlb_mm(tlb.mm);
    else
    hash__flush_range(tlb.mm, tlb.start, tlb.end);
    }
    EXPORT_SYMBOL(hash__flush_gather);
