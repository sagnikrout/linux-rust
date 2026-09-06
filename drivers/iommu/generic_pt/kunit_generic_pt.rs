//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/kunit_generic_pt.h
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
// Test the format API directly.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct check_levels_arg {
    pub test: *mut kunit,
    pub fn_arg: *mut c_void,
    pub arg): *mut *mut *mut *mut void (fn)(struct kunit test, struct pt_state pts, void,
}

//
// If we were able to use the full VA space this should always be the
// last index in each table.
//
// Index 0 is used by the test
//
// A format should not create a table with only one entry, at least this
// test approach won't work.
//
// For increase top we end up using index 0 for the original top's tree,
// so use index 1 for testing instead.
//
// Call fn for each level in the table with a pts setup to index 0 in a table
// for that level. This allows writing tests that run on every level.
// The test can use every index in the table except the last one.
//
// Map a page at the highest VA, this will populate all the levels so we
// can then iterate over them. Index 0 will be used for testing.
//
// Fixture does the setup
//
// Basic check that the log2_* functions are working, especially at the integer
// limits.
//
// Brute force the constraints described in pt_compute_best_pgsize()
// Check that the bit logic in pt_compute_best_pgsize() works.
// Try random prefixes with every suffix combination
// 0 prefix, every suffix
// 1's prefix, every suffix
// pgsize_bitmap is always 0
// over 32 bit page sizes
//
// Check that pt_install_table() and pt_table_pa() match
//
// A second install should pass because install updates pts->entry.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lvl_radix_arg {
    pub vbits: pt_vaddr_t,
}

//
// Check pt_table_oa_lg2sz() and pt_table_item_lg2sz() they need to decode a
// continuous list of VA across all the levels that covers the entire advertised
// VA space.
//
// Every bit below us is decoded
// We are not decoding bits someone else is
// Can't decode past the pt_vaddr_t size
//
// Avoid calling pt_num_items_lg2() on the top, instead we can derive
// the size of the top table from the top range.
//
extern "C" {
    pub fn ilog2(_arg: pt_range_to_end_index(&top_pts)) -> return;
}
extern "C" {
    pub fn pt_num_items_lg2(_arg: pts) -> return;
}
// Matches get_info()
// No bits for sizes that would be outside this table
//
// Non contiguous must be supported. AMDv1 has a HW bug where it does
// not support it on one of the levels.
//
// A contiguous entry should not span the whole table
// Verify that every contiguous item translates correctly
//
// Check that pt_install_leaf_entry() and pt_entry_oa() match.
// Check that pt_clear_entries() works.
//
// Check that the table can store the boundary OAs
// Test pt_attr_from_entry()
//
// If the format doesn't support this combination of
// prot bits skip it
//
// But RW has to be supported
//
// The descriptor produced by pt_attr_from_entry()
// produce an identical entry value when re-written
//
// dirty every contiguous entry
// SW bits didn't leak into the attrs

