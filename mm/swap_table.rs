//! Automatically rewritten from C Header to Rust Module
//! Source: mm/swap_table.h
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

// A typical flat array in each cluster as swap table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_table {
    pub entries: [atomic_long_t; SWAPFILE_CLUSTER],
}

// For storing memcg private id
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_memcg_table {
    pub id: [c_ushort; SWAPFILE_CLUSTER],
}

//
// A swap table entry represents the status of a swap slot on a swap
// (physical or virtual) device. The swap table in each cluster is a
// 1:1 map of the swap slots in this cluster.
//
// Swap table entry type and bits layouts:
//
// NULL:     |---------------- 0 ---------------| - Free slot
// Shadow:   |SWAP_COUNT|Z|---- SHADOW_VAL ---|1| - Swapped out slot
// PFN:      |SWAP_COUNT|Z|------ PFN -------|10| - Cached slot
// Pointer:  |----------- Pointer ----------|100| - (Unused)
// Bad:      |------------- 1 -------------|1000| - Bad slot
//
// COUNT is `SWP_TB_COUNT_BITS` long, Z is the `SWP_TB_ZERO_FLAG` bit,
// and together they form the `SWP_TB_FLAGS_BITS` wide flags field.
// Each entry is an atomic long.
//
// Usages:
//
// - NULL: Swap slot is unused, could be allocated.
//
// - Shadow: Swap slot is used and not cached (usually swapped out). It reuses
// the XA_VALUE format to be compatible with working set shadows. SHADOW_VAL
// part might be all 0 if the working shadow info is absent. In such a case,
// we still want to keep the shadow format as a placeholder.
//
// Memcg ID is embedded in SHADOW_VAL.
//
// - PFN: Swap slot is in use, and cached. Memcg info is recorded on the page
// struct.
//
// - Pointer: Unused yet. `0b100` is reserved for potential pointer usage
// because only the lower three bits can be used as a marker for 8 bytes
// aligned pointers.
//
// - Bad: Swap slot is reserved, protects swap header or holes on swap devices.
//
// NULL Entry, all 0

// Swapped out: shadow

// Cached: PFN

// Flags: For PFN or shadow, contains SWAP_COUNT, width changes

// The first flag is zero bit (SWAP_TABLE_HAS_ZEROFLAG)

// Bad slot: ends with 0b1000 and rests of bits are all 1

// Macro for shadow offset calculation

//
// Helpers for casting one type of info into a swap table entry.
//
// At least three values are needed to distinguish free (0),
// used (count > 0 && count < SWP_TB_COUNT_MAX), and
// overflow (count == SWP_TB_COUNT_MAX).
//
extern "C" {
    pub fn pfn_to_swp_tb(_arg: folio_pfn(folio), _arg: flags) -> return;
}
extern "C" {
    pub fn sizeof(long): unsigned) -> *mut BITS_PER_BYTE;
}
//
// Helpers for swap table entry type checking.
//
extern "C" {
    pub fn xa_is_value()swp_tb: *mut (void) -> return;
}
//
// Helpers for retrieving info from swap table.
//
extern "C" {
    pub fn pfn_folio(SWAP_CACHE_PFN_MARK_BITS: (swp_tb & ~SWP_TB_FLAGS_MASK) >>) -> return;
}
// No shift needed, xa_value is stored as it is in the lower bits.
extern "C" {
    pub fn __swp_tb_get_count(_arg: swp_tb) -> return;
}
//
// Helpers for accessing or modifying the swap table of a cluster,
// the swap cluster must be locked.
//
// Ordering is guaranteed by cluster lock, relax
extern "C" {
    pub fn atomic_long_xchg_relaxed(_arg: &table[off], _arg: swp_tb) -> return;
}
extern "C" {
    pub fn atomic_long_read(_arg: &table[off]) -> return;
}

extern "C" {
    pub fn test_bit(_arg: ci_off, _arg: ci->zero_bitmap) -> return;
}

