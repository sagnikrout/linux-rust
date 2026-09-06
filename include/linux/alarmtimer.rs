//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/alarmtimer.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alarmtimer_type {
    ALARM_REALTIME,
    ALARM_BOOTTIME,

// Supported types end here
    ALARM_NUMTYPE,

// Used for tracing information. No usable types.
    ALARM_REALTIME_FREEZER,
    ALARM_BOOTTIME_FREEZER,
}

pub const ALARMTIMER_STATE_INACTIVE: c_uint = 0x00;
pub const ALARMTIMER_STATE_ENQUEUED: c_uint = 0x01;
//
// struct alarm - Alarm timer structure
// @node:	timerqueue node for adding to the event list this value
// also includes the expiration time.
// @timer:	hrtimer used to schedule events while running
// @function:	Function pointer to be executed when the timer fires.
// @type:	Alarm type (BOOTTIME/REALTIME).
// @state:	Flag that represents if the alarm is set to fire or not.
// @data:	Internal data value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alarm {
    pub node: timerqueue_node,
    pub timer: hrtimer,
    pub now): *mut *mut *mut void (function)(struct alarm , ktime_t,
    pub type: alarmtimer_type,
    pub state: c_int,
    pub data: *mut c_void,
}

extern "C" {
    pub fn alarm_start_timer(alarm: *mut alarm, expires: ktime_t, relative: bool) -> bool;
}
extern "C" {
    pub fn alarm_try_to_cancel(alarm: *mut alarm) -> c_int;
}
extern "C" {
    pub fn alarm_cancel(alarm: *mut alarm) -> c_int;
}
extern "C" {
    pub fn alarm_forward(alarm: *mut alarm, now: ktime_t, interval: ktime_t) -> u64;
}
extern "C" {
    pub fn alarm_forward_now(alarm: *mut alarm, interval: ktime_t) -> u64;
}
extern "C" {
    pub fn alarm_expires_remaining(alarm: *const alarm) -> ktime_t;
}

// Provide way to access the rtc device being used by alarmtimers

