//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/compaction.h
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
// Determines how hard direct compaction should try to succeed.
// Lower value means higher priority, analogically to reclaim priority.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum compact_priority {
    COMPACT_PRIO_SYNC_FULL,
    MIN_COMPACT_PRIORITY = COMPACT_PRIO_SYNC_FULL,
    COMPACT_PRIO_SYNC_LIGHT,
    MIN_COMPACT_COSTLY_PRIORITY = COMPACT_PRIO_SYNC_LIGHT,
    DEF_COMPACT_PRIORITY = COMPACT_PRIO_SYNC_LIGHT,
    COMPACT_PRIO_ASYNC,
    INIT_COMPACT_PRIORITY = COMPACT_PRIO_ASYNC
}

// Return values for compact_zone() and try_to_compact_pages()
// When adding new states, please adjust include/trace/events/compaction.h
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum compact_result {
// For more detailed tracepoint output - internal to compaction
    COMPACT_NOT_SUITABLE_ZONE,
//
// compaction didn't start as it was not possible or direct reclaim
// was more suitable
//
    COMPACT_SKIPPED,
// compaction didn't start as it was deferred due to past failures
    COMPACT_DEFERRED,

// For more detailed tracepoint output - internal to compaction
    COMPACT_NO_SUITABLE_PAGE,
// compaction should continue to another pageblock
    COMPACT_CONTINUE,

//
// The full zone was compacted scanned but wasn't successful to compact
// suitable pages.
//
    COMPACT_COMPLETE,
//
// direct compaction has scanned part of the zone but wasn't successful
// to compact suitable pages.
//
    COMPACT_PARTIAL_SKIPPED,

// compaction terminated prematurely due to lock contentions
    COMPACT_CONTENDED,

//
// direct compaction terminated after concluding that the allocation
// should now succeed
//
    COMPACT_SUCCESS,
}

//
// Number of free order-0 pages that should be available above given watermark
// to make sure compaction has reasonable chance of not running out of free
// pages that it needs to isolate as migration target during its work.
//
// Although all the isolations for migration are temporary, compaction
// free scanner may have up to 1 << order pages on its list and then
// try to split an (order - 1) free page. At that point, a gap of
// 1 << order might not be enough, so it's safer to require twice that
// amount. Note that the number of pages on the list is also
// effectively limited by COMPACT_CLUSTER_MAX, as that's the maximum
// that the migrate scanner can have isolated on migrate list, and free
// scanner is only invoked when the number of isolated free pages is
// lower than that.
//
extern "C" {
    pub fn min(order: 2UL <<, _arg: COMPACT_CLUSTER_MAX) -> return;
}

extern "C" {
    pub fn extfrag_for_order(zone: *mut zone, order: c_uint) -> c_uint;
}
extern "C" {
    pub fn fragmentation_index(zone: *mut zone, order: c_uint) -> c_int;
}
extern "C" {
    pub fn reset_isolation_suitable(pgdat: *mut pg_data_t);
}
extern "C" {
    pub fn kcompactd_run(nid: c_int) -> void __meminit;
}
extern "C" {
    pub fn kcompactd_stop(nid: c_int) -> void __meminit;
}
extern "C" {
    pub fn wakeup_kcompactd(pgdat: *mut pg_data_t, order: c_int, highest_zoneidx: c_int);
}

extern "C" {
    pub fn compaction_register_node(node: *mut node) -> c_int;
}
extern "C" {
    pub fn compaction_unregister_node(node: *mut node);
}

