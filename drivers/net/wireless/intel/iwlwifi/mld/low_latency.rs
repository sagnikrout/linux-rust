//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/low_latency.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2024 Intel Corporation
//

// Macro flag: #define __iwl_mld_low_latency_h__
//
// struct iwl_mld_low_latency_packets_counters - Packets counters
// @lock: synchronize the counting in data path against the worker
// @vo_vi: per-mac, counts the number of TX and RX voice and video packets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_low_latency_packets_counters {
    pub lock: spinlock_t,
    pub vo_vi: [u32; NUM_MAC_INDEX_DRIVER],
    pub ____cacheline_aligned_in_smp: },
//
// enum iwl_mld_low_latency_cause - low-latency set causes
//
// @LOW_LATENCY_TRAFFIC: indicates low-latency traffic was detected
// @LOW_LATENCY_DEBUGFS: low-latency mode set from debugfs
// @LOW_LATENCY_VIF_TYPE: low-latency mode set because of vif type (AP)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mld_low_latency_cause {
    LOW_LATENCY_TRAFFIC	= BIT(0),
    LOW_LATENCY_DEBUGFS	= BIT(1),
    LOW_LATENCY_VIF_TYPE	= BIT(2),
}

//
// struct iwl_mld_low_latency - Manage low-latency detection and activation.
// @work: this work is used to detect low-latency by monitoring the number of
// voice and video packets transmitted in a period of time. If the
// threshold is reached, low-latency is activated. When active,
// it is deactivated if the threshold is not reached within a
// 10-second period.
// @timestamp: timestamp of the last update.
// @window_start: per-mac, timestamp of the start of the current window. when
// the window is over, the counters are reset.
// @pkts_counters: per-queue array voice/video packet counters
// @result: per-mac latest low-latency result
// @stopped: if true, ignore the requests to update the counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_low_latency {
    pub work: wiphy_delayed_work,
    pub timestamp: c_ulong,
    pub window_start: [c_ulong; NUM_MAC_INDEX_DRIVER],
    pub pkts_counters: *mut iwl_mld_low_latency_packets_counters,
    pub result: [bool; NUM_MAC_INDEX_DRIVER],
    pub stopped: bool,
}

extern "C" {
    pub fn iwl_mld_low_latency_init(mld: *mut iwl_mld) -> c_int;
}
extern "C" {
    pub fn iwl_mld_low_latency_free(mld: *mut iwl_mld);
}
extern "C" {
    pub fn iwl_mld_low_latency_restart_cleanup(mld: *mut iwl_mld);
}
extern "C" {
    pub fn iwl_mld_low_latency_stop(mld: *mut iwl_mld);
}
extern "C" {
    pub fn iwl_mld_low_latency_restart(mld: *mut iwl_mld);
}
