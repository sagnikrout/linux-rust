//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/preempt.h
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

// Use MSB for PREEMPT_NEED_RESCHED mostly because it is available.
pub const PREEMPT_NEED_RESCHED: c_uint = 0x8000000000000000UL;
//
// We use the PREEMPT_NEED_RESCHED bit as an inverted NEED_RESCHED such
// that a decrement hitting 0 means we can and should reschedule.
//

//
// We mask the PREEMPT_NEED_RESCHED bit so as not to confuse all current users
// that think a non-zero value indicates we cannot preempt.
//
// READ_ONCE(get_lowcore()->preempt.count) (without PREEMPT_NEED_RESCHED)
//
// We fold the NEED_RESCHED bit into the preempt count such that
// preempt_enable() can decrement and test for needing to reschedule with a
// short instruction sequence.
//
// We invert the actual bit, so that when the decrement hits 0 we know we both
// need to resched (the bit is cleared) and can resched (no preempt count).
//
// With some obscure config options and CONFIG_PROFILE_ALL_BRANCHES
// enabled, gcc 12 fails to handle __builtin_constant_p().
//
// Because we keep PREEMPT_NEED_RESCHED set when we do _not_ need to reschedule
// a decrement which hits zero means we have no preempt_count and should
// reschedule.
//

extern "C" {
    pub fn __atomic64_add_const_and_test(_arg: -1, )&get_lowcore()->preempt_count: *mut (long) -> return;
}

//
// Returns true when we need to resched and can (barring IRQ state).
//
extern "C" {
    pub fn unlikely(preempt_offset: READ_ONCE(get_lowcore()->preempt_count) ==) -> return;
}
extern "C" {
    pub fn __preempt_count_add_return(_arg: -val) -> return;
}

// Deferred to CPU bringup time

extern "C" {
    pub fn preempt_schedule();
}
extern "C" {
    pub fn preempt_schedule_notrace();
}

extern "C" {
    pub fn dynamic_preempt_schedule();
}
extern "C" {
    pub fn dynamic_preempt_schedule_notrace();
}

