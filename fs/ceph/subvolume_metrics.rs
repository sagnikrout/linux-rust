//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ceph/subvolume_metrics.h
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
// struct ceph_subvol_metric_snapshot - Point-in-time snapshot of subvolume metrics
// @subvolume_id: Subvolume identifier (inode number of subvolume root)
// @read_ops: Number of read operations since last snapshot
// @write_ops: Number of write operations since last snapshot
// @read_bytes: Total bytes read since last snapshot
// @write_bytes: Total bytes written since last snapshot
// @read_latency_us: Sum of read latencies in microseconds (for avg calculation)
// @write_latency_us: Sum of write latencies in microseconds (for avg calculation)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_subvol_metric_snapshot {
    pub subvolume_id: u64,
    pub read_ops: u64,
    pub write_ops: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_latency_us: u64,
    pub write_latency_us: u64,
}

//
// struct ceph_subvolume_metrics_tracker - Tracks per-subvolume I/O metrics
// @lock: Protects @tree and @nr_entries during concurrent access
// @tree: Red-black tree of per-subvolume entries, keyed by subvolume_id
// @nr_entries: Number of entries currently in @tree
// @enabled: Whether collection is enabled (requires MDS feature support)
// @snapshot_attempts: Debug counter: total ceph_subvolume_metrics_snapshot() calls
// @snapshot_empty: Debug counter: snapshots that found no data to report
// @snapshot_failures: Debug counter: snapshots that failed to allocate memory
// @record_calls: Debug counter: total ceph_subvolume_metrics_record() calls
// @record_disabled: Debug counter: record calls skipped because disabled
// @record_no_subvol: Debug counter: record calls skipped (no subvolume_id)
// @total_read_ops: Cumulative read ops across all snapshots (never reset)
// @total_read_bytes: Cumulative bytes read across all snapshots (never reset)
// @total_write_ops: Cumulative write ops across all snapshots (never reset)
// @total_write_bytes: Cumulative bytes written across all snapshots (never reset)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_subvolume_metrics_tracker {
    pub lock: spinlock_t,
    pub tree: rb_root_cached,
    pub nr_entries: u32,
    pub enabled: bool,
    pub snapshot_attempts: core::sync::atomic::AtomicI64,
    pub snapshot_empty: core::sync::atomic::AtomicI64,
    pub snapshot_failures: core::sync::atomic::AtomicI64,
    pub record_calls: core::sync::atomic::AtomicI64,
    pub record_disabled: core::sync::atomic::AtomicI64,
    pub record_no_subvol: core::sync::atomic::AtomicI64,
    pub total_read_ops: core::sync::atomic::AtomicI64,
    pub total_read_bytes: core::sync::atomic::AtomicI64,
    pub total_write_ops: core::sync::atomic::AtomicI64,
    pub total_write_bytes: core::sync::atomic::AtomicI64,
}

extern "C" {
    pub fn ceph_subvolume_metrics_init(tracker: *mut ceph_subvolume_metrics_tracker);
}
extern "C" {
    pub fn ceph_subvolume_metrics_destroy(tracker: *mut ceph_subvolume_metrics_tracker);
}
extern "C" {
    pub fn ceph_subvolume_metrics_free_snapshot(snapshot: *mut ceph_subvol_metric_snapshot);
}
extern "C" {
    pub fn READ_ONCE(_arg: tracker->enabled) -> return;
}
extern "C" {
    pub fn ceph_subvolume_metrics_cache_init() -> int __init;
}
extern "C" {
    pub fn ceph_subvolume_metrics_cache_destroy();
}
