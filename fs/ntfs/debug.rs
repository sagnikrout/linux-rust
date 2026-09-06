//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/debug.h
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
// NTFS kernel debug support.
//
// Copyright (c) 2001-2004 Anton Altaparmakov
//

//
// ntfs_debug - write a debug level message to syslog
// @f:		a printf format string containing the message
// @...:	the variables to substitute into @f
//
// ntfs_debug() writes a DEBUG level message to the syslog but only if the
// driver was compiled with -DDEBUG. Otherwise, the call turns into a NOP.
//

extern "C" {
    pub fn ntfs_debug_dump_runlist(rl: *const runlist_element);
}

extern "C" {
    pub fn ntfs_handle_error(sb: *mut super_block);
}
