//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/64/pgtable-4k.h
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
// Entries per page directory level.  The PTE level must use a 64b record
// for each page table entry.  The PMD and PGD level use a 32b record for
// each entry by assuming that each entry is page aligned.
//
pub const PTE_INDEX_SIZE: c_int = 9;
pub const PMD_INDEX_SIZE: c_int = 7;
pub const PUD_INDEX_SIZE: c_int = 9;
pub const PGD_INDEX_SIZE: c_int = 9;

// PMD_SHIFT determines what a second-level page table entry can map

// PUD_SHIFT determines what a third-level page table entry can map

// PGDIR_SHIFT determines what a fourth-level page table entry can map

// Bits to mask out from a PMD to get to the PTE page
pub const PMD_MASKED_BITS: c_int = 0;
// Bits to mask out from a PUD to get to the PMD page
pub const PUD_MASKED_BITS: c_int = 0;
// Bits to mask out from a P4D to get to the PUD page
pub const P4D_MASKED_BITS: c_int = 0;
//
// 4-level page tables related bits
//

// p4dp = __p4d(0);
extern "C" {
    pub fn __pte(_arg: p4d_val(p4d)) -> return;
}
extern "C" {
    pub fn __p4d(_arg: pte_val(pte)) -> return;
}

//
// On all 4K setups, remap_4k_pfn() equates to remap_pfn_range()

