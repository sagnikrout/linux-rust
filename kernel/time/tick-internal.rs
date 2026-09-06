//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/time/tick-internal.h
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
// tick internal variable and functions used by low/high res code
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_events {
    pub local: u64,
    pub global: u64,
}

extern "C" {
    pub fn tick_setup_periodic(dev: *mut clock_event_device, broadcast: c_int);
}
extern "C" {
    pub fn tick_handle_periodic(dev: *mut clock_event_device);
}
extern "C" {
    pub fn tick_check_new_device(dev: *mut clock_event_device);
}
extern "C" {
    pub fn tick_offline_cpu(cpu: c_uint);
}
extern "C" {
    pub fn tick_shutdown();
}
extern "C" {
    pub fn tick_suspend();
}
extern "C" {
    pub fn tick_resume();
}
extern "C" {
    pub fn tick_install_replacement(dev: *mut clock_event_device);
}
extern "C" {
    pub fn tick_is_oneshot_available() -> c_int;
}
extern "C" {
    pub fn clockevents_tick_resume(dev: *mut clock_event_device) -> c_int;
}
// Check, if the device is functional or a dummy for broadcast
extern "C" {
    pub fn clockevents_shutdown(dev: *mut clock_event_device);
}
extern "C" {
    pub fn clockevents_handle_noop(dev: *mut clock_event_device);
}
extern "C" {
    pub fn __clockevents_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int;
}
// Broadcasting support

extern "C" {
    pub fn tick_device_uses_broadcast(dev: *mut clock_event_device, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn tick_install_broadcast_device(dev: *mut clock_event_device, cpu: c_int);
}
extern "C" {
    pub fn tick_is_broadcast_device(dev: *mut clock_event_device) -> c_int;
}
extern "C" {
    pub fn tick_suspend_broadcast();
}
extern "C" {
    pub fn tick_resume_broadcast();
}
extern "C" {
    pub fn tick_resume_check_broadcast() -> bool;
}
extern "C" {
    pub fn tick_broadcast_init();
}
extern "C" {
    pub fn tick_set_periodic_handler(dev: *mut clock_event_device, broadcast: c_int);
}
extern "C" {
    pub fn tick_broadcast_update_freq(dev: *mut clock_event_device, freq: u32) -> c_int;
}

// Set the periodic handler in non broadcast mode

// Oneshot related functions

extern "C" {
    pub fn tick_program_event(expires: ktime_t, force: c_int) -> c_int;
}
extern "C" {
    pub fn tick_oneshot_notify();
}
extern "C" {
    pub fn tick_switch_to_oneshot(): *mut *mut void (handler)(struct clock_event_device) -> c_int;
}
extern "C" {
    pub fn tick_resume_oneshot();
}
extern "C" {
    pub fn tick_oneshot_mode_active() -> c_int;
}
extern "C" {
    pub fn tick_clock_notify();
}
extern "C" {
    pub fn tick_check_oneshot_change(allow_nohz: c_int) -> c_int;
}
extern "C" {
    pub fn tick_init_highres() -> c_int;
}

// Functions related to oneshot broadcasting

extern "C" {
    pub fn tick_broadcast_switch_to_oneshot();
}
extern "C" {
    pub fn tick_broadcast_oneshot_active() -> c_int;
}
extern "C" {
    pub fn tick_check_oneshot_broadcast_this_cpu();
}
extern "C" {
    pub fn tick_broadcast_oneshot_available() -> bool;
}

extern "C" {
    pub fn tick_broadcast_offline(cpu: c_uint);
}

// NO_HZ_FULL internal

extern "C" {
    pub fn tick_nohz_init();
}

extern "C" {
    pub fn timers_update_nohz();
}
extern "C" {
    pub fn get_jiffies_update(basej: *mut c_ulong) -> u64;
}

extern "C" {
    pub fn timer_lock_remote_bases(cpu: c_uint);
}
extern "C" {
    pub fn timer_unlock_remote_bases(cpu: c_uint);
}
extern "C" {
    pub fn timer_base_is_idle() -> bool;
}
extern "C" {
    pub fn timer_expire_remote(cpu: c_uint);
}

extern "C" {
    pub fn get_next_timer_interrupt(basej: c_ulong, basem: u64) -> u64;
}
extern "C" {
    pub fn timer_base_try_to_set_idle(basej: c_ulong, basem: u64, idle: *mut bool) -> u64;
}
extern "C" {
    pub fn timer_clear_idle();
}

extern "C" {
    pub fn clock_was_set(bases: c_uint);
}
extern "C" {
    pub fn clock_was_set_delayed();
}
extern "C" {
    pub fn hrtimers_resume_local();
}
// Since jiffies uses a simple TICK_NSEC multiplier
// conversion, the .shift value could be zero. However
// this would make NTP adjustments impossible as they are
// in units of 1/2^.shift. Thus we use JIFFIES_SHIFT to
// shift both the nominator and denominator the same
// amount, and give ntp adjustments in units of 1/2^8
//
// The value 8 is somewhat carefully chosen, as anything
// larger can result in overflows. TICK_NSEC grows as HZ
// shrinks, so values greater than 8 overflow 32bits when
// HZ=100.
//

pub const JIFFIES_SHIFT: c_int = 6;

pub const JIFFIES_SHIFT: c_int = 7;

pub const JIFFIES_SHIFT: c_int = 8;

extern "C" {
    pub fn sysfs_get_uname(buf: *const c_char, dst: *mut c_char, cnt: usize) -> isize;
}
