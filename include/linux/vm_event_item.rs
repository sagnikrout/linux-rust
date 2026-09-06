//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vm_event_item.h
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

// Macro flag: #define VM_EVENT_ITEM_H_INCLUDED

// Macro flag: #define DMA_ZONE(xx)

// Macro flag: #define DMA32_ZONE(xx)

// Macro flag: #define HIGHMEM_ZONE(xx)

// Macro flag: #define DEVICE_ZONE(xx)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vm_event_item {
    FOR_ALL_ZONES(PGALLOC)
    FOR_ALL_ZONES(ALLOCSTALL)
    FOR_ALL_ZONES(PGSCAN_SKIP)
    PGFREE, PGACTIVATE, PGDEACTIVATE, PGLAZYFREE,
    PGFAULT, PGMAJFAULT,
    PGLAZYFREED,
    PGREUSE,
    PGSCAN_DIRECT_THROTTLE,

    PGSCAN_ZONE_RECLAIM_SUCCESS,
    PGSCAN_ZONE_RECLAIM_FAILED,

    PGINODESTEAL, SLABS_SCANNED, KSWAPD_INODESTEAL,
    KSWAPD_LOW_WMARK_HIT_QUICKLY, KSWAPD_HIGH_WMARK_HIT_QUICKLY,
    PAGEOUTRUN, PGROTATED,
    DROP_PAGECACHE, DROP_SLAB,
    OOM_KILL,

    NUMA_PTE_UPDATES,
    NUMA_HUGE_PTE_UPDATES,
    NUMA_HINT_FAULTS,
    NUMA_HINT_FAULTS_LOCAL,
    NUMA_PAGE_MIGRATE,

    PGMIGRATE_SUCCESS, PGMIGRATE_FAIL,
    THP_MIGRATION_SUCCESS,
    THP_MIGRATION_FAIL,
    THP_MIGRATION_SPLIT,

    COMPACTMIGRATE_SCANNED, COMPACTFREE_SCANNED,
    COMPACTISOLATED,
    COMPACTSTALL, COMPACTFAIL, COMPACTSUCCESS,
    KCOMPACTD_WAKE,
    KCOMPACTD_MIGRATE_SCANNED, KCOMPACTD_FREE_SCANNED,

    HTLB_BUDDY_PGALLOC, HTLB_BUDDY_PGALLOC_FAIL,

    CMA_ALLOC_SUCCESS,
    CMA_ALLOC_FAIL,

    UNEVICTABLE_PGCULLED,	/* culled to noreclaim list */
    UNEVICTABLE_PGSCANNED,	/* scanned for reclaimability */
    UNEVICTABLE_PGRESCUED,	/* rescued from noreclaim list */
    UNEVICTABLE_PGMLOCKED,
    UNEVICTABLE_PGMUNLOCKED,
    UNEVICTABLE_PGCLEARED,	/* on COW, page truncate */
    UNEVICTABLE_PGSTRANDED,	/* unable to isolate on unlock */

    THP_FAULT_ALLOC,
    THP_FAULT_FALLBACK,
    THP_FAULT_FALLBACK_CHARGE,
    THP_COLLAPSE_ALLOC,
    THP_COLLAPSE_ALLOC_FAILED,
    THP_FILE_ALLOC,
    THP_FILE_FALLBACK,
    THP_FILE_FALLBACK_CHARGE,
    THP_FILE_MAPPED,
    THP_SPLIT_PAGE,
    THP_SPLIT_PAGE_FAILED,
    THP_DEFERRED_SPLIT_PAGE,
    THP_UNDERUSED_SPLIT_PAGE,
    THP_SPLIT_PMD,
    THP_SCAN_EXCEED_NONE_PTE,
    THP_SCAN_EXCEED_SWAP_PTE,
    THP_SCAN_EXCEED_SHARED_PTE,

    THP_SPLIT_PUD,

    THP_ZERO_PAGE_ALLOC,
    THP_ZERO_PAGE_ALLOC_FAILED,
    THP_SWPOUT,
    THP_SWPOUT_FALLBACK,

    BALLOON_INFLATE,
    BALLOON_DEFLATE,

    BALLOON_MIGRATE,

    NR_TLB_REMOTE_FLUSH,	/* cpu tried to flush others' tlbs */
    NR_TLB_REMOTE_FLUSH_RECEIVED,/* cpu received ipi for flush */
    NR_TLB_LOCAL_FLUSH_ALL,
    NR_TLB_LOCAL_FLUSH_ONE,

    SWAP_RA,
    SWAP_RA_HIT,
    SWPIN_ZERO,
    SWPOUT_ZERO,

    KSM_SWPIN_COPY,

    COW_KSM,

    ZSWPIN,
    ZSWPOUT,
    ZSWPWB,

    DIRECT_MAP_LEVEL2_SPLIT,
    DIRECT_MAP_LEVEL3_SPLIT,
    DIRECT_MAP_LEVEL2_COLLAPSE,
    DIRECT_MAP_LEVEL3_COLLAPSE,

    VMA_LOCK_SUCCESS,
    VMA_LOCK_ABORT,
    VMA_LOCK_RETRY,
    VMA_LOCK_MISS,

    KSTACK_1K,

    KSTACK_2K,

    KSTACK_4K,

    KSTACK_8K,

    KSTACK_16K,

    KSTACK_32K,

    KSTACK_64K,

    KSTACK_REST,

    NRSWPIN,
    NRSWPOUT,

    NR_VM_EVENT_ITEMS
}

