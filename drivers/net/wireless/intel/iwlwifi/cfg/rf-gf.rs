//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/cfg/rf-gf.c
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

// Highest firmware API version supported
pub const IWL_GF_UCODE_API_MAX: c_int = 100;
// Lowest firmware API version supported
pub const IWL_GF_UCODE_API_MIN: c_int = 100;

    IWL_BZ_A_GF_A_FW_PRE "-" __stringify(api) ".ucode"

    IWL_BZ_A_GF4_A_FW_PRE "-" __stringify(api) ".ucode"

    IWL_SC_A_GF_A_FW_PRE "-" __stringify(api) ".ucode"

    IWL_SC_A_GF4_A_FW_PRE "-" __stringify(api) ".ucode"
    const struct iwl_rf_cfg iwl_rf_gf = {
    .uhb_supported = true,
    .led_mode = IWL_LED_RF_STATE,
    .non_shared_ant = ANT_B,
    .vht_mu_mimo_supported = true,
    .ht_params = {
    .stbc = true,
    .ldpc = true,
    .ht40_bands = BIT(NL80211_BAND_2GHZ) |
    BIT(NL80211_BAND_5GHZ),
    },
    .nvm_type = IWL_NVM_EXT,
    .num_rbds = IWL_NUM_RBDS_HE,
    .ucode_api_min = IWL_GF_UCODE_API_MIN,
    .ucode_api_max = IWL_GF_UCODE_API_MAX,
    };
    const char iwl_ax210_killer_1675w_name[] =
    "Killer(R) Wi-Fi 6E AX1675w 160MHz Wireless Network Adapter (210D2W)";
    const char iwl_ax210_killer_1675x_name[] =
    "Killer(R) Wi-Fi 6E AX1675x 160MHz Wireless Network Adapter (210NGW)";
    const char iwl_ax211_killer_1675s_name[] =
    "Killer(R) Wi-Fi 6E AX1675s 160MHz Wireless Network Adapter (211D2W)";
    const char iwl_ax211_killer_1675i_name[] =
    "Killer(R) Wi-Fi 6E AX1675i 160MHz Wireless Network Adapter (211NGW)";
    const char iwl_ax411_killer_1690s_name[] =
    "Killer(R) Wi-Fi 6E AX1690s 160MHz Wireless Network Adapter (411D2W)";
    const char iwl_ax411_killer_1690i_name[] =
    "Killer(R) Wi-Fi 6E AX1690i 160MHz Wireless Network Adapter (411NGW)";
    const char iwl_ax210_name[] = "Intel(R) Wi-Fi 6E AX210 160MHz";
    const char iwl_ax211_name[] = "Intel(R) Wi-Fi 6E AX211 160MHz";
    const char iwl_ax231_name[] = "Intel(R) Wi-Fi 6 AX231";
    const char iwl_ax411_name[] = "Intel(R) Wi-Fi 6E AX411 160MHz";
    MODULE_FIRMWARE(IWL_BZ_A_GF_A_MODULE_FIRMWARE(IWL_GF_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL_BZ_A_GF4_A_MODULE_FIRMWARE(IWL_GF_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL_SC_A_GF_A_MODULE_FIRMWARE(IWL_GF_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL_SC_A_GF4_A_MODULE_FIRMWARE(IWL_GF_UCODE_API_MAX));
