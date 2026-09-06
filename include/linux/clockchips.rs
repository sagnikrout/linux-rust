//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clockchips.h
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
// linux/include/linux/clockchips.h
//
// This file contains the structure definitions for clockchips.
//
// If you are not a clockchip, or the time of day code, you should
// not be including this file!
//

//
// Possible states of a clock event device.
//
// DETACHED:	Device is not used by clockevents core. Initial state or can be
// reached from SHUTDOWN.
// SHUTDOWN:	Device is powered-off. Can be reached from PERIODIC or ONESHOT.
// PERIODIC:	Device is programmed to generate events periodically. Can be
// reached from DETACHED or SHUTDOWN.
// ONESHOT:	Device is programmed to generate event only once. Can be reached
// from DETACHED or SHUTDOWN.
// ONESHOT_STOPPED: Device was programmed in ONESHOT mode and is temporarily
// stopped.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clock_event_state {
    CLOCK_EVT_STATE_DETACHED,
    CLOCK_EVT_STATE_SHUTDOWN,
    CLOCK_EVT_STATE_PERIODIC,
    CLOCK_EVT_STATE_ONESHOT,
    CLOCK_EVT_STATE_ONESHOT_STOPPED,
}

//
// Clock event features
//

//
// x86(64) specific (mis)features:
//
// - Clockevent source stops in C3 State and needs broadcast support.
// - Local APIC timer is used as a dummy device.
//

//
// Core shall set the interrupt affinity dynamically in broadcast mode
//

//
// Clockevent device is based on a hrtimer for broadcast
//

//
// struct clock_event_device - clock event device descriptor
// @event_handler:	Assigned by the framework to be called by the low
// level handler of the event source
// @set_next_event:	set next event function using a clocksource delta
// @set_next_ktime:	set next event function using a direct ktime value
// @set_next_coupled:	set next event function for clocksource coupled mode
// @next_event:		local storage for the next event in oneshot mode
// @max_delta_ns:	maximum delta value in ns
// @min_delta_ns:	minimum delta value in ns
// @mult:		nanosecond to cycles multiplier
// @shift:		nanoseconds to cycles divisor (power of two)
// @state_use_accessors:current state of the device, assigned by the core code
// @features:		features
// @cs_id:		Clocksource ID to denote the clocksource for coupled mode
// @next_event_forced:	True if the last programming was a forced event
// @retries:		number of forced programming retries
// @set_state_periodic:	switch state to periodic
// @set_state_oneshot:	switch state to oneshot
// @set_state_oneshot_stopped: switch state to oneshot_stopped
// @set_state_shutdown:	switch state to shutdown
// @tick_resume:	resume clkevt device
// @broadcast:		function to broadcast events
// @min_delta_ticks:	minimum delta value in ticks stored for reconfiguration
// @max_delta_ticks:	maximum delta value in ticks stored for reconfiguration
// @name:		ptr to clock event name
// @rating:		variable to rate clock event devices
// @irq:		IRQ number (only for non CPU local devices)
// @bound_on:		Bound on CPU
// @cpumask:		cpumask to indicate for which CPUs this device works
// @list:		list head for the management code
// @owner:		module reference
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_event_device {
    pub ): *mut *mut void (event_handler)(struct clock_event_device,
    pub ): *mut *mut int (set_next_event)(unsigned long evt, struct clock_event_device,
    pub ): *mut *mut int (set_next_ktime)(ktime_t expires, struct clock_event_device,
    pub ): *mut *mut void (set_next_coupled)(u64 cycles, struct clock_event_device,
    pub next_event: ktime_t,
    pub max_delta_ns: u64,
    pub min_delta_ns: u64,
    pub mult: u32,
    pub shift: u32,
    pub state_use_accessors: clock_event_state,
    pub features: c_uint,
    pub cs_id: clocksource_ids,
    pub next_event_forced: c_uint,
    pub retries: c_ulong,
    pub ): *mut *mut int (set_state_periodic)(struct clock_event_device,
    pub ): *mut *mut int (set_state_oneshot)(struct clock_event_device,
    pub ): *mut *mut int (set_state_oneshot_stopped)(struct clock_event_device,
    pub ): *mut *mut int (set_state_shutdown)(struct clock_event_device,
    pub ): *mut *mut int (tick_resume)(struct clock_event_device,
    pub mask): *const *const void (broadcast)(struct cpumask,
    pub ): *mut *mut void (suspend)(struct clock_event_device,
    pub ): *mut *mut void (resume)(struct clock_event_device,
    pub min_delta_ticks: c_ulong,
    pub max_delta_ticks: c_ulong,
    pub name: *const c_char,
    pub rating: c_int,
    pub irq: c_int,
    pub bound_on: c_int,
    pub cpumask: *const cpumask,
    pub list: list_head,
    pub owner: *mut module,
    pub ____cacheline_aligned: },
// Helpers to verify state of a clockevent device
    pub CLOCK_EVT_STATE_DETACHED: return dev->state_use_accessors ==,
    pub CLOCK_EVT_STATE_SHUTDOWN: return dev->state_use_accessors ==,
    pub CLOCK_EVT_STATE_PERIODIC: return dev->state_use_accessors ==,
    pub CLOCK_EVT_STATE_ONESHOT: return dev->state_use_accessors ==,
    pub CLOCK_EVT_STATE_ONESHOT_STOPPED: return dev->state_use_accessors ==,
//
// Calculate a multiplication factor for scaled math, which is used to convert
// nanoseconds based values to clock ticks:
//
// clock_ticks = (nanoseconds * factor) >> shift.
//
// div_sc is the rearranged equation to calculate a factor from a given clock
// ticks / nanoseconds ratio:
//
// factor = (clock_ticks << shift) / nanoseconds
//
    pub shift: u64 tmp = ((u64)ticks) <<,
    pub nsec): do_div(tmp,,
    pub tmp: return (unsigned long),
// Clock event layer functions
    pub evt): *mut extern u64 clockevent_delta2ns(unsigned long latch, struct clock_event_device,
    pub dev): *mut extern void clockevents_register_device(struct clock_event_device,
    pub cpu): *mut *mut extern int clockevents_unbind_device(struct clock_event_device ced, int,
    pub max_delta): c_ulong,
    pub freq): *mut *mut extern int clockevents_update_freq(struct clock_event_device ce, u32,
    pub maxsec): return clocks_calc_mult_shift(&ce->mult, &ce->shift, NSEC_PER_SEC, freq,,
    pub clockevents_suspend(void): extern void,
    pub clockevents_resume(void): extern void,

    pub mask): *const extern void tick_broadcast(struct cpumask,

    pub tick_receive_broadcast(void): extern int,

    pub tick_setup_hrtimer_broadcast(void): extern void,
    pub tick_check_broadcast_expired(void): extern int,

    pub }: static __always_inline int tick_check_broadcast_expired(void) { return 0;,

    pub }: static __always_inline int tick_check_broadcast_expired(void) { return 0;,

