//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/orc_lookup.h
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
// Copyright (C) 2017 Josh Poimboeuf <jpoimboe@redhat.com>
//
// This is a lookup table for speeding up access to the .orc_unwind table.
// Given an input address offset, the corresponding lookup table entry
// specifies a subset of the .orc_unwind table to search.
//
// Each block represents the end of the previous range and the start of the
// next range.  An extra block is added to give the last range an end.
//
// The block size should be a power of 2 to avoid a costly 'div' instruction.
//
// A block size of 256 was chosen because it roughly doubles unwinder
// performance while only adding ~5% to the ORC data footprint.
//
pub const LOOKUP_BLOCK_ORDER: c_int = 8;

