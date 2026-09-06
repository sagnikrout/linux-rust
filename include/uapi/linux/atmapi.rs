//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/atmapi.h
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
// atmapi.h - ATM API user space/kernel compatibility
// Written 1999,2000 by Werner Almesberger, EPFL ICA

// such alignment is not required on 32 bit sparcs, but we can't

//
// Opaque type for kernel pointers. Note that _ is never accessed. We need
// the struct in order hide the array, so that we can make simple assignments
// instead of being forced to use memcpy. It also improves error reporting for
// code that still assumes that we're passing unsigned longs.
//
// Convention: NULL pointers are passed as a field of all zeroes.
//
