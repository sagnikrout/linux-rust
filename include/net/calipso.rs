//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/calipso.h
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
//
// CALIPSO - Common Architecture Label IPv6 Security Option
//
// This is an implementation of the CALIPSO protocol as specified in
// RFC 5570.
//
// Authors: Paul Moore <paul@paul-moore.com>
// Huw Davies <huw@codeweavers.com>
//
// (c) Copyright Hewlett-Packard Development Company, L.P., 2006
// (c) Copyright Huw Davies <huw@codeweavers.com>, 2015
//

// known doi values
pub const CALIPSO_DOI_UNKNOWN: c_uint = 0x00000000;
// doi mapping types
pub const CALIPSO_MAP_UNKNOWN: c_int = 0;
pub const CALIPSO_MAP_PASS: c_int = 2;
//
// CALIPSO DOI definitions
//
// DOI definition struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct calipso_doi {
    pub doi: u32,
    pub type: u32,
    pub refcount: refcount_t,
    pub list: list_head,
    pub rcu: rcu_head,
}

//
// Sysctl Variables
//

extern "C" {
    pub fn calipso_init() -> int __init;
}
extern "C" {
    pub fn calipso_exit();
}
extern "C" {
    pub fn calipso_validate(skb: *const sk_buff, option: *const c_uchar) -> bool;
}

