//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgtable-2level_types.h
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

pub type pteval_t = c_ulong;
pub type pmdval_t = c_ulong;
pub type pudval_t = c_ulong;
pub type p4dval_t = c_ulong;
pub type pgdval_t = c_ulong;
pub type pgprotval_t = c_ulong;

//
// Traditional i386 two-level paging structure:
//
pub const PGDIR_SHIFT: c_int = 22;
pub const PTRS_PER_PGD: c_int = 1024;
//
// The i386 is two-level, so we don't really have any
// PMD directory physically:
//
pub const PTRS_PER_PMD: c_int = 1;
pub const PTRS_PER_PTE: c_int = 1024;
// This covers all VMSPLIT_* and VMSPLIT_*_OPT variants

