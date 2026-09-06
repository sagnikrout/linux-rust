//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/cfg/rf-wh.c
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
// Copyright (C) 2025-2026 Intel Corporation
//

    .ht_params = {							\
    .stbc = true,						\
    .ldpc = true,						\
    .ht40_bands = BIT(NL80211_BAND_2GHZ) |			\
    BIT(NL80211_BAND_5GHZ),			\
    },								\
    .led_mode = IWL_LED_RF_STATE,					\
    .non_shared_ant = ANT_B,					\
    .vht_mu_mimo_supported = true,					\
    .uhb_supported = true,						\
    .num_rbds = IWL_NUM_RBDS_EHT,					\
    .nvm_type = IWL_NVM_EXT
// currently iwl_rf_wh/iwl_rf_wh_160mhz are just defines for the FM ones
    const struct iwl_rf_cfg iwl_rf_wh_non_eht = {
    IWL_DEVICE_WH,
    .eht_supported = false,
    };
    const char iwl_killer_be1775s_name[] =
    "Killer(R) Wi-Fi 7 BE1775s 320MHz Wireless Network Adapter (BE211D2W)";
    const char iwl_killer_be1775i_name[] =
    "Killer(R) Wi-Fi 7 BE1775i 320MHz Wireless Network Adapter (BE211NGW)";
    const char iwl_killer_be1735x_name[] =
    "Killer(TM) Wi-Fi 7 BE1735x 160MHz Wireless Network Adapter (BE213)";
    const char iwl_be211_name[] = "Intel(R) Wi-Fi 7 BE211 320MHz";
    const char iwl_be213_name[] = "Intel(R) Wi-Fi 7 BE213 160MHz";
    const char iwl_ax221_name[] = "Intel(R) Wi-Fi 6E AX221 160MHz";
