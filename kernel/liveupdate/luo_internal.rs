//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/liveupdate/luo_internal.h
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
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_ucmd {
    pub ubuffer: *mut void __user,
    pub user_size: u32,
    pub cmd: *mut c_void,
}

//
// Copy the minimum of what the user provided and what we actually
// have.
//
// Handles a deserialization failure: devices and memory is in unpredictable
// state.
//
// Continuing the boot process after a failure is dangerous because it could
// lead to leaks of private data.
//

//
// struct luo_file_set - A set of files that belong to the same sessions.
// @files_list: An ordered list of files associated with this session, it is
// ordered by preservation time.
// @block_set:  The set of serialization blocks.
// @count:      A counter tracking the number of files currently stored in the
// @files_list for this session.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_file_set {
    pub files_list: list_head,
    pub block_set: kho_block_set,
    pub count: u64,
}

//
// struct luo_session - Represents an active or incoming Live Update session.
// @name:       A unique name for this session, used for identification and
// retrieval.
// @list:       A list_head member used to link this session into a global list
// of either outgoing (to be preserved) or incoming (restored from
// previous kernel) sessions.
// @retrieved:  A boolean flag indicating whether this session has been
// retrieved by a consumer in the new kernel.
// @file_set:   A set of files that belong to this session.
// @mutex:      protects fields in the luo_session.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_session {
    pub name: [c_char; LIVEUPDATE_SESSION_NAME_LENGTH],
    pub list: list_head,
    pub retrieved: bool,
    pub file_set: luo_file_set,
    pub mutex: mutex,
}

extern "C" {
    pub fn luo_session_create(name: *const c_char, filep: *mut file) -> c_int;
}
extern "C" {
    pub fn luo_session_retrieve(name: *const c_char, filep: *mut file) -> c_int;
}
extern "C" {
    pub fn luo_session_setup_outgoing(sessions_pa: *mut u64) -> void __init;
}
extern "C" {
    pub fn luo_session_setup_incoming(sessions_pa: u64) -> int __init;
}
extern "C" {
    pub fn luo_session_serialize() -> c_int;
}
extern "C" {
    pub fn luo_session_deserialize() -> c_int;
}
extern "C" {
    pub fn luo_preserve_file(file_set: *mut luo_file_set, token: u64, fd: c_int) -> c_int;
}
extern "C" {
    pub fn luo_file_unpreserve_files(file_set: *mut luo_file_set);
}
extern "C" {
    pub fn luo_file_finish(file_set: *mut luo_file_set) -> c_int;
}
extern "C" {
    pub fn luo_file_set_init(file_set: *mut luo_file_set);
}
extern "C" {
    pub fn luo_file_set_destroy(file_set: *mut luo_file_set);
}
extern "C" {
    pub fn luo_flb_file_preserve(fh: *mut liveupdate_file_handler) -> c_int;
}
extern "C" {
    pub fn luo_flb_file_unpreserve(fh: *mut liveupdate_file_handler);
}
extern "C" {
    pub fn luo_flb_file_finish(fh: *mut liveupdate_file_handler);
}
extern "C" {
    pub fn luo_flb_unregister_all(fh: *mut liveupdate_file_handler);
}
extern "C" {
    pub fn luo_flb_setup_outgoing(flbs_pa: *mut u64) -> int __init;
}
extern "C" {
    pub fn luo_flb_setup_incoming(flbs_pa: u64) -> void __init;
}
extern "C" {
    pub fn luo_flb_serialize();
}

extern "C" {
    pub fn liveupdate_test_register(fh: *mut liveupdate_file_handler);
}

