//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/fpu/context.h
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

// Functions related to FPU context tracking
//
// The in-register FPU state for an FPU context on a CPU is assumed to be
// valid if the fpu->last_cpu matches the CPU, and the fpu_fpregs_owner_ctx
// matches the FPU.
//
// If the FPU register state is valid, the kernel can skip restoring the
// FPU state from memory.
//
// Any code that clobbers the FPU registers or updates the in-memory
// FPU state for a task MUST let the rest of the kernel know that the
// FPU registers are no longer valid for this task.
//
// Invalidate a resource you control: CPU if using the CPU for something else
// (with preemption disabled), FPU for the current task, or a task that
// is prevented from running by the current task.
//
// Internal helper for switch_fpu_return() and signal frame setup
//
// This restores _all_ xstate which has not been
// established yet.
//
// If PKRU is enabled, then the PKRU value is already
// correct because it was either set in switch_to() or in
// flush_thread(). So it is excluded because it might be
// not up to date in current->thread.fpu->xsave state.
//
// XFD state is handled in restore_fpregs_from_fpstate().
//
