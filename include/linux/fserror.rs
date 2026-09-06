//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fserror.h
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
// Copyright (c) 2025 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//
extern "C" {
    pub fn fserror_mount(sb: *mut super_block);
}
extern "C" {
    pub fn fserror_unmount(sb: *mut super_block);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fserror_type {
// pagecache I/O failed
    FSERR_BUFFERED_READ,
    FSERR_BUFFERED_WRITE,

// direct I/O failed
    FSERR_DIRECTIO_READ,
    FSERR_DIRECTIO_WRITE,

// out of band media error reported
    FSERR_DATA_LOST,

// filesystem metadata
    FSERR_METADATA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fserror_event {
    pub work: work_struct,
    pub sb: *mut super_block,
    pub inode: *mut inode,
    pub pos: loff_t,
    pub len: u64,
    pub type: fserror_type,
// negative error number
    pub error: c_int,
}
