//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/barrier.h
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
// Based on arch/arm/include/asm/barrier.h
//
// Copyright (C) 2012 ARM Ltd.
// Copyright (C) 2013 Regents of the University of California
// Copyright (C) 2017 SiFive
//

// These barriers need to enforce ordering on both devices or memory.

// These barriers do not need to enforce ordering on devices, just memory.

//
// This is a very specific barrier: it's currently only used in two places in
// the kernel, both in the scheduler.  See include/linux/spinlock.h for the two
// orderings it guarantees, but the "critical section is RCsc" guarantee
// mandates a barrier on RISC-V.  The sequence looks like:
//
// lr.aq lock
// sc    lock <= LOCKED
// smp_mb__after_spinlock()
// // critical section
// lr    lock
// sc.rl lock <= UNLOCKED
//
// The AQ/RL pair provides a RCpc critical section, but there's not really any
// way we can take advantage of that here because the ordering is only enforced
// on that one lock.  Thus, we're just doing a full fence.
//
// Since we allow writeX to be called from preemptive regions we need at least
// an "o" in the predecessor set to ensure device writes are visible before the
// task is marked as available for scheduling on a new hart.  While I don't see
// any concrete reason we need a full IO fence, it seems safer to just upgrade
// this in order to avoid any IO crossing a scheduling boundary.  In both
// instances the scheduler pairs this with an mb(), so nothing is necessary on
// the new hart.
//

