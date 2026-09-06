//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/memfd.h
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

// flags for memfd_create(2) (unsigned int)
pub const MFD_CLOEXEC: c_uint = 0x0001U;
pub const MFD_ALLOW_SEALING: c_uint = 0x0002U;
pub const MFD_HUGETLB: c_uint = 0x0004U;
// not executable and sealed to prevent changing to executable.
pub const MFD_NOEXEC_SEAL: c_uint = 0x0008U;
// executable
pub const MFD_EXEC: c_uint = 0x0010U;
//
// Huge page size encoding when MFD_HUGETLB is specified, and a huge page
// size other than the default is desired.  See hugetlb_encode.h.
// All known huge page size encodings are provided here.  It is the
// responsibility of the application to know which sizes are supported on
// the running system.  See mmap(2) man page for details.
//

