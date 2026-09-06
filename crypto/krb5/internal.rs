//! Automatically rewritten from C Header to Rust Module
//! Source: crypto/krb5/internal.h
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
// Kerberos5 crypto internals
//
// Copyright (C) 2025 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// Profile used for key derivation and encryption.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_crypto_profile {
// Pseudo-random function
    pub gfp): gfp_t,
// Checksum key derivation
    pub gfp): gfp_t,
// Encryption key derivation
    pub gfp): gfp_t,
// Integrity key derivation
    pub gfp): gfp_t,
// Derive the keys needed for an encryption AEAD object.
    pub gfp): gfp_t,
// Directly load the keys needed for an encryption AEAD object.
    pub gfp): gfp_t,
// Derive the key needed for a checksum hash object.
    pub gfp): gfp_t,
// Directly load the keys needed for a checksum hash object.
    pub gfp): gfp_t,
// Encrypt data in-place, inserting confounder and checksum.
    pub preconfounded): bool,
// Decrypt data in-place, removing confounder and checksum
    pub _len): *mut *mut size_t _offset, size_t,
// Generate a MIC on part of a packet, inserting the checksum
    pub data_len): size_t data_offset, size_t,
// Verify the MIC on a piece of data, removing the checksum
    pub _len): *mut *mut size_t _offset, size_t,
}

//
// Crypto size/alignment rounding convenience macros.
//

//
// Self-testing data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_prf_test {
    pub etype: u32,
    pub prf: *const *const *const *const char name, key, octet,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_key_test_one {
    pub use: u32,
    pub key: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_key_test {
    pub etype: u32,
    pub key: *const *const char name,,
    pub Ki: krb5_key_test_one Kc, Ke,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_enc_test {
    pub etype: u32,
    pub usage: u32,
    pub ct: *const *const *const *const *const *const *const char name, plain, conf, K0, Ke, Ki,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct krb5_mic_test {
    pub etype: u32,
    pub usage: u32,
    pub mic: *const *const *const *const *const char name, plain, K0, Kc,,
}

//
// krb5_api.c
//
// krb5_kdf.c
//
// rfc3961_simplified.c
//
// rfc3962_aes.c
//
// rfc6803_camellia.c
//
// rfc8009_aes2.c
//
// selftest.c
//

extern "C" {
    pub fn krb5_selftest() -> c_int;
}

//
// selftest_data.c
//
