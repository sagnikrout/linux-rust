//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/keyctl.h
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
// keyctl kernel bits
//
// Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_pkey_query {
    pub /: *mut *mut __u32 supported_ops; / Which ops are supported,
    pub /: *mut *mut __u32 key_size; / Size of the key in bits,
    pub /: *mut *mut __u16 max_data_size; / Maximum size of raw data to sign in bytes,
    pub /: *mut *mut __u16 max_sig_size; / Maximum size of signature in bytes,
    pub /: *mut *mut __u16 max_enc_size; / Maximum size of encrypted blob in bytes,
    pub /: *mut *mut __u16 max_dec_size; / Maximum size of decrypted blob in bytes,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kernel_pkey_operation {
    kernel_pkey_encrypt,
    kernel_pkey_decrypt,
    kernel_pkey_sign,
    kernel_pkey_verify,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_pkey_params {
    pub key: *mut key,
    pub /: *const *const *const char encoding; / Encoding (eg. "oaep" or "raw" for none),
    pub /: *const *const *const char hash_algo; / Digest algorithm used (eg. "sha1") or NULL if N/A,
    pub /: *mut *mut *mut char info; / Modified info string to be released later,
    pub /: *mut *mut __u32 in_len; / Input data size,
    pub /: *mut *mut __u32 out_len; / Output buffer size (enc/dec/sign),
    pub /: *mut *mut __u32 in2_len; / 2nd input data size (verify),
}
