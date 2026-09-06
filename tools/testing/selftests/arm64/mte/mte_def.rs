//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/arm64/mte/mte_def.h
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
// Copyright (C) 2020 ARM Limited
//
// Below definitions may be found in kernel headers, However, they are
// redefined here to decouple the MTE selftests compilations from them.
//

pub const SEGV_MTEAERR: c_int = 8;

pub const SEGV_MTESERR: c_int = 9;

pub const PROT_MTE: c_uint = 0x20;

pub const PR_MTE_TCF_SHIFT: c_int = 1;

pub const PR_MTE_TAG_SHIFT: c_int = 3;

// MTE Hardware feature definitions below.
pub const MT_TAG_SHIFT: c_int = 56;
pub const MT_TAG_MASK: c_uint = 0xFUL;
pub const MT_FREE_TAG: c_uint = 0x0UL;
pub const MT_GRANULE_SIZE: c_int = 16;
pub const MT_TAG_COUNT: c_int = 16;
pub const MT_INCLUDE_TAG_MASK: c_uint = 0xFFFF;
pub const MT_EXCLUDE_TAG_MASK: c_uint = 0x0;
pub const MT_ATAG_SHIFT: c_int = 60;
pub const MT_ATAG_MASK: c_uint = 0xFUL;

pub const MT_PSTATE_TCO_SHIFT: c_int = 25;

pub const MT_PSTATE_TCO_EN: c_int = 1;
pub const MT_PSTATE_TCO_DIS: c_int = 0;

