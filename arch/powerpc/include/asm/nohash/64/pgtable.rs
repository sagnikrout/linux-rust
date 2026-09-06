//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/64/pgtable.h
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
// This file contains the functions and defines necessary to modify and use
// the ppc64 non-hashed page table.
//

//
// Size of EA range mapped by our pagetables.
//

//
// Define the address range of the kernel non-linear virtual area
//

//
// The vmalloc space starts at the beginning of that region, and
// occupies a quarter of it on Book3E
// (we keep a quarter for the virtual memmap)
//

//
// The third quarter of the kernel virtual space is used for IO mappings,
// it's itself carved into the PIO region (ISA and PHB IO space) and
// the ioremap space
//
// ISA_IO_BASE = KERN_IO_START, 64K reserved area
// PHB_IO_BASE = ISA_IO_BASE + 64K to ISA_IO_BASE + 2G, PHB IO spaces
// IOREMAP_BASE = ISA_IO_BASE + 2G to KERN_IO_START + KERN_IO_SIZE
//

pub const FULL_IO_SIZE: c_uint = 0x80000000ul;

//
// Defines the address of the vmemap area, in its own region on
// after the vmalloc space on Book3E
//

//
// Include the PTE bits definitions
//

pub const H_PAGE_4K_PFN: c_int = 0;
// pte_clear moved to later in this file

// pmdp = __pmd(val);
// pmdp = __pmd(0);
extern "C" {
    pub fn __pte(_arg: pmd_val(pmd)) -> return;
}

// pudp = __pud(val);
// pudp = __pud(0);

extern "C" {
    pub fn __pte(_arg: pud_val(pud)) -> return;
}
extern "C" {
    pub fn __pud(_arg: pte_val(pte)) -> return;
}

// p4dp = __p4d(val);

//
// Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
// are !pte_none() && !pte_present().
//
// Format of swap PTEs:
//
// 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// <-------------------------- offset ----------------------------
//
// 3 3 3 3 3 3 3 3 4 4 4 4 4 4 4 4 4 4 5 5 5 5 5 5 5 5 5 5 6 6 6 6
// 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3
// --------------> <----------- zero ------------> E < type -> 0 0
//
// E is the exclusive marker that is not stored in swap entries.
//

pub const SWP_TYPE_BITS: c_int = 5;

// We borrow MSB 56 (LSB 7) to store the exclusive marker in swap PTEs.
pub const _PAGE_SWP_EXCLUSIVE: c_uint = 0x80;
extern "C" {
    pub fn __patch_exception(exc: c_int, addr: c_ulong);
}

