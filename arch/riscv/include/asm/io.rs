//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/io.h
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
// {read,write}{b,w,l,q} based on arch/arm64/include/asm/io.h
// which was based on arch/arm/include/io.h
//
// Copyright (C) 1996-2000 Russell King
// Copyright (C) 2012 ARM Ltd.
// Copyright (C) 2014 Regents of the University of California
//

//
// MMIO access functions are separated out to break dependency cycles
// when using {read,write}* fns in low-level headers
//

//
// I/O port access constants.
//

//
// Emulation routines for the port-mapped IO space used by some PCI drivers.
// These are defined as being "fully synchronous", but also "not guaranteed to
// be fully ordered with respect to other memory and I/O operations".  We're
// going to be on the safe side here and just make them:
// - Fully ordered WRT each other, by bracketing them with two fences.  The
// outer set contains both I/O so inX is ordered with outX, while the inner just
// needs the type of the access (I for inX and O for outX).
// - Ordered in the same manner as readX/writeX WRT memory by subsuming their
// fences.
// - Ordered WRT timer reads, so udelay and friends don't get elided by the
// implementation.
// Note that there is no way to actually enforce that outX is a non-posted
// operation on RISC-V, but hopefully the timer ordering constraint is
// sufficient to ensure this works sanely on controllers that support I/O
// writes.
//

//
// Accesses from a single hart to a single I/O address must be ordered.  This
// allows us to use the raw read macros, but we still need to fence before and
// after the block to ensure ordering WRT other macros.  These are defined to
// perform host-endian accesses so we use __raw instead of __cpu.
//

// buf++ = x;					\

