//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/page-states.h
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
//
// Copyright IBM Corp. 2017
// Author(s): Claudio Imbrenda <imbrenda@linux.vnet.ibm.com>
//

pub const ESSA_GET_STATE: c_int = 0;
pub const ESSA_SET_STABLE: c_int = 1;
pub const ESSA_SET_UNUSED: c_int = 2;
pub const ESSA_SET_VOLATILE: c_int = 3;
pub const ESSA_SET_POT_VOLATILE: c_int = 4;
pub const ESSA_SET_STABLE_RESIDENT: c_int = 5;
pub const ESSA_SET_STABLE_IF_RESIDENT: c_int = 6;
pub const ESSA_SET_STABLE_NODAT: c_int = 7;

