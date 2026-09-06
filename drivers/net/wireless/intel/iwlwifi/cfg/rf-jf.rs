//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/cfg/rf-jf.c
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
// Copyright (C) 2018-2021, 2023, 2025 Intel Corporation
//

// Highest firmware API version supported
pub const IWL_JF_UCODE_API_MAX: c_int = 77;
// Lowest firmware API version supported
pub const IWL_JF_UCODE_API_MIN: c_int = 77;

    IWL_QUZ_A_JF_B_FW_PRE "-" __stringify(api) ".ucode"

    IWL_QU_B_JF_B_FW_PRE "-" __stringify(api) ".ucode"

    IWL_QU_C_JF_B_FW_PRE "-" __stringify(api) ".ucode"

    IWL_SO_A_JF_B_FW_PRE "-" __stringify(api) ".ucode"
// NVM versions
pub const IWL_JF_NVM_VERSION: c_uint = 0x0a1d;
// Memory offsets and lengths
pub const IWL9000_DCCM_OFFSET: c_uint = 0x800000;
pub const IWL9000_DCCM_LEN: c_uint = 0x18000;
pub const IWL9000_DCCM2_OFFSET: c_uint = 0x880000;
pub const IWL9000_DCCM2_LEN: c_uint = 0x8000;
    static const struct iwl_tt_params iwl_jf_tt_params = {
    .ct_kill_entry = 115,
    .ct_kill_exit = 93,
    .ct_kill_duration = 5,
    .dynamic_smps_entry = 111,
    .dynamic_smps_exit = 107,
    .tx_protection_entry = 112,
    .tx_protection_exit = 105,
    .tx_backoff = {
    {.temperature = 110, .backoff = 200},
    {.temperature = 111, .backoff = 600},
    {.temperature = 112, .backoff = 1200},
    {.temperature = 113, .backoff = 2000},
    {.temperature = 114, .backoff = 4000},
    },
    .support_ct_kill = true,
    .support_dynamic_smps = true,
    .support_tx_protection = true,
    .support_tx_backoff = true,
    };
// these values are ignored if not with Pu/Th MAC firmware, due to offload

    .dccm_offset = IWL9000_DCCM_OFFSET,				\
    .dccm_len = IWL9000_DCCM_LEN,					\
    .dccm2_offset = IWL9000_DCCM2_OFFSET,				\
    .dccm2_len = IWL9000_DCCM2_LEN,					\
    .thermal_params = &iwl_jf_tt_params

    IWL_DEVICE_JF_PU,						\
    .led_mode = IWL_LED_RF_STATE,					\
    .non_shared_ant = ANT_B,					\
    .num_rbds = IWL_NUM_RBDS_NON_HE,				\
    .vht_mu_mimo_supported = true,					\
    .ht_params = {							\
    .stbc = true,						\
    .ldpc = true,						\
    .ht40_bands = BIT(NL80211_BAND_2GHZ) |			\
    BIT(NL80211_BAND_5GHZ),			\
    },								\
    .nvm_ver = IWL_JF_NVM_VERSION,					\
    .nvm_type = IWL_NVM_EXT,					\
    .ucode_api_min = IWL_JF_UCODE_API_MIN,				\
    .ucode_api_max = IWL_JF_UCODE_API_MAX
    const struct iwl_rf_cfg iwl_rf_jf = {
    IWL_DEVICE_JF,
    };
    const struct iwl_rf_cfg iwl_rf_jf_80mhz = {
    IWL_DEVICE_JF,
    .bw_limit = 80,
    };
    const char iwl9260_name[] = "Intel(R) Wireless-AC 9260";
    const char iwl9461_name[] = "Intel(R) Wireless-AC 9461";
    const char iwl9462_name[] = "Intel(R) Wireless-AC 9462";
    const char iwl9560_name[] = "Intel(R) Wireless-AC 9560";
    const char iwl9260_160_name[] = "Intel(R) Wireless-AC 9260 160MHz";
    const char iwl9461_160_name[] = "Intel(R) Wireless-AC 9461 160MHz";
    const char iwl9462_160_name[] = "Intel(R) Wireless-AC 9462 160MHz";
    const char iwl9560_160_name[] = "Intel(R) Wireless-AC 9560 160MHz";
    const char iwl9260_killer_1550_name[] =
    "Killer(R) Wireless-AC 1550 Wireless Network Adapter (9260NGW) 160MHz";
    const char iwl9560_killer_1550i_name[] =
    "Killer(R) Wireless-AC 1550i Wireless Network Adapter (9560NGW) 160MHz";
    const char iwl9560_killer_1550s_name[] =
    "Killer(R) Wireless-AC 1550s Wireless Network Adapter (9560D2W) 160MHz";
    MODULE_FIRMWARE(IWL_QU_B_JF_B_MODULE_FIRMWARE(IWL_JF_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL_QU_C_JF_B_MODULE_FIRMWARE(IWL_JF_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL_QUZ_A_JF_B_MODULE_FIRMWARE(IWL_JF_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL_SO_A_JF_B_MODULE_FIRMWARE(IWL_JF_UCODE_API_MAX));
