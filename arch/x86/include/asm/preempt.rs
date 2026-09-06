//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/preempt.h
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
// We use the MSB for PREEMPT_NEED_RESCHED mostly because it is available.
//

//
// We use the PREEMPT_NEED_RESCHED bit as an inverted NEED_RESCHED such
// that a decrement hitting 0 means we can and should reschedule.
//

//
// We mask the PREEMPT_NEED_RESCHED bit so as not to confuse all current users
// that think a non-zero value indicates we cannot preempt.
//
// unsigned long preempt count parameter works for both 32bit and 64bit cases:
//
// - For 32bit, "int" (the return of preempt_count()) and "unsigned long" have
// the same size.
// - For 64bit, the effective bits of a preempt count sit in 32bit, and we
// preserve the NEED_RESCHED bit from the old count.
//
// must be macros to avoid header recursion hell
//

//
// We fold the NEED_RESCHED bit into the preempt count such that
// preempt_enable() can decrement and test for needing to reschedule with a
// single instruction.
//
// We invert the actual bit, so that when the decrement hits 0 we know we both
// need to resched (the bit is cleared) and can resched (no preempt count).
//
// The various preempt_count add/sub methods
//
extern "C" {
    pub fn __pc_op(_arg: add_return, _arg: __preempt_count, _arg: val) -> return;
}
extern "C" {
    pub fn __pc_op(_arg: add_return, _arg: __preempt_count, _arg: -val) -> return;
}
//
// Because we keep PREEMPT_NEED_RESCHED set when we do _not_ need to reschedule
// a decrement which hits zero means we have no preempt_count and should
// reschedule.
//
// Returns true when we need to resched and can (barring IRQ state).
//
extern "C" {
    pub fn unlikely(_arg: __pc_op(read, preempt_offset: __preempt_count) ==) -> return;
}

extern "C" {
    pub fn preempt_schedule() -> asmlinkage void;
}
extern "C" {
    pub fn preempt_schedule_thunk() -> asmlinkage void;
}

extern "C" {
    pub fn preempt_schedule_notrace() -> asmlinkage void;
}
extern "C" {
    pub fn preempt_schedule_notrace_thunk() -> asmlinkage void;
}

extern "C" {
    pub fn volatile(ASM_CALL_CONSTRAINT: "call preempt_schedule_thunk" :) -> asm;
}

extern "C" {
    pub fn volatile(ASM_CALL_CONSTRAINT: "call preempt_schedule_notrace_thunk" :) -> asm;
}

