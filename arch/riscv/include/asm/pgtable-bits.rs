//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/pgtable-bits.h
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
pub const _PAGE_ACCESSED_OFFSET: c_int = 6;

// ext_svrsw60t59b: bit 59 for soft-dirty tracking

//
// Bit 3 is always zero for swap entry computation, so we
// can borrow it for swap page soft-dirty tracking.
//

pub const _PAGE_SOFT_DIRTY: c_int = 0;
pub const _PAGE_SWP_SOFT_DIRTY: c_int = 0;

// ext_svrsw60t59b: Bit(60) for userfaultfd tracking

//
// Bit 4 is not involved into swap entry computation, so we
// can borrow it for swap page userfaultfd tracking.
//

pub const _PAGE_UFFD: c_int = 0;
pub const _PAGE_SWP_UFFD: c_int = 0;

//
// _PAGE_PROT_NONE is set on not-present pages (and ignored by the hardware) to
// distinguish them from swapped out pages
//

// Used for swap PTEs only.

pub const _PAGE_PFN_SHIFT: c_int = 10;
//
// when all of R/W/X are zero, the PTE is a pointer to the next level
// of the page table; otherwise, it is a leaf PTE.
//

