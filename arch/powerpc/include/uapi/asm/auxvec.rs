//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/auxvec.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// We need to put in some extra aux table entries to tell glibc what
// the cache block size is, so it can use the dcbz instruction safely.
//
pub const AT_DCACHEBSIZE: c_int = 19;
pub const AT_ICACHEBSIZE: c_int = 20;
pub const AT_UCACHEBSIZE: c_int = 21;
// A special ignored type value for PPC, for glibc compatibility.
pub const AT_IGNOREPPC: c_int = 22;
// The vDSO location. We have to use the same value as x86 for glibc's
// sake :-)
//
pub const AT_SYSINFO_EHDR: c_int = 33;
//
// AT_*CACHEBSIZE above represent the cache *block* size which is
// the size that is affected by the cache management instructions.
//
// It doesn't nececssarily matches the cache *line* size which is
// more of a performance tuning hint. Additionally the latter can
// be different for the different cache levels.
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

