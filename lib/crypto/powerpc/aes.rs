//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/powerpc/aes.h
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
// Copyright (c) 2015 Markus Stockhausen <stockhausen@collogia.de>
// Copyright (C) 2015 International Business Machines Inc.
// Copyright 2026 Google LLC
//

extern "C" {
    pub fn ppc_encrypt_aes(out: *mut u8, in: *const u8, key_enc: *const u32, rounds: u32);
}
extern "C" {
    pub fn ppc_decrypt_aes(out: *mut u8, in: *const u8, key_dec: *const u32, rounds: u32);
}
// disable preemption and save users SPE registers if required
// reenable preemption

extern "C" {
    pub fn DEFINE_STATIC_KEY_FALSE(_arg: have_vec_crypto) -> static __ro_after_init;
}
//
// Convert a round key from VSX to generic format by reflecting all 16 bytes (if
// little endian) or reflecting the bytes in each 4-byte word (if big endian),
// and (if apply_inv_mix=true) applying InvMixColumn to each column.
//
// It would be nice if the VSX and generic key formats would be compatible.  But
// that's very difficult to do, with the assembly code having been borrowed from
// OpenSSL and also targeted to POWER8 rather than POWER9.
//
// Fortunately, this conversion should only be needed in extremely rare cases,
// possibly not at all in practice.  It's just included for full correctness.
//
// aes_p8_set_encrypt_key() should never fail here, since the
// key length was already validated.
//
// ... and likewise for aes_p8_set_decrypt_key().
// Mark the key as using the generic format.
//
// This handles (the hopefully extremely rare) case where a key
// was prepared using the VSX optimized format, then encryption
// is done in a context that cannot use VSX instructions.
//
// This handles (the hopefully extremely rare) case where a key
// was prepared using the VSX optimized format, then decryption
// is done in a context that cannot use VSX instructions.
//

