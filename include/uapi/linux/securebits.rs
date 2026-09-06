//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/securebits.h
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
// Each securesetting is implemented using two bits. One bit specifies

pub const SECUREBITS_DEFAULT: c_uint = 0x00000000;
// When set UID 0 has no special privileges. When unset, we support
// of the executable file* if the effective uid of the new process is
pub const SECURE_NOROOT: c_int = 0;

// When set, setuid to/from uid 0 does not trigger capability-"fixup".
pub const SECURE_NO_SETUID_FIXUP: c_int = 2;

// When set, a process can retain its capabilities even after
pub const SECURE_KEEP_CAPS: c_int = 4;

// When set, a process cannot add new capabilities to its ambient set.
pub const SECURE_NO_CAP_AMBIENT_RAISE: c_int = 6;

// See Documentation/userspace-api/check_exec.rst
pub const SECURE_EXEC_RESTRICT_FILE: c_int = 8;

// See Documentation/userspace-api/check_exec.rst
pub const SECURE_EXEC_DENY_INTERACTIVE: c_int = 10;

