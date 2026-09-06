//! Automatically rewritten from C Header to Rust Module
//! Source: crypto/ripemd.h
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
// Common values for RIPEMD algorithms
//
pub const RMD160_DIGEST_SIZE: c_int = 20;
pub const RMD160_BLOCK_SIZE: c_int = 64;
// initial values
pub const RMD_H0: c_uint = 0x67452301UL;
pub const RMD_H1: c_uint = 0xefcdab89UL;
pub const RMD_H2: c_uint = 0x98badcfeUL;
pub const RMD_H3: c_uint = 0x10325476UL;
pub const RMD_H4: c_uint = 0xc3d2e1f0UL;
// constants
pub const RMD_K1: c_uint = 0x00000000UL;
pub const RMD_K2: c_uint = 0x5a827999UL;
pub const RMD_K3: c_uint = 0x6ed9eba1UL;
pub const RMD_K4: c_uint = 0x8f1bbcdcUL;
pub const RMD_K5: c_uint = 0xa953fd4eUL;
pub const RMD_K6: c_uint = 0x50a28be6UL;
pub const RMD_K7: c_uint = 0x5c4dd124UL;
pub const RMD_K8: c_uint = 0x6d703ef3UL;
pub const RMD_K9: c_uint = 0x7a6d76e9UL;
