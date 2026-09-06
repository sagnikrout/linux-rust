//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/pte-e500.h
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

// PTE bit definitions for processors compliant to the Book3E
// architecture 2.06 or later. The position of the PTE bits
// matches the HW definition of the optional Embedded Page Table
// category.
//
// Architected bits
pub const _PAGE_PRESENT: c_uint = 0x000001 /* software: pte contains a translation */;
pub const _PAGE_SW1: c_uint = 0x000002;
pub const _PAGE_BAP_SR: c_uint = 0x000004;
pub const _PAGE_BAP_UR: c_uint = 0x000008;
pub const _PAGE_BAP_SW: c_uint = 0x000010;
pub const _PAGE_BAP_UW: c_uint = 0x000020;
pub const _PAGE_BAP_SX: c_uint = 0x000040;
pub const _PAGE_BAP_UX: c_uint = 0x000080;
pub const _PAGE_PSIZE_MSK: c_uint = 0x000f00;
pub const _PAGE_TSIZE_4K: c_uint = 0x000100;
pub const _PAGE_DIRTY: c_uint = 0x001000 /* C: page changed */;
pub const _PAGE_SW0: c_uint = 0x002000;
pub const _PAGE_U3: c_uint = 0x004000;
pub const _PAGE_U2: c_uint = 0x008000;
pub const _PAGE_U1: c_uint = 0x010000;
pub const _PAGE_U0: c_uint = 0x020000;
pub const _PAGE_ACCESSED: c_uint = 0x040000;
pub const _PAGE_ENDIAN: c_uint = 0x080000;
pub const _PAGE_GUARDED: c_uint = 0x100000;
pub const _PAGE_COHERENT: c_uint = 0x200000 /* M: enforce memory coherence */;
pub const _PAGE_NO_CACHE: c_uint = 0x400000 /* I: cache inhibit */;
pub const _PAGE_WRITETHRU: c_uint = 0x800000 /* W: cache write-through */;
pub const _PAGE_PSIZE_SHIFT: c_int = 7;
pub const _PAGE_PSIZE_SHIFT_OFFSET: c_int = 10;
// "Higher level" linux bit combinations

pub const _PAGE_NA: c_int = 0;

// On 32-bit, we never clear the top part of the PTE

pub const _PTE_NONE_MASK: c_uint = 0xffffffff00000000ULL;
pub const _PMD_PRESENT: c_int = 0;

pub const _PMD_USER: c_int = 0;

pub const _PTE_NONE_MASK: c_int = 0;

//
// We define 2 sets of base prot bits, one for basic pages (ie,
// cacheable kernel and user pages) and one for non cacheable
// pages. We always set _PAGE_COHERENT when SMP is enabled or
// the processor might need it for DMA coherency.
//

extern "C" {
    pub fn __pte(_PAGE_BAP_UX: (pte_val(pte) & ~_PAGE_BAP_SX) |) -> return;
}

extern "C" {
    pub fn pte_huge_size(_arg: __pte(pmd_val(pmd))) -> return;
}

extern "C" {
    pub fn pte_huge_size(_arg: __pte(pud_val(pud))) -> return;
}

