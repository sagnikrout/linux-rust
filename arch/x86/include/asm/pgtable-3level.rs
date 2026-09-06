//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgtable-3level.h
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
// Intel Physical Address Extension (PAE) Mode - three-level page
// tables on PPro+ CPUs.
//
// Copyright (C) 1999 Ingo Molnar <mingo@redhat.com>
//

//
// Rules for using set_pte: the pte being assigned *must* be
// either not present or in a state where the hardware will
// not attempt to update the pte.  In places where this is
// not possible, use pte_get_and_clear to obtain the old pte
// value and then use set_pte to update it.  -ben
//

//
// For PTEs and PDEs, we must clear the P-bit first when clearing a page table
// entry, so clear the bottom half first and enforce ordering with a compiler
// barrier.
//
// According to Intel App note "TLBs, Paging-Structure Caches,
// and Their Invalidation", April 2007, document 317080-001,
// section 8.1: in PAE mode we explicitly have to flush the
// TLB via cr3 if the top-level pgd is changed...
//
// Currently all places where pud_clear() is called either have
// flush_tlb_mm() followed or don't need TLB flush (x86_64 code or
// pud_clear_bad()), so we don't need TLB flush here.
//

extern "C" {
    pub fn pxx_xchg64(_arg: pte, _arg: ptep, _arg: 0ULL) -> return;
}
extern "C" {
    pub fn pxx_xchg64(_arg: pmd, _arg: pmdp, _arg: 0ULL) -> return;
}
extern "C" {
    pub fn pxx_xchg64(_arg: pud, _arg: pudp, _arg: 0ULL) -> return;
}

//
// If pmd has present bit cleared we can get away without expensive
// cmpxchg64: we can update pmdp half-by-half without racing with
// anybody.
//
// xchg acts as a barrier before setting of the high bits
extern "C" {
    pub fn pxx_xchg64(_arg: pmd, _arg: pmdp, _arg: pmd.pmd) -> return;
}

//
// Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
// are !pte_none() && !pte_present().
//
// Format of swap PTEs:
//
// 6 6 6 6 5 5 5 5 5 5 5 5 5 5 4 4 4 4 4 4 4 4 4 4 3 3 3 3 3 3 3 3
// 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2
// < type -> <---------------------- offset ----------------------
//
// 3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// --------------------------------------------> 0 E 0 0 0 0 0 0 0
//
// E is the exclusive marker that is not stored in swap entries.
//
pub const SWP_TYPE_BITS: c_int = 5;

// We always extract/encode the offset by shifting it all the way up, and then down again

//
// Normally, __swp_entry() converts from arch-independent swp_entry_t to
// arch-dependent swp_entry_t, and __swp_entry_to_pte() just stores the result
// to pte. But here we have 32bit swp_entry_t and 64bit pte, and need to use the
// whole 64 bits. Thus, we shift the "real" arch-dependent conversion to
// __swp_entry_to_pte() through the following helper macro based on 64bit
// __swp_entry().
//

//
// Analogically, __pte_to_swp_entry() doesn't just extract the arch-dependent
// swp_entry_t, but also has to convert it from 64bit to the 32bit
// intermediate representation, using the following macros based on 64bit
// __swp_type() and __swp_offset().
//

// We borrow bit 7 to store the exclusive marker in swap PTEs.

