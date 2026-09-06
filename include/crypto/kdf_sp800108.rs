//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/kdf_sp800108.h
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
// Copyright (C) 2021, Stephan Mueller <smueller@chronox.de>
//

//
// Counter KDF generate operation according to SP800-108 section 5.1
// as well as SP800-56A section 5.8.1 (Single-step KDF).
//
// @kmd Keyed message digest whose key was set with crypto_kdf108_setkey or
// unkeyed message digest
// @info optional context and application specific information - this may be
// NULL
// @info_vec number of optional context/application specific information entries
// @dst destination buffer that the caller already allocated
// @dlen length of the destination buffer - the KDF derives that amount of
// bytes.
//
// To comply with SP800-108, the caller must provide Label || 0x00 || Context
// in the info parameter.
//
// @return 0 on success, < 0 on error
//
// Counter KDF setkey operation
//
// @kmd Keyed message digest allocated by the caller. The key should not have
// been set.
// @key Seed key to be used to initialize the keyed message digest context.
// @keylen This length of the key buffer.
// @ikm The SP800-108 KDF does not support IKM - this parameter must be NULL
// @ikmlen This parameter must be 0.
//
// According to SP800-108 section 7.2, the seed key must be at least as large as
// the message digest size of the used keyed message digest. This limitation
// is enforced by the implementation.
//
// SP800-108 allows the use of either a HMAC or a hash primitive. When
// the caller intends to use a hash primitive, the call to
// crypto_kdf108_setkey is not required and the key derivation operation can
// immediately performed using crypto_kdf108_ctr_generate after allocating
// a handle.
//
// @return 0 on success, < 0 on error
//
