//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/chacha.h
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
// Common values and helper functions for the ChaCha and XChaCha stream ciphers.
//
// XChaCha extends ChaCha's nonce to 192 bits, while provably retaining ChaCha's
// security.  Here they share the same key size, tfm context, and setkey
// function; only their IV size and encrypt/decrypt function differ.
//
// The ChaCha paper specifies 20, 12, and 8-round variants.  In general, it is
// recommended to use the 20-round variant ChaCha20.  However, the other
// variants can be needed in some performance-sensitive scenarios.  The generic
// ChaCha code currently allows only the 20 and 12-round variants.
//

// 32-bit stream position, then 96-bit nonce (RFC7539 convention)
pub const CHACHA_IV_SIZE: c_int = 16;
pub const CHACHA_KEY_SIZE: c_int = 32;
pub const CHACHA_BLOCK_SIZE: c_int = 64;
pub const CHACHAPOLY_IV_SIZE: c_int = 12;
pub const CHACHA_KEY_WORDS: c_int = 8;
pub const CHACHA_STATE_WORDS: c_int = 16;
pub const HCHACHA_OUT_WORDS: c_int = 8;
// 192-bit nonce, then 64-bit stream position
pub const XCHACHA_IV_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chacha_state {
    pub x: [u32; CHACHA_STATE_WORDS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chacha_constants {
    CHACHA_CONSTANT_EXPA = 0x61707865U,
    CHACHA_CONSTANT_ND_3 = 0x3320646eU,
    CHACHA_CONSTANT_2_BY = 0x79622d32U,
    CHACHA_CONSTANT_TE_K = 0x6b206574U
}
