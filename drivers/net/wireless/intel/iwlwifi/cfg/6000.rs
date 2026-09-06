//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/intel/iwlwifi/cfg/6000.c
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
pub const IWL6000_UCODE_API_MAX: c_int = 6;
pub const IWL6050_UCODE_API_MAX: c_int = 5;
pub const IWL6000G2_UCODE_API_MAX: c_int = 6;
pub const IWL6035_UCODE_API_MAX: c_int = 6;
// Lowest firmware API version supported
pub const IWL6000_UCODE_API_MIN: c_int = 4;
pub const IWL6050_UCODE_API_MIN: c_int = 4;
pub const IWL6000G2_UCODE_API_MIN: c_int = 5;
pub const IWL6035_UCODE_API_MIN: c_int = 6;
// EEPROM versions

    static const struct iwl_family_base_params iwl6000_base = {
    .eeprom_size = OTP_LOW_IMAGE_SIZE_2K,
    .num_of_queues = IWLAGN_NUM_QUEUES,
    .max_tfd_queue_size = 256,
    .max_ll_items = OTP_MAX_LL_ITEMS_6x00,
    .shadow_ram_support = true,
    .led_compensation = 51,
    .wd_timeout = IWL_DEF_WD_TIMEOUT,
    .max_event_log_size = 512,
    .shadow_reg_enable = false, /* TODO: fix bugs using this feature */
    .scd_chain_ext_wa = true,
    };
    static const struct iwl_family_base_params iwl6050_base = {
    .eeprom_size = OTP_LOW_IMAGE_SIZE_2K,
    .num_of_queues = IWLAGN_NUM_QUEUES,
    .max_tfd_queue_size = 256,
    .max_ll_items = OTP_MAX_LL_ITEMS_6x50,
    .shadow_ram_support = true,
    .led_compensation = 51,
    .wd_timeout = IWL_DEF_WD_TIMEOUT,
    .max_event_log_size = 1024,
    .shadow_reg_enable = false, /* TODO: fix bugs using this feature */
    .scd_chain_ext_wa = true,
    };
    static const struct iwl_family_base_params iwl6000_g2_base = {
    .eeprom_size = OTP_LOW_IMAGE_SIZE_2K,
    .num_of_queues = IWLAGN_NUM_QUEUES,
    .max_tfd_queue_size = 256,
    .max_ll_items = OTP_MAX_LL_ITEMS_6x00,
    .shadow_ram_support = true,
    .led_compensation = 57,
    .wd_timeout = IWL_LONG_WD_TIMEOUT,
    .max_event_log_size = 512,
    .shadow_reg_enable = false, /* TODO: fix bugs using this feature */
    .scd_chain_ext_wa = true,
    };
    static const struct iwl_eeprom_params iwl6000_eeprom_params = {
    .regulatory_bands = {
    EEPROM_REG_BAND_1_CHANNELS,
    EEPROM_REG_BAND_2_CHANNELS,
    EEPROM_REG_BAND_3_CHANNELS,
    EEPROM_REG_BAND_4_CHANNELS,
    EEPROM_REG_BAND_5_CHANNELS,
    EEPROM_6000_REG_BAND_24_HT40_CHANNELS,
    EEPROM_REG_BAND_52_HT40_CHANNELS
    },
    .enhanced_txpower = true,
    };
    const struct iwl_mac_cfg iwl6005_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_6005,
    .base = &iwl6000_g2_base,
    };

    .fw_name_pre = IWL6005_FW_PRE,				\
    .ucode_api_max = IWL6000G2_UCODE_API_MAX,		\
    .ucode_api_min = IWL6000G2_UCODE_API_MIN,		\
    .max_inst_size = IWL60_RTC_INST_SIZE,			\
    .max_data_size = IWL60_RTC_DATA_SIZE,			\
    .nvm_ver = EEPROM_6005_EEPROM_VERSION,		\
    .nvm_calib_ver = EEPROM_6005_TX_POWER_VERSION,	\
    .eeprom_params = &iwl6000_eeprom_params,		\
    .led_mode = IWL_LED_RF_STATE
    const struct iwl_rf_cfg iwl6005_n_cfg = {
    IWL_DEVICE_6005,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ) | BIT(NL80211_BAND_5GHZ),
    },
    };
    const char iwl6005_2agn_name[] = "Intel(R) Centrino(R) Advanced-N 6205 AGN";
    const char iwl6005_2agn_sff_name[] = "Intel(R) Centrino(R) Advanced-N 6205S AGN";
    const char iwl6005_2agn_d_name[] = "Intel(R) Centrino(R) Advanced-N 6205D AGN";
    const char iwl6005_2agn_mow1_name[] = "Intel(R) Centrino(R) Advanced-N 6206 AGN";
    const char iwl6005_2agn_mow2_name[] = "Intel(R) Centrino(R) Advanced-N 6207 AGN";
    const struct iwl_rf_cfg iwl6005_non_n_cfg = {
    IWL_DEVICE_6005,
    };
    const char iwl6005_2abg_name[] = "Intel(R) Centrino(R) Advanced-N 6205 ABG";
    const char iwl6005_2bg_name[] = "Intel(R) Centrino(R) Advanced-N 6205 BG";
    const struct iwl_mac_cfg iwl6030_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_6030,
    .base = &iwl6000_g2_base,
    };

    .fw_name_pre = IWL6030_FW_PRE,				\
    .ucode_api_max = IWL6000G2_UCODE_API_MAX,		\
    .ucode_api_min = IWL6000G2_UCODE_API_MIN,		\
    .max_inst_size = IWL60_RTC_INST_SIZE,			\
    .max_data_size = IWL60_RTC_DATA_SIZE,			\
    .nvm_ver = EEPROM_6030_EEPROM_VERSION,		\
    .nvm_calib_ver = EEPROM_6030_TX_POWER_VERSION,	\
    .eeprom_params = &iwl6000_eeprom_params,		\
    .led_mode = IWL_LED_RF_STATE
    const struct iwl_rf_cfg iwl6030_n_cfg = {
    IWL_DEVICE_6030,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ) | BIT(NL80211_BAND_5GHZ),
    },
    };
    const char iwl6030_2agn_name[] = "Intel(R) Centrino(R) Advanced-N 6230 AGN";
    const char iwl6030_2bgn_name[] = "Intel(R) Centrino(R) Advanced-N 6230 BGN";
    const char iwl1030_bgn_name[] = "Intel(R) Centrino(R) Wireless-N 1030 BGN";
    const char iwl1030_bg_name[] = "Intel(R) Centrino(R) Wireless-N 1030 BG";
    const struct iwl_rf_cfg iwl6030_non_n_cfg = {
    IWL_DEVICE_6030,
    };
    const char iwl6030_2abg_name[] = "Intel(R) Centrino(R) Advanced-N 6230 ABG";
    const char iwl6030_2bg_name[] = "Intel(R) Centrino(R) Advanced-N 6230 BG";

    .fw_name_pre = IWL6030_FW_PRE,				\
    .ucode_api_max = IWL6035_UCODE_API_MAX,			\
    .ucode_api_min = IWL6035_UCODE_API_MIN,			\
    .max_inst_size = IWL60_RTC_INST_SIZE,			\
    .max_data_size = IWL60_RTC_DATA_SIZE,			\
    .nvm_ver = EEPROM_6030_EEPROM_VERSION,		\
    .nvm_calib_ver = EEPROM_6030_TX_POWER_VERSION,	\
    .eeprom_params = &iwl6000_eeprom_params,		\
    .led_mode = IWL_LED_RF_STATE
    const struct iwl_rf_cfg iwl6035_2agn_cfg = {
    IWL_DEVICE_6035,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ) | BIT(NL80211_BAND_5GHZ),
    },
    };
    const char iwl6035_2agn_name[] = "Intel(R) Centrino(R) Advanced-N 6235 AGN";
    const char iwl6035_2agn_sff_name[] = "Intel(R) Centrino(R) Ultimate-N 6235 AGN";
    const struct iwl_rf_cfg iwl130_bgn_cfg = {
    IWL_DEVICE_6030,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ) | BIT(NL80211_BAND_5GHZ),
    },
    .rx_with_siso_diversity = true,
    };
    const char iwl130_bgn_name[] = "Intel(R) Centrino(R) Wireless-N 130 BGN";
    const struct iwl_rf_cfg iwl130_bg_cfg = {
    IWL_DEVICE_6030,
    .rx_with_siso_diversity = true,
    };
    const char iwl130_bg_name[] = "Intel(R) Centrino(R) Wireless-N 130 BG";
    const struct iwl_mac_cfg iwl6000i_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_6000i,
    .base = &iwl6000_base,
    };
//
// "i": Internal configuration, use internal Power Amplifier
//

    .fw_name_pre = IWL6000_FW_PRE,				\
    .ucode_api_max = IWL6000_UCODE_API_MAX,			\
    .ucode_api_min = IWL6000_UCODE_API_MIN,			\
    .max_inst_size = IWL60_RTC_INST_SIZE,			\
    .max_data_size = IWL60_RTC_DATA_SIZE,			\
    .valid_tx_ant = ANT_BC,		/* .cfg overwrite */	\
    .valid_rx_ant = ANT_BC,		/* .cfg overwrite */	\
    .nvm_ver = EEPROM_6000_EEPROM_VERSION,		\
    .nvm_calib_ver = EEPROM_6000_TX_POWER_VERSION,	\
    .eeprom_params = &iwl6000_eeprom_params,		\
    .led_mode = IWL_LED_BLINK
    const struct iwl_rf_cfg iwl6000i_2agn_cfg = {
    IWL_DEVICE_6000i,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ) | BIT(NL80211_BAND_5GHZ),
    },
    };
    const char iwl6000i_2agn_name[] = "Intel(R) Centrino(R) Advanced-N 6200 AGN";
    const struct iwl_rf_cfg iwl6000i_non_n_cfg = {
    IWL_DEVICE_6000i,
    };
    const char iwl6000i_2abg_name[] = "Intel(R) Centrino(R) Advanced-N 6200 ABG";
    const char iwl6000i_2bg_name[] = "Intel(R) Centrino(R) Advanced-N 6200 BG";
    const struct iwl_mac_cfg iwl6050_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_6050,
    .base = &iwl6050_base,
    };

    .fw_name_pre = IWL6050_FW_PRE,				\
    .ucode_api_max = IWL6050_UCODE_API_MAX,			\
    .ucode_api_min = IWL6050_UCODE_API_MIN,			\
    .max_inst_size = IWL60_RTC_INST_SIZE,			\
    .max_data_size = IWL60_RTC_DATA_SIZE,			\
    .valid_tx_ant = ANT_AB,		/* .cfg overwrite */	\
    .valid_rx_ant = ANT_AB,		/* .cfg overwrite */	\
    .nvm_ver = EEPROM_6050_EEPROM_VERSION,		\
    .nvm_calib_ver = EEPROM_6050_TX_POWER_VERSION,	\
    .eeprom_params = &iwl6000_eeprom_params,		\
    .led_mode = IWL_LED_BLINK,				\
    .internal_wimax_coex = true
    const struct iwl_rf_cfg iwl6050_2agn_cfg = {
    IWL_DEVICE_6050,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ) | BIT(NL80211_BAND_5GHZ),
    },
    };
    const char iwl6050_2agn_name[] = "Intel(R) Centrino(R) Advanced-N + WiMAX 6250 AGN";
    const struct iwl_rf_cfg iwl6050_2abg_cfg = {
    IWL_DEVICE_6050,
    };
    const char iwl6050_2abg_name[] = "Intel(R) Centrino(R) Advanced-N + WiMAX 6250 ABG";
    const struct iwl_mac_cfg iwl6150_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_6150,
    .base = &iwl6050_base,
    };

    .fw_name_pre = IWL6050_FW_PRE,				\
    .ucode_api_max = IWL6050_UCODE_API_MAX,			\
    .ucode_api_min = IWL6050_UCODE_API_MIN,			\
    .max_inst_size = IWL60_RTC_INST_SIZE,			\
    .max_data_size = IWL60_RTC_DATA_SIZE,			\
    .nvm_ver = EEPROM_6150_EEPROM_VERSION,		\
    .nvm_calib_ver = EEPROM_6150_TX_POWER_VERSION,	\
    .eeprom_params = &iwl6000_eeprom_params,		\
    .led_mode = IWL_LED_BLINK,				\
    .internal_wimax_coex = true
    const struct iwl_rf_cfg iwl6150_bgn_cfg = {
    IWL_DEVICE_6150,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ) | BIT(NL80211_BAND_5GHZ),
    },
    };
    const char iwl6150_bgn_name[] = "Intel(R) Centrino(R) Wireless-N + WiMAX 6150 BGN";
    const struct iwl_rf_cfg iwl6150_bg_cfg = {
    IWL_DEVICE_6150,
    };
    const char iwl6150_bg_name[] = "Intel(R) Centrino(R) Wireless-N + WiMAX 6150 BG";
    const struct iwl_mac_cfg iwl6000_mac_cfg = {
    .device_family = IWL_DEVICE_FAMILY_6000,
    .base = &iwl6000_base,
    };
    const struct iwl_rf_cfg iwl6000_3agn_cfg = {
    .fw_name_pre = IWL6000_FW_PRE,
    .ucode_api_max = IWL6000_UCODE_API_MAX,
    .ucode_api_min = IWL6000_UCODE_API_MIN,
    .max_inst_size = IWL60_RTC_INST_SIZE,
    .max_data_size = IWL60_RTC_DATA_SIZE,
    .nvm_ver = EEPROM_6000_EEPROM_VERSION,
    .nvm_calib_ver = EEPROM_6000_TX_POWER_VERSION,
    .eeprom_params = &iwl6000_eeprom_params,
    .ht_params = {
    .ht_greenfield_support = true,
    .use_rts_for_aggregation = true, /* use rts/cts protection */
    .ht40_bands = BIT(NL80211_BAND_2GHZ) | BIT(NL80211_BAND_5GHZ),
    },
    .led_mode = IWL_LED_BLINK,
    };
    const char iwl6000_3agn_name[] = "Intel(R) Centrino(R) Ultimate-N 6300 AGN";
    MODULE_FIRMWARE(IWL6000_MODULE_FIRMWARE(IWL6000_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL6050_MODULE_FIRMWARE(IWL6050_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL6005_MODULE_FIRMWARE(IWL6000G2_UCODE_API_MAX));
    MODULE_FIRMWARE(IWL6030_MODULE_FIRMWARE(IWL6000G2_UCODE_API_MAX));
