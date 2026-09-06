//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hrtimer.h
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
// hrtimers - High-resolution kernel timers
//
// Copyright(C) 2005, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright(C) 2005, Red Hat, Inc., Ingo Molnar
//
// data type definitions, declarations, prototypes
//
// Started by: Thomas Gleixner and Ingo Molnar
//

//
// Mode arguments of xxx_hrtimer functions:
//
// HRTIMER_MODE_ABS		- Time value is absolute
// HRTIMER_MODE_REL		- Time value is relative to now
// HRTIMER_MODE_PINNED		- Timer is bound to CPU (is only considered
// when starting the timer)
// HRTIMER_MODE_SOFT		- Timer callback function will be executed in
// soft irq context
// HRTIMER_MODE_HARD		- Timer callback function will be executed in
// hard irq context even on PREEMPT_RT.
// HRTIMER_MODE_LAZY_REARM	- Avoid reprogramming if the timer was the
// first expiring timer and is moved into the
// future. Special mode for the HRTICK timer to
// avoid extensive reprogramming of the hardware,
// which is expensive in virtual machines. Risks
// a pointless expiry, but that's better than
// reprogramming on every context switch,
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hrtimer_mode {
    HRTIMER_MODE_ABS	= 0x00,
    HRTIMER_MODE_REL	= 0x01,
    HRTIMER_MODE_PINNED	= 0x02,
    HRTIMER_MODE_SOFT	= 0x04,
    HRTIMER_MODE_HARD	= 0x08,
    HRTIMER_MODE_LAZY_REARM	= 0x10,

    HRTIMER_MODE_ABS_PINNED = HRTIMER_MODE_ABS | HRTIMER_MODE_PINNED,
    HRTIMER_MODE_REL_PINNED = HRTIMER_MODE_REL | HRTIMER_MODE_PINNED,

    HRTIMER_MODE_ABS_SOFT	= HRTIMER_MODE_ABS | HRTIMER_MODE_SOFT,
    HRTIMER_MODE_REL_SOFT	= HRTIMER_MODE_REL | HRTIMER_MODE_SOFT,

    HRTIMER_MODE_ABS_PINNED_SOFT = HRTIMER_MODE_ABS_PINNED | HRTIMER_MODE_SOFT,
    HRTIMER_MODE_REL_PINNED_SOFT = HRTIMER_MODE_REL_PINNED | HRTIMER_MODE_SOFT,

    HRTIMER_MODE_ABS_HARD	= HRTIMER_MODE_ABS | HRTIMER_MODE_HARD,
    HRTIMER_MODE_REL_HARD	= HRTIMER_MODE_REL | HRTIMER_MODE_HARD,

    HRTIMER_MODE_ABS_PINNED_HARD = HRTIMER_MODE_ABS_PINNED | HRTIMER_MODE_HARD,
    HRTIMER_MODE_REL_PINNED_HARD = HRTIMER_MODE_REL_PINNED | HRTIMER_MODE_HARD,
}

//
// struct hrtimer_sleeper - simple sleeper structure
// @timer:	embedded timer structure
// @task:	task to wake up
//
// task is set to NULL, when the timer expires.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hrtimer_sleeper {
    pub timer: hrtimer,
    pub task: *mut task_struct,
}

extern "C" {
    pub fn hrtimer_cb_get_time(timer: *const hrtimer) -> ktime_t;
}
extern "C" {
    pub fn ktime_sub(_arg: timer->node.expires, _arg: hrtimer_cb_get_time(timer)) -> return;
}

extern "C" {
    pub fn hrtimer_interrupt(dev: *mut clock_event_device);
}
extern "C" {
    pub fn static_branch_likely(_arg: &hrtimer_highres_enabled_key) -> return;
}

//
// Adjust relative timers for the extra we added in
// hrtimer_start_range_ns() to prevent short timeouts.
//
extern "C" {
    pub fn __hrtimer_expires_remaining_adjusted(_arg: timer, _arg: hrtimer_cb_get_time(timer)) -> return;
}

extern "C" {
    pub fn timerfd_clock_was_set();
}
extern "C" {
    pub fn timerfd_resume();
}

extern "C" {
    pub fn hrtimer_cancel_wait_running(timer: *const hrtimer);
}

// Exported timer functions:
// Initialize timers:

extern "C" {
    pub fn destroy_hrtimer_on_stack(timer: *mut hrtimer);
}

// Basic timer operations:
//
// hrtimer_start - (re)start an hrtimer
// @timer:	the timer to be added
// @tim:	expiry time
// @mode:	timer mode: absolute (HRTIMER_MODE_ABS) or
// relative (HRTIMER_MODE_REL), and pinned (HRTIMER_MODE_PINNED);
// softirq based mode is considered for debug purpose only!
//
extern "C" {
    pub fn hrtimer_cancel(timer: *mut hrtimer) -> c_int;
}
extern "C" {
    pub fn hrtimer_try_to_cancel(timer: *mut hrtimer) -> c_int;
}
extern "C" {
    pub fn hrtimer_start_range_ns_user(_arg: timer, _arg: soft, _arg: delta, _arg: mode) -> return;
}
// Query timers:
extern "C" {
    pub fn __hrtimer_get_remaining(timer: *const hrtimer, adjust: bool) -> ktime_t;
}
//
// hrtimer_get_remaining - get remaining time for the timer
// @timer:	the timer to read
//
extern "C" {
    pub fn __hrtimer_get_remaining(_arg: timer, _arg: false) -> return;
}
extern "C" {
    pub fn hrtimer_get_next_event() -> ktime_t;
}
extern "C" {
    pub fn hrtimer_next_event_without(exclude: *const hrtimer) -> ktime_t;
}
extern "C" {
    pub fn hrtimer_active(timer: *const hrtimer) -> bool;
}
//
// hrtimer_is_queued - check, whether the timer is on one of the queues
// @timer:	Timer to check
//
// Returns: True if the timer is queued, false otherwise
//
// The function can be used lockless, but it gives only a current snapshot.
//
// The READ_ONCE pairs with the update functions of timer->is_queued
extern "C" {
    pub fn READ_ONCE(_arg: timer->is_queued) -> return;
}
extern "C" {
    pub fn hrtimer_restart(): *mut *mut function)(struct hrtimer) -> enum;
}
// Forward a hrtimer so it expires after now:
//
// hrtimer_forward_now() - forward the timer expiry so it expires after now
// @timer:	hrtimer to forward
// @interval:	the interval to forward
//
// It is a variant of hrtimer_forward(). The timer will expire after the current
// time of the hrtimer clock base. See hrtimer_forward() for details.
//
extern "C" {
    pub fn hrtimer_forward(_arg: timer, _arg: hrtimer_cb_get_time(timer), _arg: interval) -> return;
}
// Precise sleep:
extern "C" {
    pub fn nanosleep_copyout(: *mut restart_block, : *mut timespec64) -> c_int;
}
extern "C" {
    pub fn schedule_hrtimeout(expires: *mut ktime_t, mode: hrtimer_mode) -> c_int;
}
// Soft interrupt function to run the hrtimer queues:
extern "C" {
    pub fn hrtimer_run_queues();
}
// Bootup initialization:
extern "C" {
    pub fn hrtimers_init() -> void __init;
}
// Show pending timers:
extern "C" {
    pub fn sysrq_timer_list_show();
}
extern "C" {
    pub fn hrtimers_prepare_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn hrtimers_cpu_starting(cpu: c_uint) -> c_int;
}

extern "C" {
    pub fn hrtimers_cpu_dying(cpu: c_uint) -> c_int;
}

