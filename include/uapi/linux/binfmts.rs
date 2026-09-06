//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/binfmts.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// These are the maximum length and maximum number of strings passed to the
// execve() system call.  MAX_ARG_STRLEN is essentially random but serves to
// prevent the kernel from being unduly impacted by misaddressed pointers.
// MAX_ARG_STRINGS is chosen to fit in a signed 32-bit integer.
//

pub const MAX_ARG_STRINGS: c_uint = 0x7FFFFFFF;
// sizeof(linux_binprm->buf)
pub const BINPRM_BUF_SIZE: c_int = 256;
// preserve argv0 for the interpreter
pub const AT_FLAGS_PRESERVE_ARGV0_BIT: c_int = 0;

//
// The interpreter runs transparently: the argument vector and the exe
// link belong to the binary passed in AT_EXECFD.
//
pub const AT_FLAGS_TRANSPARENT_INTERP_BIT: c_int = 1;

