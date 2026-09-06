//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/devfreq-governor.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// governor.h - internal header for devfreq governors.
//
// Copyright (C) 2011 Samsung Electronics
// MyungJoo Ham <myungjoo.ham@samsung.com>
//
// This header is for devfreq governors
//

pub const DEVFREQ_NAME_LEN: c_int = 16;

// Devfreq events
pub const DEVFREQ_GOV_START: c_uint = 0x1;
pub const DEVFREQ_GOV_STOP: c_uint = 0x2;
pub const DEVFREQ_GOV_UPDATE_INTERVAL: c_uint = 0x3;
pub const DEVFREQ_GOV_SUSPEND: c_uint = 0x4;
pub const DEVFREQ_GOV_RESUME: c_uint = 0x5;
pub const DEVFREQ_MIN_FREQ: c_int = 0;

//
// Definition of the governor feature flags
// - DEVFREQ_GOV_FLAG_IMMUTABLE
// : This governor is never changeable to other governors.
// - DEVFREQ_GOV_FLAG_IRQ_DRIVEN
// : The devfreq won't schedule the work for this governor.
//

//
// Definition of governor attribute flags except for common sysfs attributes
// - DEVFREQ_GOV_ATTR_POLLING_INTERVAL
// : Indicate polling_interval sysfs attribute
// - DEVFREQ_GOV_ATTR_TIMER
// : Indicate timer sysfs attribute
//

//
// struct devfreq_governor - Devfreq policy governor
// @node:		list node - contains registered devfreq governors
// @name:		Governor's name
// @attrs:		Governor's sysfs attribute flags
// @flags:		Governor's feature flags
// @get_target_freq:	Returns desired operating frequency for the device.
// Basically, get_target_freq will run
// devfreq_dev_profile.get_dev_status() to get the
// status of the device (load = busy_time / total_time).
// @event_handler:      Callback for devfreq core framework to notify events
// to governors. Events include per device governor
// init and exit, opp changes out of devfreq, suspend
// and resume of per device devfreq during device idle.
//
// Note that the callbacks are called with devfreq->lock locked by devfreq.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_governor {
    pub node: list_head,
    pub name: [c_char; DEVFREQ_NAME_LEN],
    pub attrs: u64,
    pub flags: u64,
    pub freq): *mut *mut *mut int (get_target_freq)(struct devfreq this, unsigned long,
    pub data): *mut unsigned int event, void,
}

extern "C" {
    pub fn devfreq_monitor_start(devfreq: *mut devfreq);
}
extern "C" {
    pub fn devfreq_monitor_stop(devfreq: *mut devfreq);
}
extern "C" {
    pub fn devfreq_monitor_suspend(devfreq: *mut devfreq);
}
extern "C" {
    pub fn devfreq_monitor_resume(devfreq: *mut devfreq);
}
extern "C" {
    pub fn devfreq_update_interval(devfreq: *mut devfreq, delay: *mut c_uint);
}
extern "C" {
    pub fn devfreq_add_governor(governor: *mut devfreq_governor) -> c_int;
}
extern "C" {
    pub fn devfreq_remove_governor(governor: *mut devfreq_governor) -> c_int;
}
extern "C" {
    pub fn devfreq_update_status(devfreq: *mut devfreq, freq: c_ulong) -> c_int;
}
extern "C" {
    pub fn devfreq_update_target(devfreq: *mut devfreq, freq: c_ulong) -> c_int;
}
