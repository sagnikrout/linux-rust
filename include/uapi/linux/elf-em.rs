//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/elf-em.h
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
// These constants define the various ELF target machines
pub const EM_NONE: c_int = 0;
pub const EM_M32: c_int = 1;
pub const EM_SPARC: c_int = 2;
pub const EM_386: c_int = 3;
pub const EM_68K: c_int = 4;
pub const EM_88K: c_int = 5;

pub const EM_860: c_int = 7;

// Next two are historical and binaries and

pub const EM_FRV: c_uint = 0x5441	/* Fujitsu FR-V */;
//
// This is an interim value that we will use until the committee comes
// up with a final number.
//
pub const EM_ALPHA: c_uint = 0x9026;
// Bogus old m32r magic number, used by old tools.
pub const EM_CYGNUS_M32R: c_uint = 0x9041;
// This is the old interim value for S/390 architecture
pub const EM_S390_OLD: c_uint = 0xA390;
// Also Panasonic/MEI MN10300, AM33
pub const EM_CYGNUS_MN10300: c_uint = 0xbeef;
