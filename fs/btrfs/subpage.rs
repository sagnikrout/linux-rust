//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/subpage.h
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
// Extra info for subpage bitmap.
//
// For subpage we pack all uptodate/dirty/writeback/fixup bitmaps into
// one larger bitmap.
//
// This structure records how they are organized in the bitmap:
//
// /- uptodate          /- dirty        /- writeback       /- fixup
// |			|		|		   |
// v			v		v		   v
// |u|u|u|u|........|u|u|d|d|.......|d|d|w|w|.....|w|w|f|f|.....|f|f|
// |< sectors_per_page >|
//
// Unlike regular macro-like enums, here we do not go upper-case names, as
// these names will be utilized in various macros to define function names.
//
// This can be changed to atomic eventually.  But this change will rely
// on the async delalloc range rework for locked bitmap.  As async
// delalloc can unlock its range and mark blocks writeback at random
// timing.
//
// Blocks dirtied by the dirty_folio callback instead of a reserving
// write path (e.g. set_page_dirty_lock() on a GUP pin).  They have
// no space reservation and need the writepage fixup before they can
// be submitted.
//
// Structure to trace status of each sector inside a page, attached to
// page::private for both data and metadata inodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_folio_state {
// Common members for both data and metadata pages
    pub lock: spinlock_t,
//
// Structures only used by metadata
//
// @eb_refs should only be operated under private_lock, as it
// manages whether the btrfs_folio_state can be detached.
//
    pub eb_refs: core::sync::atomic::AtomicI32,
//
// Structures only used by data,
//
// How many sectors inside the page is locked.
//
    pub nr_locked: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_folio_type {
    BTRFS_SUBPAGE_METADATA,
    BTRFS_SUBPAGE_DATA,
}

//
// Subpage support for metadata is more complex, as we can have dummy extent
// buffers, where folios have no mapping to determine the owning inode.
//
// Thankfully we only need to check if node size is smaller than page size.
// Even with larger folio support, we will only allocate a folio as large as
// node size.
// Thus if nodesize < PAGE_SIZE, we know metadata needs need to subpage routine.
//
// Allocate additional data where page represents more than one sector
extern "C" {
    pub fn btrfs_folio_inc_eb_refs(fs_info: *const btrfs_fs_info, folio: *mut folio);
}
extern "C" {
    pub fn btrfs_folio_dec_eb_refs(fs_info: *const btrfs_fs_info, folio: *mut folio);
}
//
// Template for subpage related operations.
//
// btrfs_subpage_*() are for call sites where the folio has subpage attached and
// the range is ensured to be inside the folio's single page.
//
// btrfs_folio_*() are for call sites where the page can either be subpage
// specific or regular folios. The function will handle both cases.
// But the range still needs to be inside one single page.
//
// btrfs_folio_clamp_*() are similar to btrfs_folio_*(), except the range doesn't
// need to be inside the page. Those functions will truncate the range
// automatically.
//
// Both btrfs_folio_*() and btrfs_folio_clamp_*() are for data folios.
//
// For metadata, one should use btrfs_meta_folio_*() helpers instead, and there
// is no clamp version for metadata helpers, as we either go subpage
// (nodesize < PAGE_SIZE) or go regular folio helpers (nodesize >= PAGE_SIZE,
// and our folio is never larger than nodesize).
//

//
// Fixup bit helpers.
//
// The fixup bit is data-only and has no plain set helper (setting happens
// together with dirtying in btrfs_subpage_set_fixup_dirty()), so it does not
// go through DECLARE_BTRFS_SUBPAGE_OPS().  For single-block folios the
// folio_*_fixup_pending() flag takes the place of the bitmap.
//
// For a block that just got its space reserved; it stays dirty.
// For callers discarding the data; clears the dirty bits too.
//
// Helper for error cleanup, where a folio will have its dirty flag cleared,
// with writeback started and finished.
//
extern "C" {
    pub fn btrfs_meta_folio_clear_and_test_dirty(folio: *mut folio, eb: *const extent_buffer) -> bool;
}
