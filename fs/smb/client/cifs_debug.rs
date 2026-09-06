//! Automatically rewritten from C Header to Rust Module
//! Source: fs/smb/client/cifs_debug.h
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
// Copyright (c) International Business Machines  Corp., 2000,2002
// Modified by Steve French (sfrench@us.ibm.com)
//

extern "C" {
    pub fn cifs_dump_mem(label: *mut c_char, data: *mut c_void, length: c_int);
}
extern "C" {
    pub fn cifs_dump_mids(server: *mut TCP_Server_Info);
}
extern "C" {
    pub fn dump_smb(buf: *mut c_void, smb_buf_length: c_int);
}
pub const CIFS_INFO: c_uint = 0x01;
pub const CIFS_RC: c_uint = 0x02;
pub const CIFS_TIMER: c_uint = 0x04;
pub const VFS: c_int = 1;
pub const FYI: c_int = 2;

pub const NOISY: c_int = 4;

pub const NOISY: c_int = 0;

pub const ONCE: c_int = 8;
//
// debug ON
// --------
//

//
// When adding tracepoints and debug messages we have various choices.
// Some considerations:
//
// Use cifs_dbg(VFS, ...) for things we always want logged, and the user to see
// cifs_info(...) slightly less important, admin can filter via loglevel > 6
// cifs_dbg(FYI, ...) minor debugging messages, off by default
// trace_smb3_*  ftrace functions are preferred for complex debug messages
// intended for developers or experienced admins, off by default
//
// Information level messages, minor events

// information message: e.g., configuration, major event

//
// debug OFF
// ---------
//

