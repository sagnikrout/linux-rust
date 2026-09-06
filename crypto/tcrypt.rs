//! Automatically rewritten from C Header to Rust Module
//! Source: crypto/tcrypt.h
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
// Quick & dirty crypto benchmarking module.
//
// This will only exist until we have a better benchmarking mechanism
// (e.g. a char device).
//
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2002 Jean-Francois Dive <jef@linuxbe.org>
// Copyright (c) 2007 Nokia Siemens Networks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cipher_speed_template {
    pub key: *const c_char,
    pub klen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_speed_template {
    pub key: *const c_char,
    pub klen: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_speed {
    pub /: *mut *mut unsigned int blen; / buffer length,
    pub /: *mut *mut unsigned int plen; / per-update length,
}

//
// DES test vectors.
//
pub const DES3_SPEED_VECTORS: c_int = 1;
//
// Cipher speed tests
//
// AEAD speed tests
//
// Digest speed tests
//
// End marker
