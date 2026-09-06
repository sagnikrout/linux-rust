//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ptrace.h
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

// Add sp to seccomp_data, as seccomp is user API, we don't want to modify it
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_info {
    pub sp: __u64,
    pub data: seccomp_data,
}

extern "C" {
    pub fn ptracer_access_allowed(tsk: *mut task_struct) -> bool;
}
//
// Ptrace flags
//
// The owner ship rules for task->ptrace which holds the ptrace
// flags is simple.  When a task is running it owns it's task->ptrace
// flags.  When the a task is stopped the ptracer owns task->ptrace.
//
pub const PT_SEIZED: c_uint = 0x00010000	/* SEIZE used, enable new behavior */;
pub const PT_PTRACED: c_uint = 0x00000001;
pub const PT_OPT_FLAG_SHIFT: c_int = 3;
// PT_TRACE_* event enable flags

extern "C" {
    pub fn ptrace_readdata(tsk: *mut task_struct, src: c_ulong, dst: *mut char __user, len: c_int) -> c_int;
}
extern "C" {
    pub fn ptrace_writedata(tsk: *mut task_struct, src: *mut char __user, dst: c_ulong, len: c_int) -> c_int;
}
extern "C" {
    pub fn ptrace_disable(: *mut task_struct);
}
extern "C" {
    pub fn ptrace_notify(exit_code: c_int, message: c_ulong) -> c_int;
}
extern "C" {
    pub fn __ptrace_unlink(child: *mut task_struct);
}
extern "C" {
    pub fn exit_ptrace(tracer: *mut task_struct, dead: *mut list_head);
}
pub const PTRACE_MODE_READ: c_uint = 0x01;
pub const PTRACE_MODE_ATTACH: c_uint = 0x02;
pub const PTRACE_MODE_NOAUDIT: c_uint = 0x04;
pub const PTRACE_MODE_FSCREDS: c_uint = 0x08;
pub const PTRACE_MODE_REALCREDS: c_uint = 0x10;
// shorthands for READ/ATTACH and FSCREDS/REALCREDS combinations

//
// ptrace_may_access - check whether the caller is permitted to access
// a target task.
// @task: target task
// @mode: selects type of access and caller credentials
//
// Returns true on success, false on denial.
//
// One of the flags PTRACE_MODE_FSCREDS and PTRACE_MODE_REALCREDS must
// be set in @mode to specify whether the access was requested through
// a filesystem syscall (should use effective capabilities and fsuid
// of the caller) or through an explicit syscall such as
// process_vm_writev or ptrace (and should use the real credentials).
//
extern "C" {
    pub fn ptrace_may_access(task: *mut task_struct, mode: c_uint) -> bool;
}
//
// ptrace_parent - return the task that is tracing the given task
// @task: task to consider
//
// Returns %NULL if no one is tracing @task, or the &struct task_struct
// pointer to its tracer.
//
// Must called under rcu_read_lock().  The pointer returned might be kept
// live only by RCU.  During exec, this may be called with task_lock() held
// on @task, still held from when check_unsafe_exec() was called.
//
extern "C" {
    pub fn rcu_dereference(_arg: task->parent) -> return;
}
//
// ptrace_event_enabled - test whether a ptrace event is enabled
// @task: ptracee of interest
// @event: %PTRACE_EVENT_* to test
//
// Test whether @event is enabled for ptracee @task.
//
// Returns %true if @event is enabled, %false otherwise.
//
// ptrace_event - possibly stop for a ptrace event notification
// @event:	%PTRACE_EVENT_* value to report
// @message:	value for %PTRACE_GETEVENTMSG to return
//
// Check whether @event is enabled and, if so, report @event and @message
// to the ptrace parent.
//
// Called without locks.
//
// legacy EXEC report via SIGTRAP
//
// ptrace_event_pid - possibly stop for a ptrace event notification
// @event:	%PTRACE_EVENT_* value to report
// @pid:	process identifier for %PTRACE_GETEVENTMSG to return
//
// Check whether @event is enabled and, if so, report @event and @pid
// to the ptrace parent.  @pid is reported as the pid_t seen from the
// ptrace parent's pid namespace.
//
// Called without locks.
//
// FIXME: There's a potential race if a ptracer in a different pid
// namespace than parent attaches between computing message below and
// when we acquire tasklist_lock in ptrace_stop().  If this happens,
// the ptracer will get a bogus pid from PTRACE_GETEVENTMSG.
//
// ptrace_init_task - initialize ptrace state for a new child
// @child:		new child task
// @ptrace:		true if child should be ptrace'd by parent's tracer
//
// This is called immediately after adding @child to its parent's children
// list.  @ptrace is false in the normal case, and true to ptrace @child.
//
// Called with current's siglock and write_lock_irq(&tasklist_lock) held.
//
// ptrace_release_task - final ptrace-related cleanup of a zombie being reaped
// @task:	task in %EXIT_DEAD state
//
// Called with write_lock(&tasklist_lock) held.
//

//
// System call handlers that, upon successful completion, need to return a
// negative value should call force_successful_syscall_return() right before
// returning.  On architectures where the syscall convention provides for a
// separate error flag (e.g., alpha, ia64, ppc{,64}, sparc{,64}, possibly
// others), this macro can be used to ensure that the error flag will not get
// set.  On architectures which do not support a separate error flag, the macro
// is a no-op and the spurious error condition needs to be filtered out by some
// other means (e.g., in user-level, by passing an extra argument to the
// syscall handler, or something along those lines).
//

//
// On most systems we can tell if a syscall is a success based on if the retval
// is an error value.  On some systems like ia64 and powerpc they have different
// indicators of success/failure and must define their own.
//

//
// <asm/ptrace.h> should define the following things inside #ifdef __KERNEL__.
//
// These do-nothing inlines are used when the arch does not
// implement single-step.  The kerneldoc comments are here
// to document the interface for all arch definitions.
//

//
// arch_has_single_step - does this CPU support user-mode single-step?
//
// If this is defined, then there must be function declarations or
// inlines for user_enable_single_step() and user_disable_single_step().
// arch_has_single_step() should evaluate to nonzero iff the machine
// supports instruction single-step for user mode.
// It can be a constant or it can test a CPU feature bit.
//

//
// user_enable_single_step - single-step in user-mode task
// @task: either current or a task stopped in %TASK_TRACED
//
// This can only be called when arch_has_single_step() has returned nonzero.
// Set @task so that when it returns to user mode, it will trap after the
// next single instruction executes.  If arch_has_block_step() is defined,
// this must clear the effects of user_enable_block_step() too.
//
// user_disable_single_step - cancel user-mode single-step
// @task: either current or a task stopped in %TASK_TRACED
//
// Clear @task of the effects of user_enable_single_step() and
// user_enable_block_step().  This can be called whether or not either
// of those was ever called on @task, and even if arch_has_single_step()
// returned zero.
//

extern "C" {
    pub fn user_enable_single_step(: *mut task_struct);
}
extern "C" {
    pub fn user_disable_single_step(: *mut task_struct);
}

//
// arch_has_block_step - does this CPU support user-mode block-step?
//
// If this is defined, then there must be a function declaration or inline
// for user_enable_block_step(), and arch_has_single_step() must be defined
// too.  arch_has_block_step() should evaluate to nonzero iff the machine
// supports step-until-branch for user mode.  It can be a constant or it
// can test a CPU feature bit.
//

//
// user_enable_block_step - step until branch in user-mode task
// @task: either current or a task stopped in %TASK_TRACED
//
// This can only be called when arch_has_block_step() has returned nonzero,
// and will never be called when single-instruction stepping is being used.
// Set @task so that when it returns to user mode, it will trap after the
// next branch or trap taken.
//

extern "C" {
    pub fn user_enable_block_step(: *mut task_struct);
}

extern "C" {
    pub fn user_single_step_report(regs: *mut pt_regs);
}

//
// arch_ptrace_stop_needed - Decide whether arch_ptrace_stop() should be called
//
// This is called with the siglock held, to decide whether or not it's
// necessary to release the siglock and call arch_ptrace_stop().  It can be
// defined to a constant if arch_ptrace_stop() is never required, or always
// is.  On machines where this makes sense, it should be defined to a quick
// test to optimize out calling arch_ptrace_stop() when it would be
// superfluous.  For example, if the thread has not been back to user mode
// since the last stop, the thread state might indicate that nothing needs
// to be done.
//
// This is guaranteed to be invoked once before a task stops for ptrace and
// may include arch-specific operations necessary prior to a ptrace stop.
//

//
// arch_ptrace_stop - Do machine-specific work before stopping for ptrace
//
// This is called with no locks held when arch_ptrace_stop_needed() has
// just returned nonzero.  It is allowed to block, e.g. for user memory
// access.  The arch can have machine-specific work to be done before
// ptrace stops.  On ia64, register backing store gets written back to user
// memory here.  Since this can be costly (requires dropping the siglock),
// we only do it when the arch requires it for this particular stop, as
// indicated by arch_ptrace_stop_needed().
//

extern "C" {
    pub fn task_current_syscall(target: *mut task_struct, info: *mut syscall_info) -> c_int;
}
extern "C" {
    pub fn sigaction_compat_abi(act: *mut k_sigaction, oact: *mut k_sigaction);
}
//
// ptrace report for syscall entry and exit looks identical.
//
// this isn't the same as continuing with a signal, but it will do
// for normal use.  strace only continues with a signal if the
// stopping signal is not SIGTRAP.  -brl
//
// ptrace_report_syscall_permit_entry - task is about to attempt a system call
// @regs:		user register state of current task
//
// This will be called if %SYSCALL_WORK_SYSCALL_TRACE or
// %SYSCALL_WORK_SYSCALL_EMU have been set, when the current task has just
// entered the kernel for a system call.  Full user register state is
// available here.  Changing the values in @regs can affect the system
// call number and arguments to be tried.  It is safe to block here,
// preventing the system call from beginning.
//
// Returns True normally, or False if the calling architecture code should abort
// the system call.  That must prevent normal entry so no system call is
// made.  If @task ever returns to user mode after this, its register state
// is unspecified, but should be something harmless like an %ENOSYS error
// return.  It should preserve enough information so that syscall_rollback()
// can work (see asm-generic/syscall.h).
//
// Called without locks, just after entering kernel mode.
//
extern "C" {
    pub fn ptrace_report_syscall(_arg: PTRACE_EVENTMSG_SYSCALL_ENTRY) -> return;
}
//
// ptrace_report_syscall_exit - task has just finished a system call
// @regs:		user register state of current task
// @step:		nonzero if simulating single-step or block-step
//
// This will be called if %SYSCALL_WORK_SYSCALL_TRACE has been set, when
// the current task has just finished an attempted system call.  Full
// user register state is available here.  It is safe to block here,
// preventing signals from being processed.
//
// If @step is nonzero, this report is also in lieu of the normal
// trap that would follow the system call instruction because
// user_enable_block_step() or user_enable_single_step() was used.
// In this case, %SYSCALL_WORK_SYSCALL_TRACE might not be set.
//
// Called without locks, just before checking for pending signals.
//
