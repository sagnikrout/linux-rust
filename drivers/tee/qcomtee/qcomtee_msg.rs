//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/qcomtee/qcomtee_msg.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

//
// DOC: ''Qualcomm TEE'' (QTEE) Transport Message
//
// There are two buffers shared with QTEE: inbound and outbound buffers.
// The inbound buffer is used for direct object invocation, and the outbound
// buffer is used to make a request from QTEE to the kernel; i.e., a callback
// request.
//
// The unused tail of the outbound buffer is also used for sending and
// receiving asynchronous messages. An asynchronous message is independent of
// the current object invocation (i.e., contents of the inbound buffer) or
// callback request (i.e., the head of the outbound buffer); see
// qcomtee_get_async_buffer(). It is used by endpoints (QTEE or kernel) as an
// optimization to reduce the number of context switches between the secure and
// non-secure worlds.
//
// For instance, QTEE never sends an explicit callback request to release an
// object in the kernel. Instead, it sends asynchronous release messages in the
// outbound buffer when QTEE returns from the previous direct object invocation,
// or appends asynchronous release messages after the current callback request.
//
// QTEE supports two types of arguments in a message: buffer and object
// arguments. Depending on the direction of data flow, they could be input
// buffer (IO) to QTEE, output buffer (OB) from QTEE, input object (IO) to QTEE,
// or output object (OO) from QTEE. Object arguments hold object IDs. Buffer
// arguments hold (offset, size) pairs into the inbound or outbound buffers.
//
// QTEE holds an object table for objects it hosts and exposes to the kernel.
// An object ID is an index to the object table in QTEE.
//
// For the direct object invocation message format in the inbound buffer, see
// &struct qcomtee_msg_object_invoke. For the callback request message format
// in the outbound buffer, see &struct qcomtee_msg_callback. For the message
// format for asynchronous messages in the outbound buffer, see
// &struct qcomtee_async_msg_hdr.
//
// define QCOMTEE_MSG_OBJECT_NS_BIT - Non-secure bit
//
// Object ID is a globally unique 32-bit number. IDs referencing objects
// in the kernel should have %QCOMTEE_MSG_OBJECT_NS_BIT set.
//

// Static object IDs recognized by QTEE.

// Definitions from QTEE as part of the transport protocol.
// qcomtee_msg_arg is an argument as recognized by QTEE.
#[repr(C)]
#[derive(Copy, Clone)]
pub union qcomtee_msg_arg {
    pub offset: u32,
    pub size: u32,
    pub b: },
    pub o: u32,
}

// BI and BO payloads in QTEE messages should be at 64-bit boundaries.

// Operations for objects are 32-bit. Transport uses the upper 16 bits.

// Reserved Operation IDs sent to QTEE:
// QCOMTEE_MSG_OBJECT_OP_RELEASE - Reduces the refcount and releases the object.
// QCOMTEE_MSG_OBJECT_OP_RETAIN  - Increases the refcount.
//
// These operation IDs are valid for all objects.
//

// Subset of operations supported by QTEE root object.
pub const QCOMTEE_ROOT_OP_REG_WITH_CREDENTIALS: c_int = 5;
pub const QCOMTEE_ROOT_OP_NOTIFY_DOMAIN_CHANGE: c_int = 4;
pub const QCOMTEE_ROOT_OP_ADCI_ACCEPT: c_int = 8;
pub const QCOMTEE_ROOT_OP_ADCI_SHUTDOWN: c_int = 9;
// Subset of operations supported by client_env object.
pub const QCOMTEE_CLIENT_ENV_OPEN: c_int = 0;
// List of available QTEE service UIDs and subset of operations.
pub const QCOMTEE_FEATURE_VER_UID: c_int = 2033;
pub const QCOMTEE_FEATURE_VER_OP_GET: c_int = 0;
// Get QTEE version number.
pub const QCOMTEE_FEATURE_VER_OP_GET_QTEE_ID: c_int = 10;

// Response types as returned from qcomtee_object_invoke_ctx_invoke().
// The message contains a callback request.
pub const QCOMTEE_RESULT_INBOUND_REQ_NEEDED: c_int = 3;
//
// struct qcomtee_msg_object_invoke - Direct object invocation message.
// @ctx: object ID hosted in QTEE.
// @op: operation for the object.
// @counts: number of different types of arguments in @args.
// @args: array of arguments.
//
// @counts consists of 4 * 4-bit fields. Bits 0 - 3 represent the number of
// input buffers, bits 4 - 7 represent the number of output buffers,
// bits 8 - 11 represent the number of input objects, and bits 12 - 15
// represent the number of output objects. The remaining bits should be zero.
//
// 15            12 11             8 7              4 3              0
// +----------------+----------------+----------------+----------------+
// |  #OO objects   |  #IO objects   |  #OB buffers   |  #IB buffers   |
// +----------------+----------------+----------------+----------------+
//
// The maximum number of arguments of each type is defined by
// %QCOMTEE_ARGS_PER_TYPE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_msg_object_invoke {
    pub cxt: u32,
    pub op: u32,
    pub counts: u32,
    pub args: [qcomtee_msg_arg; ],
}

// Bit masks for the four 4-bit nibbles holding the counts.

//
// struct qcomtee_msg_callback - Callback request message.
// @result: result of operation @op on the object referenced by @cxt.
// @cxt: object ID hosted in the kernel.
// @op: operation for the object.
// @counts: number of different types of arguments in @args.
// @args: array of arguments.
//
// For details of @counts, see &qcomtee_msg_object_invoke.counts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_msg_callback {
    pub result: u32,
    pub cxt: u32,
    pub op: u32,
    pub counts: u32,
    pub args: [qcomtee_msg_arg; ],
}

// Offset in the message for the beginning of the buffer argument's contents.

// Pointer to the beginning of a buffer argument's content at an offset.

// Some helpers to manage msg.counts.
extern "C" {
    pub fn FIELD_GET(_arg: QCOMTEE_MASK_IB, _arg: counts) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: QCOMTEE_MASK_OB, _arg: counts) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: QCOMTEE_MASK_IO, _arg: counts) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: QCOMTEE_MASK_OO, _arg: counts) -> return;
}
extern "C" {
    pub fn qcomtee_msg_num_ib(_arg: counts) -> return;
}
extern "C" {
    pub fn qcomtee_msg_idx_ob(qcomtee_msg_num_ob(counts: counts) +) -> return;
}
extern "C" {
    pub fn qcomtee_msg_idx_io(qcomtee_msg_num_io(counts: counts) +) -> return;
}

// Sum of arguments in a message.

// Generic error codes.

// Transport layer error codes.

// If err < 0, then it is a transport error.
// If err > 0, then it is user defined error, pass it as is.
