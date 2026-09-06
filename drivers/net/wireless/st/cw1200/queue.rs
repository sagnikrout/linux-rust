//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/queue.h
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
// O(1) TX queue with built-in allocator for ST-Ericsson CW1200 drivers
//
// Copyright (c) 2010, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//

// Macro flag: #define CW1200_QUEUE_H_INCLUDED
// private */ struct cw1200_queue_item;
// extern */ struct sk_buff;
// extern */ struct wsm_tx;
// extern */ struct cw1200_common;
// extern */ struct ieee80211_tx_queue_stats;
// extern */ struct cw1200_txpriv;
// forward */ struct cw1200_queue_stats;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_queue {
    pub stats: *mut cw1200_queue_stats,
    pub capacity: usize,
    pub num_queued: usize,
    pub num_pending: usize,
    pub num_sent: usize,
    pub pool: *mut cw1200_queue_item,
    pub queue: list_head,
    pub free_pool: list_head,
    pub pending: list_head,
    pub tx_locked_cnt: c_int,
    pub link_map_cache: *mut c_int,
    pub overfull: bool,
    pub /: *mut *mut spinlock_t lock; / Protect queue entry,
    pub queue_id: u8,
    pub generation: u8,
    pub gc: timer_list,
    pub ttl: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_queue_stats {
    pub /: *mut *mut spinlock_t lock; / Protect stats entry,
    pub link_map_cache: *mut c_int,
    pub num_queued: c_int,
    pub map_capacity: usize,
    pub wait_link_id_empty: wait_queue_head_t,
    pub skb_dtor: cw1200_queue_skb_dtor_t,
    pub priv: *mut cw1200_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_txpriv {
    pub link_id: u8,
    pub raw_link_id: u8,
    pub tid: u8,
    pub rate_id: u8,
    pub offset: u8,
}

extern "C" {
    pub fn cw1200_queue_clear(queue: *mut cw1200_queue) -> c_int;
}
extern "C" {
    pub fn cw1200_queue_stats_deinit(stats: *mut cw1200_queue_stats);
}
extern "C" {
    pub fn cw1200_queue_deinit(queue: *mut cw1200_queue);
}
extern "C" {
    pub fn cw1200_queue_requeue(queue: *mut cw1200_queue, packet_id: u32) -> c_int;
}
extern "C" {
    pub fn cw1200_queue_lock(queue: *mut cw1200_queue);
}
extern "C" {
    pub fn cw1200_queue_unlock(queue: *mut cw1200_queue);
}
