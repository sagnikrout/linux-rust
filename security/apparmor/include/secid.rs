//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/secid.h
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
// This file contains AppArmor security identifier (secid) definitions
//
// Copyright 2009-2018 Canonical Ltd.
//

// secid value that will not be allocated
pub const AA_SECID_INVALID: c_int = 0;
// secid value that matches any other secid
pub const AA_SECID_WILDCARD: c_int = 1;
// sysctl to enable displaying mode when converting secid to secctx
extern "C" {
    pub fn apparmor_secid_to_secctx(secid: u32, cp: *mut lsm_context) -> c_int;
}
extern "C" {
    pub fn apparmor_lsmprop_to_secctx(prop: *mut lsm_prop, cp: *mut lsm_context) -> c_int;
}
extern "C" {
    pub fn apparmor_secctx_to_secid(secdata: *const c_char, seclen: u32, secid: *mut u32) -> c_int;
}
extern "C" {
    pub fn apparmor_release_secctx(cp: *mut lsm_context);
}
extern "C" {
    pub fn aa_alloc_secid(label: *mut aa_label, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn aa_free_secid(secid: u32);
}
