//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hrtimer_bases.h
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
// struct hrtimer_clock_base - the timer base for a specific clock
// @cpu_base:		per cpu clock base
// @index:		clock type index for per_cpu support when moving a
// timer to a base on another cpu.
// @clockid:		clock id for per_cpu support
// @seq:		seqcount around __run_hrtimer
// @expires_next:	Absolute time of the next event in this clock base
// @running:		pointer to the currently running hrtimer
// @active:		red black tree root node for the active timers
// @offset:		offset of this clock to the monotonic base
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hrtimer_clock_base {
    pub cpu_base: *mut hrtimer_cpu_base,
    pub index: c_uint,
    pub clockid: clockid_t,
    pub seq: seqcount_raw_spinlock_t,
    pub expires_next: ktime_t,
    pub running: *mut hrtimer,
    pub active: timerqueue_linked_head,
    pub offset: ktime_t,
    pub __hrtimer_clock_base_align: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hrtimer_base_type {
    HRTIMER_BASE_MONOTONIC,
    HRTIMER_BASE_REALTIME,
    HRTIMER_BASE_BOOTTIME,
    HRTIMER_BASE_TAI,
    HRTIMER_BASE_MONOTONIC_SOFT,
    HRTIMER_BASE_REALTIME_SOFT,
    HRTIMER_BASE_BOOTTIME_SOFT,
    HRTIMER_BASE_TAI_SOFT,
    HRTIMER_MAX_CLOCK_BASES
}

//
// struct hrtimer_cpu_base - the per cpu clock bases
// @lock:			lock protecting the base and associated clock bases and timers
// @cpu:			cpu number
// @active_bases:		Bitfield to mark bases with active timers
// @clock_was_set_seq:		Sequence counter of clock was set events
// @hres_active:		State of high resolution mode
// @deferred_rearm:		A deferred rearm is pending
// @deferred_needs_update:	The deferred rearm must re-evaluate the first timer
// @hang_detected:		The last hrtimer interrupt detected a hang
// @softirq_activated:		displays, if the softirq is raised - update of softirq
// related settings is not required then.
// @nr_events:			Total number of hrtimer interrupt events
// @nr_retries:			Total number of hrtimer interrupt retries
// @nr_hangs:			Total number of hrtimer interrupt hangs
// @max_hang_time:		Maximum time spent in hrtimer_interrupt
// @softirq_expiry_lock:	Lock which is taken while softirq based hrtimer are expired
// @online:			CPU is online from an hrtimers point of view
// @timer_waiters:		A hrtimer_cancel() waiters for the timer callback to finish.
// @expires_next:		Absolute time of the next event, is required for remote
// hrtimer enqueue; it is the total first expiry time (hard
// and soft hrtimer are taken into account)
// @next_timer:			Pointer to the first expiring timer
// @softirq_expires_next:	Time to check, if soft queues needs also to be expired
// @softirq_next_timer:		Pointer to the first expiring softirq based timer
// @deferred_expires_next:	Cached expires next value for deferred rearm
// @clock_base:			Array of clock bases for this cpu
//
// Note: next_timer is just an optimization for __remove_hrtimer().
// Do not dereference the pointer because it is not reliable on
// cross cpu removals.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hrtimer_cpu_base {
    pub lock: raw_spinlock_t,
    pub cpu: c_uint,
    pub active_bases: c_uint,
    pub clock_was_set_seq: u32,
    pub hres_active: bool,
    pub deferred_rearm: bool,
    pub deferred_needs_update: bool,
    pub hang_detected: bool,
    pub softirq_activated: bool,
    pub online: bool,

    pub nr_events: c_uint,
    pub nr_retries: c_ushort,
    pub nr_hangs: c_ushort,
    pub max_hang_time: c_uint,

    pub softirq_expiry_lock: spinlock_t,
    pub timer_waiters: core::sync::atomic::AtomicI32,

    pub expires_next: ktime_t,
    pub next_timer: *mut hrtimer,
    pub softirq_expires_next: ktime_t,
    pub softirq_next_timer: *mut hrtimer,
    pub deferred_expires_next: ktime_t,
    pub clock_base: [hrtimer_clock_base; HRTIMER_MAX_CLOCK_BASES],
    pub csd: call_single_data_t,
    pub ____cacheline_aligned: },
//
// Helper function to check, whether the timer is running the callback
// function
//
    pub timer: return timer->base->running ==,
