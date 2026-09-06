//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pgtable_64.h
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
// the x86-64 page table tree.
//

extern "C" {
    pub fn paging_init();
}

extern "C" {
    pub fn set_pte_vaddr_p4d(p4d_page: *mut p4d_t, vaddr: c_ulong, new_pte: pte_t);
}
extern "C" {
    pub fn set_pte_vaddr_pud(pud_page: *mut pud_t, vaddr: c_ulong, new_pte: pte_t);
}

extern "C" {
    pub fn native_make_pte(_arg: xchg(&xp->pte, _arg: 0)) -> return;
}

// native_local_ptep_get_and_clear,

extern "C" {
    pub fn native_make_pmd(_arg: xchg(&xp->pmd, _arg: 0)) -> return;
}

// native_local_pmdp_get_and_clear,

extern "C" {
    pub fn native_make_pud(_arg: xchg(&xp->pud, _arg: 0)) -> return;
}

// native_local_pudp_get_and_clear,
// but duplicated because of cyclic dependency
//

//
// Conversion functions: convert a page and protection to a page entry,
// and a page entry and page directory to the page they refer to.
//
// PGD - Level 4 access
// PUD - Level 3 access
// PMD - Level 2 access
// PTE - Level 1 access
//
// Encode and de-code a swap entry
//
// |     ...            | 11| 10|  9|8|7|6|5| 4| 3|2| 1|0| <- bit number
// |     ...            |SW3|SW2|SW1|G|L|D|A|CD|WT|U| W|P| <- bit names
// | TYPE (59-63) | ~OFFSET (9-58)  |0|0|X|X| X| E|F|SD|0| <- swp entry
//
// G (8) is aliased and used as a PROT_NONE indicator for
// !present ptes.  We need to start storing swap entries above
// there.  We also need to avoid using A and D because of an
// erratum where they can be incorrectly set by hardware on
// non-present PTEs.
//
// SD Bits 1-4 are not used in non-present format and available for
// special use described below:
//
// SD (1) in swp entry is used to store soft dirty bit, which helps us
// remember soft dirty over page migration
//
// F (2) in swp entry is used to record when a pagetable is
// writeprotected by userfaultfd WP support.
//
// E (3) in swp entry is used to remember PG_anon_exclusive.
//
// Bit 7 in swp entry should be 0 because pmd_present checks not only P,
// but also L and G.
//
// The offset is inverted by a binary not operation to make the high
// physical bits set.
//
pub const SWP_TYPE_BITS: c_int = 5;

// We always extract/encode the offset by shifting it all the way up, and then down again

// Extract the high bits for type

// Shift up (to get rid of type), then down to get value

//
// Shift the offset up "too far" by TYPE bits, then down again
// The offset is inverted by a binary not operation to make the high
// physical bits set.
//

extern "C" {
    pub fn cleanup_highmap();
}
// Macro flag: #define HAVE_ARCH_UNMAPPED_AREA
// Macro flag: #define HAVE_ARCH_UNMAPPED_AREA_TOPDOWN

pub const HAVE_PAGE_AGP: c_int = 1;
// fs/proc/kcore.c

extern "C" {
    pub fn init_extra_mapping_uc(phys: c_ulong, size: c_ulong);
}
extern "C" {
    pub fn init_extra_mapping_wb(phys: c_ulong, size: c_ulong);
}

// Automate the creation of 1 to 1 mapping pmd entries

