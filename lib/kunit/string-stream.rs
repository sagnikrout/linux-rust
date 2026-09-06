//! Automatically rewritten from C Header to Rust Module
//! Source: lib/kunit/string-stream.h
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
// C++ stream style string builder used in KUnit for building messages.
//
// Copyright (C) 2019, Google LLC.
// Author: Brendan Higgins <brendanhiggins@google.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct string_stream_fragment {
    pub node: list_head,
    pub fragment: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct string_stream {
    pub length: usize,
    pub fragments: list_head,
// length and fragments are protected by this lock
    pub lock: spinlock_t,
    pub gfp: gfp_t,
    pub append_newlines: bool,
}

extern "C" {
    pub fn kunit_free_string_stream(test: *mut kunit, stream: *mut string_stream);
}
extern "C" {
    pub fn free_string_stream(stream: *mut string_stream);
}
extern "C" {
    pub fn string_stream_clear(stream: *mut string_stream);
}
extern "C" {
    pub fn string_stream_is_empty(stream: *mut string_stream) -> bool;
}
extern "C" {
    pub fn string_stream_destroy(stream: *mut string_stream);
}
