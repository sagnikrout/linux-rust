//! Automatically rewritten from C Header to Rust Module
//! Source: include/keys/trusted-type.h
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
// Copyright (C) 2010 IBM Corporation
// Author: David Safford <safford@us.ibm.com>
//

pub const MIN_KEY_SIZE: c_int = 32;
pub const MAX_KEY_SIZE: c_int = 128;

pub const MAX_BLOB_SIZE: c_int = 1152;

pub const MAX_BLOB_SIZE: c_int = 512;

pub const MAX_PCRINFO_SIZE: c_int = 64;
pub const MAX_DIGEST_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trusted_key_payload {
    pub rcu: rcu_head,
    pub key_len: c_uint,
    pub blob_len: c_uint,
    pub migratable: c_uchar,
    pub old_format: c_uchar,
    pub 1]: unsigned char key[MAX_KEY_SIZE +,
    pub blob: [c_uchar; MAX_BLOB_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trusted_key_options {
    pub keytype: u16,
    pub keyhandle: u32,
    pub keyauth: [c_uchar; TPM_DIGEST_SIZE],
    pub blobauth_len: u32,
    pub blobauth: [c_uchar; TPM_DIGEST_SIZE],
    pub pcrinfo_len: u32,
    pub pcrinfo: [c_uchar; MAX_PCRINFO_SIZE],
    pub pcrlock: c_int,
    pub hash: u32,
    pub policydigest_len: u32,
    pub policydigest: [c_uchar; MAX_DIGEST_SIZE],
    pub policyhandle: u32,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trusted_key_ops {
//
// flag to indicate if trusted key implementation supports migration
// or not.
//
    pub migratable: c_uchar,
// Initialize key interface.
    pub (*init)(void): *mut c_int,
// Seal a key.
    pub datablob): *mut *mut *mut int (seal)(struct trusted_key_payload p, char,
// Unseal a key.
    pub datablob): *mut *mut *mut int (unseal)(struct trusted_key_payload p, char,
// Optional: Get a randomized key.
    pub key_len): *mut *mut *mut int (get_random)(unsigned char key, size_t,
// Exit key interface.
    pub (*exit)(void): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trusted_key_source {
    pub name: *mut c_char,
    pub ops: *mut trusted_key_ops,
}

