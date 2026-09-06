//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/time/timekeeping_internal.h
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
// timekeeping debug functions
//

extern "C" {
    pub fn tk_debug_account_sleep_time(t: *const timespec64);
}

// Macro flag: #define tk_debug_account_sleep_time(x)

//
// Prevent time going backwards by checking the result against
// @max_delta. If greater, return 0.
//
// Semi public for serialization of non timekeeper VDSO updates.
extern "C" {
    pub fn timekeeper_lock_irqsave() -> c_ulong;
}
extern "C" {
    pub fn timekeeper_unlock_irqrestore(flags: c_ulong);
}
// NTP specific interface to access the current seconds value
extern "C" {
    pub fn ktime_get_ntp_seconds(id: c_uint) -> c_long;
}

extern "C" {
    pub fn update_vsyscall(tk: *mut timekeeper);
}
extern "C" {
    pub fn update_vsyscall_tz();
}
extern "C" {
    pub fn vdso_time_update_aux(tk: *mut timekeeper);
}

