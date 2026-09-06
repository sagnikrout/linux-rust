//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/percpu.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2013 ARM Ltd.
//

//
// Non-VHE hyp code runs with preemption disabled. No need to hazard
// the register access against barrier() as in __kern_my_cpu_offset.
//
extern "C" {
    pub fn read_sysreg(_arg: tpidr_el2) -> return;
}
//
// We want to allow caching the value, so avoid using volatile and
// instead use a fake stack read to hazard against barrier().
//

// LL/SC */							\

// LSE atomics */						\

// LL/SC */							\

// LSE atomics */						\

//
// Use value-returning atomics for CPU-local ops as they are more likely
// to execute "near" to the CPU (e.g. in L1$).
//
// https://lore.kernel.org/r/e7d539ed-ced0-4b96-8ecd-048a5b803b85@paulmck-laptop
//

//
// It would be nice to avoid the conditional call into the scheduler when
// re-enabling preemption for preemptible kernels, but doing that in a way
// which builds inside a module would mean messing directly with the preempt
// count. If you do this, peterz and tglx will hunt you down.
//
// Not to mention it'll break the actual preemption model for missing a
// preemption point when TIF_NEED_RESCHED gets set while preemption is
// disabled.
//

extern "C" {
    pub fn __hyp_per_cpu_offset(cpu: c_uint) -> c_ulong;
}
// Macro flag: #define __per_cpu_offset

// Redefine macros for nVHE hyp under DEBUG_PREEMPT to avoid its dependencies.

