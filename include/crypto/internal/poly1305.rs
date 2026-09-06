//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/poly1305.h
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
// Common values for the Poly1305 algorithm
//

//
// Poly1305 core functions.  These only accept whole blocks; the caller must
// handle any needed block buffering and padding.  'hibit' must be 1 for any
// full blocks, or 0 for the final block if it had to be padded.  If 'nonce' is
// non-NULL, then it's added at the end to compute the Poly1305 MAC.  Otherwise,
// only the ε-almost-∆-universal hash function (not the full MAC) is computed.
//
// state = (struct poly1305_state){};
