//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/constants.h
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
// Copyright (C) 2024-2026 Intel Corporation
//

// Macro flag: #define __iwl_mld_constants_h__
pub const IWL_MLD_MISSED_BEACONS_SINCE_RX_THOLD: c_int = 6;
pub const IWL_MLD_MISSED_BEACONS_THRESHOLD: c_int = 8;
pub const IWL_MLD_MISSED_BEACONS_THRESHOLD_LONG: c_int = 19;
pub const IWL_MLD_BCN_LOSS_EXIT_ESR_THRESH_2_LINKS: c_int = 5;
pub const IWL_MLD_BCN_LOSS_EXIT_ESR_THRESH: c_int = 15;
pub const IWL_MLD_BCN_LOSS_EXIT_ESR_THRESH_BSS_PARAM_CHANGED: c_int = 11;

pub const IWL_MLD_PS_SNOOZE_INTERVAL: c_int = 25;
pub const IWL_MLD_PS_SNOOZE_INTERVAL: c_int = 25;
pub const IWL_MLD_PS_SNOOZE_WINDOW: c_int = 50;
pub const IWL_MLD_PS_SNOOZE_HEAVY_TX_THLD_PACKETS: c_int = 30;
pub const IWL_MLD_PS_SNOOZE_HEAVY_RX_THLD_PACKETS: c_int = 20;
pub const IWL_MLD_PS_HEAVY_TX_THLD_PERCENT: c_int = 50;
pub const IWL_MLD_PS_HEAVY_RX_THLD_PERCENT: c_int = 50;
pub const IWL_MLD_PS_HEAVY_TX_THLD_PACKETS: c_int = 20;
pub const IWL_MLD_PS_HEAVY_RX_THLD_PACKETS: c_int = 8;
pub const IWL_MLD_TRIGGER_LINK_SEL_TIME_SEC: c_int = 30;

pub const IWL_MLD_CONN_LISTEN_INTERVAL: c_int = 10;
pub const IWL_MLD_ADAPTIVE_DWELL_NUM_APS_OVERRIDE: c_int = 0;

pub const IWL_MLD_ENTER_EMLSR_TPT_THRESH: c_int = 400;

pub const IWL_MLD_FTM_R2I_MAX_REP: c_int = 7;
pub const IWL_MLD_FTM_I2R_MAX_REP: c_int = 7;
pub const IWL_MLD_FTM_R2I_MAX_STS: c_int = 1;
pub const IWL_MLD_FTM_I2R_MAX_STS: c_int = 1;
pub const IWL_MLD_FTM_R2I_MAX_TOTAL_LTF: c_int = 3;
pub const IWL_MLD_FTM_I2R_MAX_TOTAL_LTF: c_int = 3;

pub const IWL_MLD_FTM_NON_TB_MIN_TIME_BETWEEN_MSR: c_int = 7;
pub const IWL_MLD_FTM_NON_TB_MAX_TIME_BETWEEN_MSR: c_int = 1000;
pub const IWL_MLD_STA_EXT_CAPA_SIZE: c_int = 9;
pub const IWL_MLD_EXT_CAPA_NUM_IFTYPES: c_int = 1;
