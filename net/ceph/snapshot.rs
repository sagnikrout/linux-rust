//! Automatically rewritten from C to Rust
//! Source: net/ceph/snapshot.c
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
// snapshot.c    Ceph snapshot context utility routines (part of libceph)
//
// Copyright (C) 2013 Inktank Storage, Inc.
//

//
// Ceph snapshot contexts are reference counted objects, and the
// returned structure holds a single reference.  Acquire additional
// references with ceph_get_snap_context(), and release them with
// ceph_put_snap_context().  When the reference count reaches zero
// the entire structure is freed.
//
// Create a new ceph snapshot context large enough to hold the
// indicated number of snapshot ids (which can be 0).  Caller has
// to fill in snapc->seq and snapc->snaps[0..snap_count-1].
//
// Returns a null pointer if an error occurs.
//
    struct ceph_snap_context *ceph_create_snap_context(u32 snap_count,
    gfp_t gfp_flags)
    {
    struct ceph_snap_context *snapc;
    size_t size;
    size = sizeof (struct ceph_snap_context);
    size += snap_count * sizeof (snapc.snaps[0]);
    snapc = kzalloc(size, gfp_flags);
    if (!snapc)
    return core::ptr::null_mut();
    refcount_set(&snapc.nref, 1);
    snapc.num_snaps = snap_count;
    return snapc;
    }
    EXPORT_SYMBOL(ceph_create_snap_context);
    struct ceph_snap_context *ceph_get_snap_context(struct ceph_snap_context *sc)
    {
    if (sc)
    refcount_inc(&sc.nref);
    return sc;
    }
    EXPORT_SYMBOL(ceph_get_snap_context);
#[no_mangle]
pub unsafe extern "C" fn ceph_put_snap_context(sc: *mut ceph_snap_context) {
    void ceph_put_snap_context(struct ceph_snap_context *sc)
    {
    if (!sc)
    return;
    if (refcount_dec_and_test(&sc.nref)) {
// printk(" deleting snap_context %p\n", sc);
    kfree(sc);
    }
    }
    EXPORT_SYMBOL(ceph_put_snap_context);
