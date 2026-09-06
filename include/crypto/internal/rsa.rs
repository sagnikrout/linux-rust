//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/internal/rsa.h
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
// RSA internal helpers
//
// Copyright (c) 2015, Intel Corporation
// Authors: Tadeusz Struk <tadeusz.struk@intel.com>
//

//
// rsa_key - RSA key structure
// @n           : RSA modulus raw byte stream
// @e           : RSA public exponent raw byte stream
// @d           : RSA private exponent raw byte stream
// @p           : RSA prime factor p of n raw byte stream
// @q           : RSA prime factor q of n raw byte stream
// @dp          : RSA exponent d mod (p - 1) raw byte stream
// @dq          : RSA exponent d mod (q - 1) raw byte stream
// @qinv        : RSA CRT coefficient q^(-1) mod p raw byte stream
// @n_sz        : length in bytes of RSA modulus n
// @e_sz        : length in bytes of RSA public exponent
// @d_sz        : length in bytes of RSA private exponent
// @p_sz        : length in bytes of p field
// @q_sz        : length in bytes of q field
// @dp_sz       : length in bytes of dp field
// @dq_sz       : length in bytes of dq field
// @qinv_sz     : length in bytes of qinv field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsa_key {
    pub n: *const u8,
    pub e: *const u8,
    pub d: *const u8,
    pub p: *const u8,
    pub q: *const u8,
    pub dp: *const u8,
    pub dq: *const u8,
    pub qinv: *const u8,
    pub n_sz: usize,
    pub e_sz: usize,
    pub d_sz: usize,
    pub p_sz: usize,
    pub q_sz: usize,
    pub dp_sz: usize,
    pub dq_sz: usize,
    pub qinv_sz: usize,
}

// key_size = 0;
// Find out new modulus size from rsa implementation
// key_size = err;
