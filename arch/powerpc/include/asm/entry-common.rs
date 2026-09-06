//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/entry-common.h
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
// WARN/BUG is handled with a program interrupt so minimise checks here to
// avoid recursion and maximise the chance of getting the first oops handled.
//

extern "C" {
    pub fn search_kernel_soft_mask_table(addr: c_ulong) -> bool;
}
extern "C" {
    pub fn search_kernel_restart_table(addr: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn search_kernel_soft_mask_table(_arg: regs->nip) -> return;
}

//
// Adjust the nap return address before irq_exit_rcu(). irq_exit_rcu()
// may invoke softirqs with interrupts re-enabled, allowing a nested
// async interrupt to arrive. If _TLF_NAPPING is still set at that
// point, the nested interrupt would erroneously redirect its own
// return address to power4_idle_nap_return, corrupting the stack.
//
// Can avoid a test-and-clear because NMIs do not call this

//
// Check to see if the dbcr0 register is set up to debug.
// Use the internal debug mode bit to do this.
//

// EE in HV mode sets HSRRs like 0xea0
//
// A NMI / soft-NMI interrupt may have come in after we found
// srr_valid and before the SRRs are loaded. The interrupt then
// comes in and clobbers SRRs and clears srr_valid. Then we load
// the SRRs here and test them above and find they don't match.
//
// Test validity again after that, to catch such false positives.
//
// This test in general will have some window for false negatives
// and may not catch and fix all such cases if an NMI comes in
// later and clobbers SRRs without clearing srr_valid, but hopefully
// such things will get caught most of the time, statistically
// enough to be able to get a warning out.
//

//
// If the interrupt was taken with HARD_DIS clear, then enable MSR[EE].
// Asynchronous interrupts get here with HARD_DIS set (see below), so
// this enables MSR[EE] for synchronous interrupts. IRQs remain
// soft-masked. The interrupt handler may later call
// interrupt_cond_local_irq_enable() to achieve a regular process
// context.
//
// Enable MSR[RI] early, to support kernel SLB and hash faults

//
// CT_WARN_ON comes here via program_check_exception,
// so avoid recursion.
//
// Care should be taken to note that arch_interrupt_exit_prepare and
// arch_interrupt_async_exit_prepare do not necessarily return immediately to
// regs context (e.g., if regs is usermode, we don't necessarily return to
// user mode). Other interrupts might be taken between here and return,
// context switch / preemption may occur in the exit path after this, or a
// signal may be delivered, etc.
//
// The real interrupt exit code is platform specific, e.g.,
// interrupt_exit_user_prepare / interrupt_exit_kernel_prepare for 64s.
//
// However arch_interrupt_nmi_exit_prepare does return directly to regs, because
// NMIs do not do "exit work" or replay soft-masked interrupts.
//
// We don't need to restore AMR on the way back to userspace for KUAP.
// AMR can only have been unlocked if we interrupted the kernel.
//
// irqentry_exit expects to be called with interrupts disabled

// Ensure arch_interrupt_enter_prepare does not enable MSR[EE]

//
// RI=1 is set by arch_interrupt_enter_prepare, so this thread flags access
// has to come afterward (it can cause SLB faults).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct interrupt_nmi_state {

    pub irq_soft_mask: u8,
    pub irq_happened: u8,
    pub ftrace_enabled: u8,
    pub softe: u64,

}

// Allow DEC and PMI to be traced when they are soft-NMI

//
// Set IRQS_ALL_DISABLED unconditionally so irqs_disabled() does
// the right thing, and set IRQ_HARD_DIS. We do not want to reconcile
// because that goes through irq tracing which we don't want in NMI.
//
// Adjust regs->softe to be soft-masked if it had not been
// reconcied (e.g., interrupt entry with MSR[EE]=0 but softe
// not yet set disabled), or if it was in an implicit soft
// masked state. This makes regs_irqs_disabled(regs)
// behave as expected.
//
// Don't do any per-CPU operations until interrupt state is fixed

//
// nmi does not call nap_adjust_return because nmi should not create
// new work to do (must use irq_work for that).
//

// Check we didn't change the pending interrupt mask.

//
// When entering from userspace we mostly have the AMR/IAMR
// different from kernel default values. Hence don't compare.
//

//
// This is not required for the syscall exit path, but makes the
// stack frame look nicer. If this was initialised in the first stack
// frame, or if the unwinder was taught the first stack frame always
// returns to user with IRQS_ENABLED, this store could be avoided!
//
// If system call is called with TM active, set _TIF_RESTOREALL to
// prevent RFSCV being used to return to userspace, because POWER9
// TM implementation has problems with this instruction returning to
// transactional state. Final register values are not relevant because
// the transaction will be aborted upon return anyway. Or in the case
// of unsupported_scv SIGILL fault, the return state does not much
// matter because it's an edge case.
//
// If the system call was made with a transaction active, doom it and
// return without performing the system call. Unless it was an
// unsupported scv vector, in which case it's treated like an illegal
// instruction.
//

// Enable TM in the kernel, and disable EE (for scv)
// tabort, this dooms the transaction, nothing else
//
// Userspace will never see the return value. Execution will
// resume after the tbegin. of the aborted transaction with the
// checkpointed register state. A context switch could occur
// or signal delivered to the process before resuming the
// doomed transaction context, but that should all be handled
// as expected.
//

//
// If userspace MSR has all available FP bits set,
// then they are live and no need to restore. If not,
// it means the regs were given up and restore_math
// may decide to restore them (to avoid taking an FP
// fault).
//

// Restore user access locks last

