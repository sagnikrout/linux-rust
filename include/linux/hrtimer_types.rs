//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hrtimer_types.h
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
// Return values for the callback function
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hrtimer_restart {
    HRTIMER_NORESTART,	/* Timer is not restarted */
    HRTIMER_RESTART,	/* Timer must be restarted */
}

//
// struct hrtimer - the basic hrtimer structure
// @node:	Linked timerqueue node, which also manages node.expires,
// the absolute expiry time in the hrtimers internal
// representation. The time is related to the clock on
// which the timer is based. Is setup by adding
// slack to the _softexpires value. For non range timers
// identical to _softexpires.
// @_softexpires: the absolute earliest expiry time of the hrtimer.
// The time which was given as expiry time when the timer
// was armed.
// @function:	timer expiry callback function
// @base:	pointer to the timer base (per cpu and per clock)
// @is_queued:	Indicates whether a timer is enqueued or not
// @is_rel:	Set if the timer was armed relative
// @is_soft:	Set if hrtimer will be expired in soft interrupt context.
// @is_hard:	Set if hrtimer will be expired in hard interrupt context
// even on RT.
// @is_lazy:	Set if the timer is frequently rearmed to avoid updates
// of the clock event device
//
// The hrtimer structure must be initialized by hrtimer_setup()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hrtimer {
    pub node: timerqueue_linked_node,
    pub base: *mut hrtimer_clock_base,
    pub is_queued: bool,
    pub is_rel: bool,
    pub is_soft: bool,
    pub is_hard: bool,
    pub is_lazy: bool,
    pub _softexpires: ktime_t,
    pub ): *mut *mut hrtimer_restart (__private function)(struct hrtimer,
}
