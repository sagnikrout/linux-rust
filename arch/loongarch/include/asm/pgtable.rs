//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/pgtable.h
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
// Copyright (C) 1994, 95, 96, 97, 98, 99, 2000, 2003 Ralf Baechle
// Copyright (C) 1999, 2000, 2001 Silicon Graphics, Inc.
//

pub const VA_BITS: c_int = 32;

pub const KFENCE_AREA_SIZE: c_int = 0;

// Needed to limit get_free_mem_region()

//
// Empty pgd/p4d entries point to the invalid_pud_table.
//

//
// Empty pud entries point to the invalid_pmd_table.
//

//
// Empty pmd entries point to the invalid_pte_table.
//

extern "C" {
    pub fn set_pmd_at(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t, pmd: pmd_t);
}

//
// Initialize a new pgd / pud / pmd table with invalid pointers.
//
extern "C" {
    pub fn pgd_init(addr: *mut c_void);
}
extern "C" {
    pub fn pud_init(addr: *mut c_void);
}

extern "C" {
    pub fn pmd_init(addr: *mut c_void);
}

extern "C" {
    pub fn kernel_pte_init(addr: *mut c_void);
}

//
// Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
// are !pte_none() && !pte_present().
//
// Format of 32bit swap PTEs:
//
// 3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// <------------ offset -------------> E <- type -> <-- zeroes -->
//
// E is the exclusive marker that is not stored in swap entries.
// The zero'ed bits include _PAGE_PRESENT.
//
// Format of 64bit swap PTEs:
//
// 6 6 6 6 5 5 5 5 5 5 5 5 5 5 4 4 4 4 4 4 4 4 4 4 3 3 3 3 3 3 3 3
// 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2
// <--------------------------- offset ---------------------------
//
// 3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// --------------> E <--- type ---> <---------- zeroes ---------->
//
// E is the exclusive marker that is not stored in swap entries.
// The zero'ed bits include _PAGE_PRESENT and _PAGE_PROTNONE.
//

//
// The following only work if pte_present() is true.
// Undefined behaviour if not..
//

// We don't have hardware dirty/accessed bits, generic_pmdp_establish is fine.

extern "C" {
    pub fn pfn_to_page(_arg: pmd_pfn(pmd)) -> return;
}
extern "C" {
    pub fn pfn_to_page(PAGE_SHIFT: pmd_phys(pmd) >>) -> return;
}
//
// The generic version pmdp_huge_get_and_clear uses a version of pmd_clear() with a
// different prototype.
//

//
// We provide our own get_unmapped area to cope with the virtual aliasing
// constraints placed on us by the cache architecture.
//
// Macro flag: #define HAVE_ARCH_UNMAPPED_AREA
// Macro flag: #define HAVE_ARCH_UNMAPPED_AREA_TOPDOWN

