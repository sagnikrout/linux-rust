//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/smap.h
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
// Supervisor Mode Access Prevention support
//
// Copyright (C) 2012 Intel Corporation
// Author: H. Peter Anvin <hpa@linux.intel.com>
//

//
// The CLAC/STAC instructions toggle the enforcement of
// X86_FEATURE_SMAP along with X86_FEATURE_LASS.
//
// SMAP enforcement is based on the _PAGE_BIT_USER bit in the page
// tables. The kernel is not allowed to touch pages with that bit set
// unless the AC bit is set.
//
// Use stac()/clac() when accessing userspace (_PAGE_USER) mappings,
// regardless of location.
//
// Note: a barrier is implicit in alternative().
//
// LASS enforcement is based on bit 63 of the virtual address. The
// kernel is not allowed to touch memory in the lower half of the
// virtual address space.
//
// Use lass_stac()/lass_clac() to toggle the AC bit for kernel data
// accesses (!_PAGE_USER) that are blocked by LASS, but not by SMAP.
//
// Even with the AC bit set, LASS will continue to block instruction
// fetches from the user half of the address space. To allow those,
// clear CR4.LASS to disable the LASS mechanism entirely.
//
// Note: a barrier is implicit in alternative().
//
// These macros can be used in asm() statements

