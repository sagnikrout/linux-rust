//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/mm/fault.c
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
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//
// Derived from MIPS:
// Copyright (C) 1995 - 2000 by Ralf Baechle
//

    let mut show_unhandled_signals: c_int = 1;
#[no_mangle]
unsafe extern "C" fn spurious_fault(write: c_ulong, address: c_ulong) -> int __kprobes {
    static int __kprobes spurious_fault(unsigned long write, unsigned long address)
    {
    pgd_t *pgd;
    p4d_t *p4d;
    pud_t *pud;
    pmd_t *pmd;
    pte_t *pte;
    if (!(address & __UA_LIMIT))
    return 0;
    pgd = pgd_offset_k(address);
    if (!pgd_present(pgdp_get(pgd)))
    return 0;
    p4d = p4d_offset(pgd, address);
    if (!p4d_present(p4dp_get(p4d)))
    return 0;
    pud = pud_offset(p4d, address);
    if (!pud_present(pudp_get(pud)))
    return 0;
    pmd = pmd_offset(pud, address);
    if (!pmd_present(pmdp_get(pmd)))
    return 0;
    if (pmd_leaf(*pmd)) {
    return write ? pmd_write(pmdp_get(pmd)) : 1;
    } else {
    pte = pte_offset_kernel(pmd, address);
    if (!pte_present(ptep_get(pte)))
    return 0;
    return write ? pte_write(ptep_get(pte)) : 1;
    }
    }
    static void __kprobes no_context(struct pt_regs *regs,
    unsigned long write, unsigned long address)
    {
    let mut field: c_int = sizeof(unsigned long) * 2;
    if (spurious_fault(write, address))
    return;
// Are we prepared to handle this kernel fault?
    if (fixup_exception(regs))
    return;
    if (kfence_handle_page_fault(address, write, regs))
    return;
//
// Oops. The kernel tried to access some bad page. We'll have to
// terminate things with extreme prejudice.
//
    bust_spinlocks(1);
    pr_alert("CPU %d Unable to handle kernel paging request at "
    "virtual address %0*lx, era == %0*lx, ra == %0*lx\n",
    raw_smp_processor_id(), field, address, field, regs.csr_era,
    field,  regs.regs[1]);
    die("Oops", regs);
    }
    static void __kprobes do_out_of_memory(struct pt_regs *regs,
    unsigned long write, unsigned long address)
    {
//
// We ran out of memory, call the OOM killer, and return the userspace
// (which will retry the fault, or kill us if we got oom-killed).
//
    if (!user_mode(regs)) {
    no_context(regs, write, address);
    return;
    }
    pagefault_out_of_memory();
    }
    static void __kprobes do_sigbus(struct pt_regs *regs,
    unsigned long write, unsigned long address, int si_code)
    {
// Kernel mode? Handle exceptions or die
    if (!user_mode(regs)) {
    no_context(regs, write, address);
    return;
    }
//
// Send a sigbus, regardless of whether we were in kernel
// or user mode.
//
    current.thread.csr_badvaddr = address;
    current.thread.trap_nr = read_csr_excode();
    force_sig_fault(SIGBUS, BUS_ADRERR, (void __user *)address);
    }
    static void __kprobes do_sigsegv(struct pt_regs *regs,
    unsigned long write, unsigned long address, int si_code)
    {
    let mut field: c_int = sizeof(unsigned long) * 2;
    static DEFINE_RATELIMIT_STATE(ratelimit_state, 5 * HZ, 10);
// Kernel mode? Handle exceptions or die
    if (!user_mode(regs)) {
    no_context(regs, write, address);
    return;
    }
// User mode accesses just cause a SIGSEGV
    current.thread.csr_badvaddr = address;
    if (!write)
    current.thread.error_code = 1;
    else
    current.thread.error_code = 2;
    current.thread.trap_nr = read_csr_excode();
    if (show_unhandled_signals &&
    unhandled_signal(current, SIGSEGV) && __ratelimit(&ratelimit_state)) {
    pr_info("do_page_fault(): sending SIGSEGV to %s for invalid %s %0*lx\n",
    current.comm,
    write ? "write access to" : "read access from",
    field, address);
    pr_info("era = %0*lx in", field,
    (unsigned long) regs.csr_era);
    print_vma_addr(KERN_CONT " ", regs.csr_era);
    pr_cont("\n");
    pr_info("ra  = %0*lx in", field,
    (unsigned long) regs.regs[1]);
    print_vma_addr(KERN_CONT " ", regs.regs[1]);
    pr_cont("\n");
    }
    force_sig_fault(SIGSEGV, si_code, (void __user *)address);
    }
//
// This routine handles page faults.  It determines the address,
// and the problem, and then passes it off to one of the appropriate
// routines.
//
    static void __kprobes __do_page_fault(struct pt_regs *regs,
    unsigned long write, unsigned long address)
    {
    let mut si_code: c_int = SEGV_MAPERR;
    let mut flags: c_uint = FAULT_FLAG_DEFAULT;
    struct task_struct *tsk = current;
    struct mm_struct *mm = tsk.mm;
    struct vm_area_struct *vma = core::ptr::null_mut();
    vm_fault_t fault;
    if (kprobe_page_fault(regs, current.thread.trap_nr))
    return;
//
// We fault-in kernel-space virtual memory on-demand. The
// 'reference' page table is init_mm.pgd.
//
// NOTE! We MUST NOT take any locks for this case. We may
// be in an interrupt or a critical region, and should
// only copy the information from the master page table,
// nothing more.
//
    if (address & __UA_LIMIT) {
    if (!user_mode(regs))
    no_context(regs, write, address);
    else
    do_sigsegv(regs, write, address, si_code);
    return;
    }
//
// If we're in an interrupt or have no user
// context, we must not take the fault..
//
    if (faulthandler_disabled() || !mm) {
    do_sigsegv(regs, write, address, si_code);
    return;
    }
    if (user_mode(regs))
    flags |= FAULT_FLAG_USER;
    perf_sw_event(PERF_COUNT_SW_PAGE_FAULTS, 1, regs, address);
    if (!(flags & FAULT_FLAG_USER))
    goto lock_mmap;
    vma = lock_vma_under_rcu(mm, address);
    if (!vma)
    goto lock_mmap;
    if (write) {
    flags |= FAULT_FLAG_WRITE;
    if (!(vma.vm_flags & VM_WRITE)) {
    vma_end_read(vma);
    si_code = SEGV_ACCERR;
    count_vm_vma_lock_event(VMA_LOCK_SUCCESS);
    goto bad_area_nosemaphore;
    }
    } else {
    if (!(vma.vm_flags & VM_EXEC) && address == exception_era(regs)) {
    vma_end_read(vma);
    si_code = SEGV_ACCERR;
    count_vm_vma_lock_event(VMA_LOCK_SUCCESS);
    goto bad_area_nosemaphore;
    }
    if (!(vma.vm_flags & (VM_READ | VM_WRITE)) && address != exception_era(regs)) {
    vma_end_read(vma);
    si_code = SEGV_ACCERR;
    count_vm_vma_lock_event(VMA_LOCK_SUCCESS);
    goto bad_area_nosemaphore;
    }
    }
    fault = handle_mm_fault(vma, address, flags | FAULT_FLAG_VMA_LOCK, regs);
    if (!(fault & (VM_FAULT_RETRY | VM_FAULT_COMPLETED)))
    vma_end_read(vma);
    if (!(fault & VM_FAULT_RETRY)) {
    count_vm_vma_lock_event(VMA_LOCK_SUCCESS);
    goto done;
    }
    count_vm_vma_lock_event(VMA_LOCK_RETRY);
    if (fault & VM_FAULT_MAJOR)
    flags |= FAULT_FLAG_TRIED;
// Quick path to respond to signals
    if (fault_signal_pending(fault, regs)) {
    if (!user_mode(regs))
    no_context(regs, write, address);
    return;
    }
    lock_mmap:
    retry:
    vma = lock_mm_and_find_vma(mm, address, regs);
    if (unlikely(!vma))
    goto bad_area_nosemaphore;
    goto good_area;
//
// Something tried to access memory that isn't in our memory map..
// Fix it, but check if it's kernel or user first..
//
    bad_area:
    mmap_read_unlock(mm);
    bad_area_nosemaphore:
    do_sigsegv(regs, write, address, si_code);
    return;
//
// Ok, we have a good vm_area for this memory access, so
// we can handle it..
//
    good_area:
    si_code = SEGV_ACCERR;
    if (write) {
    flags |= FAULT_FLAG_WRITE;
    if (!(vma.vm_flags & VM_WRITE))
    goto bad_area;
    } else {
    if (!(vma.vm_flags & VM_EXEC) && address == exception_era(regs))
    goto bad_area;
    if (!(vma.vm_flags & (VM_READ | VM_WRITE)) && address != exception_era(regs))
    goto bad_area;
    }
//
// If for any reason at all we couldn't handle the fault,
// make sure we exit gracefully rather than endlessly redo
// the fault.
//
    fault = handle_mm_fault(vma, address, flags, regs);
    if (fault_signal_pending(fault, regs)) {
    if (!user_mode(regs))
    no_context(regs, write, address);
    return;
    }
// The fault is fully completed (including releasing mmap lock)
    if (fault & VM_FAULT_COMPLETED)
    return;
    if (unlikely(fault & VM_FAULT_RETRY)) {
    flags |= FAULT_FLAG_TRIED;
//
// No need to mmap_read_unlock(mm) as we would
// have already released it in __lock_page_or_retry
// in mm/filemap.c.
//
    goto retry;
    }
    mmap_read_unlock(mm);
    done:
    if (unlikely(fault & VM_FAULT_ERROR)) {
    if (fault & VM_FAULT_OOM) {
    do_out_of_memory(regs, write, address);
    return;
    } else if (fault & VM_FAULT_SIGSEGV) {
    do_sigsegv(regs, write, address, si_code);
    return;
    } else if (fault & (VM_FAULT_SIGBUS|VM_FAULT_HWPOISON|VM_FAULT_HWPOISON_LARGE)) {
    do_sigbus(regs, write, address, si_code);
    return;
    }
    BUG();
    }
    }
    asmlinkage void __kprobes do_page_fault(struct pt_regs *regs,
    unsigned long write, unsigned long address)
    {
    let mut state: irqentry_state_t = irqentry_enter(regs);
// Enable interrupt if enabled in parent context
    if (likely(regs.csr_prmd & CSR_PRMD_PIE))
    local_irq_enable();
    __do_page_fault(regs, write, address);
    local_irq_disable();
    irqentry_exit(regs, state);
    }
