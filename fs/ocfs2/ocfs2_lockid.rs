//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ocfs2/ocfs2_lockid.h
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
// ocfs2_lockid.h
//
// Defines OCFS2 lockid bits.
//
// Copyright (C) 2002, 2005 Oracle.  All rights reserved.
//
// lock ids are made up in the following manner:
// name[0]     --> type
// name[1-6]   --> 6 pad characters, reserved for now
// name[7-22]  --> block number, expressed in hex as 16 chars
// name[23-30] --> i_generation, expressed in hex 8 chars
// name[31]    --> '\0'
pub const OCFS2_LOCK_ID_MAX_LEN: c_int = 32;

pub const OCFS2_DENTRY_LOCK_INO_START: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_lock_type {
    OCFS2_LOCK_TYPE_META = 0,
    OCFS2_LOCK_TYPE_DATA,
    OCFS2_LOCK_TYPE_SUPER,
    OCFS2_LOCK_TYPE_RENAME,
    OCFS2_LOCK_TYPE_RW,
    OCFS2_LOCK_TYPE_DENTRY,
    OCFS2_LOCK_TYPE_OPEN,
    OCFS2_LOCK_TYPE_FLOCK,
    OCFS2_LOCK_TYPE_QINFO,
    OCFS2_LOCK_TYPE_NFS_SYNC,
    OCFS2_LOCK_TYPE_ORPHAN_SCAN,
    OCFS2_LOCK_TYPE_REFCOUNT,
    OCFS2_LOCK_TYPE_TRIM_FS,
    OCFS2_NUM_LOCK_TYPES
}

// Need to differentiate from [R]ename.. serializing writes is the
// important job it does, anyway.

