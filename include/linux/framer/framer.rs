//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/framer/framer.h
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
// Generic framer header file
//
// Copyright 2023 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>
//

//
// enum framer_iface - Framer interface
// @FRAMER_IFACE_E1: E1 interface
// @FRAMER_IFACE_T1: T1 interface
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum framer_iface {
    FRAMER_IFACE_E1,
    FRAMER_IFACE_T1,
}

//
// enum framer_clock_type - Framer clock type
// @FRAMER_CLOCK_EXT: External clock
// @FRAMER_CLOCK_INT: Internal clock
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum framer_clock_type {
    FRAMER_CLOCK_EXT,
    FRAMER_CLOCK_INT,
}

//
// struct framer_config - Framer configuration
// @iface: Framer line interface
// @clock_type: Framer clock type
// @line_clock_rate: Framer line clock rate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct framer_config {
    pub iface: framer_iface,
    pub clock_type: framer_clock_type,
    pub line_clock_rate: c_ulong,
}

//
// struct framer_status - Framer status
// @link_is_on: Framer link state. true, the link is on, false, the link is off.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct framer_status {
    pub link_is_on: bool,
}

//
// enum framer_event - Event available for notification
// @FRAMER_EVENT_STATUS: Event notified on framer_status changes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum framer_event {
    FRAMER_EVENT_STATUS,
}

//
// struct framer - represents the framer device
// @dev: framer device
// @id: id of the framer device
// @ops: function pointers for performing framer operations
// @mutex: mutex to protect framer_ops
// @init_count: used to protect when the framer is used by multiple consumers
// @power_count: used to protect when the framer is used by multiple consumers
// @pwr: power regulator associated with the framer
// @notify_status_work: work structure used for status notifications
// @notifier_list: notifier list used for notifications
// @polling_work: delayed work structure used for the polling task
// @prev_status: previous read status used by the polling task to detect changes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct framer {
    pub dev: device,
    pub id: c_int,
    pub ops: *const framer_ops,
    pub /: *mut *mut mutex mutex; / Protect framer,
    pub init_count: c_int,
    pub power_count: c_int,
    pub pwr: *mut regulator,
    pub notify_status_work: work_struct,
    pub notifier_list: blocking_notifier_head,
    pub polling_work: delayed_work,
    pub prev_status: framer_status,
}

extern "C" {
    pub fn framer_pm_runtime_get(framer: *mut framer) -> c_int;
}
extern "C" {
    pub fn framer_pm_runtime_get_sync(framer: *mut framer) -> c_int;
}
extern "C" {
    pub fn framer_pm_runtime_put(framer: *mut framer);
}
extern "C" {
    pub fn framer_pm_runtime_put_sync(framer: *mut framer) -> c_int;
}
extern "C" {
    pub fn framer_init(framer: *mut framer) -> c_int;
}
extern "C" {
    pub fn framer_exit(framer: *mut framer) -> c_int;
}
extern "C" {
    pub fn framer_power_on(framer: *mut framer) -> c_int;
}
extern "C" {
    pub fn framer_power_off(framer: *mut framer) -> c_int;
}
extern "C" {
    pub fn framer_get_status(framer: *mut framer, status: *mut framer_status) -> c_int;
}
extern "C" {
    pub fn framer_get_config(framer: *mut framer, config: *mut framer_config) -> c_int;
}
extern "C" {
    pub fn framer_set_config(framer: *mut framer, config: *const framer_config) -> c_int;
}
extern "C" {
    pub fn framer_notifier_register(framer: *mut framer, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn framer_notifier_unregister(framer: *mut framer, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn framer_put(dev: *mut device, framer: *mut framer);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}

