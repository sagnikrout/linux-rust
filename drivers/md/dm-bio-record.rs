//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-bio-record.h
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
// Copyright (C) 2004-2005 Red Hat, Inc. All rights reserved.
//
// This file is released under the GPL.
//

//
// There are lots of mutable fields in the bio struct that get
// changed by the lower levels of the block layer.  Some targets,
// such as multipath, may wish to resubmit a bio on error.  The
// functions in this file help the target record and restore the
// original bio state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_bio_details {
    pub bi_bdev: *mut block_device,
    pub __bi_remaining: c_int,
    pub bi_flags: c_ulong,
    pub bi_iter: bvec_iter,
    pub bi_end_io: *mut bio_end_io_t,

    pub bi_integrity: *mut bio_integrity_payload,

}

