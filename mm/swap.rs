//! Automatically rewritten from C Header to Rust Module
//! Source: mm/swap.h
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

// Swap table marker, 0x1 means shadow, 0x2 means PFN (SWP_TB_PFN_MARK)
pub const SWAP_CACHE_PFN_MARK_BITS: c_int = 2;
// At least 2 bits are needed to distinguish SWP_TB_COUNT_MAX, 1 and 0
pub const SWAP_COUNT_MIN_BITS: c_int = 2;
// If there are enough bits besides PFN and marker, store zero flag inline

pub const SWAPFILE_CLUSTER: c_int = 256;
pub const swap_entry_order(order): c_int = 0;

//
// We use this to track usage of a cluster. A cluster is a block of swap disk
// space with SWAPFILE_CLUSTER pages long and naturally aligns in disk. All
// free clusters are organized into a list. We fetch an entry from the list to
// get a free cluster.
//
// The flags field determines if a cluster is free. This is
// protected by cluster lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_cluster_info {
    pub /*: *mut spinlock_t lock;,
// Protect swap_cluster_info fields
// other than list, and swap_info_struct->swap_map
// elements corresponding to the swap cluster.
//
    pub count: u16,
    pub flags: u8,
    pub order: u8,
    pub /: *mut *mut *mut atomic_long_t __rcu table; / Swap table entries, see mm/swap_table.h,
    pub /: *mut *mut *mut unsigned int extend_table; / For large swap count, protected by ci->lock,

    pub /: *mut *mut *mut swap_memcg_table memcg_table; / Swap table entries' cgroup record,

    pub zero_bitmap: *mut c_ulong,

    pub list: list_head,
}

// All on-list cluster must have a non-zero flag.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum swap_cluster_flags {
    CLUSTER_FLAG_NONE = 0, /* For temporary off-list cluster */
    CLUSTER_FLAG_FREE,
    CLUSTER_FLAG_NONFULL,
    CLUSTER_FLAG_FRAG,
// Clusters with flags above are allocatable
    CLUSTER_FLAG_USABLE = CLUSTER_FLAG_FRAG,
    CLUSTER_FLAG_FULL,
    CLUSTER_FLAG_DISCARD,
    CLUSTER_FLAG_MAX,
}

extern "C" {
    pub fn READ_ONCE(_arg: memcg->swappiness) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: vm_swappiness) -> return;
}

//
// Callers of all helpers below must ensure the entry, type, or offset is
// valid, and protect the swap device with reference count or locks.
//
extern "C" {
    pub fn __swap_type_to_info(_arg: swp_type(entry)) -> return;
}
//
// Nothing modifies swap cache in an IRQ context. All access to
// swap cache is wrapped by swap_cache_* helpers, and swap cache
// writeback is handled outside of IRQs. Swapin or swapout never
// occurs in IRQ, and neither does in-place split or replace.
//
// Besides, modifying swap cache requires synchronization with
// swap_map, which was never IRQ safe.
//
// swap_cluster_lock - Lock and return the swap cluster of given offset.
// @si: swap device the cluster belongs to.
// @offset: the swap entry offset, pointing to a valid slot.
//
// Context: The caller must ensure the offset is in the valid range and
// protect the swap device with reference count or locks.
//
extern "C" {
    pub fn __swap_cluster_lock(_arg: si, _arg: offset, _arg: false) -> return;
}
//
// swap_cluster_get_and_lock - Locks the cluster that holds a folio's entries.
// @folio: The folio.
//
// This locks and returns the swap cluster that contains a folio's swap
// entries. The swap entries of a folio are always in one single cluster.
// The folio has to be locked so its swap entries won't change and the
// cluster won't be freed.
//
// Context: Caller must ensure the folio is locked and in the swap cache.
// Return: Pointer to the swap cluster.
//
extern "C" {
    pub fn __swap_cluster_get_and_lock(_arg: folio, _arg: false) -> return;
}
//
// swap_cluster_get_and_lock_irq - Locks the cluster that holds a folio's entries.
// @folio: The folio.
//
// Same as swap_cluster_get_and_lock but also disable IRQ.
//
// Context: Caller must ensure the folio is locked and in the swap cache.
// Return: Pointer to the swap cluster.
//
extern "C" {
    pub fn __swap_cluster_get_and_lock(_arg: folio, _arg: true) -> return;
}
extern "C" {
    pub fn swap_retry_table_alloc(entry: swp_entry_t, gfp: gfp_t) -> c_int;
}
//
// Below are the core routines for doing swap for a folio.
// All helpers requires the folio to be locked, and a locked folio
// in the swap cache pins the swap entries / slots allocated to the
// folio, swap relies heavily on the swap cache and folio lock for
// synchronization.
//
// folio_alloc_swap(): the entry point for a folio to be swapped
// out. It allocates swap slots and pins the slots with swap cache.
// The slots start with a swap count of zero. The slots are pinned
// by swap cache reference which doesn't contribute to swap count.
//
// folio_dup_swap(): increases the swap count of a folio, usually
// during it gets unmapped and a swap entry is installed to replace
// it (e.g., swap entry in page table). A swap slot with swap
// count == 0 can only be increased by this helper.
//
// folio_put_swap(): does the opposite thing of folio_dup_swap().
//
extern "C" {
    pub fn folio_alloc_swap(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn folio_dup_swap(folio: *mut folio, page: *mut page) -> c_int;
}
extern "C" {
    pub fn folio_put_swap(folio: *mut folio, page: *mut page);
}
// For internal use
// linux/mm/page_io.c
extern "C" {
    pub fn sio_pool_init() -> c_int;
}
extern "C" {
    pub fn swap_read_folio(ctx: *mut swap_io_ctx, folio: *mut folio);
}
extern "C" {
    pub fn swap_read_submit(ctx: *mut swap_io_ctx);
}
extern "C" {
    pub fn swap_write_submit(ctx: *mut swap_io_ctx);
}
extern "C" {
    pub fn swap_writeout(ctx: *mut swap_io_ctx, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn __swap_writepage(ctx: *mut swap_io_ctx, folio: *mut folio);
}
// linux/mm/swap_state.c
//
// Return the swap device position of the swap entry.
//
// folio_matches_swap_entry - Check if a folio matches a given swap entry.
// @folio: The folio.
// @entry: The swap entry to check against.
//
// Context: The caller should have the folio locked to ensure it's stable
// and nothing will move it in or out of the swap cache.
// Return: true or false.
//
// All swap cache helpers below require the caller to ensure the swap entries
// used are valid and stabilize the device by any of the following ways:
// - Hold a reference by get_swap_device(): this ensures a single entry is
// valid and increases the swap device's refcount.
// - Locking a folio in the swap cache: this ensures the folio's swap entries
// are valid and pinned, also implies reference to the device.
// - Locking anything referencing the swap entry: e.g. PTL that protects
// swap entries in the page table, similar to locking swap cache folio.
// - See the comment of get_swap_device() for more complex usage.
//
extern "C" {
    pub fn swap_cache_has_folio(entry: swp_entry_t) -> bool;
}
extern "C" {
    pub fn swap_cache_del_folio(folio: *mut folio);
}
// Below helpers require the caller to lock and pass in the swap cluster.
extern "C" {
    pub fn show_swap_cache_info();
}
extern "C" {
    pub fn swapcache_clear(si: *mut swap_info_struct, entry: swp_entry_t, nr: c_int);
}

