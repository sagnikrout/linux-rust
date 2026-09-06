//! Automatically rewritten from C Header to Rust Module
//! Source: fs/dlm/lowcomms.h
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
// Copyright (C) 2004-2009 Red Hat, Inc.  All rights reserved.
//

pub const CONN_HASH_SIZE: c_int = 32;
// This is deliberately very simple because most clusters have simple
// sequential nodeids, so we should be able to go straight to a connection
// struct in the array
//
// check if dlm is running
extern "C" {
    pub fn dlm_lowcomms_is_running() -> bool;
}
extern "C" {
    pub fn dlm_lowcomms_start() -> c_int;
}
extern "C" {
    pub fn dlm_lowcomms_shutdown();
}
extern "C" {
    pub fn dlm_lowcomms_shutdown_node(nodeid: c_int, force: bool);
}
extern "C" {
    pub fn dlm_lowcomms_stop();
}
extern "C" {
    pub fn dlm_lowcomms_init();
}
extern "C" {
    pub fn dlm_lowcomms_exit();
}
extern "C" {
    pub fn dlm_lowcomms_close(nodeid: c_int) -> c_int;
}
extern "C" {
    pub fn dlm_lowcomms_commit_msg(msg: *mut dlm_msg);
}
extern "C" {
    pub fn dlm_lowcomms_put_msg(msg: *mut dlm_msg);
}
extern "C" {
    pub fn dlm_lowcomms_resend_msg(msg: *mut dlm_msg) -> c_int;
}
extern "C" {
    pub fn dlm_lowcomms_connect_node(nodeid: c_int) -> c_int;
}
extern "C" {
    pub fn dlm_lowcomms_nodes_set_mark(nodeid: c_int, mark: c_uint) -> c_int;
}
extern "C" {
    pub fn dlm_lowcomms_addr(nodeid: c_int, addr: *mut sockaddr_storage) -> c_int;
}
extern "C" {
    pub fn dlm_midcomms_receive_done(nodeid: c_int);
}
