//! Automatically rewritten from C Header to Rust Module
//! Source: include/keys/system_keyring.h
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
// System keyring containing trusted public keys.
//
// Copyright (C) 2013 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blacklist_hash_type {
// TBSCertificate hash
    BLACKLIST_HASH_X509_TBS = 1,
// Raw data hash
    BLACKLIST_HASH_BINARY = 2,
}

extern "C" {
    pub fn load_module_cert(keyring: *mut key) -> __init int;
}

extern "C" {
    pub fn add_to_secondary_keyring(source: *const c_char, data: *const c_void, len: usize) -> void __init;
}

extern "C" {
    pub fn set_machine_trusted_keys(keyring: *mut key) -> void __init;
}

extern "C" {
    pub fn is_binary_blacklisted(hash: *const u8, hash_len: usize) -> c_int;
}

extern "C" {
    pub fn add_key_to_revocation_list(data: *const c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn is_key_on_revocation_list(pkcs7: *mut pkcs7_message) -> c_int;
}

extern "C" {
    pub fn set_platform_trusted_keys(keyring: *mut key) -> void __init;
}

