//! Automatically rewritten from C Header to Rust Module
//! Source: net/9p/protocol.h
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
// 9P Protocol Support Code
//
// Copyright (C) 2008 by Eric Van Hensbergen <ericvh@gmail.com>
//
// Base on code from Anthony Liguori <aliguori@us.ibm.com>
// Copyright (C) 2008 by IBM, Corp.
//
extern "C" {
    pub fn p9pdu_readf(pdu: *mut p9_fcall, proto_version: c_int, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn p9pdu_prepare(pdu: *mut p9_fcall, tag: i16, type: i8) -> c_int;
}
extern "C" {
    pub fn p9pdu_finalize(clnt: *mut p9_client, pdu: *mut p9_fcall) -> c_int;
}
extern "C" {
    pub fn p9pdu_reset(pdu: *mut p9_fcall);
}
extern "C" {
    pub fn pdu_read(pdu: *mut p9_fcall, data: *mut c_void, size: usize) -> usize;
}
