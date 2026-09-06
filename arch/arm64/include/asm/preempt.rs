//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/preempt.h
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
    pub fn READ_ONCE(_arg: current_thread_info()->preempt.count) -> return;
}
// Preserve existing value of PREEMPT_NEED_RESCHED

// Update only the count field, leaving need_resched unchanged
//
// If we wrote back all zeroes, then we're preemptible and in
// need of a reschedule. Otherwise, we need to reload the
// preempt_count in case the need_resched flag was cleared by an
// interrupt occurring between the non-atomic READ_ONCE/WRITE_ONCE
// pair.
//

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

