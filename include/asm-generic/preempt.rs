//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/preempt.h
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

extern "C" {
    pub fn READ_ONCE(_arg: current_thread_info()->preempt_count) -> return;
}
// preempt_count_ptr() = pc;
//
// must be macros to avoid header recursion hell
//

//
// The various preempt_count add/sub methods
//
// preempt_count_ptr() += val;
// preempt_count_ptr() -= val;
// preempt_count_ptr() += val;
// preempt_count_ptr() -= val;
//
// Because of load-store architectures cannot do per-cpu atomic
// operations; we cannot use PREEMPT_NEED_RESCHED because it might get
// lost.
//
// Returns true when we need to resched and can (barring IRQ state).
//

extern "C" {
    pub fn preempt_schedule() -> asmlinkage void;
}
extern "C" {
    pub fn preempt_schedule_notrace() -> asmlinkage void;
}

extern "C" {
    pub fn dynamic_preempt_schedule();
}
extern "C" {
    pub fn dynamic_preempt_schedule_notrace();
}

