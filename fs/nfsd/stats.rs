//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfsd/stats.h
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
// Statistics for NFS server.
//
// Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
//

extern "C" {
    pub fn nfsd_proc_stat_shutdown(net: *mut net);
}
//
// nfsd_stats_rc_hits_inc - Count a duplicate reply cache hit
// @nn: target network namespace
//
// These reply cache counters are updated once per RPC. Readers use
// percpu_counter_sum_positive(), so local batching does not affect
// read accuracy.
//
// nfsd_stats_rc_misses_inc - Count a duplicate reply cache miss
// @nn: target network namespace
//
// See nfsd_stats_rc_hits_inc() for batching rationale.
//
// nfsd_stats_rc_nocache_inc - Count a request not cached in the reply cache
// @nn: target network namespace
//
// See nfsd_stats_rc_hits_inc() for batching rationale.
//
// nfsd_stats_io_read_add - Count number of bytes for an NFS READ
// @nn: target network namespace
// @exp: target export
// @amount: byte count
//
// These counters are updated on every READ request. Readers use
// percpu_counter_sum_positive(), so local batching does not affect
// read accuracy.
//
// nfsd_stats_io_write_add - Count number of bytes for an NFS WRITE
// @nn: target network namespace
// @exp: target export
// @amount: byte count
//
// These counters are updated on every WRITE request. Readers use
// percpu_counter_sum_positive(), so local batching does not affect
// read accuracy.
//
// nfsd_stats_drc_mem_usage_add - Add memory used by a cache item
// @nn: target network namespace
// @amount: byte count
//
// percpu_counter_add_local() keeps updates on the per-CPU fast
// path. The sole reader, percpu_counter_sum_positive(), sums the
// per-CPU deltas, so batching locally does not lose accuracy.
//
// nfsd_stats_drc_mem_usage_sub - Subtract memory used by a cache item
// @nn: target network namespace
// @amount: byte count
//
// See nfsd_stats_drc_mem_usage_add() for batching rationale.
//

