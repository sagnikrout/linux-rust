//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/pt_common.h
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
// This header is included after the format. It contains definitions
// that build on the format definitions to create the basic format API.
//
// The format API is listed here, with kdocs. The functions without bodies are
// implemented in the format using the pattern:
// static inline FMTpt_XXX(..) {..}
// #define pt_XXX FMTpt_XXX
//
// If the format doesn't implement a function then pt_fmt_defaults.h can provide
// a generic version.
//
// The routines marked "@pts: Entry to query" operate on the entire contiguous
// entry and can be called with a pts->index pointing to any sub item that makes
// up that entry.
//
// The header order is:
// pt_defs.h
// FMT.h
// pt_common.h
//

//
// pt_attr_from_entry() - Convert the permission bits back to attrs
// @pts: Entry to convert from
// @attrs: Resulting attrs
//
// Fill in the attrs with the permission bits encoded in the current leaf entry.
// The attrs should be usable with pt_install_leaf_entry() to reconstruct the
// same entry.
//
// pt_can_have_leaf() - True if the current level can have an OA entry
// @pts: The current level
//
// True if the current level can support pt_install_leaf_entry(). A leaf
// entry produce an OA.
//
extern "C" {
    pub fn pt_can_have_leaf(pts: *const pt_state) -> bool;
}
//
// pt_can_have_table() - True if the current level can have a lower table
// @pts: The current level
//
// Every level except 0 is allowed to have a lower table.
//
// No further tables at level 0
//
// pt_clear_entries() - Make entries empty (non-present)
// @pts: Starting table index
// @num_contig_lg2: Number of contiguous items to clear
//
// Clear a run of entries. A cleared entry will load back as PT_ENTRY_EMPTY
// and does not have any effect on table walking. The starting index must be
// aligned to num_contig_lg2.
//
// pt_entry_make_write_dirty() - Make an entry dirty
// @pts: Table entry to change
//
// Make pt_entry_is_write_dirty() return true for this entry. This can be called
// asynchronously with any other table manipulation under a RCU lock and must
// not corrupt the table.
//
extern "C" {
    pub fn pt_entry_make_write_dirty(pts: *mut pt_state) -> bool;
}
//
// pt_entry_make_write_clean() - Make the entry write clean
// @pts: Table entry to change
//
// Modify the entry so that pt_entry_is_write_dirty() == false. The HW will
// eventually be notified of this change via a TLB flush, which is the point
// that the HW must become synchronized. Any "write dirty" prior to the TLB
// flush can be lost, but once the TLB flush completes all writes must make
// their entries write dirty.
//
// The format should alter the entry in a way that is compatible with any
// concurrent update from HW. The entire contiguous entry is changed.
//
extern "C" {
    pub fn pt_entry_make_write_clean(pts: *mut pt_state);
}
//
// pt_entry_is_write_dirty() - True if the entry has been written to
// @pts: Entry to query
//
// "write dirty" means that the HW has written to the OA translated
// by this entry. If the entry is contiguous then the consolidated
// "write dirty" for all the items must be returned.
//
extern "C" {
    pub fn pt_entry_is_write_dirty(pts: *const pt_state) -> bool;
}
//
// pt_dirty_supported() - True if the page table supports dirty tracking
// @common: Page table to query
//
extern "C" {
    pub fn pt_dirty_supported(common: *mut pt_common) -> bool;
}
//
// pt_entry_num_contig_lg2() - Number of contiguous items for this leaf entry
// @pts: Entry to query
//
// Return the number of contiguous items this leaf entry spans. If the entry
// is single item it returns ilog2(1).
//
extern "C" {
    pub fn pt_entry_num_contig_lg2(pts: *const pt_state) -> c_uint;
}
//
// pt_entry_oa() - Output Address for this leaf entry
// @pts: Entry to query
//
// Return the output address for the start of the entry. If the entry
// is contiguous this returns the same value for each sub-item. I.e.::
//
// log2_mod(pt_entry_oa(), pt_entry_oa_lg2sz()) == 0
//
// See pt_item_oa(). The format should implement one of these two functions
// depending on how it stores the OAs in the table.
//
extern "C" {
    pub fn pt_entry_oa(pts: *const pt_state) -> pt_oaddr_t;
}
//
// pt_entry_oa_lg2sz() - Return the size of an OA entry
// @pts: Entry to query
//
// If the entry is not contiguous this returns pt_table_item_lg2sz(), otherwise
// it returns the total VA/OA size of the entire contiguous entry.
//
extern "C" {
    pub fn pt_entry_num_contig_lg2(pt_table_item_lg2sz(pts: pts) +) -> return;
}
//
// pt_entry_oa_exact() - Return the complete OA for an entry
// @pts: Entry to query
//
// During iteration the first entry could have a VA with an offset from the
// natural start of the entry. Return the exact OA including the pts's VA
// offset.
//
// pt_full_va_prefix() - The top bits of the VA
// @common: Page table to query
//
// This is usually 0, but some formats have their VA space going downward from
// PT_VADDR_MAX, and will return that instead. This value must always be
// adjusted by struct pt_common max_vasz_lg2.
//
extern "C" {
    pub fn pt_full_va_prefix(common: *const pt_common) -> pt_vaddr_t;
}
//
// pt_has_system_page_size() - True if level 0 can install a PAGE_SHIFT entry
// @common: Page table to query
//
// If true the caller can use, at level 0, pt_install_leaf_entry(PAGE_SHIFT).
// This is useful to create optimized paths for common cases of PAGE_SIZE
// mappings.
//
extern "C" {
    pub fn pt_has_system_page_size(common: *const pt_common) -> bool;
}
//
// pt_install_leaf_entry() - Write a leaf entry to the table
// @pts: Table index to change
// @oa: Output Address for this leaf
// @oasz_lg2: Size in VA/OA for this leaf
// @attrs: Attributes to modify the entry
//
// A leaf OA entry will return PT_ENTRY_OA from pt_load_entry(). It translates
// the VA indicated by pts to the given OA.
//
// For a single item non-contiguous entry oasz_lg2 is pt_table_item_lg2sz().
// For contiguous it is pt_table_item_lg2sz() + num_contig_lg2.
//
// This must not be called if pt_can_have_leaf() == false. Contiguous sizes
// not indicated by pt_possible_sizes() must not be specified.
//
// pt_install_table() - Write a table entry to the table
// @pts: Table index to change
// @table_pa: CPU physical address of the lower table's memory
// @attrs: Attributes to modify the table index
//
// A table entry will return PT_ENTRY_TABLE from pt_load_entry(). The table_pa
// is the table at pts->level - 1. This is done by cmpxchg so pts must have the
// current entry loaded. The pts is updated with the installed entry.
//
// This must not be called if pt_can_have_table() == false.
//
// Returns: true if the table was installed successfully.
//
// pt_item_oa() - Output Address for this leaf item
// @pts: Item to query
//
// Return the output address for this item. If the item is part of a contiguous
// entry it returns the value of the OA for this individual sub item.
//
// See pt_entry_oa(). The format should implement one of these two functions
// depending on how it stores the OA's in the table.
//
extern "C" {
    pub fn pt_item_oa(pts: *const pt_state) -> pt_oaddr_t;
}
//
// pt_load_entry_raw() - Read from the location pts points at into the pts
// @pts: Table index to load
//
// Return the type of entry that was loaded. pts->entry will be filled in with
// the entry's content. See pt_load_entry()
//
extern "C" {
    pub fn pt_load_entry_raw(pts: *mut pt_state) -> pt_entry_type;
}
//
// pt_max_oa_lg2() - Return the maximum OA the table format can hold
// @common: Page table to query
//
// The value oalog2_to_max_int(pt_max_oa_lg2()) is the MAX for the
// OA. This is the absolute maximum address the table can hold. struct pt_common
// max_oasz_lg2 sets a lower dynamic maximum based on HW capability.
//
// pt_num_items_lg2() - Return the number of items in this table level
// @pts: The current level
//
// The number of items in a table level defines the number of bits this level
// decodes from the VA. This function is not called for the top level,
// so it does not need to compute a special value for the top case. The
// result for the top is based on pt_common max_vasz_lg2.
//
// The value is used as part of determining the table indexes via the
// equation::
//
// log2_mod(log2_div(VA, pt_table_item_lg2sz()), pt_num_items_lg2())
//
extern "C" {
    pub fn pt_num_items_lg2(pts: *const pt_state) -> c_uint;
}
//
// pt_pgsz_lg2_to_level - Return the level that maps the page size
// @common: Page table to query
// @pgsize_lg2: Log2 page size
//
// Returns the table level that will map the given page size. The page
// size must be part of the pt_possible_sizes() for some level.
//
// pt_possible_sizes() - Return a bitmap of possible output sizes at this level
// @pts: The current level
//
// Each level has a list of possible output sizes that can be installed as
// leaf entries. If pt_can_have_leaf() is false returns zero.
//
// Otherwise the bit in position pt_table_item_lg2sz() should be set indicating
// that a non-contiguous single item leaf entry is supported. The following
// pt_num_items_lg2() number of bits can be set indicating contiguous entries
// are supported. Bit pt_table_item_lg2sz() + pt_num_items_lg2() must not be
// set, contiguous entries cannot span the entire table.
//
// The OR of pt_possible_sizes() of all levels is the typical bitmask of all
// supported sizes in the entire table.
//
extern "C" {
    pub fn pt_possible_sizes(pts: *const pt_state) -> pt_vaddr_t;
}
//
// pt_table_item_lg2sz() - Size of a single item entry in this table level
// @pts: The current level
//
// The size of the item specifies how much VA and OA a single item occupies.
//
// See pt_entry_oa_lg2sz() for the same value including the effect of contiguous
// entries.
//
extern "C" {
    pub fn pt_table_item_lg2sz(pts: *const pt_state) -> c_uint;
}
//
// pt_table_oa_lg2sz() - Return the VA/OA size of the entire table
// @pts: The current level
//
// Return the size of VA decoded by the entire table level.
//
// pt_table_pa() - Return the CPU physical address of the table entry
// @pts: Entry to query
//
// This is only ever called on PT_ENTRY_TABLE entries. Must return the same
// value passed to pt_install_table().
//
extern "C" {
    pub fn pt_table_pa(pts: *const pt_state) -> pt_oaddr_t;
}
//
// pt_table_ptr() - Return a CPU pointer for a table item
// @pts: Entry to query
//
// Same as pt_table_pa() but returns a CPU pointer.
//
extern "C" {
    pub fn __va(_arg: pt_table_pa(pts)) -> return;
}
//
// pt_max_sw_bit() - Return the maximum software bit usable for any level and
// entry
// @common: Page table
//
// The swbit can be passed as bitnr to the other sw_bit functions.
//
extern "C" {
    pub fn pt_max_sw_bit(common: *mut pt_common) -> c_uint;
}
//
// pt_test_sw_bit_acquire() - Read a software bit in an item
// @pts: Entry to read
// @bitnr: Bit to read
//
// Software bits are ignored by HW and can be used for any purpose by the
// software. This does a test bit and acquire operation.
//
// pt_set_sw_bit_release() - Set a software bit in an item
// @pts: Entry to set
// @bitnr: Bit to set
//
// Software bits are ignored by HW and can be used for any purpose by the
// software. This does a set bit and release operation.
//
// pt_load_entry() - Read from the location pts points at into the pts
// @pts: Table index to load
//
// Set the type of entry that was loaded. pts->entry and pts->table_lower
// will be filled in with the entry's content.
//
