//! Automatically rewritten from C Header to Rust Module
//! Source: include/keys/asymmetric-subtype.h
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
// Asymmetric public-key cryptography key subtype
//
// See Documentation/crypto/asymmetric-keys.rst
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Keys of this type declare a subtype that indicates the handlers and
// capabilities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asymmetric_key_subtype {
    pub owner: *mut module,
    pub name: *const c_char,
    pub /: *mut *mut unsigned short name_len; / length of name,
// Describe a key of this subtype for /proc/keys
    pub m): *const *const *const void (describe)(struct key key, struct seq_file,
// Destroy a key of this subtype
    pub payload_auth): *mut *mut *mut void (destroy)(void payload_crypto, void,
    pub info): *mut kernel_pkey_query,
// Encrypt/decrypt/sign data
    pub out): *const *const void in, void,
// Verify the signature on a key of this subtype (optional)
    pub sig): *const public_key_signature,
}

//
// asymmetric_key_subtype - Get the subtype from an asymmetric key
// @key: The key of interest.
//
// Retrieves and returns the subtype pointer of the asymmetric key from the
// type-specific data attached to the key.
//
