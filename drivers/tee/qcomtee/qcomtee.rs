//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/qcomtee/qcomtee.h
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

// Flags relating to object reference.

//
// struct qcomtee - Main service struct.
// @teedev: client device.
// @pool: shared memory pool.
// @ctx: driver private context.
// @oic: context to use for the current driver invocation.
// @wq: workqueue for QTEE async operations.
// @xa_local_objects: array of objects exported to QTEE.
// @xa_last_id: next ID to allocate.
// @qtee_version: QTEE version.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee {
    pub teedev: *mut tee_device,
    pub pool: *mut tee_shm_pool,
    pub ctx: *mut tee_context,
    pub oic: qcomtee_object_invoke_ctx,
    pub wq: *mut workqueue_struct,
    pub xa_local_objects: xarray,
    pub xa_last_id: u32,
    pub qtee_version: u32,
}

extern "C" {
    pub fn qcomtee_fetch_async_reqs(oic: *mut qcomtee_object_invoke_ctx);
}
extern "C" {
    pub fn qcomtee_msg_buffers_free(oic: *mut qcomtee_object_invoke_ctx);
}
//
// qcomtee_object_do_invoke_internal() - Submit an invocation for an object.
// @oic: context to use for the current invocation.
// @object: object being invoked.
// @op: requested operation on the object.
// @u: array of arguments for the current invocation.
// @result: result returned from QTEE.
//
// The caller is responsible for keeping track of the refcount for each
// object, including @object. On return, the caller loses ownership of all
// input objects of type %QCOMTEE_OBJECT_TYPE_CB.
//
// Return: On success, returns 0; on failure, returns < 0.
//
// struct qcomtee_context_data - Clients' or supplicants' context.
// @qtee_objects_idr: QTEE objects in this context.
// @qtee_lock: mutex for @qtee_objects_idr.
// @reqs_idr: requests in this context that hold ID.
// @reqs_list: FIFO for requests in PROCESSING or QUEUED state.
// @reqs_lock: mutex for @reqs_idr, @reqs_list and request states.
// @req_c: completion used when the supplicant is waiting for requests.
// @released: state of this context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_context_data {
    pub qtee_objects_idr: idr,
// Synchronize access to @qtee_objects_idr.
    pub qtee_lock: mutex,
    pub reqs_idr: idr,
    pub reqs_list: list_head,
// Synchronize access to @reqs_idr, @reqs_list and updating requests states.
    pub reqs_lock: mutex,
    pub req_c: completion,
    pub released: bool,
}

// OBJECTS:
// (1) User Object API.
extern "C" {
    pub fn is_qcomtee_user_object(object: *mut qcomtee_object) -> c_int;
}
extern "C" {
    pub fn qcomtee_user_object_set_notify(object: *mut qcomtee_object, notify: bool);
}
extern "C" {
    pub fn qcomtee_requests_destroy(ctxdata: *mut qcomtee_context_data);
}
//
// struct qcomtee_user_object_request_data - Data for user object request.
// @id: ID assigned to the request.
// @object_id: Object ID being invoked by QTEE.
// @op: Requested operation on object.
// @np: Number of parameters in the request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcomtee_user_object_request_data {
    pub id: c_int,
    pub object_id: u64,
    pub op: u32,
    pub np: c_int,
}

// (2) Primordial Object.
// (3) Memory Object API.
// Is it a memory object using tee_shm?
extern "C" {
    pub fn is_qcomtee_memobj_object(object: *mut qcomtee_object) -> c_int;
}
//
// qcomtee_memobj_param_to_object() - OBJREF parameter to &struct qcomtee_object.
// @object: object returned.
// @param: TEE parameter.
// @ctx: context in which the conversion should happen.
//
// @param is an OBJREF with %QCOMTEE_OBJREF_FLAG_MEM flags.
//
// Return: On success return 0 or <0 on failure.
//
// Reverse what qcomtee_memobj_param_to_object() does.
//
// qcomtee_mem_object_map() - Map a memory object.
// @object: memory object.
// @map_object: created mapping object.
// @mem_paddr: physical address of the memory.
// @mem_size: size of the memory.
// @perms: QTEE access permissions.
//
// Return: On success return 0 or <0 on failure.
//
