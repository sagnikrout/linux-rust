//! Automatically rewritten from C to Rust
//! Source: lib/crypto/mpi/mpih-cmp.c
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
// mpihelp-sub.c  -  MPI helper functions
// Copyright (C) 1994, 1996 Free Software Foundation, Inc.
// Copyright (C) 1998, 1999, 2000, 2001 Free Software Foundation, Inc.
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

//
// Compare OP1_PTR/OP1_SIZE with OP2_PTR/OP2_SIZE.
// There are no restrictions on the relative sizes of
// the two arguments.
// Return 1 if OP1 > OP2, 0 if they are equal, and -1 if OP1 < OP2.
//
#[no_mangle]
pub unsafe extern "C" fn mpihelp_cmp(op1_ptr: mpi_ptr_t, op2_ptr: mpi_ptr_t, size: mpi_size_t) -> c_int {
    int mpihelp_cmp(mpi_ptr_t op1_ptr, mpi_ptr_t op2_ptr, mpi_size_t size)
    {
    mpi_size_t i;
    mpi_limb_t op1_word, op2_word;
    for (i = size - 1; i >= 0; i--) {
    op1_word = op1_ptr[i];
    op2_word = op2_ptr[i];
    if (op1_word != op2_word)
    goto diff;
    }
    return 0;
    diff:
// This can *not* be simplified to
// op2_word - op2_word
// since that expression might give signed overflow.
    return (op1_word > op2_word) ? 1 : -1;
    }
