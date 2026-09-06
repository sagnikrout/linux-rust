//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/queue.h
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
// Queue between the tx operation and the bh workqueue.
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_queue {
    pub normal: sk_buff_head,
    pub /: *mut *mut sk_buff_head cab; / Content After (DTIM) Beacon,
    pub offchan: sk_buff_head,
    pub pending_frames: core::sync::atomic::AtomicI32,
    pub priority: c_int,
}

extern "C" {
    pub fn wfx_tx_lock(wdev: *mut wfx_dev);
}
extern "C" {
    pub fn wfx_tx_unlock(wdev: *mut wfx_dev);
}
extern "C" {
    pub fn wfx_tx_flush(wdev: *mut wfx_dev);
}
extern "C" {
    pub fn wfx_tx_lock_flush(wdev: *mut wfx_dev);
}
extern "C" {
    pub fn wfx_tx_queues_init(wvif: *mut wfx_vif);
}
extern "C" {
    pub fn wfx_tx_queues_check_empty(wvif: *mut wfx_vif);
}
extern "C" {
    pub fn wfx_tx_queues_has_cab(wvif: *mut wfx_vif) -> bool;
}
extern "C" {
    pub fn wfx_tx_queues_put(wvif: *mut wfx_vif, skb: *mut sk_buff);
}
extern "C" {
    pub fn wfx_tx_queue_empty(wvif: *mut wfx_vif, queue: *mut wfx_queue) -> bool;
}
extern "C" {
    pub fn wfx_pending_drop(wdev: *mut wfx_dev, dropped: *mut sk_buff_head);
}
extern "C" {
    pub fn wfx_pending_get_pkt_us_delay(wdev: *mut wfx_dev, skb: *mut sk_buff) -> c_uint;
}
extern "C" {
    pub fn wfx_pending_dump_old_frames(wdev: *mut wfx_dev, limit_ms: c_uint);
}
