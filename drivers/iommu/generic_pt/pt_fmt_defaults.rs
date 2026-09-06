//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/pt_fmt_defaults.h
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
// Default definitions for formats that don't define these functions.
//

// Header self-compile default defines

//
// The format must provide PT_GRANULE_LG2SZ, PT_TABLEMEM_LG2SZ, and
// PT_ITEM_WORD_SIZE. They must be the same at every level excluding the top.
//

//
// If not supplied by the format then contiguous pages are not supported.
//
// If contiguous pages are supported then the format must also provide
// pt_contig_count_lg2() if it supports a single contiguous size per level,
// or pt_possible_sizes() if it supports multiple sizes per level.
//

extern "C" {
    pub fn ilog2(_arg: 1) -> return;
}
//
// Return the number of contiguous OA items forming an entry at this table level
//
extern "C" {
    pub fn ilog2(_arg: 1) -> return;
}

// If not supplied by the format then dirty tracking is not supported

// If not supplied then dirty tracking is always enabled

//
// Format supplies either:
// pt_entry_oa - OA is at the start of a contiguous entry
// or
// pt_item_oa  - OA is adjusted for every item in a contiguous entry
//
// Build the missing one
//
// The internal helper _pt_entry_oa_fast() allows generating
// an efficient pt_entry_oa_exact(), it doesn't care which
// option is selected.
//

//
// If not supplied by the format then use the constant
// PT_MAX_OUTPUT_ADDRESS_LG2.
//

//
// If not supplied by the format then assume only one contiguous size determined
// by pt_contig_count_lg2()
//

extern "C" {
    pub fn pt_contig_count_lg2(pts: *const pt_state) -> c_ushort;
}
// Return a bitmap of possible leaf page sizes at this level

// If not supplied by the format then use 0.

// If not supplied by the format then zero fill using PT_ITEM_WORD_SIZE

// If not supplied then SW bits are not supported

// Acquire, pairs with pt_set_sw_bit_release()
// For a contiguous entry the sw bit is only stored in the first item.

extern "C" {
    pub fn __pt_no_sw_bit();
}

//
// Format can call in the pt_install_leaf_entry() to check the arguments are all
// aligned correctly.
//

