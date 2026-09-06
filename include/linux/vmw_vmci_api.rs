//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vmw_vmci_api.h
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
// VMware VMCI Driver
//
// Copyright (C) 2012 VMware, Inc. All rights reserved.
//

pub const VMCI_KERNEL_API_VERSION_1: c_int = 1;
pub const VMCI_KERNEL_API_VERSION_2: c_int = 2;

extern "C" {
    pub fn void(is_host: *mut *mut vmci_vsock_cb) (bool) -> typedef;
}
extern "C" {
    pub fn vmci_datagram_destroy_handle(handle: vmci_handle) -> c_int;
}
extern "C" {
    pub fn vmci_datagram_send(msg: *mut vmci_datagram) -> c_int;
}
extern "C" {
    pub fn vmci_doorbell_destroy(handle: vmci_handle) -> c_int;
}
extern "C" {
    pub fn vmci_get_context_id() -> u32;
}
extern "C" {
    pub fn vmci_is_context_owner(context_id: u32, uid: kuid_t) -> bool;
}
extern "C" {
    pub fn vmci_register_vsock_callback(callback: vmci_vsock_cb) -> c_int;
}
extern "C" {
    pub fn vmci_event_unsubscribe(subid: u32) -> c_int;
}
extern "C" {
    pub fn vmci_context_get_priv_flags(context_id: u32) -> u32;
}
extern "C" {
    pub fn vmci_qpair_detach(qpair: *mut vmci_qp) -> c_int;
}
extern "C" {
    pub fn vmci_qpair_produce_free_space(qpair: *const vmci_qp) -> i64;
}
extern "C" {
    pub fn vmci_qpair_produce_buf_ready(qpair: *const vmci_qp) -> i64;
}
extern "C" {
    pub fn vmci_qpair_consume_free_space(qpair: *const vmci_qp) -> i64;
}
extern "C" {
    pub fn vmci_qpair_consume_buf_ready(qpair: *const vmci_qp) -> i64;
}
