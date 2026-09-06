//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/raid10.h
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
// Note: raid10_info.rdev can be set to NULL asynchronously by
// raid10_remove_disk.
// There are three safe ways to access raid10_info.rdev.
// 1/ when holding mddev->reconfig_mutex
// 2/ when resync/recovery/reshape is known to be happening - i.e. in code
// that is called as part of performing resync/recovery/reshape.
// 3/ while holding rcu_read_lock(), use rcu_dereference to get the pointer
// and if it is non-NULL, increment rdev->nr_pending before dropping the
// RCU lock.
// When .rdev is set to NULL, the nr_pending count checked again and if it has
// been incremented, the pointer is put back in .rdev.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct raid10_info {
    pub replacement: *mut *mut md_rdev rdev,,
    pub head_position: sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct r10conf {
    pub mddev: *mut mddev,
    pub mirrors: *mut raid10_info,
    pub mirrors_old: *mut *mut raid10_info mirrors_new,,
    pub device_lock: spinlock_t,
// geometry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geom {
    pub raid_disks: c_int,
    pub out: *mut *mut int near_copies; / number of copies laid,
// raid0 style
    pub out: *mut *mut int far_copies; / number of copies laid,
// at large strides across drives
//
    pub 1: *mut *mut int far_offset; / far_copies are offset by,
// stripe instead of many
//
    pub copies.: *mut *mut sector_t stride; / distance between far,
// This is size / far_copies unless
// far_offset, in which case it is
// 1 stripe.
//
    pub set,: *mut *mut int far_set_size; / The number of devices in a,
// where a 'set' are devices that
// contain far/offset copies of
// each other.
//
    pub /: *mut *mut int chunk_shift; / shift from chunks to sectors,
    pub chunk_mask: sector_t,
    pub geo: } prev,,
    pub far_copies.: *mut *mut *mut int copies; / near_copies,
// must be <= raid_disks
//
    pub of: *mut *mut sector_t dev_sectors; / temp copy,
// mddev->dev_sectors
    pub reshape_progress: sector_t,
    pub reshape_safe: sector_t,
    pub reshape_checkpoint: c_ulong,
    pub offset_diff: sector_t,
    pub retry_list: list_head,
// A separate list of r1bio which just need raid_end_bio_io called.
// This mustn't happen for writes which had any errors if the superblock
// needs to be written.
//
    pub bio_end_io_list: list_head,
// queue pending writes and submit them on unplug
    pub pending_bio_list: bio_list,
    pub resync_lock: seqlock_t,
    pub nr_pending: core::sync::atomic::AtomicI32,
    pub nr_waiting: c_int,
    pub nr_queued: c_int,
    pub barrier: c_int,
    pub array_freeze_pending: c_int,
    pub next_resync: sector_t,
    pub needed,: *mut *mut int fullsync; / set to 1 if a full sync is,
// (fresh device added).
// Cleared when a sync completes.
//
    pub one: *mut *mut int have_replacement; / There is at least,
// replacement device.
//
    pub wait_barrier: wait_queue_head_t,
    pub r10bio_pool: *mut mempool_t,
    pub r10buf_pool: mempool_t,
    pub tmppage: *mut page,
    pub bio_split: bio_set,
// When taking over an array from a different personality, we store
// the new thread here until we fully activate the array.
//
    pub thread: *mut md_thread __rcu,
//
// Keep track of cluster resync window to send to other nodes.
//
    pub cluster_sync_low: sector_t,
    pub cluster_sync_high: sector_t,
}

//
// this is our 'private' RAID10 bio.
//
// it contains information about what kind of IO operations were started
// for this RAID10 operation, and about their status:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r10bio {
    pub count,: *mut *mut atomic_t remaining; / 'have we finished',
// used from IRQ handlers
//
    pub /: *mut *mut sector_t sector; / virtual sector number,
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
    pub read_slot: c_int,
    pub retry_list: list_head,
//
// if the IO is in WRITE direction, then multiple bios are used,
// one for each copy.
// When resyncing we also use one for each copy.
// When reconstructing, we use 2 bios, one for read, one for write.
// We choose the number when they are allocated.
// We sometimes need an extra bio to write to the replacement.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r10dev {
    pub bio: *mut bio,
    pub and: *mut *mut *mut bio repl_bio; / used for resync,
// writes
    pub reads: *mut *mut *mut md_rdev rdev; / used for,
// (read_slot >= 0)
}

// bits for r10bio.state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum r10bio_state {
    R10BIO_Uptodate,
    R10BIO_IsSync,
    R10BIO_IsRecover,
    R10BIO_IsReshape,
// Set ReadError on bios that experience a read error
// so that raid10d knows what to do with them.
//
    R10BIO_ReadError,
// For bio_split errors, record that bi_end_io was called.
    R10BIO_Returned,
// If a write for this request means we can clear some
// known-bad-block records, we set this flag.
//
    R10BIO_MadeGood,
    R10BIO_WriteError,
// During a reshape we might be performing IO on the
// 'previous' part of the array, in which case this
// flag is set
//
    R10BIO_Previous,
// failfast devices did receive failfast requests.
    R10BIO_FailFast,
    R10BIO_Discard,
}
