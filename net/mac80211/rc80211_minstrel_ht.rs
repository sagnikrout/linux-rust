//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/rc80211_minstrel_ht.h
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
// Copyright (C) 2010 Felix Fietkau <nbd@openwrt.org>
//

// number of highest throughput rates to consider
pub const MAX_THR_RATES: c_int = 4;

// scaled fraction values
pub const MINSTREL_SCALE: c_int = 12;

pub const EWMA_DIV: c_int = 128;
//
// Coefficients for moving average with noise filter (period=16),
// scaled by 10 bits
//
// a1 = exp(-pi * sqrt(2) / period)
// coeff2 = 2 * a1 * cos(sqrt(2) * 2 * pi / period)
// coeff3 = -sqr(a1)
// coeff1 = 1 - coeff2 - coeff3
//

pub const MINSTREL_AVG_COEFF2: c_uint = 0x00001499;

//
// The number of streams can be changed to 2 to reduce code
// size and memory footprint.
//
pub const MINSTREL_MAX_STREAMS: c_int = 4;

pub const MINSTREL_LEGACY_GROUPS_NB: c_int = 2;

pub const MINSTREL_HT_GROUP_0: c_int = 0;

pub const MCS_GROUP_RATES: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct minstrel_priv {
    pub hw: *mut ieee80211_hw,
    pub cw_min: c_uint,
    pub cw_max: c_uint,
    pub max_retry: c_uint,
    pub segment_size: c_uint,
    pub update_interval: c_uint,
    pub cck_rates: [u8; 4],
    pub ofdm_rates: [u8; NUM_NL80211_BANDS][8],
//
// enable fixed rate processing per RC
// - write static index to debugfs:ieee80211/phyX/rc/fixed_rate_idx
// - write -1 to enable RC processing again
// - setting will be applied on next update
//
    pub fixed_rate_idx: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcs_group {
    pub flags: u16,
    pub streams: u8,
    pub shift: u8,
    pub bw: u8,
    pub duration: [u16; MCS_GROUP_RATES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct minstrel_rate_stats {
// current / last sampling period attempts/success counters
    pub last_attempts: u16 attempts,,
    pub last_success: u16 success,,
// total attempts/success counters
    pub succ_hist: u32 att_hist,,
// prob_avg - moving average of prob
    pub prob_avg: u16,
    pub prob_avg_1: u16,
// maximum retry counts
    pub retry_count: u8,
    pub retry_count_rtscts: u8,
    pub retry_updated: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum minstrel_sample_type {
    MINSTREL_SAMPLE_TYPE_INC,
    MINSTREL_SAMPLE_TYPE_JUMP,
    MINSTREL_SAMPLE_TYPE_SLOW,
    __MINSTREL_SAMPLE_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct minstrel_mcs_group_data {
    pub index: u8,
    pub column: u8,
// sorted rate set within a MCS group
    pub max_group_tp_rate: [u16; MAX_THR_RATES],
    pub max_group_prob_rate: u16,
// MCS rate statistics
    pub rates: [minstrel_rate_stats; MCS_GROUP_RATES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct minstrel_sample_category {
    pub sample_group: u8,
    pub sample_rates: [u16; MINSTREL_SAMPLE_RATES],
    pub cur_sample_rates: [u16; MINSTREL_SAMPLE_RATES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct minstrel_ht_sta {
    pub sta: *mut ieee80211_sta,
// ampdu length (average, per sampling interval)
    pub ampdu_len: c_uint,
    pub ampdu_packets: c_uint,
// ampdu length (EWMA)
    pub avg_ampdu_len: c_uint,
// overall sorted rate set
    pub max_tp_rate: [u16; MAX_THR_RATES],
    pub max_prob_rate: u16,
// time of last status update
    pub last_stats_update: c_ulong,
// overhead time in usec for each frame
    pub overhead: c_uint,
    pub overhead_rtscts: c_uint,
    pub overhead_legacy: c_uint,
    pub overhead_legacy_rtscts: c_uint,
    pub total_packets: c_uint,
    pub sample_packets: c_uint,
// tx flags to add for frames for this sta
    pub tx_flags: u32,
    pub use_short_preamble: bool,
    pub band: u8,
    pub sample_seq: u8,
    pub sample_rate: u16,
    pub sample_time: c_ulong,
    pub sample: [minstrel_sample_category; __MINSTREL_SAMPLE_TYPE_MAX],
// Bitfield of supported MCS rates of all groups
    pub supported: [u16; MINSTREL_GROUPS_NB],
// MCS rate group info and statistics
    pub groups: [minstrel_mcs_group_data; MINSTREL_GROUPS_NB],
}

extern "C" {
    pub fn minstrel_ht_add_sta_debugfs(priv: *mut c_void, priv_sta: *mut c_void, dir: *mut dentry);
}
