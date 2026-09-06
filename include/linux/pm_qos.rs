//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pm_qos.h
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
// Definitions related to Power Management Quality of Service (PM QoS).
//
// Copyright (C) 2020 Intel Corporation
//
// Authors:
// Mark Gross <mgross@linux.intel.com>
// Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_qos_flags_status {
    PM_QOS_FLAGS_UNDEFINED = -1,
    PM_QOS_FLAGS_NONE,
    PM_QOS_FLAGS_SOME,
    PM_QOS_FLAGS_ALL,
}

pub const PM_QOS_LATENCY_TOLERANCE_DEFAULT_VALUE: c_int = 0;
pub const PM_QOS_MIN_FREQUENCY_DEFAULT_VALUE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_qos_type {
    PM_QOS_UNITIALIZED,
    PM_QOS_MAX,		/* return the largest value */
    PM_QOS_MIN,		/* return the smallest value */
}

//
// Note: The lockless read path depends on the CPU accessing target_value
// or effective_flags atomically.  Atomic access is only guaranteed on all CPU
// types linux supports for 32 bit quantites
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_qos_constraints {
    pub list: plist_head,
    pub /: *mut *mut s32 target_value; / Do not change to 64 bit,
    pub default_value: i32,
    pub no_constraint_value: i32,
    pub type: pm_qos_type,
    pub notifiers: *mut blocking_notifier_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_qos_request {
    pub node: plist_node,
    pub qos: *mut pm_qos_constraints,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_qos_flags_request {
    pub node: list_head,
    pub /: *mut *mut s32 flags; / Do not change to 64 bit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_qos_flags {
    pub list: list_head,
    pub /: *mut *mut s32 effective_flags; / Do not change to 64 bit,
}

pub const FREQ_QOS_MIN_DEFAULT_VALUE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum freq_qos_req_type {
    FREQ_QOS_MIN = 1,
    FREQ_QOS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freq_constraints {
    pub min_freq: pm_qos_constraints,
    pub min_freq_notifiers: blocking_notifier_head,
    pub max_freq: pm_qos_constraints,
    pub max_freq_notifiers: blocking_notifier_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freq_qos_request {
    pub type: freq_qos_req_type,
    pub pnode: plist_node,
    pub qos: *mut freq_constraints,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_pm_qos_req_type {
    DEV_PM_QOS_RESUME_LATENCY = 1,
    DEV_PM_QOS_LATENCY_TOLERANCE,
    DEV_PM_QOS_MIN_FREQUENCY,
    DEV_PM_QOS_MAX_FREQUENCY,
    DEV_PM_QOS_FLAGS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_qos_request {
    pub type: dev_pm_qos_req_type,
    pub pnode: plist_node,
    pub flr: pm_qos_flags_request,
    pub freq: freq_qos_request,
    pub data: },
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_qos {
    pub resume_latency: pm_qos_constraints,
    pub latency_tolerance: pm_qos_constraints,
    pub freq: freq_constraints,
    pub flags: pm_qos_flags,
    pub resume_latency_req: *mut dev_pm_qos_request,
    pub latency_tolerance_req: *mut dev_pm_qos_request,
    pub flags_req: *mut dev_pm_qos_request,
}

// Action requested to pm_qos_update_target
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_qos_req_action {
    PM_QOS_ADD_REQ,		/* Add a new request */
    PM_QOS_UPDATE_REQ,	/* Update an existing request */
    PM_QOS_REMOVE_REQ	/* Remove an existing request */
}

extern "C" {
    pub fn pm_qos_read_value(c: *mut pm_qos_constraints) -> i32;
}

extern "C" {
    pub fn cpu_latency_qos_limit() -> i32;
}
extern "C" {
    pub fn cpu_latency_qos_request_active(req: *mut pm_qos_request) -> bool;
}
extern "C" {
    pub fn cpu_latency_qos_add_request(req: *mut pm_qos_request, value: i32);
}
extern "C" {
    pub fn cpu_latency_qos_update_request(req: *mut pm_qos_request, new_value: i32);
}
extern "C" {
    pub fn cpu_latency_qos_remove_request(req: *mut pm_qos_request);
}

extern "C" {
    pub fn cpu_wakeup_latency_qos_limit() -> i32;
}

extern "C" {
    pub fn __dev_pm_qos_flags(dev: *mut device, mask: i32) -> pm_qos_flags_status;
}
extern "C" {
    pub fn dev_pm_qos_flags(dev: *mut device, mask: i32) -> pm_qos_flags_status;
}
extern "C" {
    pub fn __dev_pm_qos_resume_latency(dev: *mut device) -> i32;
}
extern "C" {
    pub fn dev_pm_qos_read_value(dev: *mut device, type: dev_pm_qos_req_type) -> i32;
}
extern "C" {
    pub fn dev_pm_qos_update_request(req: *mut dev_pm_qos_request, new_value: i32) -> c_int;
}
extern "C" {
    pub fn dev_pm_qos_remove_request(req: *mut dev_pm_qos_request) -> c_int;
}
extern "C" {
    pub fn dev_pm_qos_constraints_init(dev: *mut device);
}
extern "C" {
    pub fn dev_pm_qos_constraints_destroy(dev: *mut device);
}
extern "C" {
    pub fn dev_pm_qos_expose_latency_limit(dev: *mut device, value: i32) -> c_int;
}
extern "C" {
    pub fn dev_pm_qos_hide_latency_limit(dev: *mut device);
}
extern "C" {
    pub fn dev_pm_qos_expose_flags(dev: *mut device, value: i32) -> c_int;
}
extern "C" {
    pub fn dev_pm_qos_hide_flags(dev: *mut device);
}
extern "C" {
    pub fn dev_pm_qos_update_flags(dev: *mut device, mask: i32, set: bool) -> c_int;
}
extern "C" {
    pub fn dev_pm_qos_get_user_latency_tolerance(dev: *mut device) -> i32;
}
extern "C" {
    pub fn dev_pm_qos_update_user_latency_tolerance(dev: *mut device, val: i32) -> c_int;
}
extern "C" {
    pub fn dev_pm_qos_expose_latency_tolerance(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dev_pm_qos_hide_latency_tolerance(dev: *mut device);
}

extern "C" {
    pub fn freq_constraints_init(qos: *mut freq_constraints);
}
extern "C" {
    pub fn freq_qos_update_request(req: *mut freq_qos_request, new_value: i32) -> c_int;
}
extern "C" {
    pub fn freq_qos_remove_request(req: *mut freq_qos_request) -> c_int;
}
