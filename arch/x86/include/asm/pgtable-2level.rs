//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgtable-2level.h
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
// Certain architectures need to do special things when PTEs
// within a page table are directly modified.  Thus, the following
// hook is made available.
//
// ptep = pte;
// pmdp = pmd;
// xp = native_make_pte(0);

extern "C" {
    pub fn __pte(_arg: xchg(&xp->pte_low, _arg: 0)) -> return;
}

extern "C" {
    pub fn __pmd()xp: *mut xchg((pmdval_t, _arg: 0)) -> return;
}

extern "C" {
    pub fn __pud()xp: *mut xchg((pudval_t, _arg: 0)) -> return;
}

// Bit manipulation helper on pte/pgoff entry
//
// Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
// are !pte_none() && !pte_present().
//
// Format of swap PTEs:
//
// 3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// <----------------- offset ------------------> 0 E <- type --> 0
//
// E is the exclusive marker that is not stored in swap entries.
//
pub const SWP_TYPE_BITS: c_int = 5;

// We borrow bit 7 to store the exclusive marker in swap PTEs.

// No inverted PFNs on 2 level page tables
