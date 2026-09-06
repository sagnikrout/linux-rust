//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/google/vpd_decode.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// vpd_decode.h
//
// Google VPD decoding routines.
//
// Copyright 2017 Google Inc.
//

// Callback for vpd_decode_string to invoke.
//
// vpd_decode_string
//
// Given the encoded string, this function invokes callback with extracted
// (key, value). The *consumed will be plused the number of bytes consumed in
// this function.
//
// The input_buf points to the first byte of the input buffer.
//
// The *consumed starts from 0, which is actually the next byte to be decoded.
// It can be non-zero to be used in multiple calls.
//
// If one entry is successfully decoded, sends it to callback and returns the
// result.
//
