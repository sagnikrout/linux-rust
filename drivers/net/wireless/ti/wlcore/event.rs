//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/event.h
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
// This file is part of wl1271
//
// Copyright (C) 1998-2009 Texas Instruments. All rights reserved.
// Copyright (C) 2008-2009 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//
// Mbox events
//
// The event mechanism is based on a pair of event buffers (buffers A and
// B) at fixed locations in the target's memory. The host processes one
// buffer while the other buffer continues to collect events. If the host
// is not processing events, an interrupt is issued to signal that a buffer
// is ready. Once the host is done with processing events from one buffer,
// it signals the target (with an ACK interrupt) that the event buffer is
// free.
//
// events the driver might want to wait for
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_wait_event {
    WLCORE_EVENT_ROLE_STOP_COMPLETE,
    WLCORE_EVENT_PEER_REMOVE_COMPLETE,
    WLCORE_EVENT_DFS_CONFIG_COMPLETE
}

pub const NUM_OF_RSSI_SNR_TRIGGERS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_logger_information {
    pub max_buff_size: __le32,
    pub actual_buff_size: __le32,
    pub num_trace_drop: __le32,
    pub buff_read_ptr: __le32,
    pub buff_write_ptr: __le32,
    pub __packed: },
    pub wl1271: struct,
    pub wl): *mut int wl1271_event_unmask(struct wl1271,
    pub mbox): *mut *mut int wl1271_event_handle(struct wl1271 wl, u8,
    pub enable): *mut *mut void wlcore_event_soft_gemini_sense(struct wl1271 wl, u8,
    pub status): u8,
    pub allowed_bitmap): c_ulong,
    pub success): bool,
    pub roles_bitmap): *mut *mut void wlcore_event_beacon_loss(struct wl1271 wl, unsigned long,
    pub wl): *mut void wlcore_event_dummy_packet(struct wl1271,
    pub sta_bitmap): *mut *mut void wlcore_event_max_tx_failure(struct wl1271 wl, unsigned long,
    pub sta_bitmap): *mut *mut void wlcore_event_inactive_sta(struct wl1271 wl, unsigned long,
    pub wl): *mut void wlcore_event_roc_complete(struct wl1271,
    pub metric_arr): *mut *mut void wlcore_event_rssi_trigger(struct wl1271 wl, s8,
    pub wl): *mut int wlcore_event_fw_logger(struct wl1271,
