//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/cfg/rf-hr.c
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
pub const IWL_HR_UCODE_API_MAX: c_int = 100;
// Lowest firmware API version supported
pub const IWL_HR_UCODE_API_MIN: c_int = 100;

    IWL_BZ_A_HR_B_FW_PRE "-" __stringify(api) ".ucode"

    IWL_SC_A_HR_A_FW_PRE "-" __stringify(api) ".ucode"

    IWL_SC_A_HR_B_FW_PRE "-" __stringify(api) ".ucode"

    .led_mode = IWL_LED_RF_STATE,					\
    .non_shared_ant = ANT_B,					\
    .vht_mu_mimo_supported = true,					\
    .ht_params = {							\
    .stbc = true,						\
    .ldpc = true,						\
    .ht40_bands = BIT(NL80211_BAND_2GHZ) |			\
    BIT(NL80211_BAND_5GHZ),			\
    },								\
    .num_rbds = IWL_NUM_RBDS_HE,					\
    .nvm_type = IWL_NVM_EXT,					\
    .ucode_api_min = IWL_HR_UCODE_API_MIN,				\
    .ucode_api_max = IWL_HR_UCODE_API_MAX
    const struct iwl_rf_cfg iwl_rf_hr1 = {
    IWL_DEVICE_HR,
    .tx_with_siso_diversity = true,
    };
    const struct iwl_rf_cfg iwl_rf_hr = {
    IWL_DEVICE_HR,
    };
    const struct iwl_rf_cfg iwl_rf_hr_80mhz = {
    IWL_DEVICE_HR,
    .bw_limit = 80,
    };
    const char iwl_ax101_name[] = "Intel(R) Wi-Fi 6 AX101";
    const char iwl_ax200_name[] = "Intel(R) Wi-Fi 6 AX200 160MHz";
    const char iwl_ax201_name[] = "Intel(R) Wi-Fi 6 AX201 160MHz";
    const char iwl_ax203_name[] = "Intel(R) Wi-Fi 6 AX203";
    MODULE_FIRMWARE(IWL_BZ_A_HR_B_MODULE_FIRMWARE(IWL_HR_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL_SC_A_HR_A_FW_MODULE_FIRMWARE(IWL_HR_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL_SC_A_HR_B_FW_MODULE_FIRMWARE(IWL_HR_UCODE_API_MAX));
