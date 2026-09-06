//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/qcomtee/qcomtee_object.h
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
// DOC: Overview
//
// qcomtee_object provides object refcounting, ID allocation for objects hosted
// in the kernel, and necessary message marshaling for Qualcomm TEE (QTEE).
//
// To invoke an object in QTEE, the user calls qcomtee_object_do_invoke()
// while passing an instance of &struct qcomtee_object and the requested
// operation + arguments.
//
// After boot, QTEE provides a static object %ROOT_QCOMTEE_OBJECT (type of
// %QCOMTEE_OBJECT_TYPE_ROOT). The root object is invoked to pass the user's
// credentials and obtain other instances of &struct qcomtee_object (type of
// %QCOMTEE_OBJECT_TYPE_TEE) that represent services and TAs in QTEE;
// see &enum qcomtee_object_type.
//
// The objects received from QTEE are refcounted. So the owner of these objects
// can issue qcomtee_object_get() to increase the refcount and pass objects
// to other clients, or issue qcomtee_object_put() to decrease the refcount
// and release the resources in QTEE.
//
// The kernel can host services accessible to QTEE. A driver should embed
// an instance of &struct qcomtee_object in the struct it wants to export to
// QTEE (this is called a callback object). It issues qcomtee_object_user_init()
// to set the dispatch() operation for the callback object and set its type
// to %QCOMTEE_OBJECT_TYPE_CB.
//
// core.c holds an object table for callback objects. An object ID is assigned
// to each callback object, which is an index to the object table. QTEE uses
// these IDs to reference or invoke callback objects.
//
// If QTEE invokes a callback object in the kernel, the dispatch() operation is
// called in the context of the thread that originally called
// qcomtee_object_do_invoke().
//
// enum qcomtee_object_type - Object types.
// @QCOMTEE_OBJECT_TYPE_TEE: object hosted on QTEE.
// @QCOMTEE_OBJECT_TYPE_CB: object hosted on kernel.
// @QCOMTEE_OBJECT_TYPE_ROOT: 'primordial' object.
// @QCOMTEE_OBJECT_TYPE_NULL: NULL object.
//
// The primordial object is used for bootstrapping the IPC connection between
// the kernel and QTEE. It is invoked by the kernel when it wants to get a
// 'client env'.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcomtee_object_type {
    QCOMTEE_OBJECT_TYPE_TEE,
    QCOMTEE_OBJECT_TYPE_CB,
    QCOMTEE_OBJECT_TYPE_ROOT,
    QCOMTEE_OBJECT_TYPE_NULL,
}

//
// enum qcomtee_arg_type - Type of QTEE argument.
// @QCOMTEE_ARG_TYPE_INV: invalid type.
// @QCOMTEE_ARG_TYPE_OB: output buffer (OB).
// @QCOMTEE_ARG_TYPE_OO: output object (OO).
// @QCOMTEE_ARG_TYPE_IB: input buffer (IB).
// @QCOMTEE_ARG_TYPE_IO: input object (IO).
//
// Use the invalid type to specify the end of the argument array.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcomtee_arg_type {
    QCOMTEE_ARG_TYPE_INV = 0,
    QCOMTEE_ARG_TYPE_OB,
    QCOMTEE_ARG_TYPE_OO,
    QCOMTEE_ARG_TYPE_IB,
    QCOMTEE_ARG_TYPE_IO,
    QCOMTEE_ARG_TYPE_NR,
}

//
// define QCOMTEE_ARGS_PER_TYPE - Maximum arguments of a specific type.
//
// The QTEE transport protocol limits the maximum number of arguments of
// a specific type (i.e., IB, OB, IO, and OO).
//
pub const QCOMTEE_ARGS_PER_TYPE: c_int = 16;
// Maximum arguments that can fit in a QTEE message, ignoring the type.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_buffer {
    pub addr: *mut c_void,
    pub uaddr: *mut void __user,
}

//
// struct qcomtee_arg - Argument for QTEE object invocation.
// @type: type of argument as &enum qcomtee_arg_type.
// @flags: extra flags.
// @b: address and size if the type of argument is a buffer.
// @o: object instance if the type of argument is an object.
//
// &qcomtee_arg.flags only accepts %QCOMTEE_ARG_FLAGS_UADDR for now, which
// states that &qcomtee_arg.b contains a userspace address in uaddr.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_arg {
    pub type: qcomtee_arg_type,
// 'b.uaddr' holds a __user address.

    pub flags: c_uint,
    pub b: qcomtee_buffer,
    pub o: *mut qcomtee_object,
}

// Context is busy (callback is in progress).

// Context needs to notify the current object.

// Context has shared state with QTEE.

//
// struct qcomtee_object_invoke_ctx - QTEE context for object invocation.
// @ctx: TEE context for this invocation.
// @flags: flags for the invocation context.
// @errno: error code for the invocation.
// @object: current object invoked in this callback context.
// @u: array of arguments for the current invocation (+1 for ending arg).
// @in_msg: inbound buffer shared with QTEE.
// @out_msg: outbound buffer shared with QTEE.
// @in_shm: TEE shm allocated for inbound buffer.
// @out_shm: TEE shm allocated for outbound buffer.
// @data: extra data attached to this context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_object_invoke_ctx {
    pub ctx: *mut tee_context,
    pub flags: c_ulong,
    pub errno: c_int,
    pub object: *mut qcomtee_object,
    pub 1]: qcomtee_arg u[QCOMTEE_ARGS_MAX +,
    pub in_msg: qcomtee_buffer,
    pub out_msg: qcomtee_buffer,
    pub in_shm: *mut tee_shm,
    pub out_shm: *mut tee_shm,
    pub data: *mut c_void,
}

//
// qcomtee_object_do_invoke() - Submit an invocation for an object.
// @oic: context to use for the current invocation.
// @object: object being invoked.
// @op: requested operation on the object.
// @u: array of arguments for the current invocation.
// @result: result returned from QTEE.
//
// The caller is responsible for keeping track of the refcount for each object,
// including @object. On return, the caller loses ownership of all input
// objects of type %QCOMTEE_OBJECT_TYPE_CB.
//
// @object can be of %QCOMTEE_OBJECT_TYPE_ROOT or %QCOMTEE_OBJECT_TYPE_TEE.
//
// Return: On success, returns 0; on failure, returns < 0.
//
// struct qcomtee_object_operations - Callback object operations.
// @release: release the object if QTEE is not using it.
// @dispatch: dispatch the operation requested by QTEE.
// @notify: report the status of any pending response submitted by @dispatch.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_object_operations {
    pub object): *mut *mut void (release)(struct qcomtee_object,
    pub args): *mut qcomtee_arg,
    pub err): *mut *mut qcomtee_object object, int,
}

//
// struct qcomtee_object - QTEE or kernel object.
// @name: object name.
// @refcount: reference counter.
// @object_type: object type as &enum qcomtee_object_type.
// @info: extra information for the object.
// @ops: callback operations for objects of type %QCOMTEE_OBJECT_TYPE_CB.
// @work: work for async operations on the object.
//
// @work is used for releasing objects of %QCOMTEE_OBJECT_TYPE_TEE type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_object {
    pub name: *const c_char,
    pub refcount: kref,
    pub object_type: qcomtee_object_type,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct object_info {
    pub qtee_id: c_ulong,
// TEE context for QTEE object async requests.
    pub qcomtee_async_ctx: *mut tee_context,
    pub info: },
    pub ops: *mut qcomtee_object_operations,
    pub work: work_struct,
}

// Static instances of qcomtee_object objects.

//
// qcomtee_object_user_init() - Initialize an object for the user.
// @object: object to initialize.
// @ot: type of object as &enum qcomtee_object_type.
// @ops: instance of callbacks.
// @fmt: name assigned to the object.
//
// Return: On success, returns 0; on failure, returns < 0.
//
// Object release is RCU protected.
extern "C" {
    pub fn qcomtee_object_get(object: *mut qcomtee_object) -> c_int;
}
extern "C" {
    pub fn qcomtee_object_put(object: *mut qcomtee_object);
}

// Next argument of type @type after index @i.
// Iterate over argument of given type.

