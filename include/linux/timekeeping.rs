//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/timekeeping.h
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

// Included from linux/ktime.h
extern "C" {
    pub fn timekeeping_init();
}
// Architecture timer tick functions:
extern "C" {
    pub fn legacy_timer_tick(ticks: c_ulong);
}
//
// Get and set timeofday
//
extern "C" {
    pub fn do_settimeofday64(ts: *const timespec64) -> c_int;
}
//
// ktime_get() family - read the current time in a multitude of ways.
//
// The default time reference is CLOCK_MONOTONIC, starting at
// boot time but not counting the time spent in suspend.
// For other references, use the functions with "real", "clocktai",
// "boottime" and "raw" suffixes.
//
// To get the time in a different format, use the ones with
// "ns", "ts64" and "seconds" suffix.
//
// See Documentation/core-api/timekeeping.rst for more details.
//
// timespec64 based interfaces
//
extern "C" {
    pub fn ktime_get_raw_ts64(ts: *mut timespec64);
}
extern "C" {
    pub fn ktime_get_ts64(ts: *mut timespec64);
}
extern "C" {
    pub fn ktime_get_real_ts64(tv: *mut timespec64);
}
extern "C" {
    pub fn ktime_get_coarse_ts64(ts: *mut timespec64);
}
extern "C" {
    pub fn ktime_get_coarse_real_ts64(ts: *mut timespec64);
}
// Multigrain timestamp interfaces
extern "C" {
    pub fn ktime_get_coarse_real_ts64_mg(ts: *mut timespec64);
}
extern "C" {
    pub fn ktime_get_real_ts64_mg(ts: *mut timespec64);
}
extern "C" {
    pub fn timekeeping_get_mg_floor_swaps() -> c_ulong;
}
extern "C" {
    pub fn getboottime64(ts: *mut timespec64);
}
//
// time64_t base interfaces
//
extern "C" {
    pub fn ktime_get_seconds() -> time64_t;
}
extern "C" {
    pub fn __ktime_get_real_seconds() -> time64_t;
}
extern "C" {
    pub fn ktime_get_real_seconds() -> time64_t;
}
//
// ktime_t based interfaces
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tk_offsets {
    TK_OFFS_REAL,
    TK_OFFS_BOOT,
    TK_OFFS_TAI,
    TK_OFFS_MAX,
}

extern "C" {
    pub fn ktime_get() -> ktime_t;
}
extern "C" {
    pub fn ktime_get_with_offset(offs: tk_offsets) -> ktime_t;
}
extern "C" {
    pub fn ktime_get_coarse_with_offset(offs: tk_offsets) -> ktime_t;
}
extern "C" {
    pub fn ktime_mono_to_any(tmono: ktime_t, offs: tk_offsets) -> ktime_t;
}
extern "C" {
    pub fn ktime_get_raw() -> ktime_t;
}
extern "C" {
    pub fn ktime_get_resolution_ns() -> u32;
}
//
// ktime_get_real - get the real (wall-) time in ktime_t format
//
// Returns: real (wall) time in ktime_t format
//
extern "C" {
    pub fn ktime_get_with_offset(_arg: TK_OFFS_REAL) -> return;
}
extern "C" {
    pub fn ktime_get_coarse_with_offset(_arg: TK_OFFS_REAL) -> return;
}
//
// ktime_get_boottime - Get monotonic time since boot in ktime_t format
//
// This is similar to CLOCK_MONTONIC/ktime_get, but also includes the
// time spent in suspend.
//
// Returns: monotonic time since boot in ktime_t format
//
extern "C" {
    pub fn ktime_get_with_offset(_arg: TK_OFFS_BOOT) -> return;
}
extern "C" {
    pub fn ktime_get_coarse_with_offset(_arg: TK_OFFS_BOOT) -> return;
}
//
// ktime_get_clocktai - Get the TAI time of day in ktime_t format
//
// Returns: the TAI time of day in ktime_t format
//
extern "C" {
    pub fn ktime_get_with_offset(_arg: TK_OFFS_TAI) -> return;
}
extern "C" {
    pub fn ktime_get_coarse_with_offset(_arg: TK_OFFS_TAI) -> return;
}
extern "C" {
    pub fn timespec64_to_ktime(_arg: ts) -> return;
}
extern "C" {
    pub fn ktime_to_ns(_arg: ktime_get_coarse()) -> return;
}
extern "C" {
    pub fn ktime_to_ns(_arg: ktime_get_coarse_real()) -> return;
}
extern "C" {
    pub fn ktime_to_ns(_arg: ktime_get_coarse_boottime()) -> return;
}
extern "C" {
    pub fn ktime_to_ns(_arg: ktime_get_coarse_clocktai()) -> return;
}
//
// ktime_mono_to_real - Convert monotonic time to clock realtime
// @mono: monotonic time to convert
//
// Returns: time converted to realtime clock
//
extern "C" {
    pub fn ktime_mono_to_any(_arg: mono, _arg: TK_OFFS_REAL) -> return;
}
//
// ktime_get_ns - Get the current time in nanoseconds
//
// Returns: current time converted to nanoseconds
//
extern "C" {
    pub fn ktime_to_ns(_arg: ktime_get()) -> return;
}
//
// ktime_get_real_ns - Get the current real/wall time in nanoseconds
//
// Returns: current real time converted to nanoseconds
//
extern "C" {
    pub fn ktime_to_ns(_arg: ktime_get_real()) -> return;
}
//
// ktime_get_boottime_ns - Get the monotonic time since boot in nanoseconds
//
// Returns: current boottime converted to nanoseconds
//
extern "C" {
    pub fn ktime_to_ns(_arg: ktime_get_boottime()) -> return;
}
//
// ktime_get_clocktai_ns - Get the current TAI time of day in nanoseconds
//
// Returns: current TAI time converted to nanoseconds
//
extern "C" {
    pub fn ktime_to_ns(_arg: ktime_get_clocktai()) -> return;
}
//
// ktime_get_raw_ns - Get the raw monotonic time in nanoseconds
//
// Returns: current raw monotonic time converted to nanoseconds
//
extern "C" {
    pub fn ktime_to_ns(_arg: ktime_get_raw()) -> return;
}
extern "C" {
    pub fn ktime_get_mono_fast_ns() -> u64;
}
extern "C" {
    pub fn ktime_get_raw_fast_ns() -> u64;
}
extern "C" {
    pub fn ktime_get_boot_fast_ns() -> u64;
}
extern "C" {
    pub fn ktime_get_tai_fast_ns() -> u64;
}
extern "C" {
    pub fn ktime_get_real_fast_ns() -> u64;
}
//
// timespec64/time64_t interfaces utilizing the ktime based ones
// for API completeness, these could be implemented more efficiently
// if needed.
//
// ts = ktime_to_timespec64(ktime_get_boottime());
// ts = ktime_to_timespec64(ktime_get_coarse_boottime());
extern "C" {
    pub fn ktime_divns(_arg: ktime_get_coarse_boottime(), _arg: NSEC_PER_SEC) -> return;
}
// ts = ktime_to_timespec64(ktime_get_clocktai());
// ts = ktime_to_timespec64(ktime_get_coarse_clocktai());
extern "C" {
    pub fn ktime_divns(_arg: ktime_get_coarse_clocktai(), _arg: NSEC_PER_SEC) -> return;
}
//
// RTC specific
//
extern "C" {
    pub fn timekeeping_rtc_skipsuspend() -> bool;
}
extern "C" {
    pub fn timekeeping_rtc_skipresume() -> bool;
}
extern "C" {
    pub fn timekeeping_inject_sleeptime64(delta: *const timespec64);
}
//
// Auxiliary clock interfaces
//

extern "C" {
    pub fn ktime_get_aux(id: clockid_t, kt: *mut ktime_t) -> bool __must_check;
}
extern "C" {
    pub fn ktime_get_aux_ts64(id: clockid_t, kt: *mut timespec64) -> bool __must_check;
}

//
// struct system_time_snapshot - Simultaneous time capture of monotonic raw time,
// a selected CLOCK_* and the clocksource counter value
// @cycles:		Clocksource counter value to produce the system times
// @hw_cycles:		For derived clocksources, the hardware counter value from
// which @cycles was derived
// @systime:		The system time of the selected CLOCK ID
// @monoraw:		Monotonic raw system time
// @cs_id:		Clocksource ID
// @hw_csid:		Clocksource ID of the underlying hardware counter for derived
// clocksources which implement the read_snapshot() callback.
// @clock_was_set_seq:	The sequence number of clock-was-set events
// @cs_was_changed_seq:	The sequence number of clocksource change events
// @valid:		True if the snapshot is valid
//
// @monoraw is CLOCK_MONOTONIC_RAW for system time CLOCK ids. For CLOCK_AUX$N
// clock ids it's the monotonic raw time related to the AUX clock, which is
// CLOCK_MONOTONIC_RAW plus a AUX clock specific offset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_time_snapshot {
    pub cycles: u64,
    pub hw_cycles: u64,
    pub systime: ktime_t,
    pub monoraw: ktime_t,
    pub cs_id: clocksource_ids,
    pub hw_csid: clocksource_ids,
    pub clock_was_set_seq: u32,
    pub cs_was_changed_seq: u8,
    pub valid: u8,
}

//
// struct system_counterval_t - system counter value with the ID of the
// corresponding clocksource
// @cycles:	System counter value
// @cs_id:	Clocksource ID corresponding to system counter value. Used by
// timekeeping code to verify comparability of two cycle values.
// The default ID, CSID_GENERIC, does not identify a specific
// clocksource.
// @use_nsecs:	@cycles is in nanoseconds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_counterval_t {
    pub cycles: u64,
    pub cs_id: clocksource_ids,
    pub use_nsecs: bool,
}

//
// struct system_device_crosststamp - system/device cross-timestamp
// (synchronized capture)
// @clock_id:		System time Clock ID to capture
// @device:		Device time
// @sys_counter:	Clocksource counter value simultaneous with device time
// @sys_systime:	System time for @clock_id
// @sys_monoraw:	Monotonic raw simultaneous with device time
//
// @sys_monoraw is CLOCK_MONOTONIC_RAW for system time CLOCK ids. For
// CLOCK_AUX$N clock ids it's the monotonic raw time related to the AUX clock,
// which is CLOCK_MONOTONIC_RAW plus a AUX clock specific offset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct system_device_crosststamp {
    pub clock_id: clockid_t,
    pub device: ktime_t,
    pub sys_counter: system_counterval_t,
    pub sys_systime: ktime_t,
    pub sys_monoraw: ktime_t,
}

extern "C" {
    pub fn timekeeping_clocksource_has_base(id: clocksource_ids) -> bool;
}
//
// Get cross timestamp between system clock and device clock
//
// Simultaneously snapshot a given clock with MONOTONIC_RAW and the underlying
// clocksource counter value.
//
extern "C" {
    pub fn ktime_get_snapshot_id(clock_id: clockid_t, systime_snapshot: *mut system_time_snapshot);
}
//
// Persistent clock related interfaces
//
extern "C" {
    pub fn read_persistent_clock64(ts: *mut timespec64);
}

extern "C" {
    pub fn update_persistent_clock64(now: timespec64) -> c_int;
}

