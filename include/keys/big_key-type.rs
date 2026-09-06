//! Automatically rewritten from C Header to Rust Module
//! Source: include/keys/big_key-type.h
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
// Big capacity key type.
//
// Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

extern "C" {
    pub fn big_key_preparse(prep: *mut key_preparsed_payload) -> c_int;
}
extern "C" {
    pub fn big_key_free_preparse(prep: *mut key_preparsed_payload);
}
extern "C" {
    pub fn big_key_revoke(key: *mut key);
}
extern "C" {
    pub fn big_key_destroy(key: *mut key);
}
extern "C" {
    pub fn big_key_describe(big_key: *const key, m: *mut seq_file);
}
extern "C" {
    pub fn big_key_read(key: *const key, buffer: *mut c_char, buflen: usize) -> c_long;
}
extern "C" {
    pub fn big_key_update(key: *mut key, prep: *mut key_preparsed_payload) -> c_int;
}
