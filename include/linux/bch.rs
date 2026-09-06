//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bch.h
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
// Generic binary BCH encoding/decoding library
//
// Copyright © 2011 Parrot S.A.
//
// Author: Ivan Djelic <ivan.djelic@parrot.com>
//
// Description:
//
// This library provides runtime configurable encoding/decoding of binary
// Bose-Chaudhuri-Hocquenghem (BCH) codes.
//

//
// struct bch_control - BCH control structure
// @m:          Galois field order
// @n:          maximum codeword size in bits (= 2^m-1)
// @t:          error correction capability in bits
// @ecc_bits:   ecc exact size in bits, i.e. generator polynomial degree (<=m*t)
// @ecc_bytes:  ecc max size (m*t bits) in bytes
// @a_pow_tab:  Galois field GF(2^m) exponentiation lookup table
// @a_log_tab:  Galois field GF(2^m) log lookup table
// @mod8_tab:   remainder generator polynomial lookup tables
// @ecc_buf:    ecc parity words buffer
// @ecc_buf2:   ecc parity words buffer
// @xi_tab:     GF(2^m) base for solving degree 2 polynomial roots
// @syn:        syndrome buffer
// @cache:      log-based polynomial representation buffer
// @elp:        error locator polynomial
// @poly_2t:    temporary polynomials of degree 2t
// @swap_bits:  swap bits within data and syndrome bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bch_control {
    pub m: c_uint,
    pub n: c_uint,
    pub t: c_uint,
    pub ecc_bits: c_uint,
    pub ecc_bytes: c_uint,
// private:
    pub a_pow_tab: *mut u16,
    pub a_log_tab: *mut u16,
    pub mod8_tab: *mut u32,
    pub ecc_buf: *mut u32,
    pub ecc_buf2: *mut u32,
    pub xi_tab: *mut c_uint,
    pub syn: *mut c_uint,
    pub cache: *mut c_int,
    pub elp: *mut gf_poly,
    pub poly_2t: [*mut gf_poly; 4],
    pub swap_bits: bool,
}

extern "C" {
    pub fn bch_free(bch: *mut bch_control);
}
