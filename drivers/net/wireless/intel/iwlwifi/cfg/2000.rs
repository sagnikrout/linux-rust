//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/cfg/2000.c
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
// Copyright(c) 2008 - 2014 Intel Corporation. All rights reserved.
// Copyright(c) 2018 - 2020, 2023, 2025 Intel Corporation
//

// Highest firmware API version supported
pub const IWL2030_UCODE_API_MAX: c_int = 6;
pub const IWL2000_UCODE_API_MAX: c_int = 6;
pub const IWL105_UCODE_API_MAX: c_int = 6;
pub const IWL135_UCODE_API_MAX: c_int = 6;
// Lowest firmware API version supported
pub const IWL2030_UCODE_API_MIN: c_int = 5;
pub const IWL2000_UCODE_API_MIN: c_int = 5;
pub const IWL105_UCODE_API_MIN: c_int = 5;
pub const IWL135_UCODE_API_MIN: c_int = 5;
// EEPROM version

    static const struct iwl_family_base_params iwl2000_base = {
    .eeprom_size = OTP_LOW_IMAGE_SIZE_2K,
    .num_of_queues = IWLAGN_NUM_QUEUES,
    .max_tfd_queue_size = 256,
    .max_ll_items = OTP_MAX_LL_ITEMS_2x00,
    .shadow_ram_support = true,
    .led_compensation = 51,
    .wd_timeout = IWL_DEF_WD_TIMEOUT,
    .max_event_log_size = 512,
    .shadow_reg_enable = false, /* TODO: fix bugs using this feature */
    .scd_chain_ext_wa = true,
    };
    static const struct iwl_family_base_params iwl2030_base = {
    .eeprom_size = OTP_LOW_IMAGE_SIZE_2K,
    .num_of_queues = IWLAGN_NUM_QUEUES,
    .max_tfd_queue_size = 256,
    .max_ll_items = OTP_MAX_LL_ITEMS_2x00,
    .shadow_ram_support = true,
    .led_compensation = 57,
    .wd_timeout = IWL_LONG_WD_TIMEOUT,
    .max_event_log_size = 512,
    .shadow_reg_enable = false, /* TODO: fix bugs using this feature */
    .scd_chain_ext_wa = true,
    };
    static const struct iwl_eeprom_params iwl20x0_eeprom_params = {
    .regulatory_bands = {
    EEPROM_REG_BAND_1_CHANNELS,
    EEPROM_REG_BAND_2_CHANNELS,
    EEPROM_REG_BAND_3_CHANNELS,
    EEPROM_REG_BAND_4_CHANNELS,
    EEPROM_REG_BAND_5_CHANNELS,
    EEPROM_6000_REG_BAND_24_HT40_CHANNELS,
    EEPROM_REGULATORY_BAND_NO_HT40,
    },
    .enhanced_txpower = true,
    };
    const struct iwl_mac_cfg iwl2000_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_2000,
    .base = &iwl2000_base,
    };

    .fw_name_pre = IWL2000_FW_PRE,				\
    .ucode_api_max = IWL2000_UCODE_API_MAX,			\
    .ucode_api_min = IWL2000_UCODE_API_MIN,			\
    .max_inst_size = IWL60_RTC_INST_SIZE,			\
    .max_data_size = IWL60_RTC_DATA_SIZE,			\
    .nvm_ver = EEPROM_2000_EEPROM_VERSION,			\
    .nvm_calib_ver = EEPROM_2000_TX_POWER_VERSION,		\
    .eeprom_params = &iwl20x0_eeprom_params,		\
    .led_mode = IWL_LED_RF_STATE
    const struct iwl_rf_cfg iwl2000_2bgn_cfg = {
    IWL_DEVICE_2000,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ),
    },
    };
    const char iwl2000_2bgn_name[] = "Intel(R) Centrino(R) Wireless-N 2200 BGN";
    const char iwl2000_2bgn_d_name[] = "Intel(R) Centrino(R) Wireless-N 2200D BGN";
    const struct iwl_mac_cfg iwl2030_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_2030,
    .base = &iwl2030_base,
    };

    .fw_name_pre = IWL2030_FW_PRE,				\
    .ucode_api_max = IWL2030_UCODE_API_MAX,			\
    .ucode_api_min = IWL2030_UCODE_API_MIN,			\
    .max_inst_size = IWL60_RTC_INST_SIZE,			\
    .max_data_size = IWL60_RTC_DATA_SIZE,			\
    .nvm_ver = EEPROM_2000_EEPROM_VERSION,		\
    .nvm_calib_ver = EEPROM_2000_TX_POWER_VERSION,	\
    .eeprom_params = &iwl20x0_eeprom_params,		\
    .led_mode = IWL_LED_RF_STATE
    const struct iwl_rf_cfg iwl2030_2bgn_cfg = {
    IWL_DEVICE_2030,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ),
    },
    };
    const char iwl2030_2bgn_name[] = "Intel(R) Centrino(R) Wireless-N 2230 BGN";
    const struct iwl_mac_cfg iwl105_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_105,
    .base = &iwl2000_base,
    };

    .fw_name_pre = IWL105_FW_PRE,				\
    .ucode_api_max = IWL105_UCODE_API_MAX,			\
    .ucode_api_min = IWL105_UCODE_API_MIN,			\
    .max_inst_size = IWL60_RTC_INST_SIZE,			\
    .max_data_size = IWL60_RTC_DATA_SIZE,			\
    .nvm_ver = EEPROM_2000_EEPROM_VERSION,		\
    .nvm_calib_ver = EEPROM_2000_TX_POWER_VERSION,	\
    .eeprom_params = &iwl20x0_eeprom_params,		\
    .led_mode = IWL_LED_RF_STATE,				\
    .rx_with_siso_diversity = true
    const struct iwl_rf_cfg iwl105_bgn_cfg = {
    IWL_DEVICE_105,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ),
    },
    };
    const char iwl105_bgn_name[] = "Intel(R) Centrino(R) Wireless-N 105 BGN";
    const char iwl105_bgn_d_name[] = "Intel(R) Centrino(R) Wireless-N 105D BGN";
    const struct iwl_mac_cfg iwl135_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_135,
    .base = &iwl2030_base,
    };

    .fw_name_pre = IWL135_FW_PRE,				\
    .ucode_api_max = IWL135_UCODE_API_MAX,			\
    .ucode_api_min = IWL135_UCODE_API_MIN,			\
    .max_inst_size = IWL60_RTC_INST_SIZE,			\
    .max_data_size = IWL60_RTC_DATA_SIZE,			\
    .nvm_ver = EEPROM_2000_EEPROM_VERSION,		\
    .nvm_calib_ver = EEPROM_2000_TX_POWER_VERSION,	\
    .eeprom_params = &iwl20x0_eeprom_params,		\
    .led_mode = IWL_LED_RF_STATE,				\
    .rx_with_siso_diversity = true
    const struct iwl_rf_cfg iwl135_bgn_cfg = {
    IWL_DEVICE_135,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ),
    },
    };
    const char iwl135_bgn_name[] = "Intel(R) Centrino(R) Wireless-N 135 BGN";
    MODULE_FIRMWARE(IWL2000_MODULE_FIRMWARE(IWL2000_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL2030_MODULE_FIRMWARE(IWL2030_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL105_MODULE_FIRMWARE(IWL105_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL135_MODULE_FIRMWARE(IWL135_UCODE_API_MAX));
