//! Automatically rewritten from C Header to Rust Module
//! Source: lib/crypto/mips/sha256.h
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
// SHA-256 Secure Hash Algorithm.
//
// Adapted for OCTEON by Aaro Koskinen <aaro.koskinen@iki.fi>.
//
// Based on crypto/sha256_generic.c, which is:
//
// Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
// Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
// Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
// SHA224 Support Copyright 2007 Intel Corporation <jonathan.lynch@intel.com>
//

//
// We pass everything as 64-bit. OCTEON can handle misaligned data.
//
extern "C" {
    pub fn sha256_blocks_generic(_arg: state, _arg: data, _arg: nblocks) -> return;
}
