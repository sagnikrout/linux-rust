//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/mmio.h
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

// Generic IO read/write.  These perform native-endian accesses.

extern "C" {
    pub fn volatile(%0: "sb, (val): 0(%1)" : : "r", (addr): "r") -> asm;
}

extern "C" {
    pub fn volatile(%0: "sh, (val): 0(%1)" : : "r", (addr): "r") -> asm;
}

extern "C" {
    pub fn volatile(%0: "sw, (val): 0(%1)" : : "r", (addr): "r") -> asm;
}

extern "C" {
    pub fn volatile(%0: "sd, (val): 0(%1)" : : "r", (addr): "r") -> asm;
}

extern "C" {
    pub fn volatile(%0: "lb, (addr): 0(%1)" : "=r" (val) : "r") -> asm;
}

extern "C" {
    pub fn volatile(%0: "lh, (addr): 0(%1)" : "=r" (val) : "r") -> asm;
}

extern "C" {
    pub fn volatile(%0: "lw, (addr): 0(%1)" : "=r" (val) : "r") -> asm;
}

extern "C" {
    pub fn volatile(%0: "ld, (addr): 0(%1)" : "=r" (val) : "r") -> asm;
}

//
// Unordered I/O memory access primitives.  These are even more relaxed than
// the relaxed versions, as they don't even order accesses between successive
// operations to the I/O regions.
//

//
// Relaxed I/O memory access primitives. These follow the Device memory
// ordering rules but do not guarantee any ordering relative to Normal memory
// accesses.  These are defined to order the indicated access (either a read or
// write) with all other I/O memory accesses to the same peripheral. Since the
// platform specification defines that all I/O regions are strongly ordered on
// channel 0, no explicit fences are required to enforce this ordering.
//
// FIXME: These are now the same as asm-generic

//
// I/O memory access primitives.  Reads are ordered relative to any following
// Normal memory read and delay() loop.  Writes are ordered relative to any
// prior Normal memory write.  The memory barriers here are necessary as RISC-V
// doesn't define any ordering between the memory space and the I/O space.
//

