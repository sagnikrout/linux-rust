//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/strparser.h
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
// Stream Parser
//
// Copyright (c) 2016 Tom Herbert <tom@herbertland.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strp_stats {
    pub msgs: c_ulonglong,
    pub bytes: c_ulonglong,
    pub mem_fail: c_uint,
    pub need_more_hdr: c_uint,
    pub msg_too_big: c_uint,
    pub msg_timeouts: c_uint,
    pub bad_hdr_len: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strp_aggr_stats {
    pub msgs: c_ulonglong,
    pub bytes: c_ulonglong,
    pub mem_fail: c_uint,
    pub need_more_hdr: c_uint,
    pub msg_too_big: c_uint,
    pub msg_timeouts: c_uint,
    pub bad_hdr_len: c_uint,
    pub aborts: c_uint,
    pub interrupted: c_uint,
    pub unrecov_intr: c_uint,
}

// Callbacks are called with lock held for the attached socket
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strp_callbacks {
    pub skb): *mut *mut *mut int (parse_msg)(struct strparser strp, struct sk_buff,
    pub skb): *mut *mut *mut void (rcv_msg)(struct strparser strp, struct sk_buff,
    pub recv_actor): sk_read_actor_t,
    pub err): *mut *mut *mut int (read_sock_done)(struct strparser strp, int,
    pub err): *mut *mut *mut void (abort_parser)(struct strparser strp, int,
    pub strp): *mut *mut void (lock)(struct strparser,
    pub strp): *mut *mut void (unlock)(struct strparser,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strp_msg {
    pub full_len: c_int,
    pub offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _strp_msg {
// Internal cb structure. struct strp_msg must be first for passing
// to upper layer.
//
    pub strp: strp_msg,
    pub accum_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_skb_cb {
pub const SK_SKB_CB_PRIV_LEN: c_int = 20;
    pub data: [c_uchar; SK_SKB_CB_PRIV_LEN],
// align strp on cache line boundary within skb->cb[]
    pub pad: [c_uchar; 4],
    pub strp: _strp_msg,
// strp users' data follows
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_msg {
    pub control: u8,
    pub tls: },
// temp_reg is a temporary register used for bpf_convert_data_end_access
// when dst_reg == src_reg.
//
    pub temp_reg: u64,
}

// Structure for an attached lower socket
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strparser {
    pub sk: *mut sock,
    pub 1: u32 stopped :,
    pub 1: u32 paused :,
    pub 1: u32 aborted :,
    pub 1: u32 interrupted :,
    pub 1: u32 unrecov_intr :,
    pub skb_nextp: *mut sk_buff,
    pub skb_head: *mut sk_buff,
    pub need_bytes: c_uint,
    pub msg_timer_work: delayed_work,
    pub work: work_struct,
    pub stats: strp_stats,
    pub cb: strp_callbacks,
}

// Must be called with lock held for attached socket
// May be called without holding lock for attached socket
extern "C" {
    pub fn strp_unpause(strp: *mut strparser);
}
// Save psock statistics in the mux when psock is being unattached.

extern "C" {
    pub fn strp_done(strp: *mut strparser);
}
extern "C" {
    pub fn strp_stop(strp: *mut strparser);
}
extern "C" {
    pub fn strp_check_rcv(strp: *mut strparser);
}
extern "C" {
    pub fn strp_data_ready(strp: *mut strparser);
}
