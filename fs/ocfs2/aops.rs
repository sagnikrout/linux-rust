//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/aops.h
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
// Copyright (C) 2002, 2004, 2005 Oracle.  All rights reserved.
//

extern "C" {
    pub fn ocfs2_unlock_and_free_folios(folios: *mut folio, num_folios: c_int);
}
extern "C" {
    pub fn ocfs2_size_fits_inline_data(di_bh: *mut buffer_head, new_size: u64) -> c_int;
}
// all ocfs2_dio_end_io()'s fault

//
// Using a named enum representing lock types in terms of #N bit stored in
// iocb->private, which is going to be used for communication between
// ocfs2_dio_end_io() and ocfs2_file_write/read_iter().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_iocb_lock_bits {
    OCFS2_IOCB_RW_LOCK = 0,
    OCFS2_IOCB_RW_LOCK_LEVEL,
    OCFS2_IOCB_NUM_LOCKS
}

