//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/arm64/include/asm/barrier.h
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
// From tools/perf/perf-sys.h, last modified in:
// f428ebd184c82a7914b2aa7e9f868918aaf7ea78 perf tools: Fix AAAAARGH64 memory barriers
//
// XXX: arch/arm64/include/asm/barrier.h in the kernel sources use dsb, is this
// a case like for arm32 where we do things differently in userspace?
//

//
// Kernel uses dmb variants on arm64 for smp_*() barriers. Pretty much the same
// implementation as above mb()/wmb()/rmb(), though for the latter kernel uses
// dsb. In any case, should above mb()/wmb()/rmb() change, make sure the below
// smp_*() don't.
//

// Only to shut up gcc ... */				\

// Only to shut up gcc ... */				\
