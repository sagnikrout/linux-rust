//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/processor.h
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
// Misc low level processor primitives

//
// spin_begin is used before beginning a busy-wait loop, and must be paired
// with spin_end when the loop is exited. spin_cpu_relax must be called
// within the loop.
//
// The loop body should be as small and fast as possible, on the order of
// tens of instructions/cycles as a guide. It should and avoid calling
// cpu_relax, or any "spin" or sleep type of primitive including nested uses
// of these primitives. It should not lock or take any other resource.
// Violations of these guidelies will not cause a bug, but may cause sub
// optimal performance.
//
// These loops are optimized to be used where wait times are expected to be
// less than the cost of a context switch (and associated overhead).
//
// Detection of resource owner and decision to spin or sleep or guest-yield
// (e.g., spin lock holder vcpu preempted, or mutex owner not on CPU) can be
// tested within the loop body.
//

// Macro flag: #define spin_begin()

// Macro flag: #define spin_end()

//
// spin_until_cond can be used to wait for a condition to become true. It
// may be expected that the first iteration will true in the common case
// (no spinning), so that callers should not require a first "likely" test
// for the uncontended case before using this primitive.
//
// Usage and implementation guidelines are the same as for the spin_begin
// primitives, above.
//

