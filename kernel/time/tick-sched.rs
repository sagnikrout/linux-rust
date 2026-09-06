//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/time/tick-sched.h
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
pub enum tick_device_mode {
    TICKDEV_MODE_PERIODIC,
    TICKDEV_MODE_ONESHOT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tick_device {
    pub evtdev: *mut clock_event_device,
    pub mode: tick_device_mode,
}

// The CPU is in the tick idle mode

// The idle tick has been stopped

//
// Indicator that the CPU is actively in the tick idle mode;
// it is reset during irq handling phases.
//

// CPU was the last one doing do_timer before going idle

// NO_HZ is enabled

// High resolution tick mode

//
// struct tick_sched - sched tick emulation and no idle tick control/stats
//
// @flags:		State flags gathering the TS_FLAG_* features
// @got_idle_tick:	Tick timer function has run with @inidle set
// @stalled_jiffies:	Number of stalled jiffies detected across ticks
// @last_tick_jiffies:	Value of jiffies seen on last tick
// @sched_timer:	hrtimer to schedule the periodic tick in high
// resolution mode
// @last_tick:		Store the last tick expiry time when the tick
// timer is modified for nohz sleeps. This is necessary
// to resume the tick timer operation in the timeline
// when the CPU returns from nohz sleep.
// @next_tick:		Next tick to be fired when in dynticks mode.
// @idle_waketime:	Time when the idle was interrupted
// @idle_entrytime:	Time when the idle call was entered
// @last_jiffies:	Base jiffies snapshot when next event was last computed
// @timer_expires_base:	Base time clock monotonic for @timer_expires
// @timer_expires:	Anticipated timer expiration time (in case sched tick is stopped)
// @next_timer:		Expiry time of next expiring timer for debugging purpose only
// @idle_expires:	Next tick in idle, for debugging purpose only
// @idle_calls:		Total number of idle calls
// @idle_sleeps:	Number of idle calls, where the sched tick was stopped
// @tick_dep_mask:	Tick dependency mask - is set, if someone needs the tick
// @check_clocks:	Notification mechanism about clocksource changes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tick_sched {
// Common flags
    pub flags: c_ulong,
// Tick handling: jiffies stall check
    pub stalled_jiffies: c_uint,
    pub last_tick_jiffies: c_ulong,
// Tick handling
    pub sched_timer: hrtimer,
    pub last_tick: ktime_t,
    pub next_tick: ktime_t,
    pub idle_waketime: ktime_t,
    pub got_idle_tick: c_uint,
// Idle entry
    pub idle_entrytime: ktime_t,
// Tick stop
    pub last_jiffies: c_ulong,
    pub timer_expires_base: u64,
    pub timer_expires: u64,
    pub next_timer: u64,
    pub idle_expires: ktime_t,
    pub idle_calls: c_ulong,
    pub idle_sleeps: c_ulong,
// Full dynticks handling
    pub tick_dep_mask: core::sync::atomic::AtomicI32,
// Clocksource changes
    pub check_clocks: c_ulong,
}

extern "C" {
    pub fn tick_setup_sched_timer(hrtimer: bool);
}

extern "C" {
    pub fn tick_sched_timer_dying(cpu: c_int);
}

extern "C" {
    pub fn __tick_broadcast_oneshot_control(state: tick_broadcast_state) -> c_int;
}

