//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/zutil.h
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


// zutil.h -- internal interface and configuration of the compression library
// Copyright (C) 1995-1998 Jean-loup Gailly.
// For conditions of distribution and use, see copyright notice in zlib.h
//
// WARNING: this file should *not* be used by applications. It is
//
// @(#) $Id: zutil.h,v 1.1 2000/01/01 03:32:23 davem Exp $

pub type uch = c_uchar;
pub type ush = c_ushort;
pub type ulg = c_ulong;
// common constants
pub const STORED_BLOCK: c_int = 0;
pub const STATIC_TREES: c_int = 1;
pub const DYN_TREES: c_int = 2;
// The three kinds of block type
pub const MIN_MATCH: c_int = 3;
pub const MAX_MATCH: c_int = 258;
// The minimum and maximum match lengths
pub const PRESET_DICT: c_uint = 0x20 /* preset dictionary flag in zlib header */;
// target dependencies
// Common defaults

// functions
// checksum functions

pub const NMAX: c_int = 5552;
// NMAX is the largest n such that 255n(n+1)/2 + (n+1)(BASE-1) <= 2^32-1

// =========================================================================
//
