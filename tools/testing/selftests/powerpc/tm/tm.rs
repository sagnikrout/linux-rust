//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/tm/tm.h
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
// Copyright 2015, Michael Ellerman, IBM Corp.
//

pub const TM_RETRIES: c_int = 100;

extern "C" {
    pub fn have_hwcap2(_arg: PPC_FEATURE2_HTM) -> return;
}

extern "C" {
    pub fn have_hwcap2(_arg: PPC_FEATURE2_HTM_NOSC) -> return;
}

//
// Transactional Memory was removed in ISA 3.1. A synthetic TM implementation
// is provided on P10 for threads running in P8/P9 compatibility  mode. The
// synthetic implementation immediately fails after tbegin. This failure sets
// Bit 7 (Failure Persistent) and Bit 15 (Implementation-specific).
//
// Per the ISA, the Failure Persistent bit may be incorrect. Try a few
// times in case we got an Implementation-specific failure on a non ISA
// v3.1 system. On these systems the Implementation-specific failure
// should not be persistent.
//
extern "C" {
    pub fn volatile("cr0": "tcheck 0" : "=r"(cr) : :) -> asm;
}
