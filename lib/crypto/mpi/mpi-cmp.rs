//! Automatically rewritten from C to Rust
//! Source: lib/crypto/mpi/mpi-cmp.c
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


// mpi-cmp.c  -  MPI functions
// Copyright (C) 1998, 1999 Free Software Foundation, Inc.
//
// This file is part of GnuPG.
//
// GnuPG is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// GnuPG is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA
//

#[no_mangle]
pub unsafe extern "C" fn mpi_cmp_ui(u: MPI, v: c_ulong) -> c_int {
    int mpi_cmp_ui(MPI u, unsigned long v)
    {
    let mut limb: mpi_limb_t = v;
    mpi_normalize(u);
    if (u.nlimbs == 0) {
    if (v == 0)
    return 0;
    else
    return -1;
    }
    if (u.sign)
    return -1;
    if (u.nlimbs > 1)
    return 1;
    if (u.d[0] == limb)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(limb: u->d[0] >) -> else {
    else if (u.d[0] > limb)
    return 1;
    else
    return -1;
    }
    EXPORT_SYMBOL_GPL(mpi_cmp_ui);
#[no_mangle]
pub unsafe extern "C" fn mpi_cmp(u: MPI, v: MPI) -> c_int {
    int mpi_cmp(MPI u, MPI v)
    {
    mpi_size_t usize, vsize;
    int cmp;
    mpi_normalize(u);
    mpi_normalize(v);
    usize = u.nlimbs;
    vsize = v.nlimbs;
    if (!u.sign && v.sign)
    return 1;
    if (u.sign && !v.sign)
    return -1;
    if (usize != vsize && !u.sign && !v.sign)
    return usize - vsize;
    if (usize != vsize && u.sign && v.sign)
    return vsize - usize;
    if (!usize)
    return 0;
    cmp = mpihelp_cmp(u.d, v.d, usize);
    if (u.sign)
    return -cmp;
    return cmp;
    }
    EXPORT_SYMBOL_GPL(mpi_cmp);
