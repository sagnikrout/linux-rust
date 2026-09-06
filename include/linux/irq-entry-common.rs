//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irq-entry-common.h
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
// Define dummy _TIF work flags if not defined by the architecture or for
// disabled functionality.
//

//
// TIF flags handled in exit_to_user_mode_loop()
//

//
// arch_enter_from_user_mode - Architecture specific sanity check for user mode regs
// @regs:	Pointer to currents pt_regs
//
// Defaults to an empty implementation. Can be replaced by architecture
// specific code.
//
// Invoked from enter_from_user_mode() in the non-instrumentable section. Use
// __always_inline so the compiler cannot push it out of line and make it
// instrumentable.
//
extern "C" {
    pub fn arch_enter_from_user_mode(regs: *mut pt_regs) -> static __always_inline void;
}

//
// arch_in_rcu_eqs - Architecture specific check for RCU extended quiescent
// states.
//
// Returns: true if the CPU is potentially in an RCU EQS, false otherwise.
//
// Architectures only need to define this if threads other than the idle thread
// may have an interruptible EQS. This does not need to handle idle threads. It
// is safe to over-estimate at the cost of redundant RCU management work.
//
// Invoked from irqentry_enter()
//

//
// enter_from_user_mode - Establish state when coming from user mode
// @regs:	Pointer to currents pt_regs
//
// Syscall/interrupt entry disables interrupts, but user mode is traced as
// interrupts enabled. Also with NO_HZ_FULL RCU might be idle.
//
// 1) Tell lockdep that interrupts are disabled
// 2) Invoke context tracking if enabled to reactivate RCU
// 3) Trace interrupts off state
//
// Invoked from architecture specific syscall entry code with interrupts
// disabled. The calling code has to be non-instrumentable. When the
// function returns all state is correct and interrupts are still
// disabled. The subsequent functions can be instrumented.
//
// This is invoked when there is architecture specific functionality to be
// done between establishing state and enabling interrupts. The caller must
// enable interrupts before invoking syscall_enter_from_user_mode_work().
//
// arch_exit_to_user_mode_work - Architecture specific TIF work for exit
// to user mode.
// @regs:	Pointer to currents pt_regs
// @ti_work:	Cached TIF flags gathered with interrupts disabled
//
// Invoked from exit_to_user_mode_loop() with interrupt enabled
//
// Defaults to NOOP. Can be supplied by architecture specific code.
//

//
// arch_exit_to_user_mode_prepare - Architecture specific preparation for
// exit to user mode.
// @regs:	Pointer to currents pt_regs
// @ti_work:	Cached TIF flags gathered with interrupts disabled
//
// Invoked from exit_to_user_mode_prepare() with interrupt disabled as the last
// function before return. Defaults to NOOP.
//

//
// arch_exit_to_user_mode - Architecture specific final work before
// exit to user mode.
//
// Invoked from exit_to_user_mode() with interrupt disabled as the last
// function before return. Defaults to NOOP.
//
// This needs to be __always_inline because it is non-instrumentable code
// invoked after context tracking switched to user mode.
//
// An architecture implementation must not do anything complex, no locking
// etc. The main purpose is for speculation mitigations.
//
extern "C" {
    pub fn arch_exit_to_user_mode() -> static __always_inline void;
}

//
// arch_do_signal_or_restart -  Architecture specific signal delivery function
// @regs:	Pointer to currents pt_regs
//
// Invoked from exit_to_user_mode_loop().
//
extern "C" {
    pub fn arch_do_signal_or_restart(regs: *mut pt_regs);
}
// Handle pending TIF work
extern "C" {
    pub fn exit_to_user_mode_loop(regs: *mut pt_regs, ti_work: c_ulong) -> c_ulong;
}
//
// __exit_to_user_mode_prepare - call exit_to_user_mode_loop() if required
// @regs:	Pointer to pt_regs on entry stack
// @work_mask:	Which TIF bits need to be evaluated
//
// 1) check that interrupts are disabled
// 2) call tick_nohz_user_enter_prepare()
// 3) call exit_to_user_mode_loop() if any flags from
// EXIT_TO_USER_MODE_WORK are set
// 4) check that interrupts are still disabled
//
// Don't invoke directly, use the syscall/irqentry_ prefixed variants below
//
// Flush pending rcuog wakeup before the last need_resched() check
// Ensure that kernel state is sane for a return to userspace
//
// syscall_exit_to_user_mode_prepare - call exit_to_user_mode_loop() if required
// @regs:	Pointer to pt_regs on entry stack
//
// Wrapper around __exit_to_user_mode_prepare() to separate the exit work for
// syscalls and interrupts.
//
// irqentry_exit_to_user_mode_prepare - call exit_to_user_mode_loop() if required
// @regs:	Pointer to pt_regs on entry stack
//
// Wrapper around __exit_to_user_mode_prepare() to separate the exit work for
// syscalls and interrupts.
//
// exit_to_user_mode - Fixup state when exiting to user mode
//
// Syscall/interrupt exit enables interrupts, but the kernel state is
// interrupts disabled when this is invoked. Also tell RCU about it.
//
// 1) Trace interrupts on state
// 2) Invoke context tracking if enabled to adjust RCU state
// 3) Invoke architecture specific last minute exit code, e.g. speculation
// mitigations, etc.: arch_exit_to_user_mode()
// 4) Tell lockdep that interrupts are enabled
//
// Invoked from architecture specific code when syscall_exit_to_user_mode()
// is not suitable as the last step before returning to userspace. Must be
// invoked with interrupts disabled and the caller must be
// non-instrumentable.
// The caller has to invoke syscall_exit_to_user_mode_work() before this.
//
// irqentry_enter_from_user_mode - Establish state before invoking the irq handler
// @regs:	Pointer to currents pt_regs
//
// Invoked from architecture specific entry code with interrupts disabled.
// Can only be called when the interrupt entry came from user mode. The
// calling code must be non-instrumentable.  When the function returns all
// state is correct and the subsequent functions can be instrumented.
//
// The function establishes state (lockdep, RCU (context tracking), tracing)
//
// irqentry_exit_to_user_mode - Interrupt exit work
// @regs:	Pointer to current's pt_regs
//
// Invoked with interrupts disabled and fully valid regs. Returns with all
// work handled, interrupts disabled such that the caller can immediately
// switch to user mode. Called from architecture specific interrupt
// handling code.
//
// The call order is #2 and #3 as described in syscall_exit_to_user_mode().
// Interrupt exit is not invoking #1 which is the syscall specific one time
// work.
//

//
// struct irqentry_state - Opaque object for exception state storage
// @exit_rcu: Used exclusively in the irqentry_*() calls; signals whether the
// exit path has to invoke ct_irq_exit().
// @lockdep: Used exclusively in the irqentry_nmi_*() calls; ensures that
// lockdep state is restored correctly on exit from nmi.
//
// This opaque object is filled in by the irqentry_*_enter() functions and
// must be passed back into the corresponding irqentry_*_exit() functions
// when the exception is complete.
//
// Callers of irqentry_*_[enter|exit]() must consider this structure opaque
// and all members private.  Descriptions of the members are provided to aid in
// the maintenance of the irqentry_*() functions.
//

//
// irqentry_exit_cond_resched - Conditionally reschedule on return from interrupt
//
// Conditional reschedule with additional sanity checks.
//
extern "C" {
    pub fn raw_irqentry_exit_cond_resched();
}

extern "C" {
    pub fn dynamic_irqentry_exit_cond_resched();
}

//
// irqentry_enter_from_kernel_mode - Establish state before invoking the irq handler
// @regs:	Pointer to currents pt_regs
//
// Invoked from architecture specific entry code with interrupts disabled.
// Can only be called when the interrupt entry came from kernel mode. The
// calling code must be non-instrumentable.  When the function returns all
// state is correct and the subsequent functions can be instrumented.
//
// The function establishes state (lockdep, RCU (context tracking), tracing) and
// is provided for architectures which require a strict split between entry from
// kernel and user mode and therefore cannot use irqentry_enter() which handles
// both entry modes.
//
// Returns: An opaque object that must be passed to irqentry_exit_to_kernel_mode().
//
// If this entry hit the idle task invoke ct_irq_enter() whether
// RCU is watching or not.
//
// Interrupts can nest when the first interrupt invokes softirq
// processing on return which enables interrupts.
//
// Scheduler ticks in the idle task can mark quiescent state and
// terminate a grace period, if and only if the timer interrupt is
// not nested into another interrupt.
//
// Checking for rcu_is_watching() here would prevent the nesting
// interrupt to invoke ct_irq_enter(). If that nested interrupt is
// the tick then rcu_flavor_sched_clock_irq() would wrongfully
// assume that it is the first interrupt and eventually claim
// quiescent state and end grace periods prematurely.
//
// Unconditionally invoke ct_irq_enter() so RCU state stays
// consistent.
//
// TINY_RCU does not support EQS, so let the compiler eliminate
// this part when enabled.
//
// If RCU is not watching then the same careful
// sequence vs. lockdep and tracing is required
// as in irqentry_enter_from_user_mode().
//
// If RCU is watching then RCU only wants to check whether it needs
// to restart the tick in NOHZ mode. rcu_irq_enter_check_tick()
// already contains a warning when RCU is not watching, so no point
// in having another one here.
//
// irqentry_exit_to_kernel_mode_preempt - Run preempt checks on return to kernel mode
// @regs:	Pointer to current's pt_regs
// @state:	Return value from matching call to irqentry_enter_from_kernel_mode()
//
// This is to be invoked before irqentry_exit_to_kernel_mode_after_preempt() to
// allow kernel preemption on return from interrupt.
//
// Must be invoked with interrupts disabled and CPU state which allows kernel
// preemption.
//
// After returning from this function, the caller can modify CPU state before
// invoking irqentry_exit_to_kernel_mode_after_preempt(), which is required to
// re-establish the tracing, lockdep and RCU state for returning to the
// interrupted context.
//
// irqentry_exit_to_kernel_mode_after_preempt - Establish trace, lockdep and RCU state
// @regs:	Pointer to current's pt_regs
// @state:	Return value from matching call to irqentry_enter_from_kernel_mode()
//
// This is to be invoked after irqentry_exit_to_kernel_mode_preempt() and before
// actually returning to the interrupted context.
//
// There are no requirements for the CPU state other than being able to complete
// the tracing, lockdep and RCU state transitions. After this function returns
// the caller must return directly to the interrupted context.
//
// If RCU was not watching on entry this needs to be done
// carefully and needs the same ordering of lockdep/tracing
// and RCU as the return to user mode path.
//
// Tell the tracer that IRET will enable interrupts
// Covers both tracing and lockdep
//
// IRQ flags state is correct already. Just tell RCU if it
// was not watching on entry.
//
// irqentry_exit_to_kernel_mode - Run preempt checks and establish state after
// invoking the interrupt handler
// @regs:	Pointer to current's pt_regs
// @state:	Return value from matching call to irqentry_enter_from_kernel_mode()
//
// This is the counterpart of irqentry_enter_from_kernel_mode() and combines
// the calls to irqentry_exit_to_kernel_mode_preempt() and
// irqentry_exit_to_kernel_mode_after_preempt().
//
// The requirement for the CPU state is that it can schedule. After the function
// returns the tracing, lockdep and RCU state transitions are completed and the
// caller must return directly to the interrupted context.
//
// irqentry_enter - Handle state tracking on ordinary interrupt entries
// @regs:	Pointer to pt_regs of interrupted context
//
// Invokes:
// - lockdep irqflag state tracking as low level ASM entry disabled
// interrupts.
//
// - Context tracking if the exception hit user mode.
//
// - The hardirq tracer to keep the state consistent as low level ASM
// entry disabled interrupts.
//
// As a precondition, this requires that the entry came from user mode,
// idle, or a kernel context in which RCU is watching.
//
// For kernel mode entries RCU handling is done conditional. If RCU is
// watching then the only RCU requirement is to check whether the tick has
// to be restarted. If RCU is not watching then ct_irq_enter() has to be
// invoked on entry and ct_irq_exit() on exit.
//
// Avoiding the ct_irq_enter/exit() calls is an optimization but also
// solves the problem of kernel mode pagefaults which can schedule, which
// is not possible after invoking ct_irq_enter() without undoing it.
//
// For user mode entries irqentry_enter_from_user_mode() is invoked to
// establish the proper context for NOHZ_FULL. Otherwise scheduling on exit
// would not be possible.
//
// Returns: An opaque object that must be passed to irqentry_exit()
//
extern "C" {
    pub fn irqentry_enter(regs: *mut pt_regs) -> irqentry_state_t noinstr;
}
//
// irqentry_exit - Handle return from exception that used irqentry_enter()
// @regs:	Pointer to pt_regs (exception entry regs)
// @state:	Return value from matching call to irqentry_enter()
//
// Depending on the return target (kernel/user) this runs the necessary
// preemption and work checks if possible and required and returns to
// the caller with interrupts disabled and no further work pending.
//
// This is the last action before returning to the low level ASM code which
// just needs to return to the appropriate context.
//
// Counterpart to irqentry_enter().
//
extern "C" {
    pub fn irqentry_exit(regs: *mut pt_regs, state: irqentry_state_t) -> void noinstr;
}
//
// irqentry_nmi_enter - Handle NMI entry
// @regs:	Pointer to currents pt_regs
//
// Similar to irqentry_enter() but taking care of the NMI constraints.
//
extern "C" {
    pub fn irqentry_nmi_enter(regs: *mut pt_regs) -> irqentry_state_t noinstr;
}
//
// irqentry_nmi_exit - Handle return from NMI handling
// @regs:	Pointer to pt_regs (NMI entry regs)
// @irq_state:	Return value from matching call to irqentry_nmi_enter()
//
// Last action before returning to the low level assembly code.
//
// Counterpart to irqentry_nmi_enter().
//
extern "C" {
    pub fn irqentry_nmi_exit(regs: *mut pt_regs, irq_state: irqentry_state_t) -> void noinstr;
}
