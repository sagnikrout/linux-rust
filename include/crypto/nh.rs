//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/nh.h
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
// NH hash function for Adiantum
//

// NH parameterization:
// Endianness: little
// Word size: 32 bits (works well on NEON, SSE2, AVX2)
// Stride: 2 words (optimal on ARM32 NEON; works okay on other CPUs too)
pub const NH_PAIR_STRIDE: c_int = 2;

// Num passes (Toeplitz iteration count): 4, to give ε = 2^{-128}
pub const NH_NUM_PASSES: c_int = 4;

// Max message size: 1024 bytes (32x compression factor)
pub const NH_NUM_STRIDES: c_int = 64;

//
// nh() - NH hash function for Adiantum
// @key: The key.  @message_len + 48 bytes of it are used.  This is NH_KEY_BYTES
// if @message_len has its maximum length of NH_MESSAGE_BYTES.
// @message: The message
// @message_len: The message length in bytes.  Must be a multiple of 16
// (NH_MESSAGE_UNIT) and at most 1024 (NH_MESSAGE_BYTES).
// @hash: (output) The resulting hash value
//
// Note: the pseudocode for NH in the Adiantum paper iterates over 1024-byte
// segments of the message, computes a 32-byte hash for each, and returns all
// the hashes concatenated together.  In contrast, this function just hashes one
// segment and returns one hash.  It's the caller's responsibility to call this
// function for each 1024-byte segment and collect all the hashes.
//
// Context: Any context.
//
