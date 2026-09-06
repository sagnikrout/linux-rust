//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/raspberrypi/vchiq-mmal/mmal-msg-port.h
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
// Broadcom BCM2835 V4L2 driver
//
// Copyright © 2013 Raspberry Pi (Trading) Ltd.
//
// Authors: Vincent Sanders @ Collabora
// Dave Stevenson @ Broadcom
// (now dave.stevenson@raspberrypi.org)
// Simon Mellor @ Broadcom
// Luke Diamand @ Broadcom
//
// MMAL_PORT_TYPE_T
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmal_port_type {
    MMAL_PORT_TYPE_UNKNOWN = 0,	/* Unknown port type */
    MMAL_PORT_TYPE_CONTROL,		/* Control port */
    MMAL_PORT_TYPE_INPUT,		/* Input port */
    MMAL_PORT_TYPE_OUTPUT,		/* Output port */
    MMAL_PORT_TYPE_CLOCK,		/* Clock port */
}

// The port is pass-through and doesn't need buffer headers allocated
pub const MMAL_PORT_CAPABILITY_PASSTHROUGH: c_uint = 0x01;
//
// The port wants to allocate the buffer payloads.
// This signals a preference that payload allocation should be done
// on this port for efficiency reasons.
//
pub const MMAL_PORT_CAPABILITY_ALLOCATION: c_uint = 0x02;
//
// The port supports format change events.
// This applies to input ports and is used to let the client know
// whether the port supports being reconfigured via a format
// change event (i.e. without having to disable the port).
//
pub const MMAL_PORT_CAPABILITY_SUPPORTS_EVENT_FORMAT_CHANGE: c_uint = 0x04;
//
// mmal port structure (MMAL_PORT_T)
//
// most elements are informational only, the pointer values for
// interogation messages are generally provided as additional
// structures within the message. When used to set values only the
// buffer_num, buffer_size and userdata parameters are writable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmal_port {
    pub /: *mut *mut u32 priv; / Private member used by the framework,
    pub /: *mut *mut u32 name; / Port name. Used for debugging purposes (RO),
    pub /: *mut *mut u32 type; / Type of the port (RO) enum mmal_port_type,
    pub /: *mut *mut u16 index; / Index of the port in its type list (RO),
    pub /: *mut *mut u16 index_all; / Index of the port in the list of all ports (RO),
    pub /: *mut *mut u32 is_enabled; / Indicates whether the port is enabled or not (RO),
    pub /: *mut *mut u32 format; / Format of the elementary stream,
    pub port: *mut *mut u32 buffer_num_min; / Minimum number of buffers the,
// requires (RO).  This is set by the
// component.
//
    pub port: *mut *mut u32 buffer_size_min; / Minimum size of buffers the,
// requires (RO).  This is set by the
// component.
//
    pub for: *mut *mut u32 buffer_alignment_min;/ Minimum alignment requirement,
// the buffers (RO).  A value of
// zero means no special alignment
// requirements.  This is set by the
// component.
//
    pub port: *mut *mut u32 buffer_num_recommended; / Number of buffers the,
// recommends for optimal
// performance (RO).  A value of
// zero means no special
// recommendation.  This is set
// by the component.
//
    pub port: *mut *mut u32 buffer_size_recommended; / Size of buffers the,
// recommends for optimal
// performance (RO).  A value of
// zero means no special
// recommendation.  This is set
// by the component.
//
    pub use.: *mut *mut u32 buffer_num; / Actual number of buffers the port will,
// This is set by the client.
//
    pub that: *mut *mut u32 buffer_size; / Actual maximum size of the buffers,
// will be sent to the port. This is set by
// the client.
//
    pub /: *mut *mut u32 component; / Component this port belongs to (Read Only),
    pub /: *mut *mut u32 userdata; / Field reserved for use by the client,
    pub a: *mut *mut u32 capabilities; / Flags describing the capabilities of,
// port (RO).  Bitwise combination of \ref
// portcapabilities "Port capabilities"
// values.
//
}
