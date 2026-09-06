//! Automatically rewritten from C Header to Rust Module
//! Source: net/handshake/handshake.h
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
// Generic netlink handshake service
//
// Author: Chuck Lever <chuck.lever@oracle.com>
//
// Copyright (c) 2023, Oracle and/or its affiliates.
//
// Per-net namespace context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct handshake_net {
    pub /: *mut *mut spinlock_t hn_lock; / protects next 3 fields,
    pub hn_pending: c_int,
    pub hn_pending_max: c_int,
    pub hn_requests: list_head,
    pub hn_flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hn_flags_bits {
    HANDSHAKE_F_NET_DRAINING,
}

// One handshake request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct handshake_req {
    pub hr_list: list_head,
    pub hr_rhash: rhash_head,
    pub hr_flags: c_ulong,
    pub hr_proto: *const handshake_proto,
    pub hr_file: *mut file,
    pub hr_sk: *mut sock,
    pub sk): *mut *mut void (hr_odestruct)(struct sock,
// Always the last field
    pub hr_priv: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hr_flags_bits {
    HANDSHAKE_F_REQ_COMPLETED,
    HANDSHAKE_F_REQ_SESSION,
}

// Invariants for all handshake requests for one transport layer
// security protocol
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct handshake_proto {
    pub hp_handler_class: c_int,
    pub hp_privsize: usize,
    pub hp_flags: c_ulong,
    pub fd): *mut *mut genl_info info, int,
    pub info): *mut genl_info,
    pub req): *mut *mut void (hp_destroy)(struct handshake_req,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hp_flags_bits {
    HANDSHAKE_F_PROTO_NOTIFY,
}

// alert.c
extern "C" {
    pub fn tls_alert_send(sock: *mut socket, level: u8, description: u8) -> c_int;
}
// netlink.c
// request.c
extern "C" {
    pub fn handshake_req_hash_init() -> c_int;
}
extern "C" {
    pub fn handshake_req_hash_destroy();
}
extern "C" {
    pub fn handshake_req_cancel(sk: *mut sock) -> bool;
}
