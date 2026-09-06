//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/libeth/types.h
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
// Copyright (C) 2024-2025 Intel Corporation

// Stats
//
// struct libeth_rq_napi_stats - "hot" counters to update in Rx polling loop
// @packets: received frames counter
// @bytes: sum of bytes of received frames above
// @fragments: sum of fragments of received S/G frames
// @hsplit: number of frames the device performed the header split for
// @raw: alias to access all the fields as an array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_rq_napi_stats {
    pub packets: u32,
    pub bytes: u32,
    pub fragments: u32,
    pub hsplit: u32,
}

//
// struct libeth_sq_napi_stats - "hot" counters to update in Tx completion loop
// @packets: completed frames counter
// @bytes: sum of bytes of completed frames above
// @raw: alias to access all the fields as an array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_sq_napi_stats {
    pub packets: u32,
    pub bytes: u32,
}

//
// struct libeth_xdpsq_napi_stats - "hot" counters to update in XDP Tx
// completion loop
// @packets: completed frames counter
// @bytes: sum of bytes of completed frames above
// @fragments: sum of fragments of completed S/G frames
// @raw: alias to access all the fields as an array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_xdpsq_napi_stats {
    pub packets: u32,
    pub bytes: u32,
    pub fragments: u32,
}

// XDP
//
// The following structures should be embedded into driver's queue structure
// and passed to the libeth_xdp helpers, never used directly.
//
// XDPSQ sharing
//
// struct libeth_xdpsq_lock - locking primitive for sharing XDPSQs
// @lock: spinlock for locking the queue
// @share: whether this particular queue is shared
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_xdpsq_lock {
    pub lock: spinlock_t,
    pub share: bool,
}

// XDPSQ clean-up timers
//
// struct libeth_xdpsq_timer - timer for cleaning up XDPSQs w/o interrupts
// @xdpsq: queue this timer belongs to
// @lock: lock for the queue
// @dwork: work performing cleanups
//
// XDPSQs not using interrupts but lazy cleaning, i.e. only when there's no
// space for sending the current queued frame/bulk, must fire up timers to
// make sure there are no stale buffers to free.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_xdpsq_timer {
    pub xdpsq: *mut c_void,
    pub lock: *mut libeth_xdpsq_lock,
    pub dwork: delayed_work,
}

// Rx polling path
//
// struct libeth_xdp_buff_stash - struct for stashing &xdp_buff onto a queue
// @data: pointer to the start of the frame, xdp_buff.data
// @headroom: frame headroom, xdp_buff.data - xdp_buff.data_hard_start
// @len: frame linear space length, xdp_buff.data_end - xdp_buff.data
// @frame_sz: truesize occupied by the frame, xdp_buff.frame_sz
// @flags: xdp_buff.flags
//
// &xdp_buff is 56 bytes long on x64, &libeth_xdp_buff is 64 bytes. This
// structure carries only necessary fields to save/restore a partially built
// frame on the queue structure to finish it during the next NAPI poll.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_xdp_buff_stash {
    pub data: *mut c_void,
    pub headroom: u16,
    pub len: u16,
    pub frame_sz:24: u32,
    pub flags:8: u32,
    pub __aligned_largest: },
