//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_mount.h
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
// Copyright (c) 2000-2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// dynamic preallocation free space thresholds, 5% down to 1%
//
// Error Configuration
//
// Error classes define the subsystem the configuration belongs to.
// Error numbers define the errors that are configurable.
//

//
// Although retry_timeout is in jiffies which is normally an unsigned long,
// we limit the retry timeout to 86400 seconds, or one day.  So even a
// signed 32-bit long is sufficient for a HZ value up to 24855.  Making it
// signed lets us store the special "-1" value, meaning retry forever.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_error_cfg {
    pub kobj: xfs_kobj,
    pub max_retries: c_int,
    pub /: *mut *mut long retry_timeout; / in jiffies, -1 = infinite,
}

//
// Per-cpu deferred inode inactivation GC lists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_inodegc {
    pub mp: *mut xfs_mount,
    pub list: llist_head,
    pub work: delayed_work,
    pub error: c_int,
// approximate count of inodes in the list
    pub items: c_uint,
    pub shrinker_hits: c_uint,
    pub cpu: c_uint,
}

//
// Container for each type of groups, used to look up individual groups and
// describes the geometry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_groups {
    pub xa: xarray,
//
// Maximum capacity of the group in FSBs.
//
// Each group is laid out densely in the daddr space.  For the
// degenerate case of a pre-rtgroups filesystem, the incore rtgroup
// pretends to have a zero-block and zero-blklog rtgroup.
//
    pub blocks: u32,
//
// Log(2) of the logical size of each group.
//
// Compared to the blocks field above this is rounded up to the next
// power of two, and thus lays out the xfs_fsblock_t/xfs_rtblock_t
// space sparsely with a hole from blocks to (1 << blklog) at the end
// of each group.
//
    pub blklog: u8,
//
// Zoned devices can have gaps beyond the usable capacity of a zone and
// the end in the LBA/daddr address space.  In other words, the hardware
// equivalent to the RT groups already takes care of the power of 2
// alignment for us.  In this case the sparse FSB/RTB address space maps
// 1:1 to the device address space.
//
    pub has_daddr_gaps: bool,
//
// Mask to extract the group-relative block number from a FSB.
// For a pre-rtgroups filesystem we pretend to have one very large
// rtgroup, so this mask must be 64-bit.
//
    pub blkmask: u64,
//
// Start of the first group in the device.  This is used to support a
// RT device following the data device on the same block device for
// SMR hard drives.
//
    pub start_fsb: xfs_fsblock_t,
//
// Maximum length of an atomic write for files stored in this
// collection of allocation groups, in fsblocks.
//
    pub awu_max: xfs_extlen_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_freecounter {
// free blocks for general use:
    pub count: percpu_counter,
// total reserved blocks:
    pub res_total: u64,
// available reserved blocks:
    pub res_avail: u64,
// reserved blks @ remount,ro:
    pub res_saved: u64,
}

//
// The struct xfsmount layout is optimised to separate read-mostly variables
// from variables that are frequently modified. We put the read-mostly variables
// first, then place all the other variables at the end.
//
// Typically, read-mostly variables are those that are set at mount time and
// never changed again, or only change rarely as a result of things like sysfs
// knobs being tweaked.
//
// number of rt extents per rt bitmap block if rtgroups enabled
// low free space thresholds
// max_atomic_write mount option value
//
// Bitsets of per-fs metadata that have been checked and/or are sick.
// Callers must hold m_sb_lock to access these two fields.
//
// Bitsets of rt metadata that have been checked and/or are sick.
// Callers must hold m_sb_lock to access this field.
//
// End of read-mostly variables. Frequently written variables and locks
// should be placed below this comment from now on. The first variable
// here is marked as cacheline aligned so they it is separated from
// the read-mostly variables.
//
// Count of data device blocks reserved for delayed allocations,
// including indlen blocks.  Does not include allocated CoW staging
// extents or anything related to the rt device.
//
// RT version of the above.
//
// Global count of allocation btree blocks in use across all AGs. Only
// used when perag reservation is enabled. Helps prevent block
// reservation from attempting to reserve allocation btree blocks.
//

// Memory shrinker to throttle and reprioritize inodegc
//
// Workqueue item so that we can coalesce multiple inode flush attempts
// into a single flush.
//
// Generation of the filesysyem layout.  This is incremented by each
// growfs, and used by the pNFS server to ensure the client updates
// its view of the block device once it gets a layout that might
// reference the newly added blocks.  Does not need to be persistent
// as long as we only allow file system size increments, but if we
// ever support shrinks it would have to be persisted in addition
// to various other kinds of pain inflicted on the pNFS server.
//

//
// Frequency with which errors are injected.  Replaces xfs_etest; the
// value stored in here is the inverse of the frequency with which the
// error triggers.  1 = always, 2 = half the time, etc.
//

// cpus that have inodes queued for inactivation
// Hook to feed dirent updates to an active online repair.
// Private data referring to a health monitor object.
// Index of uuid record in the uuid xarray.
//
// Old io_pages/ra_pages valued in the main bdev BDI, and our initial
// calculated values.
//

//
// Flags for m_features.
//
// These are all the active features in the filesystem, regardless of how
// they are configured.
//

// Mount features

// I/O size in stat()

// Some features can be added dynamically so they need a set wrapper, too.

// Superblock features
// all metadir file systems also allow rtgroups
extern "C" {
    pub fn xfs_has_metadir(_arg: mp) -> return;
}
// all rtgroups filesystems with an rt section have an rtsb
extern "C" {
    pub fn xfs_has_reflink(_arg: mp) -> return;
}
//
// Some features are always on for v5 file systems, allow the compiler to
// eliminiate dead code when building without v4 support.
//

//
// Mount features
//
// These do not change dynamically - features that can come and go, such as 32
// bit inodes and read-only state, are kept as operational state rather than
// features.
//
// Operational mount state flags
//
// Use these with atomic bit ops only!
//

//
// If set, inactivation worker threads will be scheduled to process queued
// inodegc work.  If not, queued inodes remain in memory waiting to be
// processed.
//
pub const XFS_OPSTATE_INODEGC_ENABLED: c_int = 5;
//
// If set, background speculative prealloc gc worker threads will be scheduled
// to process queued blockgc work.  If not, inodes retain their preallocations
// until explicitly deleted.
//
pub const XFS_OPSTATE_BLOCKGC_ENABLED: c_int = 6;
// Kernel has logged a warning about shrink being used on this fs.
pub const XFS_OPSTATE_WARNED_SHRINK: c_int = 9;
// Kernel has logged a warning about logged xattr updates being used.
pub const XFS_OPSTATE_WARNED_LARP: c_int = 10;
// Mount time quotacheck is running
pub const XFS_OPSTATE_QUOTACHECK_RUNNING: c_int = 11;
// Do we want to clear log incompat flags?
pub const XFS_OPSTATE_UNSET_LOG_INCOMPAT: c_int = 12;
// Filesystem can use logged extended attributes
pub const XFS_OPSTATE_USE_LARP: c_int = 13;
// Kernel has logged a warning about blocksize > pagesize on this fs.
pub const XFS_OPSTATE_WARNED_LBS: c_int = 14;
// Kernel has logged a warning about metadata dirs being used on this fs.
pub const XFS_OPSTATE_WARNED_METADIR: c_int = 17;
// Filesystem should use qflags to determine quotaon status
pub const XFS_OPSTATE_RESUMING_QUOTAON: c_int = 18;
// (Zoned) GC is in progress
pub const XFS_OPSTATE_ZONEGC_RUNNING: c_int = 20;

//
// Max and min values for mount-option defined I/O
// preallocation sizes.
//

//
// Flags for xfs_mountfs
//
pub const XFS_MFSI_QUIET: c_uint = 0x40	/* Be silent if mount errors found */;
extern "C" {
    pub fn xfs_uuid_table_free();
}
extern "C" {
    pub fn xfs_mountfs(mp: *mut xfs_mount_t) -> c_int;
}
extern "C" {
    pub fn xfs_unmountfs(: *mut xfs_mount_t);
}
//
// Deltas for the block count can vary from 1 to very large, but lock contention
// only occurs on frequent small block count updates such as in the delayed
// allocation path for buffered writes (page a time updates). Hence we set
// a large batch count (1024) to minimise global counter updates except when
// we get near to ENOSPC and we have to be very accurate with our updates.
//
pub const XFS_FDBLOCKS_BATCH: c_int = 1024;
//
// Sum up the freecount, but never return negative values.
//
extern "C" {
    pub fn percpu_counter_sum_positive(_arg: &mp->m_free[ctr].count) -> return;
}
//
// Same as above, but does return negative values.  Mostly useful for
// special cases like repair and tracing.
//
extern "C" {
    pub fn percpu_counter_sum(_arg: &mp->m_free[ctr].count) -> return;
}
//
// This just provides and estimate without the cpu-local updates, use
// xfs_sum_freecounter for the exact value.
//
extern "C" {
    pub fn percpu_counter_read_positive(_arg: &mp->m_free[ctr].count) -> return;
}
extern "C" {
    pub fn __percpu_counter_compare(_arg: &mp->m_free[ctr].count, _arg: rhs, _arg: batch) -> return;
}
extern "C" {
    pub fn xfs_dec_freecounter(_arg: mp, _arg: XC_FREE_BLOCKS, _arg: delta, _arg: reserved) -> return;
}
extern "C" {
    pub fn xfs_dec_freecounter(_arg: mp, _arg: XC_FREE_RTEXTENTS, _arg: delta, _arg: false) -> return;
}
extern "C" {
    pub fn xfs_readsb(: *mut xfs_mount_t, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn xfs_freesb(: *mut xfs_mount_t);
}
extern "C" {
    pub fn xfs_fs_writable(mp: *mut xfs_mount, level: c_int) -> bool;
}
extern "C" {
    pub fn xfs_sb_validate_fsb_count(: *mut xfs_sb, _arg: u64) -> c_int;
}
extern "C" {
    pub fn xfs_dev_is_read_only(: *mut xfs_mount, : *mut c_char) -> c_int;
}
extern "C" {
    pub fn xfs_set_low_space_thresholds(: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_force_summary_recalc(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_add_incompat_log_feature(mp: *mut xfs_mount, feature: u32) -> c_int;
}
extern "C" {
    pub fn xfs_clear_incompat_log_features(mp: *mut xfs_mount) -> bool;
}
