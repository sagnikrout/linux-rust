//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pm_wakeup.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// pm_wakeup.h - Power management wakeup interface
//
// Copyright (C) 2008 Alan Stern
// Copyright (C) 2010 Rafael J. Wysocki, Novell Inc.
//

//
// struct wakeup_source - Representation of wakeup sources
//
// @name: Name of the wakeup source
// @id: Wakeup source id
// @entry: Wakeup source list entry
// @lock: Wakeup source lock
// @wakeirq: Optional device specific wakeirq
// @timer: Wakeup timer list
// @timer_expires: Wakeup timer expiration
// @total_time: Total time this wakeup source has been active.
// @max_time: Maximum time this wakeup source has been continuously active.
// @last_time: Monotonic clock when the wakeup source's was touched last time.
// @prevent_sleep_time: Total time this source has been preventing autosleep.
// @event_count: Number of signaled wakeup events.
// @active_count: Number of times the wakeup source was activated.
// @relax_count: Number of times the wakeup source was deactivated.
// @expire_count: Number of times the wakeup source's timeout has expired.
// @wakeup_count: Number of times the wakeup source might abort suspend.
// @dev: Struct device for sysfs statistics about the wakeup source.
// @active: Status of the wakeup source.
// @autosleep_enabled: Autosleep is active, so update @prevent_sleep_time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wakeup_source {
    pub name: *const c_char,
    pub id: c_int,
    pub entry: list_head,
    pub lock: spinlock_t,
    pub wakeirq: *mut wake_irq,
    pub timer: timer_list,
    pub timer_expires: c_ulong,
    pub total_time: ktime_t,
    pub max_time: ktime_t,
    pub last_time: ktime_t,
    pub start_prevent_time: ktime_t,
    pub prevent_sleep_time: ktime_t,
    pub event_count: c_ulong,
    pub active_count: c_ulong,
    pub relax_count: c_ulong,
    pub expire_count: c_ulong,
    pub wakeup_count: c_ulong,
    pub dev: *mut device,
    pub active:1: bool,
    pub autosleep_enabled:1: bool,
}

//
// Changes to device_may_wakeup take effect on the next pm state change.
//
// drivers/base/power/wakeup.c
extern "C" {
    pub fn wakeup_source_unregister(ws: *mut wakeup_source);
}
extern "C" {
    pub fn wakeup_sources_read_lock() -> c_int;
}
extern "C" {
    pub fn wakeup_sources_read_unlock(idx: c_int);
}
extern "C" {
    pub fn device_wakeup_enable(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn device_wakeup_disable(dev: *mut device);
}
extern "C" {
    pub fn device_set_wakeup_capable(dev: *mut device, capable: bool);
}
extern "C" {
    pub fn device_set_wakeup_enable(dev: *mut device, enable: bool) -> c_int;
}
extern "C" {
    pub fn __pm_stay_awake(ws: *mut wakeup_source);
}
extern "C" {
    pub fn pm_stay_awake(dev: *mut device);
}
extern "C" {
    pub fn __pm_relax(ws: *mut wakeup_source);
}
extern "C" {
    pub fn pm_relax(dev: *mut device);
}
extern "C" {
    pub fn pm_wakeup_ws_event(ws: *mut wakeup_source, msec: c_uint, hard: bool);
}
extern "C" {
    pub fn pm_wakeup_dev_event(dev: *mut device, msec: c_uint, hard: bool);
}

extern "C" {
    pub fn device_wakeup_path(_arg: dev) -> return;
}
//
// device_init_wakeup - Device wakeup initialization.
// @dev: Device to handle.
// @enable: Whether or not to enable @dev as a wakeup device.
//
// By default, most devices should leave wakeup disabled.  The exceptions are
// devices that everyone expects to be wakeup sources: keyboards, power buttons,
// possibly network interfaces, etc.  Also, devices that don't generate their
// own wakeup requests but merely forward requests from one bus to another
// (like PCI bridges) should have wakeup enabled by default.
//
extern "C" {
    pub fn device_wakeup_enable(_arg: dev) -> return;
}
//
// devm_device_init_wakeup - Resource managed device wakeup initialization.
// @dev: Device to handle.
//
// This function is the devm managed version of device_init_wakeup(dev, true).
//
extern "C" {
    pub fn devm_add_action_or_reset(_arg: dev, _arg: device_disable_wakeup, _arg: dev) -> return;
}
