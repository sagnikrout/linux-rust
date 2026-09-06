//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/processor-flags.h
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
// CR3's layout varies depending on several things.
//
// If CR4.PCIDE is set (64-bit only), then CR3[11:0] is the address space ID.
// If PAE is enabled, then CR3[11:5] is part of the PDPT address
// (i.e. it's 32-byte aligned, not page-aligned) and CR3[4:0] is ignored.
// Otherwise (non-PAE, non-PCID), CR3[3] is PWT, CR3[4] is PCD, and
// CR3[2:0] and CR3[11:5] are ignored.
//
// In all cases, Linux puts zeros in the low ignored bits and in PWT and PCD.
//
// CR3[63] is always read as zero.  If CR4.PCIDE is set, then CR3[63] may be
// written as 1 to prevent the write to CR3 from flushing the TLB.
//
// On systems with SME, one bit (in a variable position!) is stolen to indicate
// that the top-level paging structure is encrypted.
//
// On systemms with LAM, bits 61 and 62 are used to indicate LAM mode.
//
// All of the remaining bits indicate the physical address of the top-level
// paging structure.
//
// CR3_ADDR_MASK is the mask used by read_cr3_pa().
//

// Mask off the address space ID and SME encryption bits.

pub const CR3_PCID_MASK: c_uint = 0xFFFull;

//
// CR3_ADDR_MASK needs at least bits 31:5 set on PAE systems, and we save
// a tiny bit of code size by setting all the bits.
//
pub const CR3_ADDR_MASK: c_uint = 0xFFFFFFFFull;

pub const CR3_NOFLUSH: c_int = 0;

