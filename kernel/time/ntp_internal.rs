//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/time/ntp_internal.h
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
    pub fn ntp_init();
}
extern "C" {
    pub fn ntp_clear(tkid: c_uint, cs_tick_adj: i64);
}
// Returns how long ticks are at present, in ns / 2^NTP_SCALE_SHIFT.
extern "C" {
    pub fn ntp_tick_length(tkid: c_uint) -> u64;
}
extern "C" {
    pub fn ntp_get_skew_delta(tkid: c_uint) -> i64;
}
extern "C" {
    pub fn ntp_drain_skew(tkid: c_uint, amount: i64, shift: c_uint) -> i64;
}
extern "C" {
    pub fn ntp_get_next_leap(tkid: c_uint) -> ktime_t;
}
extern "C" {
    pub fn second_overflow(tkid: c_uint, secs: time64_t) -> c_int;
}
extern "C" {
    pub fn __hardpps(phase_ts: *const timespec64, raw_ts: *const timespec64);
}

extern "C" {
    pub fn ntp_notify_cmos_timer(offset_set: bool);
}

