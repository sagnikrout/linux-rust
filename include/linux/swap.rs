//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/swap.h
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

pub const SWAP_FLAG_PREFER: c_uint = 0x8000	/* set if swap priority specified */;
pub const SWAP_FLAG_PRIO_MASK: c_uint = 0x7fff;
pub const SWAP_FLAG_DISCARD: c_uint = 0x10000 /* enable discard for swap */;
pub const SWAP_FLAG_DISCARD_ONCE: c_uint = 0x20000 /* discard swap area at swapon-time */;
pub const SWAP_FLAG_DISCARD_PAGES: c_uint = 0x40000 /* discard page-clusters after use */;

//
// MAX_SWAPFILES defines the maximum number of swaptypes: things which can
// be swapped to.  The swap type and the offset into that swap type are
// encoded into pte's and into pgoff_t's in the swapcache.  Using five bits
// for the type means that the maximum number of swapcache pages is 27 bits
// on 32-bit-pgoff_t architectures.  And that assumes that the architecture packs
// the type/offset into the pte as 5/27 as well.
//
pub const MAX_SWAPFILES_SHIFT: c_int = 5;
//
// Use some of the swap files numbers for other purposes. This
// is a convenient way to hook into the VM to trigger special
// actions on faults.
//
// PTE markers are used to persist information onto PTEs that otherwise
// should be a none pte.  As its name "PTE" hints, it should only be
// applied to the leaves of pgtables.
//
pub const SWP_PTE_MARKER_NUM: c_int = 1;

//
// Unaddressable device memory support. See include/linux/hmm.h and
// Documentation/mm/hmm.rst. Short description is we need struct pages for
// device memory that is unaddressable (inaccessible) by CPU, so that we can
// migrate part of a process memory to device memory.
//
// When a page is migrated from CPU to device, we set the CPU page table entry
// to a special SWP_DEVICE_{READ|WRITE} entry.
//
// When a page is mapped by the device for exclusive access we set the CPU page
// table entries to a special SWP_DEVICE_EXCLUSIVE entry.
//

pub const SWP_DEVICE_NUM: c_int = 3;

pub const SWP_DEVICE_NUM: c_int = 0;

//
// Page migration support.
//
// SWP_MIGRATION_READ_EXCLUSIVE is only applicable to anonymous pages and
// indicates that the referenced (part of) an anonymous page is exclusive to
// a single process. For SWP_MIGRATION_WRITE, that information is implicit:
// (part of) an anonymous page that are mapped writable are exclusive to a
// single process.
//

pub const SWP_MIGRATION_NUM: c_int = 3;

pub const SWP_MIGRATION_NUM: c_int = 0;

//
// Handling of hardware poisoned pages with memory corruption.
//

pub const SWP_HWPOISON_NUM: c_int = 1;

pub const SWP_HWPOISON_NUM: c_int = 0;

//
// Magic header for a swap area. The first part of the union is
// what the swap magic looks like for the old (limited to 128MB)
// swap area format, the second part of the union adds - in the
// old reserved area - some extra information. Note that the first
// kilobyte is reserved for boot loader or disk label stuff...
//
// Having the magic at the end of the PAGE_SIZE makes detecting swap
// areas somewhat tricky on machines that support multiple page sizes.
// For 2.5 we'll probably want to move the magic to just beyond the
// bootbits...
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union swap_header {
    pub 10]: char reserved[PAGE_SIZE -,
    pub /: *mut *mut char magic[10]; / SWAP-SPACE or SWAPSPACE2,
    pub magic: },
    pub /: *mut *mut char bootbits[1024]; / Space for disklabel etc.,
    pub version: __u32,
    pub last_page: __u32,
    pub nr_badpages: __u32,
    pub sws_uuid: [c_uchar; 16],
    pub sws_volume: [c_uchar; 16],
    pub padding: [__u32; 117],
    pub badpages: [__u32; 1],
    pub info: },
}

//
// current->reclaim_state points to one of these when a task is running
// memory reclaim
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reclaim_state {
// pages reclaimed outside of LRU-based reclaim
    pub reclaimed: c_ulong,

// per-thread mm walk data
    pub mm_walk: *mut lru_gen_mm_walk,

}

//
// mm_account_reclaimed_pages(): account reclaimed pages outside of LRU-based
// reclaim
// @pages: number of pages reclaimed
//
// If the current process is undergoing a reclaim operation, increment the
// number of reclaimed pages by @pages.
//

//
// A swap extent maps a range of a swapfile's PAGE_SIZE pages onto a range of
// disk blocks.  A rbtree of swap extents maps the entire swapfile (Where the
// term `swapfile' refers to either a blockdevice or an IS_REG file). Apart
// from setup, they're handled identically.
//
// We always assume that blocks are of size PAGE_SIZE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_extent {
    pub rb_node: rb_node,
    pub start_page: pgoff_t,
    pub nr_pages: pgoff_t,
    pub start_block: sector_t,
}

//
// Max bad pages in the new format..
//

// add others here before...

//
// The first page in the swap file is the swap header, which is always marked
// bad to prevent it from being allocated as an entry. This also prevents the
// cluster to which it belongs being marked free. Therefore 0 is safe to use as
// a sentinel to indicate an entry is not valid.
//
pub const SWAP_ENTRY_INVALID: c_int = 0;

pub const SWAP_NR_ORDERS: c_int = 1;

//
// We keep using same cluster for rotational device so IO will be sequential.
// The purpose is to optimize SWAP throughput on these device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_sequential_cluster {
    pub /: *mut *mut unsigned int next[SWAP_NR_ORDERS]; / Likely next allocation offset,
}

//
// The in-memory structure used to track swap areas.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swap_info_struct {
    pub /: *mut *mut percpu_ref users; / indicate and keep swap device valid.,
    pub /: *mut *mut unsigned long flags; / SWP_USED etc: see above,
    pub /: *mut *mut signed short prio; / swap priority of this type,
    pub /: *mut *mut plist_node list; / entry in swap_active_head,
    pub /: *mut *mut signed char type; / strange name for an index,
    pub /: *mut *mut unsigned int max; / size of this swap device,
    pub /: *mut *mut *mut swap_cluster_info cluster_info; / cluster info. Only for SSD,
    pub /: *mut *mut list_head free_clusters; / free clusters list,
    pub /: *mut *mut list_head full_clusters; / full clusters list,
    pub nonfull_clusters: [list_head; SWAP_NR_ORDERS],
// list of cluster that contains at least one free slot
    pub frag_clusters: [list_head; SWAP_NR_ORDERS],
// list of cluster that are fragmented or contented
    pub /: *mut *mut unsigned int pages; / total of usable pages of swap,
    pub /: *mut *mut atomic_long_t inuse_pages; / number of those currently in use,
    pub /: *mut *mut *mut swap_sequential_cluster global_cluster; / Use one global cluster for rotating device,
    pub /: *mut *mut spinlock_t global_cluster_lock; / Serialize usage of global cluster,
    pub /: *mut *mut rb_root swap_extent_root;/ root of the swap extent rbtree,
    pub /: *mut *mut *mut block_device bdev; / swap device or bdev of swap file,
    pub /: *mut *mut *mut file swap_file; / seldom referenced,
    pub /: *mut *mut completion comp; / seldom referenced,
    pub /*: *mut spinlock_t lock;,
// protect map scan related fields like
// inuse_pages and all cluster lists.
// Other fields are only changed
// at swapon/swapoff, so are protected
// by swap_lock. changing flags need
// hold this lock and swap_lock. If
// both locks need hold, hold swap_lock
// first.
//
    pub /: *mut *mut work_discard_work; / discard worker,
    pub /: *mut *mut work_reclaim_work; / reclaim worker,
    pub /: *mut *mut list_head discard_clusters; / discard clusters list,
    pub /: *mut *mut plist_node avail_list; / entry in swap_avail_head,
    pub ops: *const swap_ops,
}

// linux/mm/page_alloc.c
// Definition of global_zone_page_state not available yet

// linux/mm/folio.c
extern "C" {
    pub fn folio_add_lru(folio: *mut folio);
}
extern "C" {
    pub fn folio_mark_accessed(folio: *mut folio);
}
extern "C" {
    pub fn lru_add_drain_all();
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lru_cache_drained {
    LRU_CACHE_NOT_DRAINED,
    LRU_CACHE_DRAINED,
    LRU_CACHE_DRAINED_ALL,
}

// linux/mm/folio-compat.c
extern "C" {
    pub fn mark_page_accessed(page: *mut page);
}
extern "C" {
    pub fn atomic_read(_arg: &lru_disable_count) -> return;
}
extern "C" {
    pub fn shrink_all_memory(nr_pages: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn remove_mapping(mapping: *mut address_space, folio: *mut folio) -> c_long;
}

extern "C" {
    pub fn reclaim_register_node(node: *mut node) -> c_int;
}
extern "C" {
    pub fn reclaim_unregister_node(node: *mut node);
}

extern "C" {
    pub fn check_move_unevictable_folios(fbatch: *mut folio_batch);
}
extern "C" {
    pub fn kswapd_run(nid: c_int) -> void __meminit;
}
extern "C" {
    pub fn kswapd_stop(nid: c_int) -> void __meminit;
}

extern "C" {
    pub fn global_node_page_state(_arg: NR_SWAPCACHE) -> return;
}
extern "C" {
    pub fn free_swap_cache(folio: *mut folio);
}
extern "C" {
    pub fn free_folio_and_swap_cache(folio: *mut folio);
}
extern "C" {
    pub fn free_pages_and_swap_cache(: *mut encoded_page, _arg: c_int);
}
// linux/mm/swapfile.c
// Swap 50% full? Release swapcache more aggressively..
extern "C" {
    pub fn atomic_long_read(_arg: &nr_swap_pages) -> return;
}
extern "C" {
    pub fn si_swapinfo(: *mut sysinfo);
}
extern "C" {
    pub fn pin_hibernation_swap_type(device: dev_t, offset: sector_t) -> c_int;
}
extern "C" {
    pub fn unpin_hibernation_swap_type(type: c_int);
}
extern "C" {
    pub fn find_hibernation_swap_type(device: dev_t, offset: sector_t) -> c_int;
}
extern "C" {
    pub fn find_first_swap(device: *mut dev_t) -> c_int;
}
extern "C" {
    pub fn count_swap_pages(_arg: c_int, _arg: c_int) -> c_uint;
}
extern "C" {
    pub fn swapdev_block(_arg: c_int, _arg: pgoff_t) -> sector_t;
}
extern "C" {
    pub fn __swap_count(entry: swp_entry_t) -> c_int;
}
extern "C" {
    pub fn swap_entry_swapped(si: *mut swap_info_struct, entry: swp_entry_t) -> bool;
}
extern "C" {
    pub fn swp_swapcount(entry: swp_entry_t) -> c_int;
}
extern "C" {
    pub fn swap_folio_sector(folio: *mut folio) -> sector_t;
}
//
// If there is an existing swap slot reference (swap entry) and the caller
// guarantees that there is no race modification of it (e.g., PTL
// protecting the swap entry in page table; shmem's cmpxchg protects t
// he swap entry in shmem mapping), these two helpers below can be used
// to put/dup the entries directly.
//
// All entries must be allocated by folio_alloc_swap(). And they must have
// a swap count > 1. See comments of folio_*_swap helpers for more info.
//
extern "C" {
    pub fn swap_dup_entry_direct(entry: swp_entry_t) -> c_int;
}
extern "C" {
    pub fn swap_put_entries_direct(entry: swp_entry_t, nr: c_int);
}
//
// folio_free_swap tries to free the swap entries pinned by a swap cache
// folio, it has to be here to be called by other components.
//
extern "C" {
    pub fn folio_free_swap(folio: *mut folio) -> bool;
}
// Allocate / free (hibernation) exclusive entries
extern "C" {
    pub fn swap_alloc_hibernation_slot(type: c_int) -> swp_entry_t;
}
extern "C" {
    pub fn swap_free_hibernation_slot(entry: swp_entry_t);
}

pub const vm_swap_full(): c_int = 0;

extern "C" {
    pub fn lru_reparent_memcg(memcg: *mut mem_cgroup, parent: *mut mem_cgroup, nid: c_int);
}

extern "C" {
    pub fn __folio_throttle_swaprate(folio: *mut folio, gfp: gfp_t);
}

extern "C" {
    pub fn __mem_cgroup_try_charge_swap(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn __mem_cgroup_try_charge_swap(_arg: folio) -> return;
}
extern "C" {
    pub fn __mem_cgroup_uncharge_swap(id: c_ushort, nr_pages: c_uint);
}
extern "C" {
    pub fn mem_cgroup_get_nr_swap_pages(memcg: *mut mem_cgroup) -> c_long;
}
extern "C" {
    pub fn mem_cgroup_swap_full(folio: *mut folio) -> bool;
}

extern "C" {
    pub fn get_nr_swap_pages() -> return;
}
extern "C" {
    pub fn vm_swap_full() -> return;
}

// for_each_managed_zone_pgdat - helper macro to iterate over all managed zones in a pgdat up to
// and including the specified highidx
// @zone: The current zone in the iterator
// @pgdat: The pgdat which node_zones are being iterated
// @idx: The index variable
// @highidx: The index of the highest zone to return
//
// This macro iterates through all managed zones up to and including the specified highidx.
// The zone iterator enters an invalid state after macro call and must be reinitialized
// before it can be used again.
//

