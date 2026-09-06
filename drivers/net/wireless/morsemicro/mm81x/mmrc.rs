//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/mmrc.h
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
// Copyright (c) 2017-2026 Morse Micro
//

// The max length of a retry chain for a single packet transmission
pub const MMRC_MAX_CHAIN_LENGTH: c_int = 4;
// Rate minimum allowed attempts
pub const MMRC_MIN_CHAIN_ATTEMPTS: c_int = 1;
// Rate upper limit for attempts
pub const MMRC_MAX_CHAIN_ATTEMPTS: c_int = 2;
// The frequency of MMRC stat table updates
pub const MMRC_UPDATE_FREQUENCY_MS: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmrc_flags {
    MMRC_FLAGS_CTS_RTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmrc_mcs_rate {
    MMRC_MCS0,
    MMRC_MCS1,
    MMRC_MCS2,
    MMRC_MCS3,
    MMRC_MCS4,
    MMRC_MCS5,
    MMRC_MCS6,
    MMRC_MCS7,
    MMRC_MCS8,
    MMRC_MCS9,
    MMRC_MCS10,
    MMRC_MCS_UNUSED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmrc_bw {
    MMRC_BW_1MHZ = 0,
    MMRC_BW_2MHZ = 1,
    MMRC_BW_4MHZ = 2,
    MMRC_BW_8MHZ = 3,
    MMRC_BW_16MHZ = 4,
    MMRC_BW_MAX = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmrc_spatial_stream {
    MMRC_SPATIAL_STREAM_1 = 0,
    MMRC_SPATIAL_STREAM_2 = 1,
    MMRC_SPATIAL_STREAM_3 = 2,
    MMRC_SPATIAL_STREAM_4 = 3,
    MMRC_SPATIAL_STREAM_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmrc_guard {
    MMRC_GUARD_LONG = 0,
    MMRC_GUARD_SHORT = 1,
    MMRC_GUARD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmrc_rate {
    pub 4: u8 rate :,
    pub 3: u8 attempts :,
    pub 1: u8 guard :,
    pub 2: u8 ss :,
    pub 3: u8 bw :,
    pub 3: u8 flags :,
    pub index: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmrc_rate_table {
    pub rates: [mmrc_rate; MMRC_MAX_CHAIN_LENGTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmrc_sta_capabilities {
    pub 3: u8 max_rates :,
    pub 3: u8 max_retries :,
    pub 5: u8 bandwidth :,
    pub 4: u8 spatial_streams :,
    pub 11: u16 rates :,
    pub 2: u8 guard :,
    pub 4: u8 sta_flags :,
    pub 5: u8 sgi_per_bw :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmrc_stats_table {
    pub avg_throughput_counter: u32,
    pub sum_throughput: u32,
    pub max_throughput: u32,
    pub sent: u16,
    pub sent_success: u16,
    pub back_mpdu_success: u16,
    pub back_mpdu_failure: u16,
    pub total_sent: u32,
    pub total_success: u32,
    pub evidence: u16,
    pub prob: u8,
    pub have_sent_ampdus: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmrc_table {
    pub caps: mmrc_sta_capabilities,
    pub best_tp: mmrc_rate,
    pub second_tp: mmrc_rate,
    pub baseline: mmrc_rate,
    pub best_prob: mmrc_rate,
    pub fixed_rate: mmrc_rate,
    pub cycle_cnt: u32,
    pub last_lookaround_cycle: u32,
    pub lookaround_cnt: u8,
// The ratio of using normal rate and sampling
    pub lookaround_wrap: u8,
//
// A counter that is used to determine when we should force a
// lookaround. Should be a portion of the above lookaround with
// less constraints
//
    pub forced_lookaround: u8,
    pub current_lookaround_rate_attempts: u8,
    pub current_lookaround_rate_index: u16,
    pub total_lookaround: u32,
//
// A counter to detect if the current best rate is optimal
// and may slow down sample frequency.
//
    pub stability_cnt: u32,
    pub stability_cnt_threshold: u32,
    pub probability_variation: u8,
// The difference in MCS from each of the last 2 rate changes
    pub best_rate_diff: [i8; 2],
// Indication of random versus consistently one-sided variation
    pub probability_variation_direction: i8,
// Has rate control detected possible interference
    pub interference_likely: bool,
// Has rate control detected the best rate is no longer converged
    pub unconverged: bool,
// Is rate control just entering unconverged state
    pub newly_unconverged: bool,
//
// Number of rate control cycles the best rate has remained
// unchanged
//
    pub best_rate_cycle_count: i32,
//
// The probability table for the STA. This MUST always be the last
// element in the struct.
//
    pub table: [mmrc_stats_table; ],
}

extern "C" {
    pub fn mmrc_memory_required_for_caps(caps: *mut mmrc_sta_capabilities) -> usize;
}
extern "C" {
    pub fn mmrc_update(tb: *mut mmrc_table);
}
extern "C" {
    pub fn mmrc_set_fixed_rate(tb: *mut mmrc_table, fixed_rate: mmrc_rate) -> bool;
}
extern "C" {
    pub fn mmrc_calculate_theoretical_throughput(rate: mmrc_rate) -> u32;
}
extern "C" {
    pub fn mmrc_calculate_rate_tx_time(rate: *mut mmrc_rate, size: usize) -> u32;
}
