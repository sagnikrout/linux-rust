//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/addrspace.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//
// Derived from MIPS:
// Copyright (C) 1996, 99 Ralf Baechle
// Copyright (C) 2000, 2002  Maciej W. Rozycki
// Copyright (C) 1990, 1999 by Silicon Graphics, Inc.
//

//
// This gives the physical RAM offset.
//

pub const DMW_PABITS: c_int = 29;

pub const DMW_PABITS: c_int = 48;

//
// Memory above this physical address will be considered highmem.
//

//
// This handles the memory map.
//

//
// 32/64-bit LoongArch address spaces
//

pub const UVRANGE: c_uint = 0x00000000;
pub const KPRANGE0: c_uint = 0x80000000;
pub const KPRANGE1: c_uint = 0xa0000000;
pub const KVRANGE: c_uint = 0xc0000000;

//
// Returns the physical address of a KPRANGEx / XKPRANGE address
//

//
// On LoongArch, I/O ports mappring is following:
//
// |         ....          |
// |-----------------------|
// | pci io ports(16K~32M) |
// |-----------------------|
// | isa io ports(0  ~16K) |
// PCI_IOBASE ->|-----------------------|
// |         ....          |
//

