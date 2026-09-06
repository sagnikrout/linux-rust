//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_debug.h
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
// Copyright (C) International Business Machines Corp., 2000-2002
// Portions Copyright (C) Christoph Hellwig, 2001-2002
//
// jfs_debug.h
//
// global debug message, data structure/macro definitions
// under control of CONFIG_JFS_DEBUG, CONFIG_JFS_STATISTICS;
//
// Create /proc/fs/jfs if procfs is enabled andeither
// CONFIG_JFS_DEBUG or CONFIG_JFS_STATISTICS is defined
//

// Macro flag: #define PROC_FS_JFS
extern "C" {
    pub fn jfs_proc_init();
}
extern "C" {
    pub fn jfs_proc_clean();
}

//
// assert with traditional printf/panic
//

//
// debug ON
// --------
//

// printk verbosity
pub const JFS_LOGLEVEL_ERR: c_int = 1;
pub const JFS_LOGLEVEL_WARN: c_int = 2;
pub const JFS_LOGLEVEL_DEBUG: c_int = 3;
pub const JFS_LOGLEVEL_INFO: c_int = 4;
extern "C" {
    pub fn jfs_txanchor_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
// information message: e.g., configuration, major event

// debug message: ad hoc

// warn message:

// error event message: e.g., i/o error

//
// debug OFF
// ---------
//

//
// statistics
// ----------
//

extern "C" {
    pub fn jfs_lmstats_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn jfs_txstats_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn jfs_mpstat_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn jfs_xtstat_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}

// Macro flag: #define	INCREMENT(x)
// Macro flag: #define	DECREMENT(x)
// Macro flag: #define	HIGHWATERMARK(x,y)

