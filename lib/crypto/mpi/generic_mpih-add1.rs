//! Automatically rewritten from C to Rust
//! Source: lib/crypto/mpi/generic_mpih-add1.c
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
// mpihelp-add_1.c  -  MPI helper functions
// Copyright (C) 1994, 1996, 1997, 1998,
// 2000 Free Software Foundation, Inc.
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

    mpi_limb_t
    mpihelp_add_n(mpi_ptr_t res_ptr, mpi_ptr_t s1_ptr,
    mpi_ptr_t s2_ptr, mpi_size_t size)
    {
    mpi_limb_t x, y, cy;
    mpi_size_t j;
// The loop counter and index J goes from -SIZE to -1.  This way
    the loop becomes faster.  */
    j = -size;
// Offset the base pointers to compensate for the negative indices.
    s1_ptr -= j;
    s2_ptr -= j;
    res_ptr -= j;
    cy = 0;
    do {
    y = s2_ptr[j];
    x = s1_ptr[j];
    y += cy;	/* add previous carry to one addend */
    cy = y < cy;	/* get out carry from that addition */
    y += x;		/* add other addend */
    cy += y < x;	/* get out carry from that add, combine */
    res_ptr[j] = y;
    } while (++j);
    return cy;
    }
