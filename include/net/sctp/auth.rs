//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/sctp/auth.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// SCTP kernel implementation
// (C) Copyright 2007 Hewlett-Packard Development Company, L.P.
//
// This file is part of the SCTP kernel implementation
//
// Please send any bug reports or fixes you make to the
// email address(es):
// lksctp developers <linux-sctp@vger.kernel.org>
//
// Written or modified by:
// Vlad Yasevich     <vladislav.yasevich@hp.com>
//

// Macro flag: #define __sctp_auth_h__

// Defines an HMAC algorithm supported by SCTP chunk authentication
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_hmac {
    pub /: *mut *mut *mut __u16 hmac_id; / one of SCTP_AUTH_HMAC_ID_,
    pub /: *mut *mut __u16 hmac_len; / length of the HMAC value in bytes,
}

// This is generic structure that containst authentication bytes used
// as keying material.  It's a what is referred to as byte-vector all
// over SCTP-AUTH
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_auth_bytes {
    pub refcnt: refcount_t,
    pub len: __u32,
    pub data: [__u8; ],
}

// Definition for a shared key, weather endpoint or association
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sctp_shared_key {
    pub key_list: list_head,
    pub key: *mut sctp_auth_bytes,
    pub refcnt: refcount_t,
    pub key_id: __u16,
    pub deactivated: __u8,
}

extern "C" {
    pub fn sctp_auth_key_put(key: *mut sctp_auth_bytes);
}
extern "C" {
    pub fn sctp_auth_destroy_keys(keys: *mut list_head);
}
extern "C" {
    pub fn sctp_auth_asoc_init_active_key(asoc: *mut sctp_association, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn sctp_auth_shkey_release(sh_key: *mut sctp_shared_key);
}
extern "C" {
    pub fn sctp_auth_shkey_hold(sh_key: *mut sctp_shared_key);
}
// API Helpers
extern "C" {
    pub fn sctp_auth_ep_add_chunkid(ep: *mut sctp_endpoint, chunk_id: __u8) -> c_int;
}
extern "C" {
    pub fn sctp_auth_init(ep: *mut sctp_endpoint, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn sctp_auth_free(ep: *mut sctp_endpoint);
}
