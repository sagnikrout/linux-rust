//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mvm/constants.h
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
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2013-2014, 2018-2025 Intel Corporation
// Copyright (C) 2015 Intel Deutschland GmbH
//

pub const IWL_MVM_UAPSD_NOAGG_BSSIDS_NUM: c_int = 20;
pub const IWL_MVM_TRIGGER_LINK_SEL_TIME_SEC: c_int = 30;

pub const IWL_MVM_P2P_LOWLATENCY_PS_ENABLE: c_int = 1;

pub const IWL_MVM_PS_HEAVY_TX_THLD_PACKETS: c_int = 20;
pub const IWL_MVM_PS_HEAVY_RX_THLD_PACKETS: c_int = 8;
pub const IWL_MVM_PS_SNOOZE_HEAVY_TX_THLD_PACKETS: c_int = 30;
pub const IWL_MVM_PS_SNOOZE_HEAVY_RX_THLD_PACKETS: c_int = 20;
pub const IWL_MVM_PS_HEAVY_TX_THLD_PERCENT: c_int = 50;
pub const IWL_MVM_PS_HEAVY_RX_THLD_PERCENT: c_int = 50;
pub const IWL_MVM_PS_SNOOZE_INTERVAL: c_int = 25;
pub const IWL_MVM_PS_SNOOZE_WINDOW: c_int = 50;
pub const IWL_MVM_WOWLAN_PS_SNOOZE_WINDOW: c_int = 25;
pub const IWL_MVM_LOWLAT_QUOTA_MIN_PERCENT: c_int = 64;
pub const IWL_MVM_BT_COEX_EN_RED_TXP_THRESH: c_int = 62;
pub const IWL_MVM_BT_COEX_DIS_RED_TXP_THRESH: c_int = 65;
pub const IWL_MVM_BT_COEX_SYNC2SCO: c_int = 1;
pub const IWL_MVM_BT_COEX_MPLUT: c_int = 1;
pub const IWL_MVM_BT_COEX_RRC: c_int = 1;
pub const IWL_MVM_BT_COEX_TTC: c_int = 1;
pub const IWL_MVM_BT_COEX_MPLUT_REG0: c_uint = 0x22002200;
pub const IWL_MVM_BT_COEX_MPLUT_REG1: c_uint = 0x11118451;
pub const IWL_MVM_BT_COEX_ANTENNA_COUPLING_THRS: c_int = 30;
pub const IWL_MVM_FW_MCAST_FILTER_PASS_ALL: c_int = 0;
pub const IWL_MVM_FW_BCAST_FILTER_PASS_ALL: c_int = 0;
pub const IWL_MVM_QUOTA_THRESHOLD: c_int = 4;
pub const IWL_MVM_RS_RSSI_BASED_INIT_RATE: c_int = 0;
pub const IWL_MVM_RS_80_20_FAR_RANGE_TWEAK: c_int = 1;
pub const IWL_MVM_TOF_IS_RESPONDER: c_int = 0;
pub const IWL_MVM_ADWELL_ENABLE: c_int = 1;
pub const IWL_MVM_ADWELL_MAX_BUDGET: c_int = 0;

pub const IWL_MVM_NON_TRANSMITTING_AP: c_int = 0;
pub const IWL_MVM_CONN_LISTEN_INTERVAL: c_int = 10;
pub const IWL_MVM_RS_NUM_TRY_BEFORE_ANT_TOGGLE: c_int = 1;
pub const IWL_MVM_RS_HT_VHT_RETRIES_PER_RATE: c_int = 2;
pub const IWL_MVM_RS_HT_VHT_RETRIES_PER_RATE_TW: c_int = 1;
pub const IWL_MVM_RS_INITIAL_MIMO_NUM_RATES: c_int = 3;
pub const IWL_MVM_RS_INITIAL_SISO_NUM_RATES: c_int = 3;
pub const IWL_MVM_RS_INITIAL_LEGACY_NUM_RATES: c_int = 2;
pub const IWL_MVM_RS_INITIAL_LEGACY_RETRIES: c_int = 2;
pub const IWL_MVM_RS_SECONDARY_LEGACY_RETRIES: c_int = 1;
pub const IWL_MVM_RS_SECONDARY_LEGACY_NUM_RATES: c_int = 16;
pub const IWL_MVM_RS_SECONDARY_SISO_NUM_RATES: c_int = 3;
pub const IWL_MVM_RS_SECONDARY_SISO_RETRIES: c_int = 1;
pub const IWL_MVM_RS_RATE_MIN_FAILURE_TH: c_int = 3;
pub const IWL_MVM_RS_RATE_MIN_SUCCESS_TH: c_int = 8;

pub const IWL_MVM_RS_MISSED_RATE_MAX: c_int = 15;
pub const IWL_MVM_RS_LEGACY_FAILURE_LIMIT: c_int = 160;
pub const IWL_MVM_RS_LEGACY_SUCCESS_LIMIT: c_int = 480;
pub const IWL_MVM_RS_LEGACY_TABLE_COUNT: c_int = 160;
pub const IWL_MVM_RS_NON_LEGACY_FAILURE_LIMIT: c_int = 400;
pub const IWL_MVM_RS_NON_LEGACY_SUCCESS_LIMIT: c_int = 4500;
pub const IWL_MVM_RS_NON_LEGACY_TABLE_COUNT: c_int = 1500;

pub const IWL_MVM_RS_AGG_DISABLE_START: c_int = 3;

pub const IWL_MVM_RS_TPC_TX_POWER_STEP: c_int = 3;
pub const IWL_MVM_ENABLE_EBS: c_int = 1;

pub const IWL_MVM_FTM_R2I_MAX_REP: c_int = 7;
pub const IWL_MVM_FTM_I2R_MAX_REP: c_int = 7;
pub const IWL_MVM_FTM_R2I_MAX_STS: c_int = 1;
pub const IWL_MVM_FTM_I2R_MAX_STS: c_int = 1;
pub const IWL_MVM_FTM_R2I_MAX_TOTAL_LTF: c_int = 3;
pub const IWL_MVM_FTM_I2R_MAX_TOTAL_LTF: c_int = 3;

pub const IWL_MVM_FTM_NON_TB_MIN_TIME_BETWEEN_MSR: c_int = 7;
pub const IWL_MVM_FTM_NON_TB_MAX_TIME_BETWEEN_MSR: c_int = 1000;

pub const IWL_MVM_AMPDU_CONSEC_DROPS_DELBA: c_int = 20;

pub const IWL_MVM_FTM_INITIATOR_SMOOTH_ALPHA: c_int = 40;
// 20016 pSec is 6 meter RTT, meaning 3 meter range
pub const IWL_MVM_FTM_INITIATOR_SMOOTH_UNDERSHOOT: c_int = 20016;
pub const IWL_MVM_FTM_INITIATOR_SMOOTH_OVERSHOOT: c_int = 20016;
pub const IWL_MVM_FTM_INITIATOR_SMOOTH_AGE_SEC: c_int = 2;

pub const IWL_MVM_MIN_BEACON_INTERVAL_TU: c_int = 16;

