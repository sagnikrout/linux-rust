//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/extent_io.h
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

// write IO error
// Indicate the extent buffer is written zeroed out (for zoned)
// Indicate that extent buffer pages a being read
// these are flags for __process_pages_contig
// Page starts writeback, clear dirty bit and set writeback bit
//
// Folio private values.  Every page that is controlled by the extent map has
// folio private set to this value.
//
pub const EXTENT_FOLIO_PRIVATE: c_int = 1;
//
// The extent buffer bitmap operations are done with byte granularity instead of
// word granularity for two reasons:
// 1. The bitmaps must be little-endian on disk.
// 2. Bitmap items are not guaranteed to be aligned to a word and therefore a
// single word in a bitmap may straddle two pages in the extent buffer.
//

extern "C" {
    pub fn extent_buffer_init_cachep() -> int __init;
}
extern "C" {
    pub fn extent_buffer_free_cachep() -> void __cold;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_buffer {
    pub start: u64,
    pub len: u32,
    pub folio_size: u32,
    pub bflags: c_ulong,
    pub fs_info: *mut btrfs_fs_info,
//
// The address where the eb can be accessed without any cross-page handling.
// This can be NULL if not possible.
//
    pub addr: *mut c_void,
    pub refs_lock: spinlock_t,
    pub refs: refcount_t,
    pub read_mirror: c_int,
// Inhibit WB_SYNC_NONE writeback when > 0.
    pub writeback_inhibitors: core::sync::atomic::AtomicI32,
// >= 0 if eb belongs to a log tree, -1 otherwise
    pub log_index: i8,
    pub folio_shift: u8,
    pub rcu_head: rcu_head,
    pub lock: rw_semaphore,
//
// Pointers to all the folios of the extent buffer.
//
// For now the folio is always order 0 (aka, a single page).
//
    pub folios: [*mut folio; INLINE_EXTENT_BUFFER_PAGES],
    pub leak_list: list_head,
    pub lock_owner: pid_t,

}

//
// Wrapper struct for managing preallocating an extent_buffer, its folios and a
// btrfs_folio_state if needed.
//
// Only used to mediate allocation, do not refer to the eb directly if not
// returned from a successful eb allocating API.
//
// The eb folios and bfs should generally not be fully attached, except briefly
// before they are NULLed in the struct after successful attachment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_eb_prealloc {
    pub eb: *mut extent_buffer,
    pub bfs: *mut btrfs_folio_state,
// eb alloc may use GFP_NOWAIT; caller can drop locks and retry.
    pub supports_nowait: bool,
// GFP_NOWAIT eb alloc failed; preallocate again and retry.
    pub needs_prealloc: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_eb_write_context {
    pub wbc: *mut writeback_control,
    pub eb: *mut extent_buffer,
// Block group @eb resides in. Only used for zoned mode.
    pub zoned_bg: *mut btrfs_block_group,
}

//
// Get the correct offset inside the page of extent buffer.
//
// @eb:		target extent buffer
// @start:	offset inside the extent buffer
//
// Will handle both sectorsize == PAGE_SIZE and sectorsize < PAGE_SIZE cases.
//
// 1) sectorsize == PAGE_SIZE and nodesize >= PAGE_SIZE case
// 1.1) One large folio covering the whole eb
// The eb->start is aligned to folio size, thus adding it
// won't cause any difference.
// 1.2) Several page sized folios
// The eb->start is aligned to folio (page) size, thus
// adding it won't cause any difference.
//
// 2) sectorsize < PAGE_SIZE and nodesize < PAGE_SIZE case
// In this case there would only be one page sized folio, and there
// may be several different extent buffers in the page/folio.
// We need to add eb->start to properly access the offset inside
// that eb.
//
extern "C" {
    pub fn offset_in_folio(_arg: eb->folios[0], eb->start: offset +) -> return;
}
//
// 1) sectorsize == PAGE_SIZE and nodesize >= PAGE_SIZE case
// 1.1) One large folio covering the whole eb.
// the folio_shift would be large enough to always make us
// return 0 as index.
// 1.2) Several page sized folios
// The folio_shift would be PAGE_SHIFT, giving us the correct
// index.
//
// 2) sectorsize < PAGE_SIZE and nodesize < PAGE_SIZE case
// The folio would only be page sized, and always give us 0 as index.
//
// Structure to record how many bytes and which ranges are set/cleared
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_changeset {
// How many bytes are set/cleared in this operation
    pub bytes_changed: u64,
// Changed ranges
    pub range_changed: ulist,
}

//
// Sentinel value for range_changed.prealloc indicating that the changeset
// only tracks bytes_changed and does not record individual ranges. This
// avoids GFP_ATOMIC allocations inside add_extent_changeset() when the
// caller doesn't need to iterate the changed ranges afterwards.
//

extern "C" {
    pub fn try_release_extent_mapping(folio: *mut folio, mask: gfp_t) -> bool;
}
extern "C" {
    pub fn try_release_extent_buffer(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn btrfs_read_folio(file: *mut file, folio: *mut folio) -> c_int;
}

extern "C" {
    pub fn btrfs_check_folio_write_protected(folio: *mut folio);
}

extern "C" {
    pub fn btrfs_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn btree_writepages(mapping: *mut address_space, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn btrfs_btree_wait_writeback_range(fs_info: *mut btrfs_fs_info, start: u64, end: u64);
}
extern "C" {
    pub fn btrfs_readahead(rac: *mut readahead_control);
}
extern "C" {
    pub fn set_folio_extent_mapped(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn clear_folio_extent_mapped(folio: *mut folio);
}
extern "C" {
    pub fn btrfs_free_eb_prealloc(pa: *mut btrfs_eb_prealloc);
}
extern "C" {
    pub fn free_extent_buffer(eb: *mut extent_buffer);
}
extern "C" {
    pub fn free_extent_buffer_stale(eb: *mut extent_buffer);
}
extern "C" {
    pub fn btrfs_readahead_node_child(node: *mut extent_buffer, slot: c_int);
}
// Note: this can be used in for loops without caching the value in a variable.
//
// For sectorsize == PAGE_SIZE case, since nodesize is always aligned to
// sectorsize, it's just eb->len >> PAGE_SHIFT.
//
// For sectorsize < PAGE_SIZE case, we could have nodesize < PAGE_SIZE,
// thus have to ensure we get at least one page.
//
// This can only be determined at runtime by checking eb::folios[0].
//
// As we can have either one large folio covering the whole eb
// (either nodesize <= PAGE_SIZE, or high order folio), or multiple
// single-paged folios.
//
// Note: this can be used in for loops without caching the value in a variable.
//
extern "C" {
    pub fn num_extent_pages(_arg: eb) -> return;
}
extern "C" {
    pub fn test_bit(_arg: EXTENT_BUFFER_UPTODATE, _arg: &eb->bflags) -> return;
}
extern "C" {
    pub fn set_extent_buffer_dirty(eb: *mut extent_buffer);
}
extern "C" {
    pub fn set_extent_buffer_uptodate(eb: *mut extent_buffer);
}
extern "C" {
    pub fn clear_extent_buffer_uptodate(eb: *mut extent_buffer);
}
extern "C" {
    pub fn btrfs_zoned_release_dirty_metadata(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_alloc_page_array(nr_pages: c_uint, page_array: *mut page, gfp: gfp_t) -> c_int;
}

extern "C" {
    pub fn btrfs_extent_buffer_leak_debug_check(fs_info: *mut btrfs_fs_info);
}

extern "C" {
    pub fn btrfs_uninhibit_all_eb_writeback(trans: *mut btrfs_trans_handle);
}
