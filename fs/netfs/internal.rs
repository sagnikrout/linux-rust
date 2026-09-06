//! Automatically rewritten from C Header to Rust Module
//! Source: fs/netfs/internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Internal definitions for network filesystem support
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// buffered_read.c
//
extern "C" {
    pub fn netfs_cache_read_terminated(priv: *mut c_void, transferred_or_error: isize);
}
//
// buffered_write.c
//
// main.c
//

//
// misc.c
//
extern "C" {
    pub fn netfs_reset_iter(subreq: *mut netfs_io_subrequest);
}
extern "C" {
    pub fn netfs_wake_collector(rreq: *mut netfs_io_request);
}
extern "C" {
    pub fn netfs_subreq_clear_in_progress(subreq: *mut netfs_io_subrequest);
}
extern "C" {
    pub fn netfs_wait_for_read(rreq: *mut netfs_io_request) -> isize;
}
extern "C" {
    pub fn netfs_wait_for_write(rreq: *mut netfs_io_request) -> isize;
}
extern "C" {
    pub fn netfs_wait_for_paused_read(rreq: *mut netfs_io_request);
}
extern "C" {
    pub fn netfs_wait_for_paused_write(rreq: *mut netfs_io_request);
}
//
// objects.c
//
extern "C" {
    pub fn netfs_get_request(rreq: *mut netfs_io_request, what: netfs_rreq_ref_trace);
}
extern "C" {
    pub fn netfs_clear_subrequests(rreq: *mut netfs_io_request);
}
extern "C" {
    pub fn netfs_put_request(rreq: *mut netfs_io_request, what: netfs_rreq_ref_trace);
}
extern "C" {
    pub fn netfs_put_failed_request(rreq: *mut netfs_io_request);
}
//
// read_collect.c
//
extern "C" {
    pub fn netfs_read_collection(rreq: *mut netfs_io_request) -> bool;
}
extern "C" {
    pub fn netfs_read_collection_worker(work: *mut work_struct);
}
extern "C" {
    pub fn netfs_cancel_read(subreq: *mut netfs_io_subrequest, error: c_int);
}
extern "C" {
    pub fn netfs_cache_read_terminated(priv: *mut c_void, transferred_or_error: isize);
}
//
// read_pgpriv2.c
//
extern "C" {
    pub fn netfs_pgpriv2_copy_to_cache(rreq: *mut netfs_io_request, folio: *mut folio);
}
extern "C" {
    pub fn netfs_pgpriv2_end_copy_to_cache(rreq: *mut netfs_io_request);
}
extern "C" {
    pub fn netfs_pgpriv2_unlock_copied_folios(wreq: *mut netfs_io_request) -> bool;
}
//
// read_retry.c
//
extern "C" {
    pub fn netfs_retry_reads(rreq: *mut netfs_io_request);
}
extern "C" {
    pub fn netfs_unlock_abandoned_read_pages(rreq: *mut netfs_io_request);
}
//
// stats.c
//

extern "C" {
    pub fn netfs_stats_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}

//
// write_collect.c
//
extern "C" {
    pub fn netfs_folio_written_back(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn netfs_write_collection(wreq: *mut netfs_io_request) -> bool;
}
extern "C" {
    pub fn netfs_write_collection_worker(work: *mut work_struct);
}
//
// write_issue.c
//
// write_retry.c
//
extern "C" {
    pub fn netfs_retry_writes(wreq: *mut netfs_io_request);
}
//
// Miscellaneous functions.
//

//
// Get a ref on a netfs group attached to a dirty page (e.g. a ceph snap).
//
// Dispose of a netfs group attached to a dirty page (e.g. a ceph snap).
//
// Dispose of a netfs group attached to a dirty page (e.g. a ceph snap).
//
// Clear and wake up a NETFS_RREQ_* flag bit on a request.
//
// Test the NETFS_RREQ_IN_PROGRESS flag, inserting an appropriate barrier.
//
// Order read of flags before read of anything else, such as error.
extern "C" {
    pub fn test_bit_acquire(_arg: NETFS_RREQ_IN_PROGRESS, _arg: &rreq->flags) -> return;
}
//
// Test the NETFS_SREQ_IN_PROGRESS flag, inserting an appropriate barrier.
//
// Order read of flags before read of anything else, such as error.
extern "C" {
    pub fn test_bit_acquire(_arg: NETFS_SREQ_IN_PROGRESS, _arg: &subreq->flags) -> return;
}
//
// fscache-cache.c
//

extern "C" {
    pub fn fscache_begin_cache_access(cache: *mut fscache_cache, why: fscache_access_trace) -> bool;
}
extern "C" {
    pub fn fscache_end_cache_access(cache: *mut fscache_cache, why: fscache_access_trace);
}
extern "C" {
    pub fn fscache_put_cache(cache: *mut fscache_cache, where: fscache_cache_trace);
}
extern "C" {
    pub fn smp_load_acquire(_arg: &cache->state) -> return;
}
extern "C" {
    pub fn try_cmpxchg_release(_arg: &cache->state, _arg: &old_state, _arg: new_state) -> return;
}
//
// fscache-cookie.c
//

extern "C" {
    pub fn fscache_print_cookie(cookie: *mut fscache_cookie, prefix: c_char);
}
//
// fscache-main.c
//
extern "C" {
    pub fn fscache_hash(salt: c_uint, data: *const c_void, len: usize) -> c_uint;
}

extern "C" {
    pub fn fscache_init() -> int __init;
}
extern "C" {
    pub fn fscache_exit() -> void __exit;
}

//
// fscache-proc.c
//

extern "C" {
    pub fn fscache_proc_init() -> int __init;
}
extern "C" {
    pub fn fscache_proc_cleanup();
}

//
// fscache-stats.c
//

extern "C" {
    pub fn fscache_stats_show(m: *mut seq_file) -> c_int;
}

//
// fscache-volume.c
//

extern "C" {
    pub fn fscache_create_volume(volume: *mut fscache_volume, wait: bool);
}
//
// debug tracing
//

//
// assertions
//

