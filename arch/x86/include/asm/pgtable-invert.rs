//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgtable-invert.h
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
pub const _ASM_PGTABLE_INVERT_H: c_int = 1;
//
// A clear pte value is special, and doesn't get inverted.
//
// Note that even users that only pass a pgprot_t (rather
// than a full pte) won't trigger the special zero case,
// because even PAGE_NONE has _PAGE_PROTNONE | _PAGE_ACCESSED
// set. So the all zero case really is limited to just the
// cleared page table entry case.
//
// Get a mask to xor with the page table entry to get the correct pfn.
//
// When a PTE transitions from NONE to !NONE or vice-versa
// invert the PFN part to stop speculation.
// pte_pfn undoes this when needed.
//

