//! Automatically rewritten from C Header to Rust Module
//! Source: lib/xz/xz_stream.h
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


// SPDX-License-Identifier: 0BSD
//
// Definitions for handling the .xz file format
//
// Author: Lasse Collin <lasse.collin@tukaani.org>
//

//
// See the .xz file format specification at
// https://tukaani.org/xz/xz-file-format.txt
// to understand the container format.
//
pub const STREAM_HEADER_SIZE: c_int = 12;

pub const HEADER_MAGIC_SIZE: c_int = 6;

pub const FOOTER_MAGIC_SIZE: c_int = 2;
//
// Variable-length integer can hold a 63-bit unsigned integer or a special
// value indicating that the value is unknown.
//
// Experimental: vli_type can be defined to uint32_t to save a few bytes
// in code size (no effect on speed). Doing so limits the uncompressed and
// compressed size of the file to less than 256 MiB and may also weaken
// error detection slightly.
//
pub type vli_type = u64;

// Maximum encoded size of a VLI

// Integrity Check types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xz_check {
    XZ_CHECK_NONE = 0,
    XZ_CHECK_CRC32 = 1,
    XZ_CHECK_CRC64 = 4,
    XZ_CHECK_SHA256 = 10
}

// Maximum possible Check ID
pub const XZ_CHECK_MAX: c_int = 15;
