//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/greybus/greybus_trace.h
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
// Greybus driver and device API
//
// Copyright 2015 Google Inc.
// Copyright 2015 Linaro Ltd.
//

//
// Occurs immediately before calling a host device's message_send()
// method.
//
// Occurs after an incoming request message has been received
//
// Occurs after an incoming response message has been received,
// after its matching request has been found.
//
// Occurs after an operation has been canceled, possibly before the
// cancellation is complete.
//
// Occurs when an incoming request is cancelled; if the response has
// been queued for sending, this occurs after it is sent.
//
// Occurs in the host driver message_send() function just prior to
// handing off the data to be processed by hardware.
//

//
// Occurs after a new operation is created for an outgoing request
// has been successfully created.
//
// Occurs after a new core operation has been created.
//
// Occurs after a new operation has been created for an incoming
// request has been successfully created and initialized.
//
// Occurs when the last reference to an operation has been dropped,
// prior to freeing resources.
//
// Occurs when an operation has been marked active, after updating
// its active count.
//
// Occurs when an operation has been marked active, before updating
// its active count.
//

// name contains "hd_cport_id/intf_id:cport_id"

//
// Occurs after a new connection is successfully created.
//
// Occurs when the last reference to a connection has been dropped,
// before its resources are freed.
//
// Occurs when a new reference to connection is added, currently
// only when a message over the connection is received.
//
// Occurs when a new reference to connection is dropped, after a
// a received message is handled, or when the connection is
// destroyed.
//
// Occurs when a request to enable a connection is made, either for
// transmit only, or for both transmit and receive.
//
// Occurs when a request to disable a connection is made, either for
// receive only, or for both transmit and receive.  Also occurs when
// a request to forcefully disable a connection is made.
//

//
// Occurs after a new bundle is successfully created.
//
// Occurs when the last reference to a bundle has been dropped,
// before its resources are freed.
//
// Occurs when a bundle is added to an interface when the interface
// is enabled.
//
// Occurs when a registered bundle gets destroyed, normally at the
// time an interface is disabled.
//

//
// Occurs after a new interface is successfully created.
//
// Occurs after the last reference to an interface has been dropped.
//
// Occurs after an interface been registerd.
//
// Occurs when a registered interface gets deregisterd.
//
// Occurs when a registered interface has been successfully
// activated.
//
// Occurs when an activated interface is being deactivated.
//
// Occurs when an interface has been successfully enabled.
//
// Occurs when an enabled interface is being disabled.
//

//
// Occurs after a new module is successfully created, before
// creating any of its interfaces.
//
// Occurs after the last reference to a module has been dropped.
//
// Occurs after a module is successfully created, before registering
// any of its interfaces.
//
// Occurs when a module is deleted, before deregistering its
// interfaces.
//

//
// Occurs after a new host device is successfully created, before
// its SVC has been set up.
//
// Occurs after the last reference to a host device has been
// dropped.
//
// Occurs after a new host device has been added, after the
// connection to its SVC has been enabled.
//
// Occurs when a host device is being disconnected from the AP USB
// host controller.
//
// Occurs when a host device has passed received data to the Greybus
// core, after it has been determined it is destined for a valid
// CPort.
//

// This part must be outside protection

//
// TRACE_INCLUDE_FILE is not needed if the filename and TRACE_SYSTEM are equal
//

