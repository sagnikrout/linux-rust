//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/scrub/rtsummary.h
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
// Copyright (c) 2020-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xchk_rtsummary {

    pub tempexch: xrep_tempexch,

    pub args: xfs_rtalloc_args,
    pub rextents: u64,
    pub rbmblocks: u64,
    pub rsumblocks: xfs_filblks_t,
    pub rsumlevels: c_uint,
    pub resblks: c_uint,
// suminfo position of xfile as we write buffers to disk.
    pub prep_wordoff: xfs_rtsumoff_t,
// Memory buffer for the summary comparison.
    pub words: [xfs_suminfo_raw; ],
}

extern "C" {
    pub fn xrep_setup_rtsummary(sc: *mut xfs_scrub, rts: *mut xchk_rtsummary) -> c_int;
}

