//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/sof/abi.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// SOF ABI versioning is based on Semantic Versioning where we have a given
// MAJOR.MINOR.PATCH version number. See https://semver.org
//
// Rules for incrementing or changing version :-
//
// 1) Increment MAJOR version if you make incompatible API changes. MINOR and
// PATCH should be reset to 0.
//
// 2) Increment MINOR version if you add backwards compatible features or
// changes. PATCH should be reset to 0.
//
// 3) Increment PATCH version if you add backwards compatible bug fixes.
//

// SOF ABI version major, minor and patch numbers
pub const SOF_ABI_MAJOR: c_int = 3;
pub const SOF_ABI_MINOR: c_int = 23;
pub const SOF_ABI_PATCH: c_int = 1;
// SOF ABI version number. Format within 32bit word is MMmmmppp
pub const SOF_ABI_MAJOR_SHIFT: c_int = 24;
pub const SOF_ABI_MAJOR_MASK: c_uint = 0xff;
pub const SOF_ABI_MINOR_SHIFT: c_int = 12;
pub const SOF_ABI_MINOR_MASK: c_uint = 0xfff;
pub const SOF_ABI_PATCH_SHIFT: c_int = 0;
pub const SOF_ABI_PATCH_MASK: c_uint = 0xfff;

// SOF ABI magic number "SOF\0".
pub const SOF_ABI_MAGIC: c_uint = 0x00464F53;
// SOF IPC4 ABI magic number "SOF4".
pub const SOF_IPC4_ABI_MAGIC: c_uint = 0x34464F53;
