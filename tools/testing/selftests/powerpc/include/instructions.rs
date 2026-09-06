//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/include/instructions.h
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


// SPDX-License-Identifier: GPL-2.0

// This defines the "copy" instruction from Power ISA 3.0 Book II, section 4.4.

// This defines the "paste" instruction from Power ISA 3.0 Book II, section 4.4.

// This defines the prefixed load/store instructions

pub const PPC_PREFIX_MLS: c_uint = 0x06000000;
pub const PPC_PREFIX_8LS: c_uint = 0x04000000;
pub const PPC_INST_LBZ: c_uint = 0x88000000;
pub const PPC_INST_LHZ: c_uint = 0xa0000000;
pub const PPC_INST_LHA: c_uint = 0xa8000000;
pub const PPC_INST_LWZ: c_uint = 0x80000000;
pub const PPC_INST_STB: c_uint = 0x98000000;
pub const PPC_INST_STH: c_uint = 0xb0000000;
pub const PPC_INST_STW: c_uint = 0x90000000;
pub const PPC_INST_STD: c_uint = 0xf8000000;
pub const PPC_INST_LFS: c_uint = 0xc0000000;
pub const PPC_INST_LFD: c_uint = 0xc8000000;
pub const PPC_INST_STFS: c_uint = 0xd0000000;
pub const PPC_INST_STFD: c_uint = 0xd8000000;

// Prefixed Integer Load/Store instructions

// Prefixed Floating-Point Load/Store Instructions

// Prefixed VSX Load/Store Instructions

