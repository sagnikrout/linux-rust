//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/powerpc/include/asm/barrier.h
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
// Copied from the kernel sources:
//
// Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
//
// Memory barrier.
// The sync instruction guarantees that all memory accesses initiated
// by this processor have been performed (with respect to all other
// mechanisms that access memory).  The eieio instruction is a barrier
// providing an ordering (separately) for (a) cacheable stores and (b)
// loads and stores to non-cacheable memory (e.g. I/O devices).
//
// mb() prevents loads and stores being reordered across this point.
// rmb() prevents loads being reordered across this point.
// wmb() prevents stores being reordered across this point.
//
// *mb() variants without smp_ prefix must order all types of memory
// operations with one another. sync is the only instruction sufficient
// to do this.
//

