//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgtable_32.h
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
// The Linux memory management assumes a three-level page table setup. On
// the i386, we use that, but "fold" the mid level into the top-level page
// table, so that we physically have the same two-level page table as the
// i386 mmu expects.
//
// This file contains the functions and defines necessary to modify and use
// the i386 page table tree.
//

extern "C" {
    pub fn paging_init();
}
extern "C" {
    pub fn sync_initial_page_table();
}

// Clear a kernel PTE and flush it from the TLB

//
// This is used to calculate the .brk reservation for initial pagetables.
// Enough space is reserved to allocate pagetables sufficient to cover all
// of LOWMEM_PAGES, which is an upper bound on the size of the direct map of
// lowmem.
//
// With PAE paging (PTRS_PER_PMD > 1), we allocate PTRS_PER_PGD == 4 pages for
// the PMD's in addition to the pages required for the last level pagetables.
//

//
// Number of possible pages in the lowmem region.
//
// We shift 2 by 31 instead of 1 by 32 to the left in order to avoid a
// gas warning about overflowing shift count when gas has been compiled
// with only a host target support using a 32-bit type for internal
// representation.
//

