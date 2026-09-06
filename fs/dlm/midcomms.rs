//! Automatically rewritten from C Header to Rust Module
//! Source: fs/dlm/midcomms.h
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
// Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
// Copyright (C) 2004-2005 Red Hat, Inc.  All rights reserved.
//
extern "C" {
    pub fn dlm_validate_incoming_buffer(nodeid: c_int, buf: *mut c_uchar, len: c_int) -> c_int;
}
extern "C" {
    pub fn dlm_process_incoming_buffer(nodeid: c_int, buf: *mut c_uchar, buflen: c_int) -> c_int;
}
extern "C" {
    pub fn dlm_midcomms_addr(nodeid: c_int, addr: *mut sockaddr_storage) -> c_int;
}
extern "C" {
    pub fn dlm_midcomms_version_wait();
}
extern "C" {
    pub fn dlm_midcomms_close(nodeid: c_int) -> c_int;
}
extern "C" {
    pub fn dlm_midcomms_start() -> c_int;
}
extern "C" {
    pub fn dlm_midcomms_stop();
}
extern "C" {
    pub fn dlm_midcomms_init();
}
extern "C" {
    pub fn dlm_midcomms_exit();
}
extern "C" {
    pub fn dlm_midcomms_shutdown();
}
extern "C" {
    pub fn dlm_midcomms_add_member(nodeid: c_int);
}
extern "C" {
    pub fn dlm_midcomms_remove_member(nodeid: c_int);
}
extern "C" {
    pub fn dlm_midcomms_unack_msg_resend(nodeid: c_int);
}
extern "C" {
    pub fn dlm_midcomms_flags(node: *mut midcomms_node) -> c_ulong;
}
extern "C" {
    pub fn dlm_midcomms_send_queue_cnt(node: *mut midcomms_node) -> c_int;
}
extern "C" {
    pub fn dlm_midcomms_version(node: *mut midcomms_node) -> u32;
}
