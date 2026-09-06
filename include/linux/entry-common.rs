//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/entry-common.h
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
// SYSCALL_WORK flags handled in syscall_enter_from_user_mode_work()
//

//
// SYSCALL_WORK flags handled in syscall_exit_to_user_mode()
//

//
// arch_ptrace_report_syscall_permit_entry - Architecture specific wrapper for
// ptrace_report_syscall_permit_entry()
// @regs: Pointer to the register state at syscall entry
//
// Invoked from syscall_trace_enter() to wrap ptrace_report_syscall_permit_entry().
//
// This allows architecture specific ptrace_report_syscall_permit_entry()
// implementations. If not defined by the architecture this falls back to
// to ptrace_report_syscall_permit_entry().
//
extern "C" {
    pub fn arch_ptrace_report_syscall_permit_entry(regs: *mut pt_regs) -> static __always_inline bool;
}

extern "C" {
    pub fn ptrace_report_syscall_permit_entry(_arg: regs) -> return;
}

extern "C" {
    pub fn trace_syscall_enter(regs: *mut pt_regs);
}
extern "C" {
    pub fn trace_syscall_exit(regs: *mut pt_regs, ret: c_long);
}
extern "C" {
    pub fn syscall_enter_audit(regs: *mut pt_regs);
}
//
// Handle Syscall User Dispatch.  This must comes first, since
// the ABI here can be something that doesn't make sense for
// other syscall_work features.
//
// User space got a time slice extension granted and relinquishes
// the CPU. The work stops the slice timer to avoid an extra round
// through hrtimer_interrupt().
//
// Handle ptrace
// ptrace might have changed work flags
// Do seccomp after ptrace, to catch any tracer changes.
//
// syscall_enter_from_user_mode_work - Check and handle work before invoking
// a syscall
// @regs:	Pointer to currents pt_regs
// @syscall:	The syscall number
//
// Invoked from architecture specific syscall entry code with interrupts enabled
// after invoking enter_from_user_mode(), enabling interrupts and extra
// architecture specific work with the syscall return value preset to -ENOSYS.
//
// Returns: True if the syscall should be invoked, False otherwise.
//
// If the return value is false, the caller must skip the syscall and leave the
// syscall return value unmodified as it might have been set by one of the entry
// work functions.
//
// It handles the following work items:
//
// 1) syscall_work flag dependent invocations of
// ptrace_report_syscall_permit_entry(), __seccomp_permit_syscall(), trace_sys_enter()
// 2) Invocation of audit_syscall_entry()
//
// Reread the syscall number as it might have been modified
// syscall = syscall_get_nr(current, regs);
//
// enter_from_user_mode_randomize_stack - Establish state and add stack randomization
// before invoking syscall_enter_from_user_mode_work()
// @regs:	Pointer to currents pt_regs
//
// Invoked from architecture specific syscall entry code with interrupts
// disabled. The calling code has to be non-instrumentable. When the function
// returns all state is correct, interrupts are still disabled and the
// subsequent functions can be instrumented.
//
// Implemented as a macro so that the stack randomization is effective
// throughout the function in which it is invoked. An inline would only make it
// effective in the scope of the inline function.
//

//
// syscall_enter_from_user_mode_randomize_stack - Establish state and check and handle work
// before invoking a syscall
// @regs:	Pointer to currents pt_regs
// @syscall:	The syscall number
//
// Invoked from architecture specific syscall entry code with interrupts
// disabled. The calling code has to be non-instrumentable. When the
// function returns all state is correct, interrupts are enabled and the
// subsequent functions can be instrumented.
//
// This is the combination of enter_from_user_mode_randomize_stack() and
// syscall_enter_from_user_mode_work() to be used when there is no
// architecture specific work to be done between the two.
//
// Returns: The original or a modified syscall number. See
// syscall_enter_from_user_mode_work() for further explanation.
//
// Implemented as a macro to make stack randomization effective in the calling
// scope.
//

//
// If SYSCALL_EMU is set, then the only reason to report is when SINGLESTEP is
// set (i.e. PTRACE_SYSEMU_SINGLESTEP).  This syscall instruction has been
// already reported in syscall_enter_from_user_mode_work().
//
// arch_ptrace_report_syscall_exit - Architecture specific ptrace_report_syscall_exit()
// @regs: Pointer to the register state at syscall exit
// @step: Indicates a single-step exit rather than a normal syscall exit
//
// This allows architecture specific ptrace_report_syscall_exit()
// implementations. If not defined by the architecture this falls back to
// to ptrace_report_syscall_exit().
//

//
// syscall_exit_work - Handle work before returning to user mode
// @regs:	Pointer to current pt_regs
// @work:	Current thread syscall work
//
// Do one-time syscall specific work.
//
// If the syscall was rolled back due to syscall user dispatching,
// then the tracers below are not invoked for the same reason as
// the entry side was not invoked in syscall_trace_enter(): The ABI
// of these syscalls is unknown.
//
// syscall_exit_to_user_mode_work - Handle one time work before returning to user mode
// @regs:	Pointer to currents pt_regs
//
// Step 1 of syscall_exit_to_user_mode() with the same calling convention.
//
// The caller must invoke steps 2-3 of syscall_exit_to_user_mode() afterwards.
//
// Do one-time syscall specific work. If these work items are
// enabled, we want to run them exactly once per syscall exit with
// interrupts enabled.
//
// syscall_exit_to_user_mode - Handle work before returning to user mode
// @regs:	Pointer to currents pt_regs
//
// Invoked with interrupts enabled and fully valid @regs. Returns with all
// work handled, interrupts disabled such that the caller can immediately
// switch to user mode. Called from architecture specific syscall and ret
// from fork code.
//
// The call order is:
// 1) One-time syscall exit work:
// - rseq syscall exit
// - audit
// - syscall tracing
// - ptrace (single stepping)
//
// 2) Preparatory work
// - Disable interrupts
// - Exit to user mode loop (common TIF handling). Invokes
// arch_exit_to_user_mode_work() for architecture specific TIF work
// - Architecture specific one time work arch_exit_to_user_mode_prepare()
// - Address limit and lockdep checks
//
// 3) Final transition (lockdep, tracing, context tracking, RCU), i.e. the
// functionality in exit_to_user_mode().
//
// This is a combination of syscall_exit_to_user_mode_work() (1), disabling
// interrupts followed by syscall_exit_to_user_mode_prepare() (2) and
// exit_to_user_mode() (3). This function is preferred unless there is a
// compelling architectural reason to invoke the functions separately.
//
