//! Automatically rewritten from C Header to Rust Module
//! Source: include/keys/request_key_auth-type.h
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
// request_key authorisation token key type
//
// Copyright (C) 2005 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Authorisation record for request_key().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_key_auth {
    pub rcu: rcu_head,
    pub usage: refcount_t,
    pub target_key: *mut key,
    pub dest_keyring: *mut key,
    pub cred: *const cred,
    pub callout_info: *mut c_void,
    pub callout_len: usize,
    pub pid: pid_t,
    pub op: [c_char; 8],
    pub __randomize_layout: },
    pub key->payload.data[0]: return,
