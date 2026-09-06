//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/poly1305.h
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

pub const POLY1305_BLOCK_SIZE: c_int = 16;
pub const POLY1305_KEY_SIZE: c_int = 32;
pub const POLY1305_DIGEST_SIZE: c_int = 16;
// The poly1305_key and poly1305_state types are mostly opaque and
// implementation-defined. Limbs might be in base 2^64 or base 2^26, or
// different yet. The union type provided keeps these 64-bit aligned for the
// case in which this is implemented using 64x64 multiplies.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_key {
    pub r: [u32; 5],
    pub r64: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_core_key {
    pub key: poly1305_key,
    pub precomputed_s: poly1305_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_state {
    pub h: [u32; 5],
    pub h64: [u64; 3],
}

// Combined state for block function.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_block_state {
// accumulator
    pub h: poly1305_state,
// key
    pub opaque_r: [poly1305_key; CONFIG_CRYPTO_LIB_POLY1305_RSIZE],
    pub core_r: poly1305_core_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_desc_ctx {
// partial buffer
    pub buf: [u8; POLY1305_BLOCK_SIZE],
// bytes used in partial buffer
    pub buflen: c_uint,
// finalize key
    pub s: [u32; 4],
    pub state: poly1305_block_state,
}

extern "C" {
    pub fn poly1305_final(desc: *mut poly1305_desc_ctx, digest: *mut u8);
}
