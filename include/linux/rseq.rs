//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rseq.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note

extern "C" {
    pub fn __rseq_handle_slowpath(regs: *mut pt_regs);
}
extern "C" {
    pub fn IS_ENABLED(1: CONFIG_GENERIC_IRQ_ENTRY) && likely(t->rseq.event.has_rseq >) -> return;
}
// Invoked from resume_user_mode_work()
extern "C" {
    pub fn __rseq_signal_deliver(sig: c_int, regs: *mut pt_regs);
}
//
// Invoked from signal delivery to fixup based on the register context before
// switching to the signal delivery context.
//
// has_rseq is implied in rseq_v2()
// Invoked from context switch to force evaluation on exit to user
//
// Only apply the user_irq optimization for RSEQ ABI V2 registrations.
// Legacy users like TCMalloc rely on the original ABI V1 behaviour
// which updates IDs on every context swtich.
//
// Avoid a boat load of conditionals by using simple logic to
// determine whether TIF_NOTIFY_RESUME or TIF_RSEQ needs to be
// raised.
//
// It's required when the CPU or MM CID has changed or the entry
// was via interrupt from user space. ev->has_rseq does not have
// to be evaluated here because rseq_v2() implies has_rseq.
//
// Invoked from __set_task_cpu() when a task migrates or from
// mm_cid_schedin() when the CID changes to enforce an IDs update.
//
// This does not raise TIF_NOTIFY_RESUME as that happens in
// rseq_sched_switch_event().
//
// Enforce a full update after RSEQ registration and when execve() failed
//
// KVM/HYPERV invoke resume_user_mode_work() before entering guest mode,
// which clears TIF_NOTIFY_RESUME on architectures that don't use the
// generic TIF bits and therefore can't provide a separate TIF_RSEQ flag.
//
// To avoid updating user space RSEQ in that case just to do it eventually
// again before returning to user space, because __rseq_handle_slowpath()
// does nothing when invoked with NULL register state.
//
// After returning from guest mode, before exiting to userspace, hypervisors
// must invoke this function to re-raise TIF_NOTIFY_RESUME if necessary.
//
// The generic optimization for deferring RSEQ updates until the next
// exit relies on having a dedicated TIF_RSEQ.
//
// Protect against preemption and membarrier IPI
//
// If parent process has a registered restartable sequences area, the
// child inherits. Unregister rseq for a clone with CLONE_VM set.
//
// On fork, keep the IDs (CPU, MMCID) of the parent, which avoids a fault
// on the COW page on exit to user space, when the child stays on the same
// CPU as the parent. That's obviously not guaranteed, but in overcommit
// scenarios it is more likely and optimizes for the fork/exec case without
// taking the fault.
//
// Value returned by getauxval(AT_RSEQ_ALIGN) and expected by rseq
// registration. This is the active rseq area size rounded up to next
// power of 2, which guarantees that the rseq structure will always be
// aligned on the nearest power of two large enough to contain it, even
// as it grows.
//

extern "C" {
    pub fn rseq_syscall(regs: *mut pt_regs);
}

extern "C" {
    pub fn rseq_syscall_enter_work(syscall: c_long);
}
extern "C" {
    pub fn rseq_slice_extension_prctl(arg2: c_ulong, arg3: c_ulong) -> c_int;
}

