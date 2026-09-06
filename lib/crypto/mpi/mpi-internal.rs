//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/mpi/mpi-internal.h
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
// mpi-internal.h  -  Internal to the Multi Precision Integers
// Copyright (C) 1994, 1996 Free Software Foundation, Inc.
// Copyright (C) 1998, 2000 Free Software Foundation, Inc.
//
// This file is part of GnuPG.
//
// Note: This code is heavily based on the GNU MP Library.
// Actually it's the same code with only minor changes in the
// way the data is stored; this is to support the abstraction
// of an optional secure memory allocation which may be used
// to avoid revealing of sensitive data due to paging etc.
// The GNU MP Library itself is published under the LGPL;
// however I decided to publish this code under the plain GPL.
//

// If KARATSUBA_THRESHOLD is not already defined, define it to a
// value which is good on most machines.
// tested 4, 16, 32 and 64, where 16 gave the best performance when
// checking a 768 and a 1024 bit ElGamal signature.
// (wk 22.12.97)

pub const KARATSUBA_THRESHOLD: c_int = 16;

// The code can't handle KARATSUBA_THRESHOLD smaller than 2.

pub const KARATSUBA_THRESHOLD: c_int = 2;

extern "C" {
    pub fn mpi_resize(_arg: a, _arg: b) -> return;
}
// Copy N limbs from S to D.

// Zero N limbs at D

// Divide the two-limb number in (NH,,NL) by D, with DI being the largest
// limb not larger than (2**(2*BITS_PER_MP_LIMB))/D - (2**BITS_PER_MP_LIMB).
// If this would yield overflow, DI should be the largest possible number
// (i.e., only ones).  For correct operation, the most significant bit of D
// has to be set.  Put the quotient in Q and the remainder in R.
//

// -- mpiutil.c --
extern "C" {
    pub fn mpi_alloc_limb_space(nlimbs: unsigned) -> mpi_ptr_t;
}
extern "C" {
    pub fn mpi_free_limb_space(a: mpi_ptr_t);
}
extern "C" {
    pub fn mpi_assign_limb_space(a: MPI, ap: mpi_ptr_t, nlimbs: unsigned);
}
// -- mpih-cmp.c --
extern "C" {
    pub fn mpihelp_cmp(op1_ptr: mpi_ptr_t, op2_ptr: mpi_ptr_t, size: mpi_size_t) -> c_int;
}
// -- mpih-mul.c --
#[repr(C)]
#[derive(Copy, Clone)]
pub struct karatsuba_ctx {
    pub next: *mut karatsuba_ctx,
    pub tspace: mpi_ptr_t,
    pub tspace_size: mpi_size_t,
    pub tp: mpi_ptr_t,
    pub tp_size: mpi_size_t,
}

extern "C" {
    pub fn mpihelp_release_karatsuba_ctx(ctx: *mut karatsuba_ctx);
}
extern "C" {
    pub fn mpih_sqr_n_basecase(prodp: mpi_ptr_t, up: mpi_ptr_t, size: mpi_size_t);
}
// -- generic_mpih-mul1.c --
// -- mpih-div.c --
// -- generic_mpih-[lr]shift.c --
// Define stuff for longlong.h.

pub type UWtype = mpi_limb_t;
pub type UHWtype = c_uint;

extern "C" {
    pub fn __attribute__(_arg: (mode(QI))) -> typedef unsigned int UQItype;
}
extern "C" {
    pub fn __attribute__(_arg: (mode(SI))) -> typedef int SItype;
}
extern "C" {
    pub fn __attribute__(_arg: (mode(SI))) -> typedef unsigned int USItype;
}
extern "C" {
    pub fn __attribute__(_arg: (mode(DI))) -> typedef int DItype;
}
extern "C" {
    pub fn __attribute__(_arg: (mode(DI))) -> typedef unsigned int UDItype;
}

pub type UQItype = c_uchar;
pub type SItype = c_long;
pub type USItype = c_ulong;

