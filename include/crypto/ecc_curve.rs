//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/ecc_curve.h
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
// Copyright (c) 2021 HiSilicon

//
// struct ecc_point - elliptic curve point in affine coordinates
//
// @x:		X coordinate in vli form.
// @y:		Y coordinate in vli form.
// @ndigits:	Length of vlis in u64 qwords.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecc_point {
    pub x: *mut u64,
    pub y: *mut u64,
    pub ndigits: u8,
}

//
// struct ecc_curve - definition of elliptic curve
//
// @name:	Short name of the curve.
// @nbits:	The number of bits of a curve.
// @g:		Generator point of the curve.
// @p:		Prime number, if Barrett's reduction is used for this curve
// pre-calculated value 'mu' is appended to the @p after ndigits.
// Use of Barrett's reduction is heuristically determined in
// vli_mmod_fast().
// @n:		Order of the curve group.
// @a:		Curve parameter a.
// @b:		Curve parameter b.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecc_curve {
    pub name: *mut c_char,
    pub nbits: u32,
    pub g: ecc_point,
    pub p: *mut u64,
    pub n: *mut u64,
    pub a: *mut u64,
    pub b: *mut u64,
}

//
// ecc_get_curve() - get elliptic curve;
// @curve_id:           Curves IDs:
// defined in 'include/crypto/ecdh.h';
//
// Returns curve if get curve succssful, NULL otherwise
//
// ecc_get_curve25519() - get curve25519 curve;
//
// Returns curve25519
//
