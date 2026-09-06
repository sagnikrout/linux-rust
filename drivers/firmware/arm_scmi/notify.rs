//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/arm_scmi/notify.h
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
// System Control and Management Interface (SCMI) Message Protocol
// notification header file containing some definitions, structures
// and function prototypes related to SCMI Notification handling.
//
// Copyright (C) 2020-2021 ARM Ltd.
//

pub const SCMI_PROTO_QUEUE_SZ: c_int = 4096;
//
// struct scmi_event  - Describes an event to be supported
// @id: Event ID
// @max_payld_sz: Max possible size for the payload of a notification message
// @max_report_sz: Max possible size for the report of a notification message
//
// Each SCMI protocol, during its initialization phase, can describe the events
// it wishes to support in a few struct scmi_event and pass them to the core
// using scmi_register_protocol_events().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_event {
    pub id: u8,
    pub max_payld_sz: usize,
    pub max_report_sz: usize,
}

//
// struct scmi_event_ops  - Protocol helpers called by the notification core.
// @is_notify_supported: Return 0 if the specified notification for the
// specified resource (src_id) is supported.
// @get_num_sources: Returns the number of possible events' sources for this
// protocol
// @set_notify_enabled: Enable/disable the required evt_id/src_id notifications
// using the proper custom protocol commands.
// Return 0 on Success
// @fill_custom_report: fills a custom event report from the provided
// event message payld identifying the event
// specific src_id.
// Return NULL on failure otherwise @report now fully
// populated
//
// Context: Helpers described in &struct scmi_event_ops are called only in
// process context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_event_ops {
    pub src_id): u8 evt_id, u32,
    pub ph): *const *const int (get_num_sources)(struct scmi_protocol_handle,
    pub enabled): u8 evt_id, u32 src_id, bool,
    pub src_id): *mut *mut void report, u32,
}

//
// struct scmi_protocol_events  - Per-protocol description of available events
// @queue_sz: Size in bytes of the per-protocol queue to use.
// @ops: Array of protocol-specific events operations.
// @evts: Array of supported protocol's events.
// @num_events: Number of supported protocol's events described in @evts.
// @num_sources: Number of protocol's sources, should be greater than 0; if not
// available at compile time, it will be provided at run-time via
// @get_num_sources.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_protocol_events {
    pub queue_sz: usize,
    pub ops: *const scmi_event_ops,
    pub evts: *const scmi_event,
    pub num_events: c_uint,
    pub num_sources: c_uint,
}

extern "C" {
    pub fn scmi_notification_init(handle: *mut scmi_handle) -> c_int;
}
extern "C" {
    pub fn scmi_notification_quiesce(handle: *mut scmi_handle);
}
extern "C" {
    pub fn scmi_notification_exit(handle: *mut scmi_handle);
}
