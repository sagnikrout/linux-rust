//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/timer.h
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
// timer_init - called when the timer is initialized
// @timer:	pointer to struct timer_list
//

//
// timer_start - called when the timer is started
// @timer:		pointer to struct timer_list
// @bucket_expiry:	the bucket expiry time
//
// timer_expire_entry - called immediately before the timer callback
// @timer:	pointer to struct timer_list
// @baseclk:	value of timer_base::clk when timer expires
//
// Allows to determine the timer latency.
//
// timer_expire_exit - called immediately after the timer callback returns
// @timer:	pointer to struct timer_list
//
// When used in combination with the timer_expire_entry tracepoint we can
// determine the runtime of the timer callback function.
//
// NOTE: Do NOT dereference timer in TP_fast_assign. The pointer might
// be invalid. We solely track the pointer.
//
// timer_cancel - called when the timer is canceled
// @timer:	pointer to struct timer_list
//

//
// hrtimer_setup - called when the hrtimer is initialized
// @hrtimer:	pointer to struct hrtimer
// @clockid:	the hrtimers clock
// @mode:	the hrtimers mode
//
// hrtimer_start - called when the hrtimer is started
// @hrtimer:	pointer to struct hrtimer
// @mode:	the hrtimers mode
// @was_armed:	Was armed when hrtimer_start*() was invoked
//
// hrtimer_expire_entry - called immediately before the hrtimer callback
// @hrtimer:	pointer to struct hrtimer
// @now:	variable which contains current time of the timers base.
//
// Allows to determine the timer latency.
//
// hrtimer_start_expired - Invoked when a expired timer was started
// @hrtimer:	pointer to struct hrtimer
//
// Preceeded by a hrtimer_start tracepoint.
//
// hrtimer_expire_exit - called immediately after the hrtimer callback returns
// @hrtimer:	pointer to struct hrtimer
//
// When used in combination with the hrtimer_expire_entry tracepoint we can
// determine the runtime of the callback function.
//
// hrtimer_cancel - called when the hrtimer is canceled
// @hrtimer:	pointer to struct hrtimer
//
// hrtimer_rearm - Invoked when the clockevent device is rearmed
// @next_event:	The next expiry time (CLOCK_MONOTONIC)
//
// itimer_state - called when itimer is started or canceled
// @which:	name of the interval timer
// @value:	the itimers value, itimer is canceled if value->it_value is
// zero, otherwise it is started
// @expires:	the itimers expiry time
//
// itimer_expire - called when itimer expires
// @which:	type of the interval timer
// @pid:	pid of the process which owns the timer
// @now:	current time, used to calculate the latency of itimer
//

// The MASK will convert to their bits and they need to be processed too

// NONE only has a mask defined for it

// This part must be outside protection
