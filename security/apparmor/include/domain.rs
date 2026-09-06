//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/domain.h
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
// This file contains AppArmor security domain transition function definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2010 Canonical Ltd.
//

pub const AA_CHANGE_NOFLAGS: c_int = 0;
pub const AA_CHANGE_TEST: c_int = 1;
pub const AA_CHANGE_CHILD: c_int = 2;
pub const AA_CHANGE_ONEXEC: c_int = 4;
pub const AA_CHANGE_STACK: c_int = 8;
extern "C" {
    pub fn apparmor_bprm_creds_for_exec(bprm: *mut linux_binprm) -> c_int;
}
extern "C" {
    pub fn aa_change_hat(hats[]: *const c_char, count: c_int, token: u64, flags: c_int) -> c_int;
}
extern "C" {
    pub fn aa_change_profile(fqname: *const c_char, flags: c_int) -> c_int;
}
