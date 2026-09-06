//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/devfreq-event.h
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
// devfreq-event: a framework to provide raw data and events of devfreq devices
//
// Copyright (C) 2014 Samsung Electronics
// Author: Chanwoo Choi <cw00.choi@samsung.com>
//

//
// struct devfreq_event_dev - the devfreq-event device
//
// @node	: Contain the devfreq-event device that have been registered.
// @dev		: the device registered by devfreq-event class. dev.parent is
// the device using devfreq-event.
// @lock	: a mutex to protect accessing devfreq-event.
// @enable_count: the number of enable function have been called.
// @desc	: the description for devfreq-event device.
//
// This structure contains devfreq-event device information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_event_dev {
    pub node: list_head,
    pub dev: device,
    pub lock: mutex,
    pub enable_count: u32,
    pub desc: *const devfreq_event_desc,
}

//
// struct devfreq_event_data - the devfreq-event data
//
// @load_count	: load count of devfreq-event device for the given period.
// @total_count	: total count of devfreq-event device for the given period.
// each count may represent a clock cycle, a time unit
// (ns/us/...), or anything the device driver wants.
// Generally, utilization is load_count / total_count.
//
// This structure contains the data of devfreq-event device for polling period.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_event_data {
    pub load_count: c_ulong,
    pub total_count: c_ulong,
}

//
// struct devfreq_event_ops - the operations of devfreq-event device
//
// @enable	: Enable the devfreq-event device.
// @disable	: Disable the devfreq-event device.
// @reset	: Reset all setting of the devfreq-event device.
// @set_event	: Set the specific event type for the devfreq-event device.
// @get_event	: Get the result of the devfreq-event devie with specific
// event type.
//
// This structure contains devfreq-event device operations which can be
// implemented by devfreq-event device drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_event_ops {
// Optional functions
    pub edev): *mut *mut int (enable)(struct devfreq_event_dev,
    pub edev): *mut *mut int (disable)(struct devfreq_event_dev,
    pub edev): *mut *mut int (reset)(struct devfreq_event_dev,
// Mandatory functions
    pub edev): *mut *mut int (set_event)(struct devfreq_event_dev,
    pub edata): *mut devfreq_event_data,
}

//
// struct devfreq_event_desc - the descriptor of devfreq-event device
//
// @name	: the name of devfreq-event device.
// @event_type	: the type of the event determined and used by driver
// @driver_data	: the private data for devfreq-event driver.
// @ops		: the operation to control devfreq-event device.
//
// Each devfreq-event device is described with a this structure.
// This structure contains the various data for devfreq-event device.
// The event_type describes what is going to be counted in the register.
// It might choose to count e.g. read requests, write data in bytes, etc.
// The full supported list of types is present in specyfic header in:
// include/dt-bindings/pmu/.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_event_desc {
    pub name: *const c_char,
    pub event_type: u32,
    pub driver_data: *mut c_void,
    pub ops: *const devfreq_event_ops,
}

extern "C" {
    pub fn devfreq_event_enable_edev(edev: *mut devfreq_event_dev) -> c_int;
}
extern "C" {
    pub fn devfreq_event_disable_edev(edev: *mut devfreq_event_dev) -> c_int;
}
extern "C" {
    pub fn devfreq_event_is_enabled(edev: *mut devfreq_event_dev) -> bool;
}
extern "C" {
    pub fn devfreq_event_set_event(edev: *mut devfreq_event_dev) -> c_int;
}
extern "C" {
    pub fn devfreq_event_reset_event(edev: *mut devfreq_event_dev) -> c_int;
}
extern "C" {
    pub fn devfreq_event_remove_edev(edev: *mut devfreq_event_dev) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

