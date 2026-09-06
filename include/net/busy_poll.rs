//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/busy_poll.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// net busy poll support
// Copyright(c) 2013 Intel Corporation.
//
// Author: Eliezer Tamir
//
// Contact Information:
// e1000-devel Mailing List <e1000-devel@lists.sourceforge.net>
//

// 0 - Reserved to indicate value not set
// 1..NR_CPUS - Reserved for sender_cpu
// NR_CPUS+1..~0 - Region available for NAPI IDs
//

pub const BUSY_POLL_BUDGET: c_int = 8;

extern "C" {
    pub fn READ_ONCE(_arg: sysctl_net_busy_poll) -> return;
}
extern "C" {
    pub fn READ_ONCE(!signal_pending(current: sk->sk_ll_usec) &&) -> return;
}
extern "C" {
    pub fn sk_busy_loop_end(p: *mut c_void, start_time: c_ulong) -> bool;
}
extern "C" {
    pub fn napi_suspend_irqs(napi_id: c_uint);
}
extern "C" {
    pub fn napi_resume_irqs(napi_id: c_uint);
}

// in poll/select we use the global sysctl_net_ll_poll value

extern "C" {
    pub fn time_after(_arg: now, _arg: end_time) -> return;
}

extern "C" {
    pub fn time_after(_arg: now, _arg: end_time) -> return;
}

// used in the NIC receive handler to mark the skb

// If the skb was already marked with a valid NAPI ID, avoid overwriting
// it.
//

// used in the protocol handler to propagate the napi_id to the socket

// Variant of sk_mark_napi_id() for passive flow setup,
// as sk->sk_napi_id and sk->sk_rx_queue_mapping content
// needs to be set.
//

// variant used for unconnected sockets

