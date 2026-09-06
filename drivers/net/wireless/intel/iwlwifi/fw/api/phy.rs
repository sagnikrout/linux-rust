//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/phy.h
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
// Copyright (C) 2012-2014, 2019-2022, 2024-2025 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_phy_h__

//
// enum iwl_phy_ops_subcmd_ids - PHY group commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_phy_ops_subcmd_ids {
//
// @CMD_DTS_MEASUREMENT_TRIGGER_WIDE:
// Uses either &struct iwl_dts_measurement_cmd or
// &struct iwl_ext_dts_measurement_cmd
//
    CMD_DTS_MEASUREMENT_TRIGGER_WIDE = 0x0,

//
// @CTDP_CONFIG_CMD: &struct iwl_ctdp_cmd
//
    CTDP_CONFIG_CMD = 0x03,

//
// @TEMP_REPORTING_THRESHOLDS_CMD: &struct temp_report_ths_cmd
//
    TEMP_REPORTING_THRESHOLDS_CMD = 0x04,

//
// @PER_CHAIN_LIMIT_OFFSET_CMD: &struct iwl_geo_tx_power_profiles_cmd_v1,
// &struct iwl_geo_tx_power_profiles_cmd_v2,
// &struct iwl_geo_tx_power_profiles_cmd_v3,
// &struct iwl_geo_tx_power_profiles_cmd_v4 or
// &struct iwl_geo_tx_power_profiles_cmd_v5
//
    PER_CHAIN_LIMIT_OFFSET_CMD = 0x05,

//
// @PER_PLATFORM_ANT_GAIN_CMD: &union iwl_ppag_table_cmd
//
    PER_PLATFORM_ANT_GAIN_CMD = 0x07,

//
// @AP_TX_POWER_CONSTRAINTS_CMD: &struct iwl_txpower_constraints_cmd
//
    AP_TX_POWER_CONSTRAINTS_CMD = 0x0C,

//
// @CT_KILL_NOTIFICATION: &struct ct_kill_notif
//
    CT_KILL_NOTIFICATION = 0xFE,

//
// @DTS_MEASUREMENT_NOTIF_WIDE:
// &struct iwl_dts_measurement_notif_v1 or
// &struct iwl_dts_measurement_notif
//
    DTS_MEASUREMENT_NOTIF_WIDE = 0xFF,
}

// DTS measurements
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dts_measurement_flags {
    DTS_TRIGGER_CMD_FLAGS_TEMP	= BIT(0),
    DTS_TRIGGER_CMD_FLAGS_VOLT	= BIT(1),
}

//
// struct iwl_dts_measurement_cmd - request DTS temp and/or voltage measurements
//
// @flags: indicates which measurements we want as specified in
// &enum iwl_dts_measurement_flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dts_measurement_cmd {
    pub flags: __le32,
    pub /: *mut *mut } __packed; / TEMPERATURE_MEASUREMENT_TRIGGER_CMD_S,
//
// enum iwl_dts_control_measurement_mode - DTS measurement type
// @DTS_AUTOMATIC: Automatic mode (full SW control). Provide temperature read
// back (latest value. Not waiting for new value). Use automatic
// SW DTS configuration.
// @DTS_REQUEST_READ: Request DTS read. Configure DTS with manual settings,
// trigger DTS reading and provide read back temperature read
// when available.
// @DTS_OVER_WRITE: over-write the DTS temperatures in the SW until next read
// @DTS_DIRECT_WITHOUT_MEASURE: DTS returns its latest temperature result,
// without measurement trigger.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dts_control_measurement_mode {
    DTS_AUTOMATIC			= 0,
    DTS_REQUEST_READ		= 1,
    DTS_OVER_WRITE			= 2,
    DTS_DIRECT_WITHOUT_MEASURE	= 3,
}

//
// enum iwl_dts_used - DTS to use or used for measurement in the DTS request
// @DTS_USE_TOP: Top
// @DTS_USE_CHAIN_A: chain A
// @DTS_USE_CHAIN_B: chain B
// @DTS_USE_CHAIN_C: chain C
// @XTAL_TEMPERATURE: read temperature from xtal
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dts_used {
    DTS_USE_TOP		= 0,
    DTS_USE_CHAIN_A		= 1,
    DTS_USE_CHAIN_B		= 2,
    DTS_USE_CHAIN_C		= 3,
    XTAL_TEMPERATURE	= 4,
}

//
// enum iwl_dts_bit_mode - bit-mode to use in DTS request read mode
// @DTS_BIT6_MODE: bit 6 mode
// @DTS_BIT8_MODE: bit 8 mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dts_bit_mode {
    DTS_BIT6_MODE	= 0,
    DTS_BIT8_MODE	= 1,
}

//
// struct iwl_ext_dts_measurement_cmd - request extended DTS temp measurements
// @control_mode: see &enum iwl_dts_control_measurement_mode
// @temperature: used when over write DTS mode is selected
// @sensor: set temperature sensor to use. See &enum iwl_dts_used
// @avg_factor: average factor to DTS in request DTS read mode
// @bit_mode: value defines the DTS bit mode to use. See &enum iwl_dts_bit_mode
// @step_duration: step duration for the DTS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ext_dts_measurement_cmd {
    pub control_mode: __le32,
    pub temperature: __le32,
    pub sensor: __le32,
    pub avg_factor: __le32,
    pub bit_mode: __le32,
    pub step_duration: __le32,
    pub /: *mut *mut } __packed; / XVT_FW_DTS_CONTROL_MEASUREMENT_REQUEST_API_S,
//
// struct iwl_dts_measurement_notif_v1 - measurements notification
//
// @temp: the measured temperature
// @voltage: the measured voltage
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dts_measurement_notif_v1 {
    pub temp: __le32,
    pub voltage: __le32,
    pub TEMPERATURE_MEASUREMENT_TRIGGER_NTFY_S_VER_1*/: *mut *mut } __packed; /,
//
// struct iwl_dts_measurement_notif - measurements notification
//
// @temp: the measured temperature
// @voltage: the measured voltage
// @threshold_idx: the trip index that was crossed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dts_measurement_notif {
    pub temp: __le32,
    pub voltage: __le32,
    pub threshold_idx: __le32,
    pub /: *mut *mut } __packed; / TEMPERATURE_MEASUREMENT_TRIGGER_NTFY_S_VER_2,
//
// struct iwl_dts_measurement_resp - measurements response
//
// @temp: the measured temperature
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dts_measurement_resp {
    pub temp: __le32,
    pub /: *mut *mut } __packed; / CMD_DTS_MEASUREMENT_RSP_API_S_VER_1,
//
// struct ct_kill_notif - CT-kill entry notification
// This structure represent both versions of this notification.
//
// @temperature: the current temperature in celsius
// @dts: only in v2: DTS that trigger the CT Kill bitmap:
// bit 0: ToP master
// bit 1: PA chain A master
// bit 2: PA chain B master
// bit 3: ToP slave
// bit 4: PA chain A slave
// bit 5: PA chain B slave)
// bits 6,7: reserved (set to 0)
// @scheme: only for v2: scheme that trigger the CT Kill (0-SW, 1-HW)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ct_kill_notif {
    pub temperature: __le16,
    pub dts: u8,
    pub scheme: u8,
    pub /: *mut *mut } __packed; / CT_KILL_NOTIFICATION_API_S_VER_1, CT_KILL_NOTIFICATION_API_S_VER_2,
//
// enum iwl_ctdp_cmd_operation - CTDP command operations
// @CTDP_CMD_OPERATION_START: update the current budget
// @CTDP_CMD_OPERATION_STOP: stop ctdp
// @CTDP_CMD_OPERATION_REPORT: get the average budget
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ctdp_cmd_operation {
    CTDP_CMD_OPERATION_START	= 0x1,
    CTDP_CMD_OPERATION_STOP		= 0x2,
    CTDP_CMD_OPERATION_REPORT	= 0x4,
}

//
// struct iwl_ctdp_cmd - track and manage the FW power consumption budget
//
// @operation: see &enum iwl_ctdp_cmd_operation
// @budget: the budget in milliwatt
// @window_size: defined in API but not used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ctdp_cmd {
    pub operation: __le32,
    pub budget: __le32,
    pub window_size: __le32,
    pub __packed: },
pub const IWL_MAX_DTS_TRIPS: c_int = 8;
//
// struct temp_report_ths_cmd - set temperature thresholds
//
// @num_temps: number of temperature thresholds passed
// @thresholds: array with the thresholds to be configured
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct temp_report_ths_cmd {
    pub num_temps: __le32,
    pub thresholds: [__le16; IWL_MAX_DTS_TRIPS],
    pub /: *mut *mut } __packed; / GRP_PHY_TEMP_REPORTING_THRESHOLDS_CMD,
