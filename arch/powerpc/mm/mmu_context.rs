//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/mmu_context.c
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
// Common implementation of switch_mm_irqs_off
//
// Copyright IBM Corp. 2017
//

    static inline void switch_mm_pgdir(struct task_struct *tsk,
    struct mm_struct *mm)
    {
// 32-bit keeps track of the current PGDIR in the thread struct
    tsk.thread.pgdir = mm.pgd;

    tsk.thread.sr0 = mm.context.sr0;

    tsk.thread.pid = mm.context.id;

    }

    static inline void switch_mm_pgdir(struct task_struct *tsk,
    struct mm_struct *mm)
    {
// 64-bit Book3E keeps track of current PGD in the PACA
    get_paca().pgd = mm.pgd;

    tsk.thread.pid = mm.context.id;

    }

    static inline void switch_mm_pgdir(struct task_struct *tsk,
    struct mm_struct *mm) { }

    void switch_mm_irqs_off(struct mm_struct *prev, struct mm_struct *next,
    struct task_struct *tsk)
    {
    let mut cpu: c_int = smp_processor_id();
    let mut new_on_cpu: bool = false;
// Mark this context has been used on the new CPU
    if (!cpumask_test_cpu(cpu, mm_cpumask(next))) {
    VM_WARN_ON_ONCE(next == &init_mm);
    cpumask_set_cpu(cpu, mm_cpumask(next));
    inc_mm_active_cpus(next);
//
// This full barrier orders the store to the cpumask above vs
// a subsequent load which allows this CPU/MMU to begin loading
// translations for 'next' from page table PTEs into the TLB.
//
// When using the radix MMU, that operation is the load of the
// MMU context id, which is then moved to SPRN_PID.
//
// For the hash MMU it is either the first load from slb_cache
// in switch_slb() to preload the SLBs, or the load of
// get_user_context which loads the context for the VSID hash
// to insert a new SLB, in the SLB fault handler.
//
// On the other side, the barrier is in mm/tlb-radix.c for
// radix which orders earlier stores to clear the PTEs before
// the load of mm_cpumask to check which CPU TLBs should be
// flushed. For hash, pte_xchg to clear the PTE includes the
// barrier.
//
// This full barrier is also needed by membarrier when
// switching between processes after store to rq->curr, before
// user-space memory accesses.
//
    smp_mb();
    new_on_cpu = true;
    }
// Some subarchs need to track the PGD elsewhere
    switch_mm_pgdir(tsk, next);
// Nothing else to do if we aren't actually switching
    if (prev == next)
    return;
//
// We must stop all altivec streams before changing the HW
// context
//
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    asm volatile (PPC_DSSALL);
    if (!new_on_cpu)
    membarrier_arch_switch_mm(prev, next, tsk);
//
// The actual HW switching method differs between the various
// sub architectures. Out of line for now
//
    switch_mmu_context(prev, next, tsk);
    VM_WARN_ON_ONCE(!cpumask_test_cpu(cpu, mm_cpumask(prev)));
    }

#[no_mangle]
pub unsafe extern "C" fn arch_exit_mmap(mm: *mut mm_struct) {
    void arch_exit_mmap(struct mm_struct *mm)
    {
    void *frag = pte_frag_get(&mm.context);
    if (frag)
    pte_frag_destroy(frag);
    }
