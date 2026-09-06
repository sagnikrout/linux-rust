//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kernel/image.h
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
// Linker script macros to generate Image header fields.
//
// Copyright (C) 2014 ARM Ltd.
//

//
// There aren't any ELF relocations we can use to endian-swap values known only
// at link time (e.g. the subtraction of two symbol addresses), so we must get
// the linker to endian-swap certain values before emitting them.
//
// Note that, in order for this to work when building the ELF64 PIE executable
// (for KASLR), these values should not be referenced via R_AARCH64_ABS64
// relocations, since these are fixed up at runtime rather than at build time
// when PIE is in effect. So we need to split them up in 32-bit high and low
// words.
//

pub const __HEAD_FLAG_PHYS_BASE: c_int = 1;

//
// These will output as part of the Image header, which should be little-endian
// regardless of the endianness of the kernel. While constant values could be
// endian swapped in head.S, all are done here for consistency.
//

