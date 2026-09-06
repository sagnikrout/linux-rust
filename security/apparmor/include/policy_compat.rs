//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/policy_compat.h
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
// Code to provide backwards compatibility with older policy versions,
// by converting/mapping older policy formats into the newer internal
// formats.
//
// Copyright 2022 Canonical Ltd.
//

pub const K_ABI_MASK: c_uint = 0x3ff;
pub const FORCE_COMPLAIN_FLAG: c_uint = 0x800;

pub const v7: c_int = 7;

extern "C" {
    pub fn aa_compat_map_xmatch(policy: *mut aa_policydb) -> c_int;
}
extern "C" {
    pub fn aa_compat_map_policy(policy: *mut aa_policydb, version: u32) -> c_int;
}
extern "C" {
    pub fn aa_compat_map_file(policy: *mut aa_policydb) -> c_int;
}
