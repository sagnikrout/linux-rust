//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl18xx/conf.h
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
// This file is part of wl18xx
//
// Copyright (C) 2011 Texas Instruments Inc.
//
pub const WL18XX_CONF_MAGIC: c_uint = 0x10e100ca;

pub const WL18XX_CONF_MASK: c_uint = 0x0000ffff;

pub const NUM_OF_CHANNELS_11_ABG: c_int = 150;
pub const NUM_OF_CHANNELS_11_P: c_int = 7;
pub const SRF_TABLE_LEN: c_int = 16;
pub const PIN_MUXING_SIZE: c_int = 2;
pub const WL18XX_TRACE_LOSS_GAPS_TX: c_int = 10;
pub const WL18XX_TRACE_LOSS_GAPS_RX: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_mac_and_phy_params {
    pub phy_standalone: u8,
    pub spare0: u8,
    pub enable_clpc: u8,
    pub enable_tx_low_pwr_on_siso_rdl: u8,
    pub auto_detect: u8,
    pub dedicated_fem: u8,
    pub low_band_component: u8,
// Bit 0: One Hot, Bit 1: Control Enable, Bit 2: 1.8V, Bit 3: 3V
    pub low_band_component_type: u8,
    pub high_band_component: u8,
// Bit 0: One Hot, Bit 1: Control Enable, Bit 2: 1.8V, Bit 3: 3V
    pub high_band_component_type: u8,
    pub number_of_assembled_ant2_4: u8,
    pub number_of_assembled_ant5: u8,
    pub pin_muxing_platform_options: [u8; PIN_MUXING_SIZE],
    pub external_pa_dc2dc: u8,
    pub tcxo_ldo_voltage: u8,
    pub xtal_itrim_val: u8,
    pub srf_state: u8,
    pub srf1: [u8; SRF_TABLE_LEN],
    pub srf2: [u8; SRF_TABLE_LEN],
    pub srf3: [u8; SRF_TABLE_LEN],
    pub io_configuration: u8,
    pub sdio_configuration: u8,
    pub settings: u8,
    pub rx_profile: u8,
    pub per_chan_pwr_limit_arr_11abg: [u8; NUM_OF_CHANNELS_11_ABG],
    pub pwr_limit_reference_11_abg: u8,
    pub per_chan_pwr_limit_arr_11p: [u8; NUM_OF_CHANNELS_11_P],
    pub pwr_limit_reference_11p: u8,
    pub spare1: u8,
    pub per_chan_bo_mode_11_abg: [u8; 13],
    pub per_chan_bo_mode_11_p: [u8; 4],
    pub primary_clock_setting_time: u8,
    pub clock_valid_on_wake_up: u8,
    pub secondary_clock_setting_time: u8,
    pub board_type: u8,
// enable point saturation
    pub psat: u8,
// low/medium/high Tx power in dBm for STA-HP BG
    pub low_power_val: i8,
    pub med_power_val: i8,
    pub high_power_val: i8,
    pub per_sub_band_tx_trace_loss: [i8; WL18XX_TRACE_LOSS_GAPS_TX],
    pub per_sub_band_rx_trace_loss: [i8; WL18XX_TRACE_LOSS_GAPS_RX],
    pub tx_rf_margin: u8,
// low/medium/high Tx power in dBm for other role
    pub low_power_val_2nd: i8,
    pub med_power_val_2nd: i8,
    pub high_power_val_2nd: i8,
    pub padding: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl18xx_ht_mode {
// Default - use MIMO, fallback to SISO20
    HT_MODE_DEFAULT = 0,

// Wide - use SISO40
    HT_MODE_WIDE = 1,

// Use SISO20
    HT_MODE_SISO20 = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_ht_settings {
// DEFAULT / WIDE / SISO20
    pub mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct conf_ap_sleep_settings {
// Duty Cycle (20-80% of staying Awake) for IDLE AP
// (0: disable)
//
    pub idle_duty_cycle: u8,
// Duty Cycle (20-80% of staying Awake) for Connected AP
// (0: disable)
//
    pub connected_duty_cycle: u8,
// Maximum stations that are allowed to be connected to AP
// (255: no limit)
//
    pub max_stations_thresh: u8,
// Timeout till enabling the Sleep Mechanism after data stops
// [unit: 100 msec]
//
    pub idle_conn_thresh: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl18xx_priv_conf {
// Module params structures
    pub ht: wl18xx_ht_settings,
// this structure is copied wholesale to FW
    pub phy: wl18xx_mac_and_phy_params,
    pub ap_sleep: conf_ap_sleep_settings,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl18xx_sg_params {
    WL18XX_CONF_SG_PARAM_0 = 0,

// Configuration Parameters
    WL18XX_CONF_SG_ANTENNA_CONFIGURATION,
    WL18XX_CONF_SG_ZIGBEE_COEX,
    WL18XX_CONF_SG_TIME_SYNC,

    WL18XX_CONF_SG_PARAM_4,
    WL18XX_CONF_SG_PARAM_5,
    WL18XX_CONF_SG_PARAM_6,
    WL18XX_CONF_SG_PARAM_7,
    WL18XX_CONF_SG_PARAM_8,
    WL18XX_CONF_SG_PARAM_9,
    WL18XX_CONF_SG_PARAM_10,
    WL18XX_CONF_SG_PARAM_11,
    WL18XX_CONF_SG_PARAM_12,
    WL18XX_CONF_SG_PARAM_13,
    WL18XX_CONF_SG_PARAM_14,
    WL18XX_CONF_SG_PARAM_15,
    WL18XX_CONF_SG_PARAM_16,
    WL18XX_CONF_SG_PARAM_17,
    WL18XX_CONF_SG_PARAM_18,
    WL18XX_CONF_SG_PARAM_19,
    WL18XX_CONF_SG_PARAM_20,
    WL18XX_CONF_SG_PARAM_21,
    WL18XX_CONF_SG_PARAM_22,
    WL18XX_CONF_SG_PARAM_23,
    WL18XX_CONF_SG_PARAM_24,
    WL18XX_CONF_SG_PARAM_25,

// Active Scan Parameters
    WL18XX_CONF_SG_AUTO_SCAN_PROBE_REQ,
    WL18XX_CONF_SG_ACTIVE_SCAN_DURATION_FACTOR_HV3,

    WL18XX_CONF_SG_PARAM_28,

// Passive Scan Parameters
    WL18XX_CONF_SG_PARAM_29,
    WL18XX_CONF_SG_PARAM_30,
    WL18XX_CONF_SG_PASSIVE_SCAN_DURATION_FACTOR_HV3,

// Passive Scan in Dual Antenna Parameters
    WL18XX_CONF_SG_CONSECUTIVE_HV3_IN_PASSIVE_SCAN,
    WL18XX_CONF_SG_BEACON_HV3_COLL_TH_IN_PASSIVE_SCAN,
    WL18XX_CONF_SG_TX_RX_PROTECT_BW_IN_PASSIVE_SCAN,

// General Parameters
    WL18XX_CONF_SG_STA_FORCE_PS_IN_BT_SCO,
    WL18XX_CONF_SG_PARAM_36,
    WL18XX_CONF_SG_BEACON_MISS_PERCENT,
    WL18XX_CONF_SG_PARAM_38,
    WL18XX_CONF_SG_RXT,
    WL18XX_CONF_SG_UNUSED,
    WL18XX_CONF_SG_ADAPTIVE_RXT_TXT,
    WL18XX_CONF_SG_GENERAL_USAGE_BIT_MAP,
    WL18XX_CONF_SG_HV3_MAX_SERVED,
    WL18XX_CONF_SG_PARAM_44,
    WL18XX_CONF_SG_PARAM_45,
    WL18XX_CONF_SG_CONSECUTIVE_CTS_THRESHOLD,
    WL18XX_CONF_SG_GEMINI_PARAM_47,
    WL18XX_CONF_SG_STA_CONNECTION_PROTECTION_TIME,

// AP Parameters
    WL18XX_CONF_SG_AP_BEACON_MISS_TX,
    WL18XX_CONF_SG_PARAM_50,
    WL18XX_CONF_SG_AP_BEACON_WINDOW_INTERVAL,
    WL18XX_CONF_SG_AP_CONNECTION_PROTECTION_TIME,
    WL18XX_CONF_SG_PARAM_53,
    WL18XX_CONF_SG_PARAM_54,

// CTS Diluting Parameters
    WL18XX_CONF_SG_CTS_DILUTED_BAD_RX_PACKETS_TH,
    WL18XX_CONF_SG_CTS_CHOP_IN_DUAL_ANT_SCO_MASTER,

    WL18XX_CONF_SG_TEMP_PARAM_1,
    WL18XX_CONF_SG_TEMP_PARAM_2,
    WL18XX_CONF_SG_TEMP_PARAM_3,
    WL18XX_CONF_SG_TEMP_PARAM_4,
    WL18XX_CONF_SG_TEMP_PARAM_5,
    WL18XX_CONF_SG_TEMP_PARAM_6,
    WL18XX_CONF_SG_TEMP_PARAM_7,
    WL18XX_CONF_SG_TEMP_PARAM_8,
    WL18XX_CONF_SG_TEMP_PARAM_9,
    WL18XX_CONF_SG_TEMP_PARAM_10,

    WL18XX_CONF_SG_PARAMS_MAX,
    WL18XX_CONF_SG_PARAMS_ALL = 0xff
}
