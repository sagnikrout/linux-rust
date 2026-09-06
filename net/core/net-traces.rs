//! Automatically rewritten from C to Rust
//! Source: net/core/net-traces.c
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
// consolidates trace point definitions
//
// Copyright (C) 2009 Neil Horman <nhorman@tuxdriver.com>
//

// Macro flag: #define CREATE_TRACE_POINTS

    EXPORT_TRACEPOINT_SYMBOL_GPL(br_fdb_add);
    EXPORT_TRACEPOINT_SYMBOL_GPL(br_fdb_external_learn_add);
    EXPORT_TRACEPOINT_SYMBOL_GPL(fdb_delete);
    EXPORT_TRACEPOINT_SYMBOL_GPL(br_fdb_update);
    EXPORT_TRACEPOINT_SYMBOL_GPL(br_mdb_full);

    EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_update);
    EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_update_done);
    EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_timer_handler);
    EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_event_send_done);
    EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_event_send_dead);
    EXPORT_TRACEPOINT_SYMBOL_GPL(neigh_cleanup_and_release);
    EXPORT_TRACEPOINT_SYMBOL_GPL(kfree_skb);
    EXPORT_TRACEPOINT_SYMBOL_GPL(napi_poll);
    EXPORT_TRACEPOINT_SYMBOL_GPL(tcp_send_reset);
    EXPORT_TRACEPOINT_SYMBOL_GPL(tcp_bad_csum);
    EXPORT_TRACEPOINT_SYMBOL_GPL(udp_fail_queue_rcv_skb);
    EXPORT_TRACEPOINT_SYMBOL_GPL(sk_data_ready);
