//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/power.h
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
// Copyright (C) 2012-2014, 2018-2026 Intel Corporation
// Copyright (C) 2013-2014 Intel Mobile Communications GmbH
// Copyright (C) 2015-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_power_h__

// Power Management Commands, Responses, Notifications
//
// enum iwl_ltr_config_flags - masks for LTR config command flags
// @LTR_CFG_FLAG_FEATURE_ENABLE: Feature operational status
// @LTR_CFG_FLAG_HW_DIS_ON_SHADOW_REG_ACCESS: allow LTR change on shadow
// memory access
// @LTR_CFG_FLAG_HW_EN_SHRT_WR_THROUGH: allow LTR msg send on ANY LTR
// reg change
// @LTR_CFG_FLAG_HW_DIS_ON_D0_2_D3: allow LTR msg send on transition from
// D0 to D3
// @LTR_CFG_FLAG_SW_SET_SHORT: fixed static short LTR register
// @LTR_CFG_FLAG_SW_SET_LONG: fixed static short LONG register
// @LTR_CFG_FLAG_DENIE_C10_ON_PD: allow going into C10 on PD
// @LTR_CFG_FLAG_UPDATE_VALUES: update config values and short
// idle timeout
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ltr_config_flags {
    LTR_CFG_FLAG_FEATURE_ENABLE = BIT(0),
    LTR_CFG_FLAG_HW_DIS_ON_SHADOW_REG_ACCESS = BIT(1),
    LTR_CFG_FLAG_HW_EN_SHRT_WR_THROUGH = BIT(2),
    LTR_CFG_FLAG_HW_DIS_ON_D0_2_D3 = BIT(3),
    LTR_CFG_FLAG_SW_SET_SHORT = BIT(4),
    LTR_CFG_FLAG_SW_SET_LONG = BIT(5),
    LTR_CFG_FLAG_DENIE_C10_ON_PD = BIT(6),
    LTR_CFG_FLAG_UPDATE_VALUES = BIT(7),
}

//
// struct iwl_ltr_config_cmd_v1 - configures the LTR
// @flags: See &enum iwl_ltr_config_flags
// @static_long: static LTR Long register value.
// @static_short: static LTR Short register value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ltr_config_cmd_v1 {
    pub flags: __le32,
    pub static_long: __le32,
    pub static_short: __le32,
    pub /: *mut *mut } __packed; / LTR_CAPABLE_API_S_VER_1,
pub const LTR_VALID_STATES_NUM: c_int = 4;
//
// struct iwl_ltr_config_cmd - configures the LTR
// @flags: See &enum iwl_ltr_config_flags
// @static_long: static LTR Long register value.
// @static_short: static LTR Short register value.
// @ltr_cfg_values: LTR parameters table values (in usec) in following order:
// TX, RX, Short Idle, Long Idle. Used only if %LTR_CFG_FLAG_UPDATE_VALUES
// is set.
// @ltr_short_idle_timeout: LTR Short Idle timeout (in usec). Used only if
// %LTR_CFG_FLAG_UPDATE_VALUES is set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ltr_config_cmd {
    pub flags: __le32,
    pub static_long: __le32,
    pub static_short: __le32,
    pub ltr_cfg_values: [__le32; LTR_VALID_STATES_NUM],
    pub ltr_short_idle_timeout: __le32,
    pub /: *mut *mut } __packed; / LTR_CAPABLE_API_S_VER_2,
// Radio LP RX Energy Threshold measured in dBm
pub const POWER_LPRX_RSSI_THRESHOLD: c_int = 75;
pub const POWER_LPRX_RSSI_THRESHOLD_MAX: c_int = 94;
pub const POWER_LPRX_RSSI_THRESHOLD_MIN: c_int = 30;
//
// enum iwl_power_flags - masks for power table command flags
// @POWER_FLAGS_POWER_SAVE_ENA_MSK: '1' Allow to save power by turning off
// receiver and transmitter. '0' - does not allow.
// @POWER_FLAGS_POWER_MANAGEMENT_ENA_MSK: '0' Driver disables power management,
// '1' Driver enables PM (use rest of parameters)
// @POWER_FLAGS_SKIP_OVER_DTIM_MSK: '0' PM have to walk up every DTIM,
// '1' PM could sleep over DTIM till listen Interval.
// @POWER_FLAGS_SNOOZE_ENA_MSK: Enable snoozing only if uAPSD is enabled and all
// access categories are both delivery and trigger enabled.
// (Not supported since version 3)
// @POWER_FLAGS_BT_SCO_ENA: Enable BT SCO coex only if uAPSD and
// PBW Snoozing enabled
// @POWER_FLAGS_ADVANCE_PM_ENA_MSK: Advanced PM (uAPSD) enable mask
// @POWER_FLAGS_LPRX_ENA_MSK: Low Power RX enable.
// @POWER_FLAGS_UAPSD_MISBEHAVING_ENA_MSK: AP/GO's uAPSD misbehaving
// detection enablement (Not supported since version 3)
// @POWER_FLAGS_ENABLE_SMPS_MSK: SMPS is allowed for this vif
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_power_flags {
    POWER_FLAGS_POWER_SAVE_ENA_MSK		= BIT(0),
    POWER_FLAGS_POWER_MANAGEMENT_ENA_MSK	= BIT(1),
    POWER_FLAGS_SKIP_OVER_DTIM_MSK		= BIT(2),
    POWER_FLAGS_SNOOZE_ENA_MSK		= BIT(5),
    POWER_FLAGS_BT_SCO_ENA			= BIT(8),
    POWER_FLAGS_ADVANCE_PM_ENA_MSK		= BIT(9),
    POWER_FLAGS_LPRX_ENA_MSK		= BIT(11),
    POWER_FLAGS_UAPSD_MISBEHAVING_ENA_MSK	= BIT(12),
    POWER_FLAGS_ENABLE_SMPS_MSK		= BIT(14),
}

pub const IWL_POWER_VEC_SIZE: c_int = 5;
//
// struct iwl_powertable_cmd - legacy power command. Beside old API support this
// is used also with a new	power API for device wide power settings.
// POWER_TABLE_CMD = 0x77 (command, has simple generic response)
//
// @flags:		Power table command flags from POWER_FLAGS_
// @keep_alive_seconds: Keep alive period in seconds. Default - 25 sec.
// Minimum allowed:- 3 * DTIM. Keep alive period must be
// set regardless of power scheme or current power state.
// FW use this value also when PM is disabled.
// @debug_flags:	debug flags
// @rx_data_timeout:    Minimum time (usec) from last Rx packet for AM to
// PSM transition - legacy PM
// @tx_data_timeout:    Minimum time (usec) from last Tx packet for AM to
// PSM transition - legacy PM
// @sleep_interval:	not in use
// @skip_dtim_periods:	Number of DTIM periods to skip if Skip over DTIM flag
// is set. For example, if it is required to skip over
// one DTIM, this value need to be set to 2 (DTIM periods).
// @lprx_rssi_threshold: Signal strength up to which LP RX can be enabled.
// Default: 80dbm
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_powertable_cmd {
// PM_POWER_TABLE_CMD_API_S_VER_6
    pub flags: __le16,
    pub keep_alive_seconds: u8,
    pub debug_flags: u8,
    pub rx_data_timeout: __le32,
    pub tx_data_timeout: __le32,
    pub sleep_interval: [__le32; IWL_POWER_VEC_SIZE],
    pub skip_dtim_periods: __le32,
    pub lprx_rssi_threshold: __le32,
    pub __packed: },
//
// enum iwl_device_power_flags - masks for device power command flags
// @DEVICE_POWER_FLAGS_POWER_SAVE_ENA_MSK:
// '1' Allow to save power by turning off
// receiver and transmitter. '0' - does not allow.
// @DEVICE_POWER_FLAGS_ALLOW_MEM_RETENTION_MSK:
// Device Retention indication, '1' indicate retention is enabled.
// @DEVICE_POWER_FLAGS_NO_SLEEP_TILL_D3_MSK:
// Prevent power save until entering d3 is completed.
// @DEVICE_POWER_FLAGS_32K_CLK_VALID_MSK:
// 32Khz external slow clock valid indication, '1' indicate cloack is
// valid.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_device_power_flags {
    DEVICE_POWER_FLAGS_POWER_SAVE_ENA_MSK		= BIT(0),
    DEVICE_POWER_FLAGS_ALLOW_MEM_RETENTION_MSK	= BIT(1),
    DEVICE_POWER_FLAGS_NO_SLEEP_TILL_D3_MSK		= BIT(7),
    DEVICE_POWER_FLAGS_32K_CLK_VALID_MSK		= BIT(12),
}

//
// struct iwl_device_power_cmd - device wide power command.
// DEVICE_POWER_CMD = 0x77 (command, has simple generic response)
//
// @flags:	Power table command flags from &enum iwl_device_power_flags
// @reserved: reserved (padding)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_device_power_cmd {
// PM_POWER_TABLE_CMD_API_S_VER_7
    pub flags: __le16,
    pub reserved: __le16,
    pub __packed: },
//
// struct iwl_mac_power_cmd_v2 - power command V2 containing uAPSD support
// MAC_PM_POWER_TABLE = 0xA9 (command, has simple generic response)
// @id_and_color:	MAC context identifier, &enum iwl_ctxt_id_and_color
// @flags:		Power table command flags from POWER_FLAGS_
// @keep_alive_seconds:	Keep alive period in seconds. Default - 25 sec.
// Minimum allowed:- 3 * DTIM. Keep alive period must be
// set regardless of power scheme or current power state.
// FW use this value also when PM is disabled.
// @rx_data_timeout:    Minimum time (usec) from last Rx packet for AM to
// PSM transition - legacy PM
// @tx_data_timeout:    Minimum time (usec) from last Tx packet for AM to
// PSM transition - legacy PM
// @skip_dtim_periods:	Number of DTIM periods to skip if Skip over DTIM flag
// is set. For example, if it is required to skip over
// one DTIM, this value need to be set to 2 (DTIM periods).
// @rx_data_timeout_uapsd: Minimum time (usec) from last Rx packet for AM to
// PSM transition - uAPSD
// @tx_data_timeout_uapsd: Minimum time (usec) from last Tx packet for AM to
// PSM transition - uAPSD
// @lprx_rssi_threshold: Signal strength up to which LP RX can be enabled.
// Default: 80dbm
// @snooze_interval:	Maximum time between attempts to retrieve buffered data
// from the AP [msec]
// @snooze_window:	A window of time in which PBW snoozing insures that all
// packets received. It is also the minimum time from last
// received unicast RX packet, before client stops snoozing
// for data. [msec]
// @snooze_step:	TBD
// @qndp_tid:		TID client shall use for uAPSD QNDP triggers
// @uapsd_ac_flags:	Set trigger-enabled and delivery-enabled indication for
// each corresponding AC.
// Use IEEE80211_WMM_IE_STA_QOSINFO_AC* for correct values.
// @uapsd_max_sp:	Use IEEE80211_WMM_IE_STA_QOSINFO_SP_* for correct
// values.
// @heavy_tx_thld_packets:	TX threshold measured in number of packets
// @heavy_rx_thld_packets:	RX threshold measured in number of packets
// @heavy_tx_thld_percentage:	TX threshold measured in load's percentage
// @heavy_rx_thld_percentage:	RX threshold measured in load's percentage
// @limited_ps_threshold: (unused)
// @reserved: reserved (padding)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_power_cmd_v2 {
// CONTEXT_DESC_API_T_VER_1
    pub id_and_color: __le32,
    pub flags: __le16,
    pub keep_alive_seconds: __le16,
    pub rx_data_timeout: __le32,
    pub tx_data_timeout: __le32,
    pub rx_data_timeout_uapsd: __le32,
    pub tx_data_timeout_uapsd: __le32,
    pub lprx_rssi_threshold: u8,
    pub skip_dtim_periods: u8,
    pub snooze_interval: __le16,
    pub snooze_window: __le16,
    pub snooze_step: u8,
    pub qndp_tid: u8,
    pub uapsd_ac_flags: u8,
    pub uapsd_max_sp: u8,
    pub heavy_tx_thld_packets: u8,
    pub heavy_rx_thld_packets: u8,
    pub heavy_tx_thld_percentage: u8,
    pub heavy_rx_thld_percentage: u8,
    pub limited_ps_threshold: u8,
    pub reserved: u8,
    pub /: *mut *mut } __packed; / CLIENT_PM_POWER_TABLE_S_VER_1, VER_2,
//
// struct iwl_mac_power_cmd - power command
// MAC_PM_POWER_TABLE = 0xA9 (command, has simple generic response)
// @id_and_color:	MAC context identifier, &enum iwl_ctxt_id_and_color
// @flags:		Power table command flags from POWER_FLAGS_
// @keep_alive_seconds:	Keep alive period in seconds. Default - 25 sec.
// Minimum allowed:- 3 * DTIM. Keep alive period must be
// set regardless of power scheme or current power state.
// FW use this value also when PM is disabled.
// @rx_data_timeout:    Minimum time (usec) from last Rx packet for AM to
// PSM transition - legacy PM
// @tx_data_timeout:    Minimum time (usec) from last Tx packet for AM to
// PSM transition - legacy PM
// @lprx_rssi_threshold: Signal strength up to which LP RX can be enabled.
// Default: 80dbm
// @skip_dtim_periods:	Number of DTIM periods to skip if Skip over DTIM flag
// is set. For example, if it is required to skip over
// one DTIM, this value need to be set to 2 (DTIM periods).
// @qndp_tid:		TID client shall use for uAPSD QNDP triggers
// @uapsd_ac_flags:	Set trigger-enabled and delivery-enabled indication for
// each corresponding AC.
// Use IEEE80211_WMM_IE_STA_QOSINFO_AC* for correct values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mac_power_cmd {
// CONTEXT_DESC_API_T_VER_1
    pub id_and_color: __le32,
    pub flags: __le16,
    pub keep_alive_seconds: __le16,
    pub rx_data_timeout: __le32,
    pub tx_data_timeout: __le32,
    pub lprx_rssi_threshold: u8,
    pub skip_dtim_periods: u8,
    pub qndp_tid: u8,
    pub uapsd_ac_flags: u8,
    pub /: *mut *mut } __packed; / CLIENT_PM_POWER_TABLE_S_VER_3,
//
// struct iwl_uapsd_misbehaving_ap_notif - FW sends this notification when
// associated AP is identified as improperly implementing uAPSD protocol.
// PSM_UAPSD_AP_MISBEHAVING_NOTIFICATION = 0x78
// @sta_id: index of station in uCode's station table - associated AP ID in
// this context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_uapsd_misbehaving_ap_notif {
    pub sta_id: __le32,
    pub mac_id: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_dev_tx_power_cmd_mode {
    IWL_TX_POWER_MODE_SET_LINK = 0,
    IWL_TX_POWER_MODE_SET_DEVICE = 1,
    IWL_TX_POWER_MODE_SET_CHAINS = 2,
    IWL_TX_POWER_MODE_SET_ACK = 3,
    IWL_TX_POWER_MODE_SET_SAR_TIMER = 4,
    IWL_TX_POWER_MODE_SET_SAR_TIMER_DEFAULT_TABLE = 5,
}

pub const IWL_NUM_CHAIN_TABLES: c_int = 1;
pub const IWL_NUM_CHAIN_TABLES_V2: c_int = 2;
pub const IWL_NUM_CHAIN_LIMITS: c_int = 2;
pub const IWL_NUM_SUB_BANDS_V1: c_int = 5;
pub const IWL_NUM_SUB_BANDS_V2: c_int = 11;
pub const IWL_NUM_SUB_BANDS_V3: c_int = 12;
//
// struct iwl_dev_tx_power_common - Common part of the TX power reduction cmd
// @set_mode: see &enum iwl_dev_tx_power_cmd_mode
// @link_id: id of the link ctx for which we are reducing TX power.
// For version 9 / 10, this is the link id. For earlier versions, it is
// the mac id.
// @pwr_restriction: TX power restriction in 1/8 dBms.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_common {
    pub set_mode: __le32,
    pub link_id: __le32,
    pub pwr_restriction: __le16,
    pub __packed: },
//
// struct iwl_dev_tx_power_cmd_v3 - TX power reduction command version 3
// @per_chain: per chain restrictions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_cmd_v3 {
    pub per_chain: [__le16; IWL_NUM_CHAIN_TABLES][IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V1],
    pub /: *mut *mut } __packed; / TX_REDUCED_POWER_API_S_VER_3,
pub const IWL_DEV_MAX_TX_POWER: c_uint = 0x7FFF;
//
// struct iwl_dev_tx_power_cmd_v4 - TX power reduction command version 4
// @per_chain: per chain restrictions
// @enable_ack_reduction: enable or disable close range ack TX power
// reduction.
// @reserved: reserved (padding)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_cmd_v4 {
    pub per_chain: [__le16; IWL_NUM_CHAIN_TABLES][IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V1],
    pub enable_ack_reduction: u8,
    pub reserved: [u8; 3],
    pub /: *mut *mut } __packed; / TX_REDUCED_POWER_API_S_VER_4,
//
// struct iwl_dev_tx_power_cmd_v5 - TX power reduction command version 5
// @per_chain: per chain restrictions
// @enable_ack_reduction: enable or disable close range ack TX power
// reduction.
// @per_chain_restriction_changed: is per_chain_restriction has changed
// from last command. used if set_mode is
// IWL_TX_POWER_MODE_SET_SAR_TIMER.
// note: if not changed, the command is used for keep alive only.
// @reserved: reserved (padding)
// @timer_period: timer in milliseconds. if expires FW will change to default
// BIOS values. relevant if setMode is IWL_TX_POWER_MODE_SET_SAR_TIMER
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_cmd_v5 {
    pub per_chain: [__le16; IWL_NUM_CHAIN_TABLES][IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V1],
    pub enable_ack_reduction: u8,
    pub per_chain_restriction_changed: u8,
    pub reserved: [u8; 2],
    pub timer_period: __le32,
    pub /: *mut *mut } __packed; / TX_REDUCED_POWER_API_S_VER_5,
//
// struct iwl_dev_tx_power_cmd_v8 - TX power reduction command version 8
// @per_chain: per chain restrictions
// @enable_ack_reduction: enable or disable close range ack TX power
// reduction.
// @per_chain_restriction_changed: is per_chain_restriction has changed
// from last command. used if set_mode is
// IWL_TX_POWER_MODE_SET_SAR_TIMER.
// note: if not changed, the command is used for keep alive only.
// @reserved: reserved (padding)
// @timer_period: timer in milliseconds. if expires FW will change to default
// BIOS values. relevant if setMode is IWL_TX_POWER_MODE_SET_SAR_TIMER
// @flags: reduce power flags.
// @tpc_vlp_backoff_level: user backoff of UNII5,7 VLP channels in USA.
// Not in use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_cmd_v8 {
    pub per_chain: [__le16; IWL_NUM_CHAIN_TABLES_V2][IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V2],
    pub enable_ack_reduction: u8,
    pub per_chain_restriction_changed: u8,
    pub reserved: [u8; 2],
    pub timer_period: __le32,
    pub flags: __le32,
    pub tpc_vlp_backoff_level: __le32,
    pub /: *mut *mut } __packed; / TX_REDUCED_POWER_API_S_VER_8,
//
// @dev_24: device TX power restriction in 1/8 dBms
// @dev_52_low: device TX power restriction upper band - low
// @dev_52_high: device TX power restriction upper band - high
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_cmd_per_band {
    pub dev_24: __le16,
    pub dev_52_low: __le16,
    pub dev_52_high: __le16,
    pub __packed: },
//
// struct iwl_dev_tx_power_cmd_v3_v8 - TX power reduction command (multiversion)
// @per_band: per band restrictions
// @common: common part of the command
// @v3: version 3 part of the command
// @v4: version 4 part of the command
// @v5: version 5 part of the command
// @v8: version 8 part of the command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_cmd_v3_v8 {
    pub common: iwl_dev_tx_power_common,
    pub per_band: iwl_dev_tx_power_cmd_per_band,
    pub v3: iwl_dev_tx_power_cmd_v3,
    pub v4: iwl_dev_tx_power_cmd_v4,
    pub v5: iwl_dev_tx_power_cmd_v5,
    pub v8: iwl_dev_tx_power_cmd_v8,
}

//
// struct iwl_dev_tx_power_cmd_v9 - TX power reduction cmd
// @reserved: reserved (padding)
// @per_chain: per chain restrictions
// @per_chain_restriction_changed: is per_chain_restriction has changed
// from last command. used if set_mode is
// IWL_TX_POWER_MODE_SET_SAR_TIMER.
// note: if not changed, the command is used for keep alive only.
// @reserved1: reserved (padding)
// @timer_period: timer in milliseconds. if expires FW will change to default
// BIOS values. relevant if setMode is IWL_TX_POWER_MODE_SET_SAR_TIMER
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_cmd_v9 {
    pub reserved: __le16,
    pub per_chain: [__le16; IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V1],
    pub per_chain_restriction_changed: u8,
    pub reserved1: [u8; 3],
    pub timer_period: __le32,
    pub /: *mut *mut } __packed; / TX_REDUCED_POWER_API_S_VER_9,
//
// struct iwl_dev_tx_power_cmd_v10 - TX power reduction cmd
// @per_chain: per chain restrictions
// @per_chain_restriction_changed: is per_chain_restriction has changed
// from last command. used if set_mode is
// IWL_TX_POWER_MODE_SET_SAR_TIMER.
// note: if not changed, the command is used for keep alive only.
// @reserved: reserved (padding)
// @timer_period: timer in milliseconds. if expires FW will change to default
// BIOS values. relevant if setMode is IWL_TX_POWER_MODE_SET_SAR_TIMER
// @flags: reduce power flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_cmd_v10 {
    pub per_chain: [__le16; IWL_NUM_CHAIN_TABLES_V2][IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V2],
    pub per_chain_restriction_changed: u8,
    pub reserved: u8,
    pub timer_period: __le32,
    pub flags: __le32,
    pub /: *mut *mut } __packed; / TX_REDUCED_POWER_API_S_VER_10,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_cmd_v11 {
    pub per_chain: [__le16; IWL_NUM_CHAIN_TABLES_V2][IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V3],
    pub per_chain_restriction_changed: u8,
    pub reserved: u8,
    pub timer_period: __le32,
    pub flags: __le32,
    pub /: *mut *mut } __packed; / TX_REDUCED_POWER_API_S_VER_11,
//
// struct iwl_dev_tx_power_cmd - TX power reduction command (multiversion)
// @common: common part of the command
// @v9: version 9 part of the command
// @v10: version 10 part of the command
// @v11: version 11 part of the command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dev_tx_power_cmd {
    pub common: iwl_dev_tx_power_common,
    pub v9: iwl_dev_tx_power_cmd_v9,
    pub v10: iwl_dev_tx_power_cmd_v10,
    pub v11: iwl_dev_tx_power_cmd_v11,
}

// TX_REDUCED_POWER_API_S_VER_10
// TX_REDUCED_POWER_API_S_VER_11
//
pub const IWL_NUM_GEO_PROFILES: c_int = 3;
pub const IWL_NUM_GEO_PROFILES_V3: c_int = 8;
pub const IWL_NUM_BANDS_PER_CHAIN_V1: c_int = 2;
pub const IWL_NUM_BANDS_PER_CHAIN_V2: c_int = 3;
pub const IWL_NUM_BANDS_PER_CHAIN_V6: c_int = 4;
//
// enum iwl_geo_per_chain_offset_operation - type of operation
// @IWL_PER_CHAIN_OFFSET_SET_TABLES: send the tables from the host to the FW.
// @IWL_PER_CHAIN_OFFSET_GET_CURRENT_TABLE: retrieve the last configured table.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_geo_per_chain_offset_operation {
    IWL_PER_CHAIN_OFFSET_SET_TABLES,
    IWL_PER_CHAIN_OFFSET_GET_CURRENT_TABLE,
}

//
// struct iwl_per_chain_offset - embedded struct for PER_CHAIN_LIMIT_OFFSET_CMD.
// @max_tx_power: maximum allowed tx power.
// @chain_a: tx power offset for chain a.
// @chain_b: tx power offset for chain b.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_per_chain_offset {
    pub max_tx_power: __le16,
    pub chain_a: u8,
    pub chain_b: u8,
    pub /: *mut *mut } __packed; / PER_CHAIN_LIMIT_OFFSET_PER_CHAIN_S_VER_1,
//
// struct iwl_geo_tx_power_profiles_cmd_v1 - struct for PER_CHAIN_LIMIT_OFFSET_CMD cmd.
// @ops: operations, value from &enum iwl_geo_per_chain_offset_operation
// @table: offset profile per band.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_geo_tx_power_profiles_cmd_v1 {
    pub ops: __le32,
    pub table: [iwl_per_chain_offset; IWL_NUM_GEO_PROFILES][IWL_NUM_BANDS_PER_CHAIN_V1],
    pub /: *mut *mut } __packed; / PER_CHAIN_LIMIT_OFFSET_CMD_VER_1,
//
// struct iwl_geo_tx_power_profiles_cmd_v2 - struct for PER_CHAIN_LIMIT_OFFSET_CMD cmd.
// @ops: operations, value from &enum iwl_geo_per_chain_offset_operation
// @table: offset profile per band.
// @table_revision: 0 for not-South Korea, 1 for South Korea (the name is misleading)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_geo_tx_power_profiles_cmd_v2 {
    pub ops: __le32,
    pub table: [iwl_per_chain_offset; IWL_NUM_GEO_PROFILES][IWL_NUM_BANDS_PER_CHAIN_V1],
    pub table_revision: __le32,
    pub /: *mut *mut } __packed; / PER_CHAIN_LIMIT_OFFSET_CMD_VER_2,
//
// struct iwl_geo_tx_power_profiles_cmd_v3 - struct for PER_CHAIN_LIMIT_OFFSET_CMD cmd.
// @ops: operations, value from &enum iwl_geo_per_chain_offset_operation
// @table: offset profile per band.
// @table_revision: 0 for not-South Korea, 1 for South Korea (the name is misleading)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_geo_tx_power_profiles_cmd_v3 {
    pub ops: __le32,
    pub table: [iwl_per_chain_offset; IWL_NUM_GEO_PROFILES][IWL_NUM_BANDS_PER_CHAIN_V2],
    pub table_revision: __le32,
    pub /: *mut *mut } __packed; / PER_CHAIN_LIMIT_OFFSET_CMD_VER_3,
//
// struct iwl_geo_tx_power_profiles_cmd_v4 - struct for PER_CHAIN_LIMIT_OFFSET_CMD cmd.
// @ops: operations, value from &enum iwl_geo_per_chain_offset_operation
// @table: offset profile per band.
// @table_revision: 0 for not-South Korea, 1 for South Korea (the name is misleading)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_geo_tx_power_profiles_cmd_v4 {
    pub ops: __le32,
    pub table: [iwl_per_chain_offset; IWL_NUM_GEO_PROFILES_V3][IWL_NUM_BANDS_PER_CHAIN_V1],
    pub table_revision: __le32,
    pub /: *mut *mut } __packed; / PER_CHAIN_LIMIT_OFFSET_CMD_VER_4,
//
// struct iwl_geo_tx_power_profiles_cmd_v5 - struct for PER_CHAIN_LIMIT_OFFSET_CMD cmd.
// @ops: operations, value from &enum iwl_geo_per_chain_offset_operation
// @table: offset profile per band.
// @table_revision: 0 for not-South Korea, 1 for South Korea (the name is misleading)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_geo_tx_power_profiles_cmd_v5 {
    pub ops: __le32,
    pub table: [iwl_per_chain_offset; IWL_NUM_GEO_PROFILES_V3][IWL_NUM_BANDS_PER_CHAIN_V2],
    pub table_revision: __le32,
    pub /: *mut *mut } __packed; / PER_CHAIN_LIMIT_OFFSET_CMD_VER_5,
//
// struct iwl_geo_tx_power_profiles_cmd_v6 - struct for PER_CHAIN_LIMIT_OFFSET_CMD cmd.
// @ops: operations, value from &enum iwl_geo_per_chain_offset_operation
// @table: offset profile per band.
// @bios_hdr: describes the revision and the source of the BIOS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_geo_tx_power_profiles_cmd_v6 {
    pub ops: __le32,
    pub table: [iwl_per_chain_offset; IWL_NUM_GEO_PROFILES_V3][IWL_NUM_BANDS_PER_CHAIN_V6],
    pub bios_hdr: iwl_bios_config_hdr,
    pub /: *mut *mut } __packed; / PER_CHAIN_LIMIT_OFFSET_CMD_VER_6,
#[repr(C)]
#[derive(Copy, Clone)]
pub union iwl_geo_tx_power_profiles_cmd {
    pub v1: iwl_geo_tx_power_profiles_cmd_v1,
    pub v2: iwl_geo_tx_power_profiles_cmd_v2,
    pub v3: iwl_geo_tx_power_profiles_cmd_v3,
    pub v4: iwl_geo_tx_power_profiles_cmd_v4,
    pub v5: iwl_geo_tx_power_profiles_cmd_v5,
    pub v6: iwl_geo_tx_power_profiles_cmd_v6,
}

//
// struct iwl_geo_tx_power_profiles_resp -  response to PER_CHAIN_LIMIT_OFFSET_CMD cmd
// @profile_idx: current geo profile in use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_geo_tx_power_profiles_resp {
    pub profile_idx: __le32,
    pub /: *mut *mut } __packed; / PER_CHAIN_LIMIT_OFFSET_RSP,
//
// enum iwl_ppag_flags - PPAG enable masks
// @IWL_PPAG_ETSI_MASK: enable PPAG in ETSI
// @IWL_PPAG_CHINA_MASK: enable PPAG in China
// @IWL_PPAG_ETSI_LPI_UHB_MASK: enable LPI in ETSI for UHB
// @IWL_PPAG_ETSI_VLP_UHB_MASK: enable VLP in ETSI for UHB
// @IWL_PPAG_ETSI_SP_UHB_MASK: enable SP in ETSI for UHB
// @IWL_PPAG_USA_LPI_UHB_MASK: enable LPI in USA for UHB
// @IWL_PPAG_USA_VLP_UHB_MASK: enable VLP in USA for UHB
// @IWL_PPAG_USA_SP_UHB_MASK: enable SP in USA for UHB
// @IWL_PPAG_CANADA_LPI_UHB_MASK: enable LPI in CANADA for UHB
// @IWL_PPAG_CANADA_VLP_UHB_MASK: enable VLP in CANADA for UHB
// @IWL_PPAG_CANADA_SP_UHB_MASK: enable SP in CANADA for UHB
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ppag_flags {
    IWL_PPAG_ETSI_MASK = BIT(0),
    IWL_PPAG_CHINA_MASK = BIT(1),
    IWL_PPAG_ETSI_LPI_UHB_MASK = BIT(2),
    IWL_PPAG_ETSI_VLP_UHB_MASK = BIT(3),
    IWL_PPAG_ETSI_SP_UHB_MASK = BIT(4),
    IWL_PPAG_USA_LPI_UHB_MASK = BIT(5),
    IWL_PPAG_USA_VLP_UHB_MASK = BIT(6),
    IWL_PPAG_USA_SP_UHB_MASK = BIT(7),
    IWL_PPAG_CANADA_LPI_UHB_MASK = BIT(8),
    IWL_PPAG_CANADA_VLP_UHB_MASK = BIT(9),
    IWL_PPAG_CANADA_SP_UHB_MASK = BIT(10),
}

//
// union iwl_ppag_table_cmd - union for all versions of PPAG command
// @v1: command version 1 structure.
// @v5: command version 5 structure.
// @v7: command version 7 structure.
// @v8: command version 8 structure.
// @v1.flags: values from &enum iwl_ppag_flags
// @v1.gain: table of antenna gain values per chain and sub-band
// @v1.reserved: reserved
// @v5.flags: values from &enum iwl_ppag_flags
// @v5.gain: table of antenna gain values per chain and sub-band
// @v7.ppag_config_info: see @struct bios_value_u32
// @v7.gain: table of antenna gain values per chain and sub-band
// @v7.reserved: reserved
// @v8.ppag_config_info: see @struct bios_value_u32
// @v8.gain: table of antenna gain values per chain and sub-band
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union iwl_ppag_table_cmd {
    pub flags: __le32,
    pub gain: [i8; IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V1],
    pub reserved: [i8; 2],
    pub /: *mut *mut } __packed v1; / PER_PLAT_ANTENNA_GAIN_CMD_API_S_VER_1,
    pub flags: __le32,
    pub gain: [i8; IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V2],
    pub reserved: [i8; 2],
    pub /: *mut *mut } __packed v5; / PER_PLAT_ANTENNA_GAIN_CMD_API_S_VER_5,
    pub ppag_config_info: bios_value_u32,
    pub gain: [i8; IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V2],
    pub reserved: [i8; 2],
    pub /: *mut *mut } __packed v7; / PER_PLAT_ANTENNA_GAIN_CMD_API_S_VER_7,
    pub ppag_config_info: bios_value_u32,
    pub gain: [i8; IWL_NUM_CHAIN_LIMITS][IWL_NUM_SUB_BANDS_V3],
    pub /: *mut *mut } __packed v8; / PER_PLAT_ANTENNA_GAIN_CMD_API_S_VER_8,
    pub __packed: },

pub const MCC_TO_SAR_OFFSET_TABLE_ROW_SIZE: c_int = 26;
pub const MCC_TO_SAR_OFFSET_TABLE_COL_SIZE: c_int = 13;
//
// struct iwl_sar_offset_mapping_cmd - struct for SAR_OFFSET_MAPPING_TABLE_CMD
// @offset_map: mapping a mcc to a geo sar group
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sar_offset_mapping_cmd {
    pub reserved: __le16,
    pub /*SAR_OFFSET_MAPPING_TABLE_CMD_API_S*/: *mut } __packed;,
//
// struct iwl_beacon_filter_cmd - beacon filter command
// REPLY_BEACON_FILTERING_CMD = 0xd2 (command)
// @bf_energy_delta: Used for RSSI filtering, if in 'normal' state. Send beacon
// to driver if delta in Energy values calculated for this and last
// passed beacon is greater than this threshold. Zero value means that
// the Energy change is ignored for beacon filtering, and beacon will
// not be forced to be sent to driver regardless of this delta. Typical
// energy delta 5dB.
// @bf_roaming_energy_delta: Used for RSSI filtering, if in 'roaming' state.
// Send beacon to driver if delta in Energy values calculated for this
// and last passed beacon is greater than this threshold. Zero value
// means that the Energy change is ignored for beacon filtering while in
// Roaming state, typical energy delta 1dB.
// @bf_roaming_state: Used for RSSI filtering. If absolute Energy values
// calculated for current beacon is less than the threshold, use
// Roaming Energy Delta Threshold, otherwise use normal Energy Delta
// Threshold. Typical energy threshold is -72dBm.
// @bf_temp_threshold: This threshold determines the type of temperature
// filtering (Slow or Fast) that is selected (Units are in Celsius):
// If the current temperature is above this threshold - Fast filter
// will be used, If the current temperature is below this threshold -
// Slow filter will be used.
// @bf_temp_fast_filter: Send Beacon to driver if delta in temperature values
// calculated for this and the last passed beacon is greater than this
// threshold. Zero value means that the temperature change is ignored for
// beacon filtering; beacons will not be  forced to be sent to driver
// regardless of whether its temperature has been changed.
// @bf_temp_slow_filter: Send Beacon to driver if delta in temperature values
// calculated for this and the last passed beacon is greater than this
// threshold. Zero value means that the temperature change is ignored for
// beacon filtering; beacons will not be forced to be sent to driver
// regardless of whether its temperature has been changed.
// @bf_enable_beacon_filter: 1, beacon filtering is enabled; 0, disabled.
// @bf_debug_flag: beacon filtering debug configuration
// @bf_escape_timer: Send beacons to to driver if no beacons were passed
// for a specific period of time. Units: Beacons.
// @ba_escape_timer: Fully receive and parse beacon if no beacons were passed
// for a longer period of time then this escape-timeout. Units: Beacons.
// @ba_enable_beacon_abort: 1, beacon abort is enabled; 0, disabled.
// @bf_threshold_absolute_low: See below.
// @bf_threshold_absolute_high: Send Beacon to driver if Energy value calculated
// for this beacon crossed this absolute threshold. For the 'Increase'
// direction the bf_energy_absolute_low[i] is used. For the 'Decrease'
// direction the bf_energy_absolute_high[i] is used. Zero value means
// that this specific threshold is ignored for beacon filtering, and
// beacon will not be forced to be sent to driver due to this setting.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_beacon_filter_cmd {
    pub bf_energy_delta: __le32,
    pub bf_roaming_energy_delta: __le32,
    pub bf_roaming_state: __le32,
    pub bf_temp_threshold: __le32,
    pub bf_temp_fast_filter: __le32,
    pub bf_temp_slow_filter: __le32,
    pub bf_enable_beacon_filter: __le32,
    pub bf_debug_flag: __le32,
    pub bf_escape_timer: __le32,
    pub ba_escape_timer: __le32,
    pub ba_enable_beacon_abort: __le32,
    pub bf_threshold_absolute_low: [__le32; 2],
    pub bf_threshold_absolute_high: [__le32; 2],
    pub /: *mut *mut } __packed; / BEACON_FILTER_CONFIG_API_S_VER_4,
// Beacon filtering and beacon abort
pub const IWL_BF_ENERGY_DELTA_DEFAULT: c_int = 5;
pub const IWL_BF_ENERGY_DELTA_D0I3: c_int = 20;
pub const IWL_BF_ENERGY_DELTA_MAX: c_int = 255;
pub const IWL_BF_ENERGY_DELTA_MIN: c_int = 0;
pub const IWL_BF_ROAMING_ENERGY_DELTA_DEFAULT: c_int = 1;
pub const IWL_BF_ROAMING_ENERGY_DELTA_D0I3: c_int = 20;
pub const IWL_BF_ROAMING_ENERGY_DELTA_MAX: c_int = 255;
pub const IWL_BF_ROAMING_ENERGY_DELTA_MIN: c_int = 0;
pub const IWL_BF_ROAMING_STATE_DEFAULT: c_int = 72;
pub const IWL_BF_ROAMING_STATE_D0I3: c_int = 72;
pub const IWL_BF_ROAMING_STATE_MAX: c_int = 255;
pub const IWL_BF_ROAMING_STATE_MIN: c_int = 0;
pub const IWL_BF_TEMP_THRESHOLD_DEFAULT: c_int = 112;
pub const IWL_BF_TEMP_THRESHOLD_D0I3: c_int = 112;
pub const IWL_BF_TEMP_THRESHOLD_MAX: c_int = 255;
pub const IWL_BF_TEMP_THRESHOLD_MIN: c_int = 0;
pub const IWL_BF_TEMP_FAST_FILTER_DEFAULT: c_int = 1;
pub const IWL_BF_TEMP_FAST_FILTER_D0I3: c_int = 1;
pub const IWL_BF_TEMP_FAST_FILTER_MAX: c_int = 255;
pub const IWL_BF_TEMP_FAST_FILTER_MIN: c_int = 0;
pub const IWL_BF_TEMP_SLOW_FILTER_DEFAULT: c_int = 5;
pub const IWL_BF_TEMP_SLOW_FILTER_D0I3: c_int = 20;
pub const IWL_BF_TEMP_SLOW_FILTER_MAX: c_int = 255;
pub const IWL_BF_TEMP_SLOW_FILTER_MIN: c_int = 0;
pub const IWL_BF_ENABLE_BEACON_FILTER_DEFAULT: c_int = 1;
pub const IWL_BF_DEBUG_FLAG_DEFAULT: c_int = 0;
pub const IWL_BF_DEBUG_FLAG_D0I3: c_int = 0;
pub const IWL_BF_ESCAPE_TIMER_DEFAULT: c_int = 0;
pub const IWL_BF_ESCAPE_TIMER_D0I3: c_int = 0;
pub const IWL_BF_ESCAPE_TIMER_MAX: c_int = 1024;
pub const IWL_BF_ESCAPE_TIMER_MIN: c_int = 0;
pub const IWL_BA_ESCAPE_TIMER_DEFAULT: c_int = 6;
pub const IWL_BA_ESCAPE_TIMER_D0I3: c_int = 6;
pub const IWL_BA_ESCAPE_TIMER_D3: c_int = 9;
pub const IWL_BA_ESCAPE_TIMER_MAX: c_int = 1024;
pub const IWL_BA_ESCAPE_TIMER_MIN: c_int = 0;
pub const IWL_BA_ENABLE_BEACON_ABORT_DEFAULT: c_int = 1;

pub const DEFAULT_TPE_TX_POWER: c_uint = 0x7F;
//
// Bandwidth: 20/40/80/(160/80+80)/320
//
pub const IWL_MAX_TX_EIRP_PWR_MAX_SIZE: c_int = 5;
pub const IWL_MAX_TX_EIRP_PSD_PWR_MAX_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_6ghz_ap_type {
    IWL_6GHZ_AP_TYPE_LPI,
    IWL_6GHZ_AP_TYPE_SP,
    IWL_6GHZ_AP_TYPE_VLP,
}

//
// struct iwl_txpower_constraints_cmd - TX power constraints command
// AP_TX_POWER_CONSTRAINTS_CMD
// Used for VLP/LPI/AFC Access Point power constraints for 6GHz channels
// @link_id: linkId
// @ap_type: see &enum iwl_6ghz_ap_type
// @eirp_pwr: 8-bit 2s complement signed integer in the range
// -64 dBm to 63 dBm with a 0.5 dB step
// default &DEFAULT_TPE_TX_POWER (no maximum limit)
// @psd_pwr: 8-bit 2s complement signed integer in the range
// -63.5 to +63 dBm/MHz with a 0.5 step
// value - 128 indicates that the corresponding 20
// MHz channel cannot be used for transmission.
// value +127 indicates that no maximum PSD limit
// is specified for the corresponding 20 MHz channel
// default &DEFAULT_TPE_TX_POWER (no maximum limit)
// @reserved: reserved (padding)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_txpower_constraints_cmd {
    pub link_id: __le16,
    pub ap_type: __le16,
    pub eirp_pwr: [__s8; IWL_MAX_TX_EIRP_PWR_MAX_SIZE],
    pub psd_pwr: [__s8; IWL_MAX_TX_EIRP_PSD_PWR_MAX_SIZE],
    pub reserved: [u8; 3],
    pub /: *mut *mut } __packed; / PHY_AP_TX_POWER_CONSTRAINTS_CMD_API_S_VER_1,
