//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/pgtable-32.h
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
// Copyright (C) 2012 Regents of the University of California
//

// Size of region mapped by a page global directory
pub const PGDIR_SHIFT: c_int = 22;

pub const MAX_POSSIBLE_PHYSMEM_BITS: c_int = 34;
//
// rv32 PTE format:
// | XLEN-1  10 | 9             8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0
// PFN      reserved for SW   D   A   G   U   X   W   R   V
//

pub const _PAGE_NOCACHE: c_int = 0;
pub const _PAGE_IO: c_int = 0;
pub const _PAGE_MTMASK: c_int = 0;
// Set of bits to preserve across pte_modify()

