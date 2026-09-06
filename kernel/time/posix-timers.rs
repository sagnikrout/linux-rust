//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/time/posix-timers.h
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
pub const TIMER_RETRY: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum posix_timer_state {
    POSIX_TIMER_DISARMED,
    POSIX_TIMER_ARMED,
    POSIX_TIMER_REQUEUE_PENDING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k_clock {
    pub tp): *mut timespec64,
    pub tp): *const timespec64,
// Returns the clock value in the current time namespace.
    pub tp): *mut timespec64,
// Returns the clock value in the root time namespace.
    pub which_clock): *const *const ktime_t (clock_get_ktime)(clockid_t,
    pub tx): *const *const int (clock_adj)(clockid_t which_clock, struct __kernel_timex,
    pub timer): *mut *mut int (timer_create)(struct k_itimer,
    pub ): *const timespec64,
    pub old_setting): *mut itimerspec64,
    pub timr): *mut *mut int (timer_del)(struct k_itimer,
    pub cur_setting): *mut itimerspec64,
    pub timr): *mut *mut bool (timer_rearm)(struct k_itimer,
    pub now): *mut *mut *mut s64 (timer_forward)(struct k_itimer timr, ktime_t,
    pub now): *mut *mut *mut ktime_t (timer_remaining)(struct k_itimer timr, ktime_t,
    pub timr): *mut *mut int (timer_try_to_cancel)(struct k_itimer,
    pub sigev_none): bool absolute, bool,
    pub timr): *mut *mut void (timer_wait_running)(struct k_itimer,
}

extern "C" {
    pub fn posix_timer_queue_signal(timr: *mut k_itimer);
}
extern "C" {
    pub fn common_timer_get(timr: *mut k_itimer, cur_setting: *mut itimerspec64);
}
extern "C" {
    pub fn posix_timer_set_common(timer: *mut k_itimer, new_setting: *mut itimerspec64);
}
extern "C" {
    pub fn common_timer_del(timer: *mut k_itimer) -> c_int;
}
