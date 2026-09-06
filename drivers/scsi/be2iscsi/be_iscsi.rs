//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/be2iscsi/be_iscsi.h
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
// Copyright 2017 Broadcom. All Rights Reserved.
// The term "Broadcom" refers to Broadcom Limited and/or its subsidiaries.
//
// Contact Information:
// linux-drivers@broadcom.com
//

extern "C" {
    pub fn beiscsi_iface_create_default(phba: *mut beiscsi_hba);
}
extern "C" {
    pub fn beiscsi_iface_destroy_default(phba: *mut beiscsi_hba);
}
extern "C" {
    pub fn beiscsi_attr_is_visible(param_type: c_int, param: c_int) -> umode_t;
}
extern "C" {
    pub fn beiscsi_session_destroy(cls_session: *mut iscsi_cls_session);
}
extern "C" {
    pub fn beiscsi_session_fail(cls_session: *mut iscsi_cls_session);
}
// cls_session, uint32_t cid);
extern "C" {
    pub fn beiscsi_get_macaddr(buf: *mut c_char, phba: *mut beiscsi_hba) -> c_int;
}
extern "C" {
    pub fn beiscsi_conn_start(cls_conn: *mut iscsi_cls_conn) -> c_int;
}
extern "C" {
    pub fn beiscsi_ep_poll(ep: *mut iscsi_endpoint, timeout_ms: c_int) -> c_int;
}
extern "C" {
    pub fn beiscsi_ep_disconnect(ep: *mut iscsi_endpoint);
}
