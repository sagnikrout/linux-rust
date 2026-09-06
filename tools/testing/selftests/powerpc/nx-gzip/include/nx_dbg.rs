//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/nx-gzip/include/nx_dbg.h
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
// Copyright 2020 IBM Corporation
//

// Use in case of an error

// Use in case of an warning

// Informational printouts

// Trace zlib wrapper code

// Trace statistics

// Trace zlib hardware implementation

// Trace zlib software implementation

//
// str_to_num - Convert string into number and copy with endings like
// KiB for kilobyte
// MiB for megabyte
// GiB for gigabyte
//
extern "C" {
    pub fn str_to_num(str: *mut c_char) -> u64;
}
extern "C" {
    pub fn nx_lib_debug(onoff: c_int);
}
