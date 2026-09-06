//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/atomic.h
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
// Atomic operations usable in machine independent code

//
// Relaxed variants of xchg, cmpxchg and some atomic operations.
//
// We support four variants:
//
// - Fully ordered: The default implementation, no suffix required.
// - Acquire: Provides ACQUIRE semantics, _acquire suffix.
// - Release: Provides RELEASE semantics, _release suffix.
// - Relaxed: No ordering guarantees, _relaxed suffix.
//
// For compound atomics performing both a load and a store, ACQUIRE
// semantics apply only to the load and RELEASE semantics only to the
// store portion of the operation. Note that a failed cmpxchg_acquire
// does -not- imply any memory ordering constraints.
//
// See Documentation/memory-barriers.txt for ACQUIRE/RELEASE definitions.
//

//
// The idea here is to build acquire/release variants by adding explicit
// barriers on top of the relaxed variant. In the case where the relaxed
// variant is already fully ordered, no additional barriers are needed.
//
// If an architecture overrides __atomic_acquire_fence() it will probably
// want to define smp_mb__after_spinlock().
//

