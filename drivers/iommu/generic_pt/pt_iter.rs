//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/pt_iter.h
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
// Iterators for Generic Page Table
//

//
// Use to mangle symbols so that backtraces and the symbol table are
// understandable. Any non-inlined function should get mangled like this.
//

//
// pt_check_range() - Validate the range can be iterated
// @range: Range to validate
//
// Check that VA and last_va fall within the permitted range of VAs. If the
// format is using PT_FEAT_SIGN_EXTEND then this also checks the sign extension
// is correct.
//
// pt_index_to_va() - Update range->va to the current pts->index
// @pts: Iteration State
//
// Adjust range->va to match the current index. This is done in a lazy manner
// since computing the VA takes several instructions and is rarely required.
//
// Add index_count_lg2 number of entries to pts's VA and index. The VA will be
// adjusted to the end of the contiguous block if it is currently in the middle.
//
// pt_entry_fully_covered() - Check if the item or entry is entirely contained
// within pts->range
// @pts: Iteration State
// @oasz_lg2: The size of the item to check, pt_table_item_lg2sz() or
// pt_entry_oa_lg2sz()
//
// Returns: true if the item is fully enclosed by the pts->range.
//
// Range begins at the start of the entry
// Range ends past the end of the entry
// Range ends at the end of the entry
extern "C" {
    pub fn log2_mod_eq_max(_arg: range->last_va, _arg: oasz_lg2) -> return;
}
//
// pt_range_to_index() - Starting index for an iteration
// @pts: Iteration State
//
// Return: the starting index for the iteration in pts.
//
// pt_range_to_end_index() - Ending index iteration
// @pts: Iteration State
//
// Return: the last index for the iteration in pts.
//
// last_va falls within this table
extern "C" {
    pub fn log2_to_int(_arg: num_entries_lg2) -> return;
}
//
// pt_next_entry() - Advance pts to the next entry
// @pts: Iteration State
//
// Update pts to go to the next index at this level. If pts is pointing at a
// contiguous entry then the index may advance my more than one.
//
// for_each_pt_level_entry() - For loop wrapper over entries in the range
// @pts: Iteration State
//
// This is the basic iteration primitive. It iterates over all the entries in
// pts->range that fall within the pts's current table level. Each step does
// pt_load_entry(pts).
//

//
// pt_load_single_entry() - Version of pt_load_entry() usable within a walker
// @pts: Iteration State
//
// Alternative to for_each_pt_level_entry() if the walker function uses only a
// single entry.
//
// The top range will default to the lower region only with sign extend.
//
// pt_top_range() - Return a range that spans part of the top level
// @common: Table
//
// For PT_FEAT_SIGN_EXTEND this will return the lower range, and cover half the
// total page table. Otherwise it returns the entire page table.
//
// The top pointer can change without locking. We capture the value and
// it's level here and are safe to walk it so long as both values are
// captured without tearing.
//
extern "C" {
    pub fn _pt_top_range(_arg: common, _arg: READ_ONCE(common->top_of_table)) -> return;
}
//
// pt_all_range() - Return a range that spans the entire page table
// @common: Table
//
// The returned range spans the whole page table. Due to how PT_FEAT_SIGN_EXTEND
// is supported range->va and range->last_va will be incorrect during the
// iteration and must not be accessed.
//
// Pretend the table is linear from 0 without a sign extension. This
// generates the correct indexes for iteration.
//
// pt_upper_range() - Return a range that spans part of the top level
// @common: Table
//
// For PT_FEAT_SIGN_EXTEND this will return the upper range, and cover half the
// total page table. Otherwise it returns the entire page table.
//
// pt_make_range() - Return a range that spans part of the table
// @common: Table
// @va: Start address
// @last_va: Last address
//
// The caller must validate the range with pt_check_range() before using it.
//
// Span a slice of the table starting at a lower table level from an active
// walk.
//
// pt_init() - Initialize a pt_state on the stack
// @range: Range pointer to embed in the state
// @level: Table level for the state
// @table: Pointer to the table memory at level
//
// Helper to initialize the on-stack pt_state from walker arguments.
//
// pt_init_top() - Initialize a pt_state on the stack
// @range: Range pointer to embed in the state
//
// The pt_state points to the top most level.
//
extern "C" {
    pub fn pt_init(_arg: range, _arg: range->top_level, _arg: range->top_table) -> return;
}
//
// pt_descend() - Recursively invoke the walker for the lower level
// @pts: Iteration State
// @arg: Value to pass to the function
// @fn: Walker function to call
//
// pts must point to a table item. Invoke fn as a walker on the table
// pts points to.
//
// pt_walk_range() - Walk over a VA range
// @range: Range pointer
// @fn: Walker function to call
// @arg: Value to pass to the function
//
// Walk over a VA range. The caller should have done a validity check, at
// least calling pt_check_range(), when building range. The walk will
// start at the top most table.
//
extern "C" {
    pub fn fn(_arg: range, _arg: arg, _arg: range->top_level, _arg: range->top_table) -> return;
}
//
// pt_walk_descend() - Recursively invoke the walker for a slice of a lower
// level
// @pts: Iteration State
// @va: Start address
// @last_va: Last address
// @fn: Walker function to call
// @arg: Value to pass to the function
//
// With pts pointing at a table item this will descend and over a slice of the
// lower table. The caller must ensure that va/last_va are within the table
// item. This creates a new walk and does not alter pts or pts->range.
//
extern "C" {
    pub fn fn(_arg: &range, _arg: arg, 1: pts->level -, _arg: pts->table_lower) -> return;
}
//
// pt_walk_descend_all() - Recursively invoke the walker for a table item
// @parent_pts: Iteration State
// @fn: Walker function to call
// @arg: Value to pass to the function
//
// With pts pointing at a table item this will descend and over the entire lower
// table. This creates a new walk and does not alter pts or pts->range.
//
// pt_range_slice() - Return a range that spans indexes
// @pts: Iteration State
// @start_index: Starting index within pts
// @end_index: Ending index within pts
//
// Create a range than spans an index range of the current table level
// pt_state points at.
//
extern "C" {
    pub fn pt_make_child_range(_arg: pts->range, _arg: va, _arg: last_va) -> return;
}
//
// pt_top_memsize_lg2()
// @common: Table
// @top_of_table: Top of table value from _pt_top_set()
//
// Compute the allocation size of the top table. For PT_FEAT_DYNAMIC_TOP this
// will compute the top size assuming the table will grow.
//
// Round up the allocation size to the minimum alignment
//
// pt_compute_best_pgsize() - Determine the best page size for leaf entries
// @pgsz_bitmap: Permitted page sizes
// @va: Starting virtual address for the leaf entry
// @last_va: Last virtual address for the leaf entry, sets the max page size
// @oa: Starting output address for the leaf entry
//
// Compute the largest page size for va, last_va, and oa together and return it
// in lg2. The largest page size depends on the format's supported page sizes at
// this level, and the relative alignment of the VA and OA addresses. 0 means
// the OA cannot be stored with the provided pgsz_bitmap.
//
// Given a VA/OA pair the best page size is the largest page size
// where:
//
// 1) VA and OA start at the page. Bitwise this is the count of least
// significant 0 bits.
// This also implies that last_va/oa has the same prefix as va/oa.
//
// 2) The page size is not larger than the last_va (length). Since page
// sizes are always power of two this can't be larger than the
// largest power of two factor of the length.
//
// Choose the highest bit <= best_pgsz_lg2
//
// Return the number of pgsize_lg2 leaf entries that can be mapped for
// va to oa. This accounts for any requirement to reduce or increase the page
// size across the VA range.
//
extern "C" {
    pub fn log2_div(_arg: len, _arg: pgsize_lg2) -> return;
}

//
// PT_MAKE_LEVELS() - Build an unwound walker
// @fn: Name of the walker function
// @do_fn: Function to call at each level
//
// This builds a function call tree that can be fully inlined.
// The caller must provide a function body in an __always_inline function::
//
// static __always_inline int do_fn(struct pt_range *range, void *arg,
// unsigned int level, struct pt_table_p *table,
// pt_level_fn_t descend_fn)
//
// An inline function will be created for each table level that calls do_fn with
// a compile time constant for level and a pointer to the next lower function.
// This generates an optimally inlined walk where each of the functions sees a
// constant level and can codegen the exact constants/etc for that level.
//
// Note this can produce a lot of code!
//

