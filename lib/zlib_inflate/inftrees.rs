//! Automatically rewritten from C Header to Rust Module
//! Source: lib/zlib_inflate/inftrees.h
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


// inftrees.h -- header to use inftrees.c
// Copyright (C) 1995-2005 Mark Adler
// For conditions of distribution and use, see copyright notice in zlib.h
//
// WARNING: this file should *not* be used by applications. It is
//
// Structure for decoding tables.  Each entry provides either the
// op values as set by inflate_table():
//
// Maximum size of dynamic tree.  The maximum found in a long but non-
pub const ENOUGH: c_int = 2048;
pub const MAXD: c_int = 592;
// Type of code to build for inftable()
