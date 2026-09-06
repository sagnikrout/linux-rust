//! Automatically rewritten from C Header to Rust Module
//! Source: fs/dlm/config.h
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
// Copyright (C) 2004-2011 Red Hat, Inc.  All rights reserved.
//
pub const DLM_MAX_SOCKET_BUFSIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_config_node {
    pub nodeid: c_int,
    pub weight: c_int,
    pub gone: bool,
    pub new: c_int,
    pub comm_seq: u32,
    pub release_recover: c_uint,
}

pub const DLM_MAX_ADDR_COUNT: c_int = 8;
pub const DLM_PROTO_TCP: c_int = 0;
pub const DLM_PROTO_SCTP: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlm_config_info {
    pub ci_tcp_port: __be16,
    pub ci_buffer_size: c_uint,
    pub ci_rsbtbl_size: c_uint,
    pub ci_recover_timer: c_uint,
    pub ci_toss_secs: c_uint,
    pub ci_scan_secs: c_uint,
    pub ci_log_debug: c_uint,
    pub ci_log_info: c_uint,
    pub ci_protocol: c_uint,
    pub ci_mark: c_uint,
    pub ci_new_rsb_count: c_uint,
    pub ci_recover_callbacks: c_uint,
    pub ci_cluster_name: [c_char; DLM_LOCKSPACE_LEN],
}

extern "C" {
    pub fn dlm_config_init() -> c_int;
}
extern "C" {
    pub fn dlm_config_exit();
}
extern "C" {
    pub fn dlm_comm_seq(nodeid: c_int, seq: *mut u32, locked: bool) -> c_int;
}
extern "C" {
    pub fn dlm_our_nodeid() -> c_int;
}
extern "C" {
    pub fn dlm_our_addr(addr: *mut sockaddr_storage, num: c_int) -> c_int;
}
