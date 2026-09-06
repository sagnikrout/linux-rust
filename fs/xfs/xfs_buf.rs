//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_buf.h
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

//
// Base types
//

// flags used only internally

// flags used only as arguments to access routines
//
// Online fsck is scanning the buffer cache for live buffers.  Do not warn
// about length mismatches during lookups and do not return stale buffers.
//

pub type xfs_buf_flags_t = c_uint;

// The following interface flags should never be set */ \
//
// The xfs_buftarg contains 2 notions of "sector size" -
//
// 1) The metadata sector size, which is the minimum unit and
// alignment of IO which will be performed by metadata operations.
// 2) The device logical sector size
//
// The first is specified at mkfs time, and is stored on-disk in the
// superblock's sb_sectsize.
//
// The latter is derived from the underlying device, and controls direct IO
// alignment constraints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_buftarg {
    pub bt_dev: dev_t,
    pub bt_bdev: *mut block_device,
    pub bt_daxdev: *mut dax_device,
    pub bt_file: *mut file,
    pub bt_dax_part_off: u64,
    pub bt_mount: *mut xfs_mount,
    pub bt_meta_sectorsize: c_uint,
    pub bt_meta_sectormask: usize,
    pub bt_logical_sectorsize: usize,
    pub bt_logical_sectormask: usize,
    pub bt_nr_sectors: xfs_daddr_t,
// LRU control structures
    pub bt_shrinker: *mut shrinker,
    pub bt_lru: list_lru,
    pub bt_readahead_count: percpu_counter,
    pub bt_ioerror_rl: ratelimit_state,
// Hardware atomic write unit values, bytes
    pub bt_awu_min: c_uint,
    pub bt_awu_max: c_uint,
    pub bt_hash: rhashtable,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_buf_map {
    pub /: *mut *mut xfs_daddr_t bm_bn; / block number for I/O,
    pub /: *mut *mut int bm_len; / size of I/O,
    pub bm_flags: c_uint,
}

//
// Online fsck is scanning the buffer cache for live buffers.  Do not warn
// about length mismatches during lookups and do not return stale buffers.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_buf_ops {
    pub name: *mut c_char,
    pub /: *mut *mut __be32 magic[2]; / v4 and v5 on disk magic values,
    pub /: *mut *mut __be16 magic16[2]; / v4 and v5 on disk magic values,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_buf {
//
// first cacheline holds all the fields needed for an uncontended cache
// hit to be fully processed. The semaphore straddles the cacheline
// boundary, but the counter and lock sits on the first cacheline,
// which is the only bit that is touched if we hit the semaphore
// fast-path on locking.
//
    pub /: *mut *mut rhash_head b_rhash_head; / pag buffer hash node,
    pub /: *mut *mut xfs_daddr_t b_rhash_key; / buffer cache index,
    pub /: *mut *mut int b_length; / size of buffer in BBs,
    pub /: *mut *mut lockref b_lockref; / refcount + lock,
    pub /: *mut *mut atomic_t b_lru_ref; / lru reclaim ref count,
    pub /: *mut *mut xfs_buf_flags_t b_flags; / status flags,
    pub /: *mut *mut semaphore b_sema; / semaphore for lockables,
//
// concurrent access to b_lru and b_lru_flags are protected by
// bt_lru_lock and not by b_sema
//
    pub /: *mut *mut list_head b_lru; / lru list,
    pub /: *mut *mut wait_queue_head_t b_waiters; / unpin waiters,
    pub b_list: list_head,
    pub b_pag: *mut xfs_perag,
    pub b_mount: *mut xfs_mount,
    pub /: *mut *mut *mut xfs_buftarg b_target; / buffer target (device),
    pub /: *mut *mut *mut void b_addr; / virtual address of buffer,
    pub b_ioend_work: work_struct,
    pub /: *mut *mut completion b_iowait; / queue for I/O waiters,
    pub b_log_item: *mut xfs_buf_log_item,
    pub /: *mut *mut list_head b_li_list; / Log items list head,
    pub b_transp: *mut xfs_trans,
    pub /: *mut *mut *mut xfs_buf_map b_maps; / compound buffer map,
    pub /: *mut *mut xfs_buf_map __b_map; / inline compound buffer map,
    pub b_map_count: c_int,
    pub /: *mut *mut atomic_t b_pin_count; / pin count,
    pub /: *mut *mut int b_error; / error code on I/O,
    pub bp): *mut *mut void (b_iodone)(struct xfs_buf,
//
// async write failure retry count. Initialised to zero on the first
// failure, then when it exceeds the maximum configured without a
// success the write is considered to be failed permanently and the
// iodone handler will take appropriate action.
//
// For retry timeouts, we record the jiffy of the first failure. This
// means that we can change the retry timeout for buffers already under
// I/O and thus avoid getting stuck in a retry loop with a long timeout.
//
// last_error is used to ensure that we are getting repeated errors, not
// different errors. e.g. a block device might change ENOSPC to EIO when
// a failure timeout occurs, so we want to re-initialise the error
// retry behaviour appropriately when that happens.
//
    pub b_retries: c_int,
    pub /: *mut *mut unsigned long b_first_retry_time; / in jiffies,
    pub b_last_error: c_int,
    pub b_ops: *const xfs_buf_ops,
    pub b_rcu: rcu_head,
}

// Finding and Reading Buffers
extern "C" {
    pub fn xfs_buf_get_map(_arg: target, _arg: &map, _arg: 1, flags: XBF_INCORE |, _arg: bpp) -> return;
}
extern "C" {
    pub fn xfs_buf_get_map(_arg: target, _arg: &map, _arg: 1, _arg: 0, _arg: bpp) -> return;
}
extern "C" {
    pub fn xfs_buf_readahead_map(_arg: target, _arg: &map, _arg: 1, _arg: ops) -> return;
}
extern "C" {
    pub fn _xfs_buf_read(bp: *mut xfs_buf) -> c_int;
}
extern "C" {
    pub fn xfs_buf_hold(bp: *mut xfs_buf);
}
// Releasing Buffers
extern "C" {
    pub fn xfs_buf_rele(: *mut xfs_buf);
}
// Locking and Unlocking Buffers
extern "C" {
    pub fn xfs_buf_trylock(: *mut xfs_buf) -> c_int;
}
extern "C" {
    pub fn xfs_buf_lock(: *mut xfs_buf);
}
extern "C" {
    pub fn xfs_buf_unlock(: *mut xfs_buf);
}

// Buffer Read and Write Routines
extern "C" {
    pub fn xfs_bwrite(bp: *mut xfs_buf) -> c_int;
}

extern "C" {
    pub fn xfs_buf_ioerror_alert(bp: *mut xfs_buf, fa: xfs_failaddr_t);
}
extern "C" {
    pub fn xfs_buf_fail(bp: *mut xfs_buf);
}
extern "C" {
    pub fn __xfs_buf_mark_corrupt(bp: *mut xfs_buf, fa: xfs_failaddr_t);
}

// Buffer Utility Routines
extern "C" {
    pub fn xfs_buf_set_uptodate(bp: *mut xfs_buf);
}
extern "C" {
    pub fn xfs_buf_stale(bp: *mut xfs_buf);
}
extern "C" {
    pub fn xfs_buf_clear_stale(bp: *mut xfs_buf);
}
// Delayed Write Buffer Routines
extern "C" {
    pub fn xfs_buf_delwri_cancel(: *mut list_head);
}
extern "C" {
    pub fn xfs_buf_delwri_queue(: *mut xfs_buf, : *mut list_head) -> bool;
}
extern "C" {
    pub fn xfs_buf_delwri_queue_here(bp: *mut xfs_buf, bl: *mut list_head);
}
extern "C" {
    pub fn xfs_buf_delwri_submit(: *mut list_head) -> c_int;
}
extern "C" {
    pub fn xfs_buf_delwri_submit_nowait(: *mut list_head) -> c_int;
}
extern "C" {
    pub fn xfs_buf_set_ref(bp: *mut xfs_buf, lru_ref: c_int);
}
//
// If the buffer is already on the LRU, do nothing. Otherwise set the buffer
// up with a reference count of 0 so it will be tossed from the cache when
// released.
//
extern "C" {
    pub fn atomic_read(_arg: &bp->b_pin_count) -> return;
}
//
// Handling of buftargs.
//
extern "C" {
    pub fn xfs_free_buftarg(: *mut xfs_buftarg);
}
extern "C" {
    pub fn xfs_buftarg_wait(: *mut xfs_buftarg);
}
extern "C" {
    pub fn xfs_buftarg_drain(: *mut xfs_buftarg);
}

extern "C" {
    pub fn xfs_verify_magic(bp: *mut xfs_buf, dmagic: __be32) -> bool;
}
extern "C" {
    pub fn xfs_verify_magic16(bp: *mut xfs_buf, dmagic: __be16) -> bool;
}
// for xfs_buf_mem.c only:
extern "C" {
    pub fn xfs_destroy_buftarg(btp: *mut xfs_buftarg);
}
