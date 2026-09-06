//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/pt_defs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES
//
// This header is included before the format. It contains definitions
// that are required to compile the format. The header order is:
// pt_defs.h
// fmt_XX.h
// pt_common.h
//

// Header self-compile default defines

pub type pt_vaddr_t = u64;
pub type pt_oaddr_t = u64;

//
// The format instantiation can have features wired off or on to optimize the
// code gen. Supported features are just a reflection of what the current set of
// kernel users want to use.
//

pub const PT_SUPPORTED_FEATURES: c_int = 0;

//
// When in debug mode we compile all formats with all features. This allows the
// kunit to test the full matrix. SIGN_EXTEND can't co-exist with DYNAMIC_TOP or
// FULL_VA. DMA_INCOHERENT requires a SW bit that not all formats have
//

pub const PT_FORCE_ENABLED_FEATURES: c_int = 0;

//
// DOC: Generic Page Table Language
//
// Language used in Generic Page Table
// VA
// The input address to the page table, often the virtual address.
// OA
// The output address from the page table, often the physical address.
// leaf
// An entry that results in an output address.
// start/end
// An half-open range, e.g. [0,0) refers to no VA.
// start/last
// An inclusive closed range, e.g. [0,0] refers to the VA 0
// common
// The generic page table container struct pt_common
// level
// Level 0 is always a table of only leaves with no futher table pointers.
// Increasing levels increase the size of the table items. The least
// significant VA bits used to index page tables are used to index the Level
// 0 table. The various labels for table levels used by HW descriptions are
// not used.
// top_level
// The inclusive highest level of the table. A two-level table
// has a top level of 1.
// table
// A linear array of translation items for that level.
// index
// The position in a table of an element: item = table[index]
// item
// A single index in a table
// entry
// A single logical element in a table. If contiguous pages are not
// supported then item and entry are the same thing, otherwise entry refers
// to all the items that comprise a single contiguous translation.
// item/entry_size
// The number of bytes of VA the table index translates for.
// If the item is a table entry then the next table covers
// this size. If the entry translates to an output address then the
// full OA is: OA | (VA % entry_size)
// contig_count
// The number of consecutive items fused into a single entry.
// item_size * contig_count is the size of that entry's translation.
// lg2
// Indicates the value is encoded as log2, i.e. 1<<x is the actual value.
// Normally the compiler is fine to optimize divide and mod with log2 values
// automatically when inlining, however if the values are not constant
// expressions it can't. So we do it by hand; we want to avoid 64-bit
// divmod.
//
// Returned by pt_load_entry() and for_each_pt_level_entry()
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pt_entry_type {
    PT_ENTRY_EMPTY,
// Entry is valid and points to a lower table level
    PT_ENTRY_TABLE,
// Entry is valid and returns an output address
    PT_ENTRY_OA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_range {
    pub common: *mut pt_common,
    pub top_table: *mut pt_table_p,
    pub va: pt_vaddr_t,
    pub last_va: pt_vaddr_t,
    pub top_level: u8,
    pub max_vasz_lg2: u8,
}

//
// Similar to xa_state, this records information about an in-progress parse at a
// single level.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_state {
    pub range: *mut pt_range,
    pub table: *mut pt_table_p,
    pub table_lower: *mut pt_table_p,
    pub entry: u64,
    pub type: pt_entry_type,
    pub index: c_ushort,
    pub end_index: c_ushort,
    pub level: u8,
}

//
// Try to install a new table pointer. The locking methodology requires this to
// be atomic (multiple threads can race to install a pointer). The losing
// threads will fail the atomic and return false. They should free any memory
// and reparse the table level again.
//

//
// Ensure the zero'd table content itself is visible before its PTE can
// be. release is a NOP on !SMP, but the HW is still doing an acquire.
//

//
// Ensure the zero'd table content itself is visible before its PTE can
// be. release is a NOP on !SMP, but the HW is still doing an acquire.
//

extern "C" {
    pub fn pt_feature(_arg: pts->range->common, _arg: feature_nr) -> return;
}
//
// PT_WARN_ON is used for invariants that the kunit should be checking can't
// happen.
//

// These all work on the VA type

//
// The full VA (fva) versions permit the lg2 value to be == PT_VADDR_MAX_LG2 and
// generate a useful defined result. The non-fva versions will malfunction at
// this extreme.
//
extern "C" {
    pub fn log2_div_t(_arg: pt_vaddr_t, _arg: a, _arg: b_lg2) -> return;
}
extern "C" {
    pub fn log2_mod_t(_arg: pt_vaddr_t, _arg: a, _arg: b_lg2) -> return;
}
extern "C" {
    pub fn log2_div_eq_t(_arg: pt_vaddr_t, _arg: a, _arg: b, _arg: c_lg2) -> return;
}
extern "C" {
    pub fn log2_set_mod_t(_arg: pt_vaddr_t, _arg: a, _arg: val, _arg: b_lg2) -> return;
}
extern "C" {
    pub fn log2_set_mod_max_t(_arg: pt_vaddr_t, _arg: a, _arg: b_lg2) -> return;
}
// These all work on the OA type

extern "C" {
    pub fn READ_ONCE(PT_TOP_LEVEL_BITS: common->top_of_table) % (1 <<) -> return;
}
