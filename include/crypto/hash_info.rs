//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/hash_info.h
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
// Hash Info: Hash algorithms information
//
// Copyright (c) 2013 Dmitry Kasatkin <d.kasatkin@samsung.com>
//

// not defined in include/crypto/
pub const RMD128_DIGEST_SIZE: c_int = 16;
pub const RMD160_DIGEST_SIZE: c_int = 20;
pub const RMD256_DIGEST_SIZE: c_int = 32;
pub const RMD320_DIGEST_SIZE: c_int = 40;
// not defined in include/crypto/
pub const WP512_DIGEST_SIZE: c_int = 64;
pub const WP384_DIGEST_SIZE: c_int = 48;
pub const WP256_DIGEST_SIZE: c_int = 32;
// not defined in include/crypto/
pub const TGR128_DIGEST_SIZE: c_int = 16;
pub const TGR160_DIGEST_SIZE: c_int = 20;
pub const TGR192_DIGEST_SIZE: c_int = 24;
// not defined in include/crypto/
pub const SM3256_DIGEST_SIZE: c_int = 32;
