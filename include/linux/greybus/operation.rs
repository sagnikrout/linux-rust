//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/greybus/operation.h
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
// Greybus operations
//
// Copyright 2014 Google Inc.
// Copyright 2014 Linaro Ltd.
//

// The default amount of time a request is given to complete

//
// The top bit of the type in an operation message header indicates
// whether the message is a request (bit clear) or response (bit set)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gb_operation_result {
    GB_OP_SUCCESS		= 0x00,
    GB_OP_INTERRUPTED	= 0x01,
    GB_OP_TIMEOUT		= 0x02,
    GB_OP_NO_MEMORY		= 0x03,
    GB_OP_PROTOCOL_BAD	= 0x04,
    GB_OP_OVERFLOW		= 0x05,
    GB_OP_INVALID		= 0x06,
    GB_OP_RETRY		= 0x07,
    GB_OP_NONEXISTENT	= 0x08,
    GB_OP_UNKNOWN_ERROR	= 0xfe,
    GB_OP_MALFUNCTION	= 0xff,
}

//
// Protocol code should only examine the payload and payload_size fields, and
// host-controller drivers may use the hcpriv field. All other fields are
// intended to be private to the operations core code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_message {
    pub operation: *mut gb_operation,
    pub header: *mut gb_operation_msg_hdr,
    pub payload: *mut c_void,
    pub payload_size: usize,
    pub buffer: *mut c_void,
    pub hcpriv: *mut c_void,
}

//
// A Greybus operation is a remote procedure call performed over a
// connection between two UniPro interfaces.
//
// Every operation consists of a request message sent to the other
// end of the connection coupled with a reply message returned to
// the sender.  Every operation has a type, whose interpretation is
// dependent on the protocol associated with the connection.
//
// Only four things in an operation structure are intended to be
// directly usable by protocol handlers:  the operation's connection
// pointer; the operation type; the request message payload (and
// size); and the response message payload (and size).  Note that a
// message with a 0-byte payload has a null message payload pointer.
//
// In addition, every operation has a result, which is an errno
// value.  Protocol handlers access the operation result using
// gb_operation_result().
//
extern "C" {
    pub fn void(: *mut *mut gb_operation_callback)(struct gb_operation) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_operation {
    pub connection: *mut gb_connection,
    pub request: *mut gb_message,
    pub response: *mut gb_message,
    pub flags: c_ulong,
    pub type: u8,
    pub id: u16,
    pub /: *mut *mut int errno; / Operation result,
    pub work: work_struct,
    pub callback: gb_operation_callback,
    pub completion: completion,
    pub timer: timer_list,
    pub kref: kref,
    pub waiters: core::sync::atomic::AtomicI32,
    pub active: c_int,
    pub /: *mut *mut list_head links; / connection->operations,
    pub private: *mut c_void,
}

extern "C" {
    pub fn gb_operation_result(operation: *mut gb_operation) -> c_int;
}
extern "C" {
    pub fn gb_operation_get_payload_size_max(connection: *mut gb_connection) -> usize;
}
extern "C" {
    pub fn gb_operation_get(operation: *mut gb_operation);
}
extern "C" {
    pub fn gb_operation_put(operation: *mut gb_operation);
}
extern "C" {
    pub fn gb_operation_cancel(operation: *mut gb_operation, errno: c_int);
}
extern "C" {
    pub fn gb_operation_cancel_incoming(operation: *mut gb_operation, errno: c_int);
}
extern "C" {
    pub fn gb_operation_init() -> c_int;
}
extern "C" {
    pub fn gb_operation_exit();
}
