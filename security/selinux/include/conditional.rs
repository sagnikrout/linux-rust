//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/include/conditional.h
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
// Interface to booleans in the security server. This is exported
// for the selinuxfs.
//
// Author: Karl MacMillan <kmacmillan@tresys.com>
//
// Copyright (C) 2003 - 2004 Tresys Technology, LLC
//

extern "C" {
    pub fn security_set_bools(len: u32, values: *const c_int) -> c_int;
}
extern "C" {
    pub fn security_get_bool_value(index: u32) -> c_int;
}
