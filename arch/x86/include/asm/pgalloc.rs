//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgalloc.h
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
// In case of Page Table Isolation active, we acquire two PGDs instead of one.
// Being order-1, it is both 8k in size and 8k-aligned.  That lets us just
// flip bit 12 in a pointer to swap between the two 4k halves.
//
// Allocate and free page tables.
//
extern "C" {
    pub fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t);
}
extern "C" {
    pub fn pte_alloc_one(: *mut mm_struct) -> pgtable_t;
}
extern "C" {
    pub fn ___pte_free_tlb(tlb: *mut mmu_gather, pte: *mut page);
}

extern "C" {
    pub fn ___pmd_free_tlb(tlb: *mut mmu_gather, pmd: *mut pmd_t);
}

extern "C" {
    pub fn pud_populate(mm: *mut mm_struct, pudp: *mut pud_t, pmd: *mut pmd_t);
}

extern "C" {
    pub fn ___pud_free_tlb(tlb: *mut mmu_gather, pud: *mut pud_t);
}

extern "C" {
    pub fn ___p4d_free_tlb(tlb: *mut mmu_gather, p4d: *mut p4d_t);
}

