//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/mldsa.h
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
//
// Support for verifying ML-DSA signatures
//
// Copyright 2025 Google LLC
//

// Identifier for an ML-DSA parameter set
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mldsa_alg {
    MLDSA44, /* ML-DSA-44 */
    MLDSA65, /* ML-DSA-65 */
    MLDSA87, /* ML-DSA-87 */
}

// Lengths of ML-DSA public keys and signatures in bytes
pub const MLDSA44_PUBLIC_KEY_SIZE: c_int = 1312;
pub const MLDSA65_PUBLIC_KEY_SIZE: c_int = 1952;
pub const MLDSA87_PUBLIC_KEY_SIZE: c_int = 2592;
pub const MLDSA44_SIGNATURE_SIZE: c_int = 2420;
pub const MLDSA65_SIGNATURE_SIZE: c_int = 3309;
pub const MLDSA87_SIGNATURE_SIZE: c_int = 4627;
//
// mldsa_verify() - Verify an ML-DSA signature
// @alg: The ML-DSA parameter set to use
// @sig: The signature
// @sig_len: Length of the signature in bytes.  Should match the
// MLDSA*_SIGNATURE_SIZE constant associated with @alg,
// otherwise -EBADMSG will be returned.
// @msg: The message
// @msg_len: Length of the message in bytes
// @pk: The public key
// @pk_len: Length of the public key in bytes.  Should match the
// MLDSA*_PUBLIC_KEY_SIZE constant associated with @alg,
// otherwise -EBADMSG will be returned.
//
// This verifies a signature using pure ML-DSA with the specified parameter set.
// The context string is assumed to be empty.  This corresponds to FIPS 204
// Algorithm 3 "ML-DSA.Verify" with the ctx parameter set to the empty string
// and the lengths of the signature and key given explicitly by the caller.
//
// Context: Might sleep
//
// Return:
// * 0 if the signature is valid
// * -EBADMSG if the signature and/or public key is malformed
// * -EKEYREJECTED if the signature is invalid but otherwise well-formed
// * -ENOMEM if out of memory so the validity of the signature is unknown
//

// Internal function, exposed only for unit testing
extern "C" {
    pub fn mldsa_use_hint(h: u8, r: i32, gamma2: i32) -> i32;
}

