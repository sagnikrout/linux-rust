//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/surface_aggregator/cdev.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Surface System Aggregator Module (SSAM) user-space EC interface.
//
// Definitions, structs, and IOCTLs for the /dev/surface/aggregator misc
// device. This device provides direct user-space access to the SSAM EC.
// Intended for debugging and development.
//
// Copyright (C) 2020-2021 Maximilian Luz <luzmaximilian@gmail.com>
//

//
// enum ssam_cdev_request_flags - Request flags for SSAM cdev request IOCTL.
//
// @SSAM_CDEV_REQUEST_HAS_RESPONSE:
// Specifies that the request expects a response. If not set, the request
// will be directly completed after its underlying packet has been
// transmitted. If set, the request transport system waits for a response
// of the request.
//
// @SSAM_CDEV_REQUEST_UNSEQUENCED:
// Specifies that the request should be transmitted via an unsequenced
// packet. If set, the request must not have a response, meaning that this
// flag and the %SSAM_CDEV_REQUEST_HAS_RESPONSE flag are mutually
// exclusive.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssam_cdev_request_flags {
    SSAM_CDEV_REQUEST_HAS_RESPONSE = 0x01,
    SSAM_CDEV_REQUEST_UNSEQUENCED  = 0x02,
}

//
// struct ssam_cdev_request - Controller request IOCTL argument.
// @target_category: Target category of the SAM request.
// @target_id:       Target ID of the SAM request.
// @command_id:      Command ID of the SAM request.
// @instance_id:     Instance ID of the SAM request.
// @flags:           Request flags (see &enum ssam_cdev_request_flags).
// @status:          Request status (output).
// @payload:         Request payload (input data).
// @payload.data:    Pointer to request payload data.
// @payload.length:  Length of request payload data (in bytes).
// @response:        Request response (output data).
// @response.data:   Pointer to response buffer.
// @response.length: On input: Capacity of response buffer (in bytes).
// On output: Length of request response (number of bytes
// in the buffer that are actually used).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_cdev_request {
    pub target_category: __u8,
    pub target_id: __u8,
    pub command_id: __u8,
    pub instance_id: __u8,
    pub flags: __u16,
    pub status: __s16,
    pub data: __u64,
    pub length: __u16,
    pub __pad: [__u8; 6],
    pub payload: },
    pub data: __u64,
    pub length: __u16,
    pub __pad: [__u8; 6],
    pub response: },
    pub __attribute__((__packed__)): },
//
// struct ssam_cdev_notifier_desc - Notifier descriptor.
// @priority:        Priority value determining the order in which notifier
// callbacks will be called. A higher value means higher
// priority, i.e. the associated callback will be executed
// earlier than other (lower priority) callbacks.
// @target_category: The event target category for which this notifier should
// receive events.
//
// Specifies the notifier that should be registered or unregistered,
// specifically with which priority and for which target category of events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_cdev_notifier_desc {
    pub priority: __s32,
    pub target_category: __u8,
    pub __attribute__((__packed__)): },
//
// struct ssam_cdev_event_desc - Event descriptor.
// @reg:                 Registry via which the event will be enabled/disabled.
// @reg.target_category: Target category for the event registry requests.
// @reg.target_id:       Target ID for the event registry requests.
// @reg.cid_enable:      Command ID for the event-enable request.
// @reg.cid_disable:     Command ID for the event-disable request.
// @id:                  ID specifying the event.
// @id.target_category:  Target category of the event source.
// @id.instance:         Instance ID of the event source.
// @flags:               Flags used for enabling the event.
//
// Specifies which event should be enabled/disabled and how to do that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_cdev_event_desc {
    pub target_category: __u8,
    pub target_id: __u8,
    pub cid_enable: __u8,
    pub cid_disable: __u8,
    pub reg: },
    pub target_category: __u8,
    pub instance: __u8,
    pub id: },
    pub flags: __u8,
    pub __attribute__((__packed__)): },
//
// struct ssam_cdev_event - SSAM event sent by the EC.
// @target_category: Target category of the event source. See &enum ssam_ssh_tc.
// @target_id:       Target ID of the event source.
// @command_id:      Command ID of the event.
// @instance_id:     Instance ID of the event source.
// @length:          Length of the event payload in bytes.
// @data:            Event payload data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_cdev_event {
    pub target_category: __u8,
    pub target_id: __u8,
    pub command_id: __u8,
    pub instance_id: __u8,
    pub length: __u16,
    pub data: [__u8; ],
    pub __attribute__((__packed__)): },

