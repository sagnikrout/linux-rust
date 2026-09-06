//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/rtbitmap.h
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
// Copyright (C) 2023 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
// We use an xfile to construct new bitmap blocks for the portion of the
// rtbitmap file that we're replacing.  Whereas the ondisk bitmap must be
// accessed through the buffer cache, the xfile bitmap supports direct
// word-level accesses.  Therefore, we create a small abstraction for linear
// access.
//
pub type xrep_wordoff_t = c_ulonglong;
pub type xrep_wordcnt_t = c_uint;
// Mask to round an rtx down to the nearest bitmap word.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_rtbitmap {
    pub sc: *mut xfs_scrub,
    pub rextents: u64,
    pub rbmblocks: u64,
    pub rextslog: c_uint,
    pub resblks: c_uint,
// The next free rt group block number that we expect to see.
    pub next_free_rgbno: xfs_rgblock_t,

// stuff for staging a new bitmap
    pub args: xfs_rtalloc_args,
    pub tempexch: xrep_tempexch,

// The next rtgroup block we expect to see during our rtrmapbt walk.
    pub next_rgbno: xfs_rgblock_t,
// rtgroup lock flags
    pub rtglock_flags: c_uint,
// rtword position of xfile as we write buffers to disk.
    pub prep_wordoff: xrep_wordoff_t,
// In-Memory rtbitmap for repair.
    pub words: [xfs_rtword_raw; ],
}

extern "C" {
    pub fn xrep_setup_rtbitmap(sc: *mut xfs_scrub, rtb: *mut xchk_rtbitmap) -> c_int;
}
//
// How big should the words[] buffer be?
//
// For repairs, we want a full fsblock worth of space so that we can memcpy a
// buffer full of 1s into the xfile bitmap.  The xfile bitmap doesn't have
// rtbitmap block headers, so we don't use blockwsize.  Scrub doesn't use the
// words buffer at all.
//

