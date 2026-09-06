//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/public_key.h
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
// Asymmetric public-key algorithm definitions
//
// See Documentation/crypto/asymmetric-keys.rst
//
// Copyright (C) 2012 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Cryptographic data for the public-key subtype of the asymmetric key type.
//
// Note that this may include private part of the key as well as the public
// part.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct public_key {
    pub key: *mut c_void,
    pub keylen: u32,
    pub algo: OID,
    pub params: *mut c_void,
    pub paramlen: u32,
    pub key_is_private: bool,
    pub id_type: *const c_char,
    pub pkey_algo: *const c_char,
    pub /: *mut *mut unsigned long key_eflags; / key extension flags,

}

extern "C" {
    pub fn public_key_free(key: *mut public_key);
}
//
// Public key cryptography signature data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct public_key_signature {
    pub auth_ids: [*mut asymmetric_key_id; 3],
    pub /: *mut *mut *mut u8 s; / Signature,
    pub /: *mut *mut *mut u8 m; / Message data to pass to verifier,
    pub /: *mut *mut u32 s_size; / Number of bytes in signature,
    pub /: *mut *mut u32 m_size; / Number of bytes in ->m,
    pub /: *mut *mut bool m_free; / T if ->m needs freeing,
    pub /: *mut *mut bool algo_takes_data; / T if public key algo operates on data, not a hash,
    pub pkey_algo: *const c_char,
    pub hash_algo: *const c_char,
    pub encoding: *const c_char,
}

extern "C" {
    pub fn public_key_signature_free(sig: *mut public_key_signature);
}

