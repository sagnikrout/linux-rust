//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/nohash/tlb.c
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
// On machines where the MMU does not use a hash table to store virtual to
// physical translations (ie, SW loaded TLBs or Book3E compilant processors,
// this does -not- include 603 however which shares the implementation with
// hash based processors)
//
// -- BenH
//
// Copyright 2008,2009 Ben Herrenschmidt <benh@kernel.crashing.org>
// IBM Corp.
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
// This struct lists the sw-supported page sizes.  The hardawre MMU may support
// other sizes not listed here.   The .ind field is only used on MMUs that have
// indirect page table entries.
//

    struct mmu_psize_def mmu_psize_defs[MMU_PAGE_COUNT] = {
    [MMU_PAGE_4K] = {
    .shift	= 12,
    },
    [MMU_PAGE_2M] = {
    .shift	= 21,
    },
    [MMU_PAGE_4M] = {
    .shift	= 22,
    },
    [MMU_PAGE_16M] = {
    .shift	= 24,
    },
    [MMU_PAGE_64M] = {
    .shift	= 26,
    },
    [MMU_PAGE_256M] = {
    .shift	= 28,
    },
    [MMU_PAGE_1G] = {
    .shift	= 30,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn mmu_get_tsize(psize: c_int) -> c_int {
    static inline int mmu_get_tsize(int psize)
    {
    return mmu_psize_defs[psize].shift - 10;
    }

#[no_mangle]
pub unsafe extern "C" fn mmu_get_tsize(psize: c_int) -> c_int {
    static inline int mmu_get_tsize(int psize)
    {
// This isn't used on !Book3E for now
    return 0;
    }

    struct mmu_psize_def mmu_psize_defs[MMU_PAGE_COUNT] = {
    [MMU_PAGE_4K] = {
    .shift	= 12,
    },
    [MMU_PAGE_16K] = {
    .shift	= 14,
    },
    [MMU_PAGE_512K] = {
    .shift	= 19,
    },
    [MMU_PAGE_8M] = {
    .shift	= 23,
    },
    };

// next_tlbcam_idx is used to round-robin tlbcam entry assignment
    DEFINE_PER_CPU(int, next_tlbcam_idx);
    EXPORT_PER_CPU_SYMBOL(next_tlbcam_idx);

//
// Base TLB flushing operations:
//
// - flush_tlb_mm(mm) flushes the specified mm context TLB's
// - flush_tlb_page(vma, vmaddr) flushes one page
// - flush_tlb_range(vma, start, end) flushes a range of pages
// - flush_tlb_kernel_range(start, end) flushes kernel pages
//
// - local_* variants of page and mm only apply to the current
// processor
//

//
// These are the base non-SMP variants of page and mm flushing
//
#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_mm(mm: *mut mm_struct) {
    void local_flush_tlb_mm(struct mm_struct *mm)
    {
    unsigned int pid;
    preempt_disable();
    pid = mm.context.id;
    if (pid != MMU_NO_CONTEXT)
    _tlbil_pid(pid);
    preempt_enable();
    }
    EXPORT_SYMBOL(local_flush_tlb_mm);
    void __local_flush_tlb_page(struct mm_struct *mm, unsigned long vmaddr,
    int tsize, int ind)
    {
    unsigned int pid;
    preempt_disable();
    pid = mm ? mm.context.id : 0;
    if (pid != MMU_NO_CONTEXT)
    _tlbil_va(vmaddr, pid, tsize, ind);
    preempt_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn local_flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    void local_flush_tlb_page(struct vm_area_struct *vma, unsigned long vmaddr)
    {
    __local_flush_tlb_page(vma ? vma.vm_mm : core::ptr::null_mut(), vmaddr,
    mmu_get_tsize(mmu_virtual_psize), 0);
    }
    EXPORT_SYMBOL(local_flush_tlb_page);
    void local_flush_tlb_page_psize(struct mm_struct *mm,
    unsigned long vmaddr, int psize)
    {
    __local_flush_tlb_page(mm, vmaddr, mmu_get_tsize(psize), 0);
    }
    EXPORT_SYMBOL(local_flush_tlb_page_psize);

//
// And here are the SMP non-local implementations
//

    static DEFINE_RAW_SPINLOCK(tlbivax_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlb_flush_param {
    pub addr: c_ulong,
    pub pid: c_uint,
    pub tsize: c_uint,
    pub ind: c_uint,
}

#[no_mangle]
unsafe extern "C" fn do_flush_tlb_mm_ipi(param: *mut c_void) {
    static void do_flush_tlb_mm_ipi(void *param)
    {
    struct tlb_flush_param *p = param;
    _tlbil_pid(p ? p.pid : 0);
    }
#[no_mangle]
unsafe extern "C" fn do_flush_tlb_page_ipi(param: *mut c_void) {
    static void do_flush_tlb_page_ipi(void *param)
    {
    struct tlb_flush_param *p = param;
    _tlbil_va(p.addr, p.pid, p.tsize, p.ind);
    }
// Note on invalidations and PID:
//
// We snapshot the PID with preempt disabled. At this point, it can still
// change either because:
// - our context is being stolen (PID -> NO_CONTEXT) on another CPU
// - we are invaliating some target that isn't currently running here
// and is concurrently acquiring a new PID on another CPU
// - some other CPU is re-acquiring a lost PID for this mm
// etc...
//
// However, this shouldn't be a problem as we only guarantee
// invalidation of TLB entries present prior to this call, so we
// don't care about the PID changing, and invalidating a stale PID
// is generally harmless.
//
#[no_mangle]
pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) {
    void flush_tlb_mm(struct mm_struct *mm)
    {
    unsigned int pid;
    preempt_disable();
    pid = mm.context.id;
    if (unlikely(pid == MMU_NO_CONTEXT))
    goto no_context;
    if (!mm_is_core_local(mm)) {
    let mut p: tlb_flush_param = { .pid = pid };
// Ignores smp_processor_id() even if set.
    smp_call_function_many(mm_cpumask(mm),
    do_flush_tlb_mm_ipi, &p, 1);
    }
    _tlbil_pid(pid);
    no_context:
    preempt_enable();
    }
    EXPORT_SYMBOL(flush_tlb_mm);
    void __flush_tlb_page(struct mm_struct *mm, unsigned long vmaddr,
    int tsize, int ind)
    {
    struct cpumask *cpu_mask;
    unsigned int pid;
//
// This function as well as __local_flush_tlb_page() must only be called
// for user contexts.
//
    if (WARN_ON(!mm))
    return;
    preempt_disable();
    pid = mm.context.id;
    if (unlikely(pid == MMU_NO_CONTEXT))
    goto bail;
    cpu_mask = mm_cpumask(mm);
    if (!mm_is_core_local(mm)) {
// If broadcast tlbivax is supported, use it
    if (mmu_has_feature(MMU_FTR_USE_TLBIVAX_BCAST)) {
    let mut lock: c_int = mmu_has_feature(MMU_FTR_LOCK_BCAST_INVAL);
    if (lock)
    raw_spin_lock(&tlbivax_lock);
    _tlbivax_bcast(vmaddr, pid, tsize, ind);
    if (lock)
    raw_spin_unlock(&tlbivax_lock);
    goto bail;
    } else {
    struct tlb_flush_param p = {
    .pid = pid,
    .addr = vmaddr,
    .tsize = tsize,
    .ind = ind,
    };
// Ignores smp_processor_id() even if set in cpu_mask
    smp_call_function_many(cpu_mask,
    do_flush_tlb_page_ipi, &p, 1);
    }
    }
    _tlbil_va(vmaddr, pid, tsize, ind);
    bail:
    preempt_enable();
    }
#[no_mangle]
pub unsafe extern "C" fn flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    void flush_tlb_page(struct vm_area_struct *vma, unsigned long vmaddr)
    {

    if (vma && is_vm_hugetlb_page(vma))
    flush_hugetlb_page(vma, vmaddr);

    __flush_tlb_page(vma ? vma.vm_mm : core::ptr::null_mut(), vmaddr,
    mmu_get_tsize(mmu_virtual_psize), 0);
    }
    EXPORT_SYMBOL(flush_tlb_page);

//
// Flush kernel TLB entries in the given range
//

#[no_mangle]
pub unsafe extern "C" fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong) {
    void flush_tlb_kernel_range(unsigned long start, unsigned long end)
    {

    preempt_disable();
    smp_call_function(do_flush_tlb_mm_ipi, core::ptr::null_mut(), 1);
    _tlbil_pid(0);
    preempt_enable();

    _tlbil_pid(0);

    }
    EXPORT_SYMBOL(flush_tlb_kernel_range);

//
// Currently, for range flushing, we just do a full mm flush. This should
// be optimized based on a threshold on the size of the range, since
// some implementation can stack multiple tlbivax before a tlbsync but
// for now, we keep it that way
//
    void flush_tlb_range(struct vm_area_struct *vma, unsigned long start,
    unsigned long end)
    {
    if (end - start == PAGE_SIZE && !(start & ~PAGE_MASK))
    flush_tlb_page(vma, start);
    else
    flush_tlb_mm(vma.vm_mm);
    }
    EXPORT_SYMBOL(flush_tlb_range);
#[no_mangle]
pub unsafe extern "C" fn tlb_flush(tlb: *mut mmu_gather) {
    void tlb_flush(struct mmu_gather *tlb)
    {
    flush_tlb_mm(tlb.mm);
    }

#[no_mangle]
pub unsafe extern "C" fn early_init_mmu() -> void __init {
    void __init early_init_mmu(void)
    {
    let mut root: c_ulong = of_get_flat_dt_root();
    if (IS_ENABLED(CONFIG_PPC_47x) && IS_ENABLED(CONFIG_SMP) &&
    of_get_flat_dt_prop(root, "cooperative-partition", core::ptr::null_mut()))
    mmu_clear_feature(MMU_FTR_USE_TLBIVAX_BCAST);
    }
