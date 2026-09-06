//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/barrier.h
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
// Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
//

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
// For the smp_ barriers, ordering is for cacheable memory operations
// only. We have to use the sync instruction for smp_mb(), since lwsync
// doesn't order loads with respect to previous stores.  Lwsync can be
// used for smp_rmb() and smp_wmb().
//
// However, on CPUs that don't support lwsync, lwsync actually maps to a
// heavy-weight sync, so smp_wmb() can be a lighter-weight eieio.
//

// The sub-arch has lwsync

// clang defines this macro for a builtin, which will not work with runtime patching

//
// This is a barrier which prevents following instructions from being
// started until the value of the argument x is known.  For example, if
// x is a variable loaded from memory, this prevents following
// instructions from being executed until the load has been performed.
//

extern "C" {
    pub fn volatile(0: "twi, _arg: %0, "memory": 0; isync" : : "r" (x) :) -> asm;
}

//
// Prevent execution of subsequent instructions until preceding branches have
// been fully resolved and are no longer executing speculatively.
//

// This also acts as a compiler barrier due to the memory clobber.

// Macro flag: #define barrier_nospec_asm

//
// pmem_wmb() ensures that all stores for which the modification
// are written to persistent storage by preceding dcbfps/dcbstps
// instructions have updated persistent storage before any data
// access or data transfer caused by subsequent instructions is
// initiated.
//

