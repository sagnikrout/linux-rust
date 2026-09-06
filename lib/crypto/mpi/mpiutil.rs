//! Automatically rewritten from C to Rust
//! Source: lib/crypto/mpi/mpiutil.c
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


// mpiutil.ac  -  Utility functions for MPI
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

//
// Note:  It was a bad idea to use the number of limbs to allocate
// because on a alpha the limbs are large but we normally need
// integers of n bits - So we should change this to bits (or bytes).
//
// But mpi_alloc is used in a lot of places :-)
//
#[no_mangle]
pub unsafe extern "C" fn mpi_alloc(nlimbs: unsigned) -> MPI {
    MPI mpi_alloc(unsigned nlimbs)
    {
    MPI a;
    a = kmalloc_obj(*a);
    if (!a)
    return a;
    if (nlimbs) {
    a.d = mpi_alloc_limb_space(nlimbs);
    if (!a.d) {
    kfree(a);
    return core::ptr::null_mut();
    }
    } else {
    a.d = core::ptr::null_mut();
    }
    a.alloced = nlimbs;
    a.nlimbs = 0;
    a.sign = 0;
    a.flags = 0;
    a.nbits = 0;
    return a;
    }
    EXPORT_SYMBOL_GPL(mpi_alloc);
#[no_mangle]
pub unsafe extern "C" fn mpi_alloc_limb_space(nlimbs: unsigned) -> mpi_ptr_t {
    mpi_ptr_t mpi_alloc_limb_space(unsigned nlimbs)
    {
    let mut len: usize = nlimbs * sizeof(mpi_limb_t);
    if (!len)
    return core::ptr::null_mut();
    return kmalloc(len, GFP_KERNEL);
    }
#[no_mangle]
pub unsafe extern "C" fn mpi_free_limb_space(a: mpi_ptr_t) {
    void mpi_free_limb_space(mpi_ptr_t a)
    {
    if (!a)
    return;
    kfree_sensitive(a);
    }
#[no_mangle]
pub unsafe extern "C" fn mpi_assign_limb_space(a: MPI, ap: mpi_ptr_t, nlimbs: unsigned) {
    void mpi_assign_limb_space(MPI a, mpi_ptr_t ap, unsigned nlimbs)
    {
    mpi_free_limb_space(a.d);
    a.d = ap;
    a.alloced = nlimbs;
    }
//
// Resize the array of A to NLIMBS. the additional space is cleared
// (set to 0) [done by m_realloc()]
//
#[no_mangle]
pub unsafe extern "C" fn mpi_resize(a: MPI, nlimbs: unsigned) -> c_int {
    int mpi_resize(MPI a, unsigned nlimbs)
    {
    void *p;
    if (nlimbs <= a.alloced)
    return 0;	/* no need to do it */
    if (a.d) {
    p = kzalloc_objs(mpi_limb_t, nlimbs);
    if (!p)
    return -ENOMEM;
    memcpy(p, a.d, a.alloced * sizeof(mpi_limb_t));
    kfree_sensitive(a.d);
    a.d = p;
    } else {
    a.d = kzalloc_objs(mpi_limb_t, nlimbs);
    if (!a.d)
    return -ENOMEM;
    }
    a.alloced = nlimbs;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mpi_free(a: MPI) {
    void mpi_free(MPI a)
    {
    if (!a)
    return;
    if (a.flags & 4)
    kfree_sensitive(a.d);
    else
    mpi_free_limb_space(a.d);
    if (a.flags & ~7)
    pr_info("invalid flag value in mpi\n");
    kfree(a);
    }
    EXPORT_SYMBOL_GPL(mpi_free);
//
// Note: This copy function should not interpret the MPI
// but copy it transparently.
//
#[no_mangle]
pub unsafe extern "C" fn mpi_copy(a: MPI) -> MPI {
    MPI mpi_copy(MPI a)
    {
    int i;
    MPI b;
    if (a) {
    b = mpi_alloc(a.nlimbs);
    if (!b)
    return core::ptr::null_mut();
    b.nlimbs = a.nlimbs;
    b.sign = a.sign;
    b.flags = a.flags;
    b.flags &= ~(16|32); /* Reset the immutable and constant flags. */
    for (i = 0; i < b.nlimbs; i++)
    b.d[i] = a.d[i];
    } else
    b = core::ptr::null_mut();
    return b;
    }
    MODULE_DESCRIPTION("Multiprecision maths library");
    MODULE_LICENSE("GPL");
