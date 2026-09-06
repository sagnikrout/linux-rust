//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mvm/rs.h
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
// Copyright(c) 2015 Intel Mobile Communications GmbH
// Copyright(c) 2017 Intel Deutschland GmbH
// Copyright (C) 2003 - 2014, 2018 - 2025 Intel Corporation
//

// Macro flag: #define __rs_h__

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rs_rate_info {
    pub /: *mut *mut u8 plcp; / uCode API: IWL_RATE_6M_PLCP, etc.,
    pub /: *mut *mut u8 plcp_ht_siso; / uCode API: IWL_RATE_SISO_6M_PLCP, etc.,
    pub /: *mut *mut u8 plcp_ht_mimo2; / uCode API: IWL_RATE_MIMO2_6M_PLCP, etc.,
    pub plcp_vht_siso: u8,
    pub plcp_vht_mimo2: u8,
    pub /: *mut *mut u8 prev_rs; / previous rate used in rs algo,
    pub /: *mut *mut u8 next_rs; / next rate used in rs algo,
}

pub const IWL_RATE_60M_PLCP: c_int = 3;
pub const LINK_QUAL_MAX_RETRY_NUM: c_int = 16;
// #define vs. enum to keep from defaulting to 'large integer'

// uCode API values for HT/VHT bit rates

pub const TPC_MAX_REDUCTION: c_int = 15;
pub const TPC_NO_REDUCTION: c_int = 0;
pub const TPC_INVALID: c_uint = 0xff;

// load per tid defines for A-MPDU activation
pub const IWL_AGG_TPT_THREHOLD: c_int = 0;
pub const IWL_AGG_ALL_TID: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_table_type {
    LQ_NONE,
    LQ_LEGACY_G,	/* legacy types */
    LQ_LEGACY_A,
    LQ_HT_SISO,	/* HT types */
    LQ_HT_MIMO2,
    LQ_VHT_SISO,    /* VHT types */
    LQ_VHT_MIMO2,
    LQ_HE_SISO,     /* HE types */
    LQ_HE_MIMO2,
    LQ_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rs_rate {
    pub index: c_int,
    pub type: iwl_table_type,
    pub ant: u8,
    pub bw: u32,
    pub sgi: bool,
    pub ldpc: bool,
    pub stbc: bool,
    pub bfer: bool,
}

//
// struct iwl_lq_sta_rs_fw - rate and related statistics for RS in FW
// @last_rate_n_flags: last rate reported by FW
// @pers: persistent fields
// @pers.sta_id: the id of the station
// @pers.chains: bitmask of chains reported in %chain_signal
// @pers.chain_signal: per chain signal strength
// @pers.last_rssi: last rssi reported
// @pers.drv: pointer back to the driver data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lq_sta_rs_fw {
// last tx rate_n_flags
    pub last_rate_n_flags: u32,
// persistent fields - initialized only once - keep last!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lq_sta_pers_rs_fw {
    pub sta_id: u32,

//
// @pers.dbg_fixed_rate: for debug, use fixed rate if not 0
//
    pub dbg_fixed_rate: u32,
//
// @pers.dbg_agg_frame_count_lim: for debug, max number of
// frames in A-MPDU
//
    pub dbg_agg_frame_count_lim: u16,

    pub chains: u8,
    pub chain_signal: [i8; IEEE80211_MAX_CHAINS],
    pub last_rssi: i8,
    pub drv: *mut iwl_mvm,
    pub pers: },
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
}

// Possible Tx columns
// Tx Column = a combo of legacy/siso/mimo x antenna x SGI
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rs_column {
    RS_COLUMN_LEGACY_ANT_A = 0,
    RS_COLUMN_LEGACY_ANT_B,
    RS_COLUMN_SISO_ANT_A,
    RS_COLUMN_SISO_ANT_B,
    RS_COLUMN_SISO_ANT_A_SGI,
    RS_COLUMN_SISO_ANT_B_SGI,
    RS_COLUMN_MIMO2,
    RS_COLUMN_MIMO2_SGI,

    RS_COLUMN_LAST = RS_COLUMN_MIMO2_SGI,
    RS_COLUMN_COUNT = RS_COLUMN_LAST + 1,
    RS_COLUMN_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rs_ss_force_opt {
    RS_SS_FORCE_NONE = 0,
    RS_SS_FORCE_STBC,
    RS_SS_FORCE_BFER,
    RS_SS_FORCE_SISO,
}

// Packet stats per rate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rs_rate_stats {
    pub success: u64,
    pub total: u64,
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
    pub rate: rs_rate,
    pub column: rs_column,
    pub /: *const *const *const u16 expected_tpt; / throughput metrics; expected_tpt_G, etc.,
    pub /: *mut *mut iwl_rate_scale_data win[IWL_RATE_COUNT]; / rate histories,
// per txpower-reduction history
    pub 1]: iwl_rate_scale_data tpc_win[TPC_MAX_REDUCTION +,
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
    pub /: *mut *mut *mut u8 rs_state; / RS_STATE_,
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
    pub were: *mut *mut u32 visited_columns; / Bitmask marking which Tx columns,
// explored during a search cycle
//
    pub last_tx: u64,
    pub is_vht: bool,
    pub /: *mut *mut bool ldpc; / LDPC Rx is supported by the STA,
    pub /: *mut *mut bool stbc_capable; / Tx STBC is supported by chip and Rx by STA,
    pub /: *mut *mut bool bfer_capable; / Remote supports beamformee and we BFer,
    pub band: nl80211_band,
// The following are bitmaps of rates; IWL_RATE_6M_MASK, etc.
    pub active_legacy_rate: c_ulong,
    pub active_siso_rate: c_ulong,
    pub active_mimo2_rate: c_ulong,
// Highest rate per Tx mode
    pub max_legacy_rate_idx: u8,
    pub max_siso_rate_idx: u8,
    pub max_mimo2_rate_idx: u8,
// Optimal rate based on RSSI and STA caps.
// Used only to reflect link speed to userspace.
//
    pub optimal_rate: rs_rate,
    pub optimal_rate_mask: c_ulong,
    pub optimal_rates: *const rs_init_rate_info,
    pub optimal_nentries: c_int,
    pub missed_rate_counter: u8,
    pub lq: iwl_lq_cmd,
    pub /: *mut *mut iwl_scale_tbl_info lq_info[LQ_SIZE]; / "active", "search",
    pub tx_agg_tid_en: u8,
// last tx rate_n_flags
    pub last_rate_n_flags: u32,
// packets destined for this STA are aggregated
    pub is_agg: u8,
// tx power reduce for this sta
    pub tpc_reduce: c_int,
// persistent fields - initialized only once - keep last!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lq_sta_pers {

    pub dbg_fixed_rate: u32,
    pub dbg_fixed_txp_reduction: u8,
// force STBC/BFER/SISO for testing
    pub ss_force: rs_ss_force_opt,

    pub chains: u8,
    pub chain_signal: [i8; IEEE80211_MAX_CHAINS],
    pub last_rssi: i8,
    pub max_agg_bufsize: u16,
    pub tx_stats: [rs_rate_stats; RS_COLUMN_COUNT][IWL_RATE_COUNT],
    pub drv: *mut iwl_mvm,
    pub /: *mut *mut spinlock_t lock; / for races in reinit/update table,
    pub pers: },
}

// ieee80211_tx_info's status_driver_data[0] is packed with lq color and txp
// Note, it's iwlmvm <-> mac80211 interface.
// bits 0-7: reduced tx power
// bits 8-10: LQ command's color
//
pub const RS_DRV_DATA_TXP_MSK: c_uint = 0xff;
pub const RS_DRV_DATA_LQ_COLOR_POS: c_int = 8;

// Initialize station's rate scaling information after adding station
// Notify RS about Tx status
//
// iwl_mvm_rate_control_register - Register the rate control algorithm callbacks
//
// Since the rate control algorithm is hardware specific, there is no need
// or reason to place it as a stand alone module.  The driver can call
// iwl_rate_control_register in order to register the rate control callbacks
// with the mac80211 subsystem.  This should be performed prior to calling
// ieee80211_register_hw
//
// Return: negative error code, or 0 on success
//
extern "C" {
    pub fn iwl_mvm_rate_control_register() -> c_int;
}
//
// iwl_mvm_rate_control_unregister - Unregister the rate control callbacks
//
// This should be called after calling ieee80211_unregister_hw, but before
// the driver is unloaded.
//
extern "C" {
    pub fn iwl_mvm_rate_control_unregister();
}

extern "C" {
    pub fn iwl_mvm_reset_frame_stats(mvm: *mut iwl_mvm);
}

extern "C" {
    pub fn iwl_mvm_rs_add_sta(mvm: *mut iwl_mvm, mvmsta: *mut iwl_mvm_sta);
}
