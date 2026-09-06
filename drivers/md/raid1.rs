//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/raid1.h
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
// each barrier unit size is 64MB fow now
// note: it must be larger than RESYNC_DEPTH
//
pub const BARRIER_UNIT_SECTOR_BITS: c_int = 17;

//
// In struct r1conf, the following members are related to I/O barrier
// buckets,
// atomic_t	*nr_pending;
// atomic_t	*nr_waiting;
// atomic_t	*nr_queued;
// atomic_t	*barrier;
// Each of them points to array of atomic_t variables, each array is
// designed to have BARRIER_BUCKETS_NR elements and occupy a single
// memory page. The data width of atomic_t variables is 4 bytes, equal
// to 1<<(ilog2(sizeof(atomic_t))), BARRIER_BUCKETS_NR_BITS is defined
// as (PAGE_SHIFT - ilog2(sizeof(int))) to make sure an array of
// atomic_t variables with BARRIER_BUCKETS_NR elements just exactly
// occupies a single memory page.
//

// Note: raid1_info.rdev can be set to NULL asynchronously by raid1_remove_disk.
// There are three safe ways to access raid1_info.rdev.
// 1/ when holding mddev->reconfig_mutex
// 2/ when resync/recovery is known to be happening - i.e. in code that is
// called as part of performing resync/recovery.
// 3/ while holding rcu_read_lock(), use rcu_dereference to get the pointer
// and if it is non-NULL, increment rdev->nr_pending before dropping the
// RCU lock.
// When .rdev is set to NULL, the nr_pending count checked again and if it has
// been incremented, the pointer is put back in .rdev.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid1_info {
    pub rdev: *mut md_rdev,
    pub head_position: sector_t,
// When choose the best device for a read (read_balance())
// we try to keep sequential reads one the same device
//
    pub next_seq_sect: sector_t,
    pub seq_start: sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r1conf {
    pub mddev: *mut mddev,
    pub to: *mut *mut *mut raid1_info mirrors; / twice 'raid_disks',
// allow for replacements.
//
    pub raid_disks: c_int,
    pub nonrot_disks: c_int,
    pub device_lock: spinlock_t,
// list of 'struct r1bio' that need to be processed by raid1d,
// whether to retry a read, writeout a resync or recovery
// block, or anything else.
//
    pub retry_list: list_head,
// A separate list of r1bio which just need raid_end_bio_io called.
// This mustn't happen for writes which had any errors if the superblock
// needs to be written.
//
    pub bio_end_io_list: list_head,
// queue pending writes to be submitted on unplug
    pub pending_bio_list: bio_list,
// for use when syncing mirrors:
// We don't allow both normal IO and resync/recovery IO at
// the same time - resync/recovery can only happen when there
// is no other IO.  So when either is active, the other has to wait.
// See more details description in raid1.c near raise_barrier().
//
    pub wait_barrier: wait_queue_head_t,
    pub resync_lock: spinlock_t,
    pub nr_sync_pending: core::sync::atomic::AtomicI32,
    pub nr_pending: *mut core::sync::atomic::AtomicI32,
    pub nr_waiting: *mut core::sync::atomic::AtomicI32,
    pub nr_queued: *mut core::sync::atomic::AtomicI32,
    pub barrier: *mut core::sync::atomic::AtomicI32,
    pub array_frozen: c_int,
// Set to 1 if a full sync is needed, (fresh device added).
// Cleared when a sync completes.
//
    pub fullsync: c_int,
    pub r1bio_pool: *mut mempool_t,
    pub r1buf_pool: mempool_t,
    pub bio_split: bio_set,
// temporary buffer to synchronous IO when attempting to repair
// a read error.
//
    pub tmppage: *mut page,
// When taking over an array from a different personality, we store
// the new thread here until we fully activate the array.
//
    pub thread: *mut md_thread __rcu,
// Keep track of cluster resync window to send to other
// nodes.
//
    pub cluster_sync_low: sector_t,
    pub cluster_sync_high: sector_t,
}

//
// this is our 'private' RAID1 bio.
//
// it contains information about what kind of IO operations were started
// for this RAID1 operation, and about their status:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r1bio {
    pub count,: *mut *mut atomic_t remaining; / 'have we finished',
// used from IRQ handlers
//
    pub remaining: *mut *mut atomic_t behind_remaining; / number of write-behind ios,
// in this BehindIO request
//
    pub sector: sector_t,
    pub sectors: c_int,
    pub state: c_ulong,
    pub mddev: *mut mddev,
//
// original bio going to /dev/mdx
//
    pub master_bio: *mut bio,
//
// if the IO is in READ direction, then this is where we read
//
    pub read_disk: c_int,
    pub retry_list: list_head,
//
// When R1BIO_BehindIO is set, we store pages for write behind
// in behind_master_bio.
//
    pub behind_master_bio: *mut bio,
//
// if the IO is in WRITE direction, then multiple bios are used.
// We choose the number when they are allocated.
//
    pub bios: [*mut bio; ],
// DO NOT PUT ANY NEW FIELDS HERE - bios array is contiguously alloced
}

// bits for r1bio.state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r1bio_state {
    R1BIO_Uptodate,
    R1BIO_IsSync,
    R1BIO_BehindIO,
// Set ReadError on bios that experience a readerror so that
// raid1d knows what to do with them.
//
    R1BIO_ReadError,
// For write-behind requests, we call bi_end_io when
// the last non-write-behind device completes, providing
// any write was successful.  Otherwise we call when
// any write-behind write succeeds, otherwise we call
// with failure when last write completes (and all failed).
//
// And for bio_split errors, record that bi_end_io was called
// with this flag...
//
    R1BIO_Returned,
// If a write for this request means we can clear some
// known-bad-block records, we set this flag
//
    R1BIO_MadeGood,
    R1BIO_WriteError,
    R1BIO_FailFast,
}
