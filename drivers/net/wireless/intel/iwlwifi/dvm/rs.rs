//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/dvm/rs.h
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
// Copyright(c) 2003 - 2014, 2023 Intel Corporation. All rights reserved.
//

// Macro flag: #define __iwl_agn_rs_h__

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rate_info {
    pub /: *mut *mut u8 plcp; / uCode API: IWL_RATE_6M_PLCP, etc.,
    pub /: *mut *mut u8 plcp_siso; / uCode API: IWL_RATE_SISO_6M_PLCP, etc.,
    pub /: *mut *mut u8 plcp_mimo2; / uCode API: IWL_RATE_MIMO2_6M_PLCP, etc.,
    pub /: *mut *mut u8 plcp_mimo3; / uCode API: IWL_RATE_MIMO3_6M_PLCP, etc.,
    pub /: *mut *mut u8 ieee; / MAC header: IWL_RATE_6M_IEEE, etc.,
    pub /: *mut *mut u8 prev_ieee; / previous rate in IEEE speeds,
    pub /: *mut *mut u8 next_ieee; / next rate in IEEE speeds,
    pub /: *mut *mut u8 prev_rs; / previous rate used in rs algo,
    pub /: *mut *mut u8 next_rs; / next rate used in rs algo,
    pub /: *mut *mut u8 prev_rs_tgg; / previous rate used in TGG rs algo,
    pub /: *mut *mut u8 next_rs_tgg; / next rate used in TGG rs algo,
}

//
// These serve as indexes into
// struct iwl_rate_info iwl_rates[IWL_RATE_COUNT];
//
// #define vs. enum to keep from defaulting to 'large integer'

// uCode API values for legacy bit rates, both OFDM and CCK
// FIXME:RS:change to IWL_RATE_LEGACY_??M_PLCP
// FIXME:RS:add IWL_RATE_LEGACY_INVM_PLCP = 0,
// uCode API values for OFDM high-throughput (HT) bit rates
// MAC header values for bit rates

pub const IWL_MAX_RSSI_VAL: c_int = 0;
// These values specify how many Tx frame attempts before
// searching for a new modulation mode
pub const IWL_LEGACY_FAILURE_LIMIT: c_int = 160;
pub const IWL_LEGACY_SUCCESS_LIMIT: c_int = 480;
pub const IWL_LEGACY_TABLE_COUNT: c_int = 160;
pub const IWL_NONE_LEGACY_FAILURE_LIMIT: c_int = 400;
pub const IWL_NONE_LEGACY_SUCCESS_LIMIT: c_int = 4500;
pub const IWL_NONE_LEGACY_TABLE_COUNT: c_int = 1500;
// Success ratio (ACKed / attempted tx frames) values (perfect is 128 * 100)

// possible actions when in legacy mode
pub const IWL_LEGACY_SWITCH_ANTENNA1: c_int = 0;
pub const IWL_LEGACY_SWITCH_ANTENNA2: c_int = 1;
pub const IWL_LEGACY_SWITCH_SISO: c_int = 2;
pub const IWL_LEGACY_SWITCH_MIMO2_AB: c_int = 3;
pub const IWL_LEGACY_SWITCH_MIMO2_AC: c_int = 4;
pub const IWL_LEGACY_SWITCH_MIMO2_BC: c_int = 5;
pub const IWL_LEGACY_SWITCH_MIMO3_ABC: c_int = 6;
// possible actions when in siso mode
pub const IWL_SISO_SWITCH_ANTENNA1: c_int = 0;
pub const IWL_SISO_SWITCH_ANTENNA2: c_int = 1;
pub const IWL_SISO_SWITCH_MIMO2_AB: c_int = 2;
pub const IWL_SISO_SWITCH_MIMO2_AC: c_int = 3;
pub const IWL_SISO_SWITCH_MIMO2_BC: c_int = 4;
pub const IWL_SISO_SWITCH_GI: c_int = 5;
pub const IWL_SISO_SWITCH_MIMO3_ABC: c_int = 6;
// possible actions when in mimo mode
pub const IWL_MIMO2_SWITCH_ANTENNA1: c_int = 0;
pub const IWL_MIMO2_SWITCH_ANTENNA2: c_int = 1;
pub const IWL_MIMO2_SWITCH_SISO_A: c_int = 2;
pub const IWL_MIMO2_SWITCH_SISO_B: c_int = 3;
pub const IWL_MIMO2_SWITCH_SISO_C: c_int = 4;
pub const IWL_MIMO2_SWITCH_GI: c_int = 5;
pub const IWL_MIMO2_SWITCH_MIMO3_ABC: c_int = 6;
// possible actions when in mimo3 mode
pub const IWL_MIMO3_SWITCH_ANTENNA1: c_int = 0;
pub const IWL_MIMO3_SWITCH_ANTENNA2: c_int = 1;
pub const IWL_MIMO3_SWITCH_SISO_A: c_int = 2;
pub const IWL_MIMO3_SWITCH_SISO_B: c_int = 3;
pub const IWL_MIMO3_SWITCH_SISO_C: c_int = 4;
pub const IWL_MIMO3_SWITCH_MIMO2_AB: c_int = 5;
pub const IWL_MIMO3_SWITCH_MIMO2_AC: c_int = 6;
pub const IWL_MIMO3_SWITCH_MIMO2_BC: c_int = 7;
pub const IWL_MIMO3_SWITCH_GI: c_int = 8;

// FIXME:RS:add possible actions for MIMO3

// load per tid defines for A-MPDU activation
pub const IWL_AGG_TPT_THREHOLD: c_int = 0;
pub const IWL_AGG_LOAD_THRESHOLD: c_int = 10;
pub const IWL_AGG_ALL_TID: c_uint = 0xff;

pub const TID_QUEUE_MAX_SIZE: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_table_type {
    LQ_NONE,
    LQ_G,		/* legacy types */
    LQ_A,
    LQ_SISO,	/* high-throughput types */
    LQ_MIMO2,
    LQ_MIMO3,
    LQ_MAX,
}

pub const IWL_MAX_MCS_DISPLAY_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rate_mcs_info {
    pub mbps: [c_char; IWL_MAX_MCS_DISPLAY_SIZE],
    pub mcs: [c_char; IWL_MAX_MCS_DISPLAY_SIZE],
}

//
// struct iwl_rate_scale_data -- tx success history for one rate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rate_scale_data {
    pub /: *mut *mut u64 data; / bitmap of successful frames,
    pub /: *mut *mut s32 success_counter; / number of frames successful,
    pub /: *mut *mut *mut s32 success_ratio; / per-cent  128,
    pub /: *mut *mut s32 counter; / number of frames attempted,
    pub /: *mut *mut *mut s32 average_tpt; / success ratio  expected throughput,
    pub stamp: c_ulong,
}

//
// struct iwl_scale_tbl_info -- tx params and success history for all rates
//
// There are two of these in struct iwl_lq_sta,
// one for "active", and one for "search".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scale_tbl_info {
    pub lq_type: iwl_table_type,
    pub ant_type: u8,
    pub /: *mut *mut u8 is_SGI; / 1 = short guard interval,
    pub /: *mut *mut u8 is_ht40; / 1 = 40 MHz channel width,
    pub /: *mut *mut u8 is_dup; / 1 = duplicated data streams,
    pub /: *mut *mut *mut u8 action; / change modulation; IWL_[LEGACY/SISO/MIMO]_SWITCH_,
    pub /: *mut *mut u8 max_search; / maximun number of tables we can search,
    pub /: *const *const *const u16 expected_tpt; / throughput metrics; expected_tpt_G, etc.,
    pub /: *mut *mut u32 current_rate; / rate_n_flags, uCode API format,
    pub /: *mut *mut iwl_rate_scale_data win[IWL_RATE_COUNT]; / rate histories,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_traffic_load {
    pub /: *mut *mut unsigned long time_stamp; / age of the oldest statistics,
    pub time: *mut *mut u32 packet_count[TID_QUEUE_MAX_SIZE]; / packet count in this,
// slice
    pub the: *mut *mut u32 total; / total num of packets during,
// last TID_MAX_TIME_DIFF
    pub has: *mut *mut u8 queue_count; / number of queues that,
// been used since the last cleanup
    pub /: *mut *mut u8 head; / start of the circular buffer,
}

//
// struct iwl_lq_sta -- driver's rate scaling private structure
//
// Pointer to this gets passed back and forth between driver and mac80211.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lq_sta {
    pub /: *mut *mut u8 active_tbl; / index of active table, range 0-1,
    pub /: *mut *mut u8 enable_counter; / indicates HT mode,
    pub /: *mut *mut u8 stay_in_tbl; / 1: disallow, 0: allow search for new mode,
    pub /: *mut *mut u8 search_better_tbl; / 1: currently trying alternate mode,
    pub last_tpt: i32,
// The following determine when to search for a new mode
    pub table_count_limit: u32,
    pub /: *mut *mut u32 max_failure_limit; / # failed frames before new search,
    pub /: *mut *mut u32 max_success_limit; / # successful frames before new search,
    pub table_count: u32,
    pub /: *mut *mut u32 total_failed; / total failed frames, any/all rates,
    pub /: *mut *mut u32 total_success; / total successful frames, any/all rates,
    pub /: *mut *mut u64 flush_timer; / time staying in mode before new search,
    pub /: *mut *mut u8 action_counter; / # mode-switch actions tried,
    pub is_green: u8,
    pub is_dup: u8,
    pub band: nl80211_band,
// The following are bitmaps of rates; IWL_RATE_6M_MASK, etc.
    pub supp_rates: u32,
    pub active_legacy_rate: u16,
    pub active_siso_rate: u16,
    pub active_mimo2_rate: u16,
    pub active_mimo3_rate: u16,
    pub /: *mut *mut s8 max_rate_idx; / Max rate set by user,
    pub missed_rate_counter: u8,
    pub lq: iwl_link_quality_cmd,
    pub /: *mut *mut iwl_scale_tbl_info lq_info[LQ_SIZE]; / "active", "search",
    pub load: [iwl_traffic_load; IWL_MAX_TID_COUNT],
    pub tx_agg_tid_en: u8,

    pub dbg_fixed_rate: u32,

    pub drv: *mut iwl_priv,
// used to be in sta_info
    pub last_txrate_idx: c_int,
// last tx rate_n_flags
    pub last_rate_n_flags: u32,
// packets destined for this STA are aggregated
    pub is_agg: u8,
// BT traffic this sta was last updated in
    pub last_bt_traffic: u8,
}

// Initialize station's rate scaling information after adding station
//
// iwl_rate_control_register - Register the rate control algorithm callbacks
//
// Since the rate control algorithm is hardware specific, there is no need
// or reason to place it as a stand alone module.  The driver can call
// iwl_rate_control_register in order to register the rate control callbacks
// with the mac80211 subsystem.  This should be performed prior to calling
// ieee80211_register_hw
//
extern "C" {
    pub fn iwlagn_rate_control_register() -> c_int;
}
//
// iwl_rate_control_unregister - Unregister the rate control callbacks
//
// This should be called after calling ieee80211_unregister_hw, but before
// the driver is unloaded.
//
extern "C" {
    pub fn iwlagn_rate_control_unregister();
}
