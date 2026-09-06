//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/net.h
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
// AppArmor security module
//
// This file contains AppArmor network mediation definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2017 Canonical Ltd.
//

pub const AA_MAY_ACCEPT: c_uint = 0x00100000;
pub const AA_MAY_BIND: c_uint = 0x00200000;
pub const AA_MAY_LISTEN: c_uint = 0x00400000;
pub const AA_MAY_SETOPT: c_uint = 0x01000000;
pub const AA_MAY_GETOPT: c_uint = 0x02000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_sk_ctx {
    pub label: *mut aa_label __rcu,
    pub peer: *mut aa_label __rcu,
    pub /: *mut *mut *mut aa_label __rcu peer_lastupdate; / ptr cmp only, no deref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_secmark {
    pub audit: u8,
    pub deny: u8,
    pub secid: u32,
    pub label: *mut c_char,
}

// passing in state returned by XXX_mediates_AF()
extern "C" {
    pub fn audit_net_cb(ab: *mut audit_buffer, va: *mut c_void);
}
extern "C" {
    pub fn aa_sk_perm(op: *const c_char, request: u32, sk: *const sock) -> c_int;
}
