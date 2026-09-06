//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/af_unix.h
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
// This file contains AppArmor af_unix fine grained mediation
//
// Copyright 2023 Canonical Ltd.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, version 2 of the
// License.
//

extern "C" {
    pub fn aa_unix_sock_perm(op: *const c_char, request: u32, sock: *mut socket) -> c_int;
}
extern "C" {
    pub fn aa_unix_listen_perm(sock: *mut socket, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn aa_unix_accept_perm(sock: *mut socket, newsock: *mut socket) -> c_int;
}
