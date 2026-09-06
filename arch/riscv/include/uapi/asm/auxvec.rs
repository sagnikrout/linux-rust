//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/uapi/asm/auxvec.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Copyright (C) 2012 ARM Ltd.
// Copyright (C) 2015 Regents of the University of California
//
// vDSO location
pub const AT_SYSINFO_EHDR: c_int = 33;
//
// The set of entries below represent more extensive information
// about the caches, in the form of two entry per cache type,
// one entry containing the cache size in bytes, and the other
// containing the cache line size in bytes in the bottom 16 bits
// and the cache associativity in the next 16 bits.
//
// The associativity is such that if N is the 16-bit value, the
// cache is N way set associative. A value if 0xffff means fully
// associative, a value of 1 means directly mapped.
//
// For all these fields, a value of 0 means that the information
// is not known.
//
pub const AT_L1I_CACHESIZE: c_int = 40;
pub const AT_L1I_CACHEGEOMETRY: c_int = 41;
pub const AT_L1D_CACHESIZE: c_int = 42;
pub const AT_L1D_CACHEGEOMETRY: c_int = 43;
pub const AT_L2_CACHESIZE: c_int = 44;
pub const AT_L2_CACHEGEOMETRY: c_int = 45;
pub const AT_L3_CACHESIZE: c_int = 46;
pub const AT_L3_CACHEGEOMETRY: c_int = 47;
// entries in ARCH_DLINFO
pub const AT_VECTOR_SIZE_ARCH: c_int = 10;
pub const AT_MINSIGSTKSZ: c_int = 51;
