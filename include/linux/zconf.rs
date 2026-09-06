//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/zconf.h
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


// zconf.h -- configuration of the zlib compression library
// Copyright (C) 1995-1998 Jean-loup Gailly.
// For conditions of distribution and use, see copyright notice in zlib.h
//
// @(#) $Id$
// The memory requirements for deflate are (in bytes):
//
// Maximum value for memLevel in deflateInit2

// Maximum value for windowBits in deflateInit2 and inflateInit2.
// WARNING: reducing MAX_WBITS makes minigzip unable to extract .gz files
// created by gzip. (Files created by minigzip can still be extracted by
// gzip.)
//

// default windowBits for decompression. MAX_WBITS is for compression only

// default memLevel

// Type declarations
