//! Automatically rewritten from C Header to Rust Module
//! Source: security/safesetid/lsm.h
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
// SafeSetID Linux Security Module
//
// Author: Micah Morton <mortonm@chromium.org>
//
// Copyright (C) 2018 The Chromium OS Authors.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2, as
// published by the Free Software Foundation.
//

// Flag indicating whether initialization completed
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sid_policy_type {
    SIDPOL_DEFAULT, /* source ID is unaffected by policy */
    SIDPOL_CONSTRAINED, /* source ID is affected by policy */
    SIDPOL_ALLOWED /* target ID explicitly allowed */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum setid_type {
    UID,
    GID
}

//
// Hash table entry to store safesetid policy signifying that 'src_id'
// can set*id to 'dst_id'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct setid_rule {
    pub next: hlist_node,
    pub src_id: kid_t,
    pub dst_id: kid_t,
// Flag to signal if rule is for UID's or GID's
    pub type: setid_type,
}

// Extension of INVALID_UID/INVALID_GID for kid_t type

#[repr(C)]
#[derive(Copy, Clone)]
pub struct setid_ruleset {
    pub SETID_HASH_BITS): DECLARE_HASHTABLE(rules,,
    pub policy_str: *mut c_char,
    pub rcu: rcu_head,
// Flag to signal if ruleset is for UID's or GID's
    pub type: setid_type,
}

extern "C" {
    pub fn safesetid_init_securityfs() -> c_int;
}
