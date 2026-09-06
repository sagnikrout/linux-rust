//! Automatically rewritten from C Header to Rust Module
//! Source: include/keys/asymmetric-parser.h
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
// Asymmetric public-key cryptography data parser
//
// See Documentation/crypto/asymmetric-keys.rst
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// Key data parser.  Called during key instantiation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asymmetric_key_parser {
    pub link: list_head,
    pub owner: *mut module,
    pub name: *const c_char,
// Attempt to parse a key from the data blob passed to add_key() or
// keyctl_instantiate().  Should also generate a proposed description
// that the caller can optionally use for the key.
//
// Return EBADMSG if not recognised.
//
    pub prep): *mut *mut int (parse)(struct key_preparsed_payload,
}

extern "C" {
    pub fn register_asymmetric_key_parser(: *mut asymmetric_key_parser) -> c_int;
}
extern "C" {
    pub fn unregister_asymmetric_key_parser(: *mut asymmetric_key_parser);
}
