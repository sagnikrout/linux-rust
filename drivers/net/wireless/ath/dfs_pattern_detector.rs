//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/dfs_pattern_detector.h
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


//
// Copyright (c) 2012 Neratec Solutions AG
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// tolerated deviation of radar time stamp in usecs on both sides
// TODO: this might need to be HW-dependent
//
pub const PRI_TOLERANCE: c_int = 16;
//
// struct ath_dfs_pool_stats - DFS Statistics for global pools
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_dfs_pool_stats {
    pub pool_reference: u32,
    pub pulse_allocated: u32,
    pub pulse_alloc_error: u32,
    pub pulse_used: u32,
    pub pseq_allocated: u32,
    pub pseq_alloc_error: u32,
    pub pseq_used: u32,
}

//
// struct pulse_event - describing pulses reported by PHY
// @ts: pulse time stamp in us
// @freq: channel frequency in MHz
// @width: pulse duration in us
// @rssi: rssi of radar event
// @chirp: chirp detected in pulse
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pulse_event {
    pub ts: u64,
    pub freq: u16,
    pub width: u8,
    pub rssi: u8,
    pub chirp: bool,
}

//
// struct radar_detector_specs - detector specs for a radar pattern type
// @type_id: pattern type, as defined by regulatory
// @width_min: minimum radar pulse width in [us]
// @width_max: maximum radar pulse width in [us]
// @pri_min: minimum pulse repetition interval in [us] (including tolerance)
// @pri_max: minimum pri in [us] (including tolerance)
// @num_pri: maximum number of different pri for this type
// @ppb: pulses per bursts for this type
// @ppb_thresh: number of pulses required to trigger detection
// @max_pri_tolerance: pulse time stamp tolerance on both sides [us]
// @chirp: chirp required for the radar pattern
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radar_detector_specs {
    pub type_id: u8,
    pub width_min: u8,
    pub width_max: u8,
    pub pri_min: u16,
    pub pri_max: u16,
    pub num_pri: u8,
    pub ppb: u8,
    pub ppb_thresh: u8,
    pub max_pri_tolerance: u8,
    pub chirp: bool,
}

//
// struct dfs_pattern_detector - DFS pattern detector
// @exit(): destructor
// @set_dfs_domain(): set DFS domain, resets detector lines upon domain changes
// @add_pulse(): add radar pulse to detector, returns true on detection
// @region: active DFS region, NL80211_DFS_UNSET until set
// @num_radar_types: number of different radar types
// @last_pulse_ts: time stamp of last valid pulse in usecs
// @radar_detector_specs: array of radar detection specs
// @channel_detectors: list connecting channel_detector elements
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfs_pattern_detector {
    pub dpd): *mut *mut void (exit)(struct dfs_pattern_detector,
    pub region): nl80211_dfs_regions,
    pub rs): *mut radar_detector_specs,
    pub dpd): *mut *mut ath_dfs_pool_stats (get_stats)(dfs_pattern_detector,
    pub region: nl80211_dfs_regions,
    pub num_radar_types: u8,
    pub last_pulse_ts: u64,
// needed for ath_dbg()
    pub common: *mut ath_common,
    pub radar_spec: *const radar_detector_specs,
    pub channel_detectors: list_head,
}

//
// dfs_pattern_detector_init() - constructor for pattern detector class
// @param region: DFS domain to be used, can be NL80211_DFS_UNSET at creation
// @return instance pointer on success, NULL otherwise
//
