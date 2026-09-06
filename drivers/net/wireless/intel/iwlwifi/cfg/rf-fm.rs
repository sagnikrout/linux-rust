//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/cfg/rf-fm.c
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
// Copyright (C) 2015-2017 Intel Deutschland GmbH
// Copyright (C) 2018-2026 Intel Corporation
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
    .eht_supported = true,						\
    .num_rbds = IWL_NUM_RBDS_EHT,					\
    .nvm_type = IWL_NVM_EXT
    const struct iwl_rf_cfg iwl_rf_fm = {
    IWL_DEVICE_FM,
    };
    const struct iwl_rf_cfg iwl_rf_fm_160mhz = {
    IWL_DEVICE_FM,
    .bw_limit = 160,
    };
    const char iwl_killer_be1750s_name[] =
    "Killer(R) Wi-Fi 7 BE1750s 320MHz Wireless Network Adapter (BE201D2W)";
    const char iwl_killer_be1750i_name[] =
    "Killer(R) Wi-Fi 7 BE1750i 320MHz Wireless Network Adapter (BE201NGW)";
    const char iwl_killer_be1750w_name[] =
    "Killer(TM) Wi-Fi 7 BE1750w 320MHz Wireless Network Adapter (BE200D2W)";
    const char iwl_killer_be1750x_name[] =
    "Killer(TM) Wi-Fi 7 BE1750x 320MHz Wireless Network Adapter (BE200NGW)";
    const char iwl_killer_be1790s_name[] =
    "Killer(R) Wi-Fi 7 BE1790s 320MHz Wireless Network Adapter (BE401D2W)";
    const char iwl_killer_be1790i_name[] =
    "Killer(R) Wi-Fi 7 BE1790i 320MHz Wireless Network Adapter (BE401NGW)";
    const char iwl_killer_be1730x_name[] =
    "Killer(TM) Wi-Fi 7 BE1730x 160MHz Wireless Network Adapter (BE202)";
    const char iwl_be201_name[] = "Intel(R) Wi-Fi 7 BE201 320MHz";
    const char iwl_be200_name[] = "Intel(R) Wi-Fi 7 BE200 320MHz";
    const char iwl_be202_name[] = "Intel(R) Wi-Fi 7 BE202 160MHz";
    const char iwl_be401_name[] = "Intel(R) Wi-Fi 7 BE401 320MHz";
