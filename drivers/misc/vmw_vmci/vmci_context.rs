//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/vmw_vmci/vmci_context.h
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
// VMware VMCI driver (vmciContext.h)
//
// Copyright (C) 2012 VMware, Inc. All rights reserved.
//

// Used to determine what checkpoint state to get and set.
// Host specific struct used for signalling
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_host {
    pub wait_queue: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_handle_list {
    pub node: list_head,
    pub handle: vmci_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_ctx {
    pub /: *mut *mut list_head list_item; / For global VMCI list.,
    pub cid: u32,
    pub kref: kref,
    pub /: *mut *mut list_head datagram_queue; / Head of per VM queue.,
    pub pending_datagrams: u32,
    pub /: *mut *mut size_t datagram_queue_size; / Size of datagram queue in bytes.,
//
// Version of the code that created
// this context; e.g., VMX.
//
    pub user_version: c_int,
    pub /: *mut *mut spinlock_t lock; / Locks callQueue and handle_arrays.,
//
// queue_pairs attached to.  The array of
// handles for queue pairs is accessed
// from the code for QP API, and there
// it is protected by the QP lock.  It
// is also accessed from the context
// clean up path, which does not
// require a lock.  VMCILock is not
// used to protect the QP array field.
//
    pub queue_pair_array: *mut vmci_handle_arr,
// Doorbells created by context.
    pub doorbell_array: *mut vmci_handle_arr,
// Doorbells pending for context.
    pub pending_doorbell_array: *mut vmci_handle_arr,
// Contexts current context is subscribing to.
    pub notifier_list: list_head,
    pub n_notifiers: c_uint,
    pub host_context: vmci_host,
    pub priv_flags: u32,
    pub cred: *const cred,
    pub /: *mut *mut *mut bool notify; / Notify flag pointer - hosted only.,
    pub /: *mut *mut *mut page notify_page; / Page backing the notify UVA.,
}

// VMCINotifyAddRemoveInfo: Used to add/remove remote context notifications.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_ctx_info {
    pub remote_cid: u32,
    pub result: c_int,
}

// VMCICptBufInfo: Used to set/get current context's checkpoint state.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_ctx_chkpt_buf_info {
    pub cpt_buf: u64,
    pub cpt_type: u32,
    pub buf_size: u32,
    pub result: i32,
    pub _pad: u32,
}

//
// VMCINotificationReceiveInfo: Used to receive pending notifications
// for doorbells and queue pairs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_ctx_notify_recv_info {
    pub db_handle_buf_uva: u64,
    pub db_handle_buf_size: u64,
    pub qp_handle_buf_uva: u64,
    pub qp_handle_buf_size: u64,
    pub result: i32,
    pub _pad: u32,
}

//
// Utilility function that checks whether two entities are allowed
// to interact. If one of them is restricted, the other one must
// be trusted.
//
extern "C" {
    pub fn vmci_ctx_destroy(context: *mut vmci_ctx);
}
extern "C" {
    pub fn vmci_ctx_supports_host_qp(context: *mut vmci_ctx) -> bool;
}
extern "C" {
    pub fn vmci_ctx_enqueue_datagram(cid: u32, dg: *mut vmci_datagram) -> c_int;
}
extern "C" {
    pub fn vmci_ctx_put(context: *mut vmci_ctx);
}
extern "C" {
    pub fn vmci_ctx_exists(cid: u32) -> bool;
}
extern "C" {
    pub fn vmci_ctx_add_notification(context_id: u32, remote_cid: u32) -> c_int;
}
extern "C" {
    pub fn vmci_ctx_remove_notification(context_id: u32, remote_cid: u32) -> c_int;
}
extern "C" {
    pub fn vmci_ctx_qp_create(context: *mut vmci_ctx, handle: vmci_handle) -> c_int;
}
extern "C" {
    pub fn vmci_ctx_qp_destroy(context: *mut vmci_ctx, handle: vmci_handle) -> c_int;
}
extern "C" {
    pub fn vmci_ctx_qp_exists(context: *mut vmci_ctx, handle: vmci_handle) -> bool;
}
extern "C" {
    pub fn vmci_ctx_check_signal_notify(context: *mut vmci_ctx);
}
extern "C" {
    pub fn vmci_ctx_unset_notify(context: *mut vmci_ctx);
}
extern "C" {
    pub fn vmci_ctx_dbell_create(context_id: u32, handle: vmci_handle) -> c_int;
}
extern "C" {
    pub fn vmci_ctx_dbell_destroy(context_id: u32, handle: vmci_handle) -> c_int;
}
// db_handle_array, struct vmci_handle_arr
// qp_handle_array);
// db_handle_array, struct vmci_handle_arr
// qp_handle_array, bool success);
