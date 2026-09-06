//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/surface_aggregator/dtx.h
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
// Surface DTX (clipboard detachment system driver) user-space interface.
//
// Definitions, structs, and IOCTLs for the /dev/surface/dtx misc device. This
// device allows user-space to control the clipboard detachment process on
// Surface Book series devices.
//
// Copyright (C) 2020-2021 Maximilian Luz <luzmaximilian@gmail.com>
//

// Status/error categories
pub const SDTX_CATEGORY_STATUS: c_uint = 0x0000;
pub const SDTX_CATEGORY_RUNTIME_ERROR: c_uint = 0x1000;
pub const SDTX_CATEGORY_HARDWARE_ERROR: c_uint = 0x2000;
pub const SDTX_CATEGORY_UNKNOWN: c_uint = 0xf000;
pub const SDTX_CATEGORY_MASK: c_uint = 0xf000;

// Latch status values

// Base state values

// Runtime errors (non-critical)

// Hardware errors (critical)

// Base types
pub const SDTX_DEVICE_TYPE_HID: c_uint = 0x0100;
pub const SDTX_DEVICE_TYPE_SSH: c_uint = 0x0200;
pub const SDTX_DEVICE_TYPE_MASK: c_uint = 0x0f00;

//
// enum sdtx_device_mode - Mode describing how (and if) the clipboard is
// attached to the base of the device.
// @SDTX_DEVICE_MODE_TABLET: The clipboard is detached from the base and the
// device operates as tablet.
// @SDTX_DEVICE_MODE_LAPTOP: The clipboard is attached normally to the base
// and the device operates as laptop.
// @SDTX_DEVICE_MODE_STUDIO: The clipboard is attached to the base in reverse.
// The device operates as tablet with keyboard and
// touchpad deactivated, however, the base battery
// and, if present in the specific device model, dGPU
// are available to the system.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdtx_device_mode {
    SDTX_DEVICE_MODE_TABLET		= 0x00,
    SDTX_DEVICE_MODE_LAPTOP		= 0x01,
    SDTX_DEVICE_MODE_STUDIO		= 0x02,
}

//
// struct sdtx_event - Event provided by reading from the DTX device file.
// @length: Length of the event payload, in bytes.
// @code:   Event code, detailing what type of event this is.
// @data:   Payload of the event, containing @length bytes.
//
// See &enum sdtx_event_code for currently valid event codes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdtx_event {
    pub length: __u16,
    pub code: __u16,
    pub data: [__u8; ],
    pub __attribute__((__packed__)): },
//
// enum sdtx_event_code - Code describing the type of an event.
// @SDTX_EVENT_REQUEST:         Detachment request event type.
// @SDTX_EVENT_CANCEL:          Cancel detachment process event type.
// @SDTX_EVENT_BASE_CONNECTION: Base/clipboard connection change event type.
// @SDTX_EVENT_LATCH_STATUS:    Latch status change event type.
// @SDTX_EVENT_DEVICE_MODE:     Device mode change event type.
//
// Used in &struct sdtx_event to describe the type of the event. Further event
// codes are reserved for future use. Any event parser should be able to
// gracefully handle unknown events, i.e. by simply skipping them.
//
// Consult the DTX user-space interface documentation for details regarding
// the individual event types.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdtx_event_code {
    SDTX_EVENT_REQUEST		= 1,
    SDTX_EVENT_CANCEL		= 2,
    SDTX_EVENT_BASE_CONNECTION	= 3,
    SDTX_EVENT_LATCH_STATUS		= 4,
    SDTX_EVENT_DEVICE_MODE		= 5,
}

//
// struct sdtx_base_info - Describes if and what type of base is connected.
// @state:   The state of the connection. Valid values are %SDTX_BASE_DETACHED,
// %SDTX_BASE_ATTACHED, and %SDTX_DETACH_NOT_FEASIBLE (in case a base
// is attached but low clipboard battery prevents detachment). Other
// values are currently reserved.
// @base_id: The type of base connected. Zero if no base is connected.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdtx_base_info {
    pub state: __u16,
    pub base_id: __u16,
    pub __attribute__((__packed__)): },
// IOCTLs

