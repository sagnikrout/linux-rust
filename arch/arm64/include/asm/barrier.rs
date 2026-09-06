//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/barrier.h
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
//

//
// Data Gathering Hint:
// This instruction prevents merging memory accesses with Normal-NC or
// Device-GRE attributes before the hint instruction with any memory accesses
// appearing after the hint instruction.
//

// \
// CPUs affected by Arm Erratum 2054223 or 2067961 needs	\
// another TSB to ensure the trace is flushed. The barriers	\
// don't have to be strictly back to back, as long as the	\
// CPU is in trace prohibited state.				\
// \
//
// Generate a mask for array_index__nospec() that is ~0UL when 0 <= idx < sz
// and 0 otherwise.
//

//
// Ensure that reads of the counter are treated the same as memory reads
// for the purposes of ordering by subsequent memory barriers.
//
// This insanity brought to you by speculative system register reads,
// out-of-order memory accesses, sequence locks and Thomas Gleixner.
//
// https://lore.kernel.org/r/alpine.DEB.2.21.1902081950260.1662@nanos.tec.linutronix.de
//

