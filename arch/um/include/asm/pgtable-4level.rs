//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/asm/pgtable-4level.h
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
// Copyright 2003 PathScale Inc
// Derived from include/asm-i386/pgtable.h
//

// PGDIR_SHIFT determines what a fourth-level page table entry can map
pub const PGDIR_SHIFT: c_int = 39;

// PUD_SHIFT determines the size of the area a third-level page table can
// map
//
pub const PUD_SHIFT: c_int = 30;

// PMD_SHIFT determines the size of the area a second-level page table can
// map
//
pub const PMD_SHIFT: c_int = 21;

//
// entries per page directory level
//
pub const PTRS_PER_PTE: c_int = 512;
pub const PTRS_PER_PMD: c_int = 512;
pub const PTRS_PER_PUD: c_int = 512;
pub const PTRS_PER_PGD: c_int = 512;

extern "C" {
    pub fn phys_to_pfn(_arg: pte_val(pte)) -> return;
}
extern "C" {
    pub fn __pmd(pgprot_val(pgprot): (page_nr << PAGE_SHIFT) |) -> return;
}
