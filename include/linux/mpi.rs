//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mpi.h
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
// mpi.h  -  Multi Precision Integers
// Copyright (C) 1994, 1996, 1998, 1999,
// 2000, 2001 Free Software Foundation, Inc.
//
// This file is part of GNUPG.
//
// Note: This code is heavily based on the GNU MP Library.
// Actually it's the same code with only minor changes in the
// way the data is stored; this is to support the abstraction
// of an optional secure memory allocation which may be used
// to avoid revealing of sensitive data due to paging etc.
// The GNU MP Library itself is published under the LGPL;
// however I decided to publish this code under the plain GPL.
//

pub type mpi_limb_t = unsigned long int;
pub type mpi_limb_signed_t = signed long int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gcry_mpi {
    pub /: *mut *mut int alloced; / array size (# of allocated limbs),
    pub /: *mut *mut int nlimbs; / number of valid limbs,
    pub /: *mut *mut int nbits; / the real number of valid bits (info only),
    pub /: *mut *mut int sign; / indicates a negative number,
    pub /: *mut *mut unsigned flags; / bit 0: array must be allocated in secure memory space,
// bit 1: not used
// bit 2: the limb is a pointer to some m_alloced data
    pub /: *mut *mut *mut mpi_limb_t d; / array with the limbs,
}

// -- mpiutil.c --
extern "C" {
    pub fn mpi_alloc(nlimbs: unsigned) -> MPI;
}
extern "C" {
    pub fn mpi_free(a: MPI);
}
extern "C" {
    pub fn mpi_resize(a: MPI, nlimbs: unsigned) -> c_int;
}
extern "C" {
    pub fn mpi_copy(a: MPI) -> MPI;
}
// -- mpicoder.c --
extern "C" {
    pub fn mpi_read_raw_data(xbuffer: *const c_void, nbytes: usize) -> MPI;
}
extern "C" {
    pub fn mpi_read_from_buffer(buffer: *const c_void, ret_nread: *mut unsigned) -> MPI;
}
extern "C" {
    pub fn mpi_read_raw_from_sgl(sgl: *mut scatterlist, len: c_uint) -> MPI;
}
// -- mpi-mod.c --
extern "C" {
    pub fn mpi_mod(rem: MPI, dividend: MPI, divisor: MPI) -> c_int;
}
// -- mpi-pow.c --
extern "C" {
    pub fn mpi_powm(res: MPI, base: MPI, exp: MPI, mod: MPI) -> c_int;
}
// -- mpi-cmp.c --
extern "C" {
    pub fn mpi_cmp_ui(u: MPI, v: c_ulong) -> c_int;
}
extern "C" {
    pub fn mpi_cmp(u: MPI, v: MPI) -> c_int;
}
// -- mpi-sub-ui.c --
extern "C" {
    pub fn mpi_sub_ui(w: MPI, u: MPI, vval: c_ulong) -> c_int;
}
// -- mpi-bit.c --
extern "C" {
    pub fn mpi_normalize(a: MPI);
}
extern "C" {
    pub fn mpi_get_nbits(a: MPI) -> unsigned;
}
extern "C" {
    pub fn mpi_test_bit(a: MPI, n: c_uint) -> c_int;
}
extern "C" {
    pub fn mpi_set_bit(a: MPI, n: c_uint) -> c_int;
}
extern "C" {
    pub fn mpi_rshift(x: MPI, a: MPI, n: c_uint) -> c_int;
}
// -- mpi-add.c --
extern "C" {
    pub fn mpi_add(w: MPI, u: MPI, v: MPI) -> c_int;
}
extern "C" {
    pub fn mpi_sub(w: MPI, u: MPI, v: MPI) -> c_int;
}
extern "C" {
    pub fn mpi_addm(w: MPI, u: MPI, v: MPI, m: MPI) -> c_int;
}
extern "C" {
    pub fn mpi_subm(w: MPI, u: MPI, v: MPI, m: MPI) -> c_int;
}
// -- mpi-mul.c --
extern "C" {
    pub fn mpi_mul(w: MPI, u: MPI, v: MPI) -> c_int;
}
extern "C" {
    pub fn mpi_mulm(w: MPI, u: MPI, v: MPI, m: MPI) -> c_int;
}
// -- mpi-div.c --
extern "C" {
    pub fn mpi_tdiv_r(rem: MPI, num: MPI, den: MPI) -> c_int;
}
extern "C" {
    pub fn mpi_fdiv_r(rem: MPI, dividend: MPI, divisor: MPI) -> c_int;
}
// inline functions
//
// mpi_get_size() - returns max size required to store the number
//
// @a:	A multi precision integer for which we want to allocate a buffer
//
// Return: size required to store the number
//
