//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fcntl.h
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


// SPDX-License-Identifier: GPL-2.0

// List of all valid flags for the open/openat flags argument:

// List of all valid flags for openat2(2)'s how->flags argument.

//
// Kernel-internal carrier for OPENAT2_REGULAR. The UAPI bit lives in the
// upper 32 bits of open_how::flags so open()/openat() cannot encode it.
// build_open_flags() translates it to this internal flag, which then
// propagates through op->open_flag and f->f_flags exactly like __FMODE_EXEC.
// do_dentry_open() strips it so userspace cannot observe it via
// fcntl(F_GETFL).
//
// Bit 30 is not claimed by any O_* flag on any architecture and stays clear
// of the sign bit of the int op->open_flag. fcntl_init() enforces that it
// never aliases an open-flag bit.
//

// List of all valid flags for the how->resolve argument:

// List of all open_how "versions".

