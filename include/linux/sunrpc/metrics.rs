//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/metrics.h
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
// linux/include/linux/sunrpc/metrics.h
//
// Declarations for RPC client per-operation metrics
//
// Copyright (C) 2005	Chuck Lever <cel@netapp.com>
//
// RPC client per-operation statistics provide latency and retry
// information about each type of RPC procedure in a given RPC program.
// These statistics are not for detailed problem diagnosis, but simply
// to indicate whether the problem is local or remote.
//
// These counters are not meant to be human-readable, but are meant to be
// integrated into system monitoring tools such as "sar" and "iostat".  As
// such, the counters are sampled by the tools over time, and are never
// zeroed after a file system is mounted.  Moving averages can be computed
// by the tools by taking the difference between two instantaneous samples
// and dividing that by the time between the samples.
//
// The counters are maintained in a single array per RPC client, indexed
// by procedure number.  There is no need to maintain separate counter
// arrays per-CPU because these counters are always modified behind locks.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_iostats {
    pub om_lock: spinlock_t,
//
// These counters give an idea about how many request
// transmissions are required, on average, to complete that
// particular procedure.  Some procedures may require more
// than one transmission because the server is unresponsive,
// the client is retransmitting too aggressively, or the
// requests are large and the network is congested.
//
    pub /: *mut *mut om_timeouts; / count of major timeouts,
//
// These count how many bytes are sent and received for a
// given RPC procedure type.  This indicates how much load a
// particular procedure is putting on the network.  These
// counts include the RPC and ULP headers, and the request
// payload.
//
    pub /: *mut *mut om_bytes_recv; / count of bytes in,
//
// The length of time an RPC request waits in queue before
// transmission, the network + server latency of the request,
// and the total time the request spent from init to release
// are measured.
//
    pub /: *mut *mut om_execute; / RPC execution,
//
// The count of operations that complete with tk_status < 0.
// These statuses usually indicate error conditions.
//
    pub om_error_status: c_ulong,
    pub ____cacheline_aligned: },
    pub rpc_task: struct,
    pub rpc_clnt: struct,
//
// EXPORTed functions for managing rpc_iostats structures
//

    pub ): *mut *mut rpc_iostats  rpc_alloc_iostats(rpc_clnt,
    pub ): *mut rpc_iostats,
    pub ): *mut rpc_iostats,
    pub ): *mut *mut void rpc_clnt_show_stats(struct seq_file , struct rpc_clnt,
    pub ): *mut void rpc_free_iostats(struct rpc_iostats,

    pub }: *mut *mut *mut static inline struct rpc_iostats rpc_alloc_iostats(struct rpc_clnt clnt) { return NULL;,

