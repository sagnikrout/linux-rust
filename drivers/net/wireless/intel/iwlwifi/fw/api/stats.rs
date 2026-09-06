//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/stats.h
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
// Copyright (C) 2012-2014, 2018, 2020-2021, 2023-2026 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_stats_h__

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_dbg {
    pub burst_check: __le32,
    pub burst_count: __le32,
    pub wait_for_silence_timeout_cnt: __le32,
    pub reserved: [u8; 12],
    pub /: *mut *mut } __packed; / STATISTICS_DEBUG_API_S_VER_2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_div {
    pub tx_on_a: __le32,
    pub tx_on_b: __le32,
    pub exec_time: __le32,
    pub probe_time: __le32,
    pub rssi_ant: __le32,
    pub reserved2: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_SLOW_DIV_API_S_VER_2,
//
// struct mvm_statistics_rx_non_phy - non-PHY RX statistics
// @bogus_cts: CTS received when not expecting CTS
// @bogus_ack: ACK received when not expecting ACK
// @non_channel_beacons: beacons with our bss id but not on our serving channel
// @channel_beacons: beacons with our bss id and in our serving channel
// @num_missed_bcon: number of missed beacons
// @adc_rx_saturation_time: count in 0.8us units the time the ADC was in
// saturation
// @ina_detection_search_time: total time (in 0.8us) searched for INA
// @beacon_silence_rssi_a: RSSI silence after beacon frame
// @beacon_silence_rssi_b: RSSI silence after beacon frame
// @beacon_silence_rssi_c: RSSI silence after beacon frame
// @interference_data_flag: flag for interference data availability. 1 when data
// is available.
// @channel_load: counts RX Enable time in uSec
// @beacon_rssi_a: beacon RSSI on antenna A
// @beacon_rssi_b: beacon RSSI on antenna B
// @beacon_rssi_c: beacon RSSI on antenna C
// @beacon_energy_a: beacon energy on antenna A
// @beacon_energy_b: beacon energy on antenna B
// @beacon_energy_c: beacon energy on antenna C
// @num_bt_kills: number of BT "kills" (frame TX aborts)
// @mac_id: mac ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_rx_non_phy {
    pub bogus_cts: __le32,
    pub bogus_ack: __le32,
    pub non_channel_beacons: __le32,
    pub channel_beacons: __le32,
    pub num_missed_bcon: __le32,
    pub adc_rx_saturation_time: __le32,
    pub ina_detection_search_time: __le32,
    pub beacon_silence_rssi_a: __le32,
    pub beacon_silence_rssi_b: __le32,
    pub beacon_silence_rssi_c: __le32,
    pub interference_data_flag: __le32,
    pub channel_load: __le32,
    pub beacon_rssi_a: __le32,
    pub beacon_rssi_b: __le32,
    pub beacon_rssi_c: __le32,
    pub beacon_energy_a: __le32,
    pub beacon_energy_b: __le32,
    pub beacon_energy_c: __le32,
    pub num_bt_kills: __le32,
    pub mac_id: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_RX_NON_PHY_API_S_VER_4,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_rx_non_phy_v3 {
    pub /: *mut *mut __le32 bogus_cts; / CTS received when not expecting CTS,
    pub /: *mut *mut __le32 bogus_ack; / ACK received when not expecting ACK,
    pub that: *mut *mut __le32 non_bssid_frames; / number of frames with BSSID,
// doesn't belong to the STA BSSID
    pub the: *mut *mut __le32 filtered_frames; / count frames that were dumped in,
// filtering process
    pub on: *mut *mut __le32 non_channel_beacons; / beacons with our bss id but not,
// our serving channel
    pub our: *mut *mut __le32 channel_beacons; / beacons with our bss id and in,
// serving channel
    pub /: *mut *mut __le32 num_missed_bcon; / number of missed beacons,
    pub the: *mut *mut __le32 adc_rx_saturation_time; / count in 0.8us units the time,
// ADC was in saturation
    pub searched: *mut *mut __le32 ina_detection_search_time;/ total time (in 0.8us),
// for INA
    pub /: *mut *mut __le32 beacon_silence_rssi_a; / RSSI silence after beacon frame,
    pub /: *mut *mut __le32 beacon_silence_rssi_b; / RSSI silence after beacon frame,
    pub /: *mut *mut __le32 beacon_silence_rssi_c; / RSSI silence after beacon frame,
    pub data: *mut *mut __le32 interference_data_flag; / flag for interference,
// availability. 1 when data is
// available.
    pub /: *mut *mut __le32 channel_load; / counts RX Enable time in uSec,
    pub OFDM: *mut *mut __le32 dsp_false_alarms; / DSP false alarm (both,
// and CCK) counter
    pub beacon_rssi_a: __le32,
    pub beacon_rssi_b: __le32,
    pub beacon_rssi_c: __le32,
    pub beacon_energy_a: __le32,
    pub beacon_energy_b: __le32,
    pub beacon_energy_c: __le32,
    pub num_bt_kills: __le32,
    pub mac_id: __le32,
    pub directed_data_mpdu: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_RX_NON_PHY_API_S_VER_3,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_rx_phy {
    pub unresponded_rts: __le32,
    pub rxe_frame_lmt_overrun: __le32,
    pub sent_ba_rsp_cnt: __le32,
    pub dsp_self_kill: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_RX_PHY_API_S_VER_3,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_rx_phy_v2 {
    pub ina_cnt: __le32,
    pub fina_cnt: __le32,
    pub plcp_err: __le32,
    pub crc32_err: __le32,
    pub overrun_err: __le32,
    pub early_overrun_err: __le32,
    pub crc32_good: __le32,
    pub false_alarm_cnt: __le32,
    pub fina_sync_err_cnt: __le32,
    pub sfd_timeout: __le32,
    pub fina_timeout: __le32,
    pub unresponded_rts: __le32,
    pub rxe_frame_lmt_overrun: __le32,
    pub sent_ack_cnt: __le32,
    pub sent_cts_cnt: __le32,
    pub sent_ba_rsp_cnt: __le32,
    pub dsp_self_kill: __le32,
    pub mh_format_err: __le32,
    pub re_acq_main_rssi_sum: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_RX_PHY_API_S_VER_2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_rx_ht_phy_v1 {
    pub plcp_err: __le32,
    pub overrun_err: __le32,
    pub early_overrun_err: __le32,
    pub crc32_good: __le32,
    pub crc32_err: __le32,
    pub mh_format_err: __le32,
    pub agg_crc32_good: __le32,
    pub agg_mpdu_cnt: __le32,
    pub agg_cnt: __le32,
    pub unsupport_mcs: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_HT_RX_PHY_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_rx_ht_phy {
    pub mh_format_err: __le32,
    pub agg_mpdu_cnt: __le32,
    pub agg_cnt: __le32,
    pub unsupport_mcs: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_HT_RX_PHY_API_S_VER_2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_tx_non_phy_v3 {
    pub preamble_cnt: __le32,
    pub rx_detected_cnt: __le32,
    pub bt_prio_defer_cnt: __le32,
    pub bt_prio_kill_cnt: __le32,
    pub few_bytes_cnt: __le32,
    pub cts_timeout: __le32,
    pub ack_timeout: __le32,
    pub expected_ack_cnt: __le32,
    pub actual_ack_cnt: __le32,
    pub dump_msdu_cnt: __le32,
    pub burst_abort_next_frame_mismatch_cnt: __le32,
    pub burst_abort_missing_next_frame_cnt: __le32,
    pub cts_timeout_collision: __le32,
    pub ack_or_ba_timeout_collision: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_TX_NON_PHY_API_S_VER_3,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_tx_non_phy {
    pub bt_prio_defer_cnt: __le32,
    pub bt_prio_kill_cnt: __le32,
    pub few_bytes_cnt: __le32,
    pub cts_timeout: __le32,
    pub ack_timeout: __le32,
    pub dump_msdu_cnt: __le32,
    pub burst_abort_next_frame_mismatch_cnt: __le32,
    pub burst_abort_missing_next_frame_cnt: __le32,
    pub cts_timeout_collision: __le32,
    pub ack_or_ba_timeout_collision: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_TX_NON_PHY_API_S_VER_4,
pub const MAX_CHAINS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_tx_non_phy_agg {
    pub ba_timeout: __le32,
    pub ba_reschedule_frames: __le32,
    pub scd_query_agg_frame_cnt: __le32,
    pub scd_query_no_agg: __le32,
    pub scd_query_agg: __le32,
    pub scd_query_mismatch: __le32,
    pub frame_not_ready: __le32,
    pub underrun: __le32,
    pub bt_prio_kill: __le32,
    pub rx_ba_rsp_cnt: __le32,
    pub txpower: [__s8; MAX_CHAINS],
    pub reserved: __s8,
    pub reserved2: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_TX_NON_PHY_AGG_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_tx_channel_width {
    pub ext_cca_narrow_ch20: [__le32; 1],
    pub ext_cca_narrow_ch40: [__le32; 2],
    pub ext_cca_narrow_ch80: [__le32; 3],
    pub ext_cca_narrow_ch160: [__le32; 4],
    pub last_tx_ch_width_indx: __le32,
    pub rx_detected_per_ch_width: [__le32; 4],
    pub success_per_ch_width: [__le32; 4],
    pub fail_per_ch_width: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_tx_v4 {
    pub general: mvm_statistics_tx_non_phy_v3,
    pub agg: mvm_statistics_tx_non_phy_agg,
    pub channel_width: mvm_statistics_tx_channel_width,
    pub /: *mut *mut } __packed; / STATISTICS_TX_API_S_VER_4,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_tx {
    pub general: mvm_statistics_tx_non_phy,
    pub agg: mvm_statistics_tx_non_phy_agg,
    pub channel_width: mvm_statistics_tx_channel_width,
    pub /: *mut *mut } __packed; / STATISTICS_TX_API_S_VER_5,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_bt_activity {
    pub hi_priority_tx_req_cnt: __le32,
    pub hi_priority_tx_denied_cnt: __le32,
    pub lo_priority_tx_req_cnt: __le32,
    pub lo_priority_tx_denied_cnt: __le32,
    pub hi_priority_rx_req_cnt: __le32,
    pub hi_priority_rx_denied_cnt: __le32,
    pub lo_priority_rx_req_cnt: __le32,
    pub lo_priority_rx_denied_cnt: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_BT_ACTIVITY_API_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_general_common_v19 {
    pub radio_temperature: __le32,
    pub radio_voltage: __le32,
    pub dbg: mvm_statistics_dbg,
    pub sleep_time: __le32,
    pub slots_out: __le32,
    pub slots_idle: __le32,
    pub ttl_timestamp: __le32,
    pub slow_div: mvm_statistics_div,
    pub rx_enable_counter: __le32,
//
// num_of_sos_states:
// count the number of times we have to re-tune
// in order to get out of bad PHY status
//
    pub num_of_sos_states: __le32,
    pub beacon_filtered: __le32,
    pub missed_beacons: __le32,
    pub beacon_filter_average_energy: u8,
    pub beacon_filter_reason: u8,
    pub beacon_filter_current_energy: u8,
    pub beacon_filter_reserved: u8,
    pub beacon_filter_delta_time: __le32,
    pub bt_activity: mvm_statistics_bt_activity,
    pub rx_time: __le64,
    pub on_time_rf: __le64,
    pub on_time_scan: __le64,
    pub tx_time: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_general_common {
    pub radio_temperature: __le32,
    pub dbg: mvm_statistics_dbg,
    pub sleep_time: __le32,
    pub slots_out: __le32,
    pub slots_idle: __le32,
    pub ttl_timestamp: __le32,
    pub slow_div: mvm_statistics_div,
    pub rx_enable_counter: __le32,
//
// num_of_sos_states:
// count the number of times we have to re-tune
// in order to get out of bad PHY status
//
    pub num_of_sos_states: __le32,
    pub beacon_filtered: __le32,
    pub missed_beacons: __le32,
    pub beacon_filter_average_energy: u8,
    pub beacon_filter_reason: u8,
    pub beacon_filter_current_energy: u8,
    pub beacon_filter_reserved: u8,
    pub beacon_filter_delta_time: __le32,
    pub bt_activity: mvm_statistics_bt_activity,
    pub rx_time: __le64,
    pub on_time_rf: __le64,
    pub on_time_scan: __le64,
    pub tx_time: __le64,
    pub /: *mut *mut } __packed; / STATISTICS_GENERAL_API_S_VER_10,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_general_v8 {
    pub common: mvm_statistics_general_common_v19,
    pub beacon_counter: [__le32; NUM_MAC_INDEX],
    pub beacon_average_energy: [u8; NUM_MAC_INDEX],
    pub 4)]: u8 reserved[4 - (NUM_MAC_INDEX %,
    pub /: *mut *mut } __packed; / STATISTICS_GENERAL_API_S_VER_8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_general {
    pub common: mvm_statistics_general_common,
    pub beacon_counter: [__le32; MAC_INDEX_AUX],
    pub beacon_average_energy: [u8; MAC_INDEX_AUX],
    pub MAC_INDEX_AUX]: u8 reserved[8 -,
    pub /: *mut *mut } __packed; / STATISTICS_GENERAL_API_S_VER_10,
//
// struct mvm_statistics_load - RX statistics for multi-queue devices
// @air_time: accumulated air time, per mac
// @byte_count: accumulated byte count, per mac
// @pkt_count: accumulated packet count, per mac
// @avg_energy: average RSSI, per station
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_load {
    pub air_time: [__le32; MAC_INDEX_AUX],
    pub byte_count: [__le32; MAC_INDEX_AUX],
    pub pkt_count: [__le32; MAC_INDEX_AUX],
    pub avg_energy: [u8; IWL_STATION_COUNT_MAX],
    pub /: *mut *mut } __packed; / STATISTICS_RX_MAC_STATION_S_VER_3,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_load_v1 {
    pub air_time: [__le32; NUM_MAC_INDEX],
    pub byte_count: [__le32; NUM_MAC_INDEX],
    pub pkt_count: [__le32; NUM_MAC_INDEX],
    pub avg_energy: [u8; IWL_STATION_COUNT_MAX],
    pub /: *mut *mut } __packed; / STATISTICS_RX_MAC_STATION_S_VER_1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_rx {
    pub ofdm: mvm_statistics_rx_phy,
    pub cck: mvm_statistics_rx_phy,
    pub general: mvm_statistics_rx_non_phy,
    pub ofdm_ht: mvm_statistics_rx_ht_phy,
    pub /: *mut *mut } __packed; / STATISTICS_RX_API_S_VER_4,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvm_statistics_rx_v3 {
    pub ofdm: mvm_statistics_rx_phy_v2,
    pub cck: mvm_statistics_rx_phy_v2,
    pub general: mvm_statistics_rx_non_phy_v3,
    pub ofdm_ht: mvm_statistics_rx_ht_phy_v1,
    pub /: *mut *mut } __packed; / STATISTICS_RX_API_S_VER_3,
//
// STATISTICS_NOTIFICATION = 0x9d (notification only, not a command)
//
// By default, uCode issues this notification after receiving a beacon
// while associated.  To disable this behavior, set DISABLE_NOTIF flag in the
// STATISTICS_CMD (0x9c), below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_notif_statistics_v10 {
    pub flag: __le32,
    pub rx: mvm_statistics_rx_v3,
    pub tx: mvm_statistics_tx_v4,
    pub general: mvm_statistics_general_v8,
    pub /: *mut *mut } __packed; / STATISTICS_NTFY_API_S_VER_10,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_notif_statistics_v11 {
    pub flag: __le32,
    pub rx: mvm_statistics_rx_v3,
    pub tx: mvm_statistics_tx_v4,
    pub general: mvm_statistics_general_v8,
    pub load_stats: mvm_statistics_load_v1,
    pub /: *mut *mut } __packed; / STATISTICS_NTFY_API_S_VER_11,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_notif_statistics {
    pub flag: __le32,
    pub rx: mvm_statistics_rx,
    pub tx: mvm_statistics_tx,
    pub general: mvm_statistics_general,
    pub load_stats: mvm_statistics_load,
    pub /: *mut *mut } __packed; / STATISTICS_NTFY_API_S_VER_13,
//
// enum iwl_statistics_notif_flags - flags used in statistics notification
// @IWL_STATISTICS_REPLY_FLG_CLEAR: statistics were cleared after this report
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_statistics_notif_flags {
    IWL_STATISTICS_REPLY_FLG_CLEAR		= 0x1,
}

//
// enum iwl_statistics_cmd_flags - flags used in statistics command
// @IWL_STATISTICS_FLG_CLEAR: request to clear statistics after the report
// that's sent after this command
// @IWL_STATISTICS_FLG_DISABLE_NOTIF: disable unilateral statistics
// notifications
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_statistics_cmd_flags {
    IWL_STATISTICS_FLG_CLEAR		= 0x1,
    IWL_STATISTICS_FLG_DISABLE_NOTIF	= 0x2,
}

//
// struct iwl_statistics_cmd - statistics config command
// @flags: flags from &enum iwl_statistics_cmd_flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_cmd {
    pub flags: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_CMD_API_S_VER_1,
pub const MAX_BCAST_FILTER_NUM: c_int = 8;
//
// enum iwl_statistics_notify_type_id - type_id used in system statistics
// command
// @IWL_STATS_NTFY_TYPE_ID_OPER: request legacy statistics
// @IWL_STATS_NTFY_TYPE_ID_OPER_PART1: request operational part1 statistics
// @IWL_STATS_NTFY_TYPE_ID_OPER_PART2: request operational part2 statistics
// @IWL_STATS_NTFY_TYPE_ID_OPER_PART3: request operational part3 statistics
// @IWL_STATS_NTFY_TYPE_ID_OPER_PART4: request operational part4 statistics
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_statistics_notify_type_id {
    IWL_STATS_NTFY_TYPE_ID_OPER		= BIT(0),
    IWL_STATS_NTFY_TYPE_ID_OPER_PART1	= BIT(1),
    IWL_STATS_NTFY_TYPE_ID_OPER_PART2	= BIT(2),
    IWL_STATS_NTFY_TYPE_ID_OPER_PART3	= BIT(3),
    IWL_STATS_NTFY_TYPE_ID_OPER_PART4	= BIT(4),
}

//
// enum iwl_statistics_cfg_flags - cfg_mask used in system statistics command
// @IWL_STATS_CFG_FLG_DISABLE_NTFY_MSK: 0 for enable, 1 for disable
// @IWL_STATS_CFG_FLG_ON_DEMAND_NTFY_MSK: 0 for periodic, 1 for on-demand
// @IWL_STATS_CFG_FLG_RESET_MSK: 0 for reset statistics after
// sending the notification, 1 for do not reset statistics after sending
// the notification
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_statistics_cfg_flags {
    IWL_STATS_CFG_FLG_DISABLE_NTFY_MSK	= BIT(0),
    IWL_STATS_CFG_FLG_ON_DEMAND_NTFY_MSK	= BIT(1),
    IWL_STATS_CFG_FLG_RESET_MSK		= BIT(2),
}

//
// struct iwl_system_statistics_cmd - system statistics command
// @cfg_mask: configuration mask, &enum iwl_statistics_cfg_flags
// @config_time_sec: time in sec for periodic notification
// @type_id_mask: type_id masks, &enum iwl_statistics_notify_type_id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_system_statistics_cmd {
    pub cfg_mask: __le32,
    pub config_time_sec: __le32,
    pub type_id_mask: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_FW_CMD_API_S_VER_1,
//
// enum iwl_fw_statistics_type - statistics type
//
// @FW_STATISTICS_OPERATIONAL: operational statistics
// @FW_STATISTICS_PHY: phy statistics
// @FW_STATISTICS_MAC: mac statistics
// @FW_STATISTICS_RX: rx statistics
// @FW_STATISTICS_TX: tx statistics
// @FW_STATISTICS_DURATION: duration statistics
// @FW_STATISTICS_HE: he statistics
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_statistics_type {
    FW_STATISTICS_OPERATIONAL,
    FW_STATISTICS_PHY,
    FW_STATISTICS_MAC,
    FW_STATISTICS_RX,
    FW_STATISTICS_TX,
    FW_STATISTICS_DURATION,
    FW_STATISTICS_HE,
}

pub const IWL_STATISTICS_TYPE_MSK: c_uint = 0x7f;
//
// struct iwl_statistics_ntfy_hdr - statistics notification header
//
// @type: struct type
// @version: version of the struct
// @size: size in bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_ntfy_hdr {
    pub type: u8,
    pub version: u8,
    pub size: __le16,
}

//
// struct iwl_stats_ntfy_per_link - per-link statistics
//
// @beacon_filter_average_energy: Average energy [-dBm] of the 2
// antennas.
// @air_time: air time
// @beacon_counter: all beacons (both filtered and not filtered)
// @beacon_average_energy: Average energy [-dBm] of all beacons
// (both filtered and not filtered)
// @beacon_rssi_a: beacon RSSI on antenna A
// @beacon_rssi_b: beacon RSSI on antenna B
// @rx_bytes: RX byte count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_stats_ntfy_per_link {
    pub beacon_filter_average_energy: __le32,
    pub air_time: __le32,
    pub beacon_counter: __le32,
    pub beacon_average_energy: __le32,
    pub beacon_rssi_a: __le32,
    pub beacon_rssi_b: __le32,
    pub rx_bytes: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_NTFY_PER_LINK_API_S_VER_1,
//
// struct iwl_stats_ntfy_part1_per_link - part1 per link statistics
//
// @rx_time: rx time
// @tx_time: tx time
// @rx_action: action frames handled by FW
// @tx_action: action frames generated and transmitted by FW
// @cca_defers: cca defer count
// @beacon_filtered: filtered out beacons
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_stats_ntfy_part1_per_link {
    pub rx_time: __le64,
    pub tx_time: __le64,
    pub rx_action: __le32,
    pub tx_action: __le32,
    pub cca_defers: __le32,
    pub beacon_filtered: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_FW_NTFY_OPERATIONAL_PART1_PER_LINK_API_S_VER_1,
//
// struct iwl_stats_ntfy_per_mac - per MAC statistics
//
// @beacon_filter_average_energy: Average energy [-dBm] of the 2
// antennas.
// @air_time: air time
// @beacon_counter: all beacons (both filtered and not filtered)
// @beacon_average_energy: all beacons (both filtered and not
// filtered)
// @beacon_rssi_a: beacon RSSI on antenna A
// @beacon_rssi_b: beacon RSSI on antenna B
// @rx_bytes: RX byte count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_stats_ntfy_per_mac {
    pub beacon_filter_average_energy: __le32,
    pub air_time: __le32,
    pub beacon_counter: __le32,
    pub beacon_average_energy: __le32,
    pub beacon_rssi_a: __le32,
    pub beacon_rssi_b: __le32,
    pub rx_bytes: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_NTFY_PER_MAC_API_S_VER_1,
pub const IWL_STATS_MAX_BW_INDEX: c_int = 5;
//
// struct iwl_stats_ntfy_per_phy_v1 - per PHY statistics
// @channel_load: channel load
// @channel_load_by_us: device contribution to MCLM
// @channel_load_not_by_us: other devices' contribution to MCLM
// @clt: CLT HW timer (TIM_CH_LOAD2)
// @act: active accumulator SW
// @elp: elapsed time accumulator SW
// @rx_detected_per_ch_width: number of deferred TX per channel width,
// 0 - 20, 1/2/3 - 40/80/160
// @success_per_ch_width: number of frames that got ACK/BACK/CTS
// per channel BW. note, BACK counted as 1
// @fail_per_ch_width: number of frames that didn't get ACK/BACK/CTS
// per channel BW. note BACK counted as 1
// @last_tx_ch_width_indx: last txed frame channel width index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_stats_ntfy_per_phy_v1 {
    pub channel_load: __le32,
    pub channel_load_by_us: __le32,
    pub channel_load_not_by_us: __le32,
    pub clt: __le32,
    pub act: __le32,
    pub elp: __le32,
    pub rx_detected_per_ch_width: [__le32; IWL_STATS_MAX_BW_INDEX],
    pub success_per_ch_width: [__le32; IWL_STATS_MAX_BW_INDEX],
    pub fail_per_ch_width: [__le32; IWL_STATS_MAX_BW_INDEX],
    pub last_tx_ch_width_indx: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_NTFY_PER_PHY_API_S_VER_1,
//
// struct iwl_stats_ntfy_coex - coex statistics
//
// @wifi_kill_cnt: count of wifi frames killed by BT
// @wifi_tx_cts_kill_cnt: count of wifi Tx CTS frames killed by BT
// @ttc2_ppdu_error_count: Count PPDU errors on TTC2 - BT Tx indication rises
// within wifi Tx packet on non-shared antenna and wifi is NOT killed by
// PTA/TCL.
// @trc2_ppdu_error_count: count PPDU errors on TRC2 - BT Rx indication rises
// within wifi Tx packet on non-shared antenna and wifi is  NOT killed by
// PTA/TCL
// @rrc1_collision_count: count RRC1 - BT Rx indication rises within wifi Rx
// packet on shared antenna
// @rrc2_collision_count: count RRC2 - BT Rx indication rises within wifi Rx
// packet on non-shared antenna
// @rtc2_collision_count: count RTC2 - BT Tx indication rises within wifi Rx
// packet on non-shared antenna
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_stats_ntfy_coex {
    pub wifi_kill_cnt: __le16,
    pub wifi_tx_cts_kill_cnt: __le16,
    pub ttc2_ppdu_error_count: __le16,
    pub trc2_ppdu_error_count: __le16,
    pub rrc1_collision_count: __le16,
    pub rrc2_collision_count: __le16,
    pub rtc2_collision_count: __le16,
    pub reserved: __le16,
    pub /: *mut *mut } __packed; / STATISTICS_FW_NTFY_COEX_TELEMETRY_API_S_VER_1,
//
// struct iwl_stats_ntfy_per_phy - per PHY statistics
// @channel_load: channel load
// @channel_load_by_us: device contribution to MCLM
// @channel_load_not_by_us: other devices' contribution to MCLM
// @clt: CLT HW timer (TIM_CH_LOAD2)
// @act: active accumulator SW
// @elp: elapsed time accumulator SW
// @rx_detected_per_ch_width: number of deferred TX per channel width,
// 0 - 20, 1/2/3 - 40/80/160
// @success_per_ch_width: number of frames that got ACK/BACK/CTS
// per channel BW. note, BACK counted as 1
// @fail_per_ch_width: number of frames that didn't get ACK/BACK/CTS
// per channel BW. note BACK counted as 1
// @last_tx_ch_width_indx: last txed frame channel width index
// @coex: coex related data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_stats_ntfy_per_phy {
    pub channel_load: __le32,
    pub channel_load_by_us: __le32,
    pub channel_load_not_by_us: __le32,
    pub clt: __le32,
    pub act: __le32,
    pub elp: __le32,
    pub rx_detected_per_ch_width: [__le32; IWL_STATS_MAX_BW_INDEX],
    pub success_per_ch_width: [__le32; IWL_STATS_MAX_BW_INDEX],
    pub fail_per_ch_width: [__le32; IWL_STATS_MAX_BW_INDEX],
    pub last_tx_ch_width_indx: __le32,
    pub coex: iwl_stats_ntfy_coex,
    pub /: *mut *mut } __packed; / STATISTICS_NTFY_PER_PHY_API_S_VER_2,
// unknown channel load (due to not being active on channel)
pub const IWL_STATS_UNKNOWN_CHANNEL_LOAD: c_uint = 0xffffffff;
//
// struct iwl_stats_ntfy_per_sta - per STA statistics
//
// @average_energy: in fact it is minus the energy..
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_stats_ntfy_per_sta {
    pub average_energy: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_NTFY_PER_STA_API_S_VER_1,
pub const IWL_STATS_MAX_PHY_OPERATIONAL: c_int = 3;
//
// struct iwl_system_statistics_notif_oper_v3 - statistics notification
//
// @time_stamp: time when the notification is sent from firmware
// @per_link: per link statistics, &struct iwl_stats_ntfy_per_link
// @per_phy: per phy statistics, &struct iwl_stats_ntfy_per_phy_v1
// @per_sta: per sta statistics, &struct iwl_stats_ntfy_per_sta
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_system_statistics_notif_oper_v3 {
    pub time_stamp: __le32,
    pub per_link: [iwl_stats_ntfy_per_link; IWL_FW_MAX_LINKS],
    pub per_phy: [iwl_stats_ntfy_per_phy_v1; IWL_STATS_MAX_PHY_OPERATIONAL],
    pub per_sta: [iwl_stats_ntfy_per_sta; IWL_STATION_COUNT_MAX],
    pub /: *mut *mut } __packed; / STATISTICS_FW_NTFY_OPERATIONAL_API_S_VER_3,
//
// struct iwl_system_statistics_notif_oper - statistics notification
//
// @time_stamp: time when the notification is sent from firmware
// @per_link: per link statistics, &struct iwl_stats_ntfy_per_link
// @per_phy: per phy statistics, &struct iwl_stats_ntfy_per_phy_v1
// @per_sta: per sta statistics, &struct iwl_stats_ntfy_per_sta
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_system_statistics_notif_oper {
    pub time_stamp: __le32,
    pub per_link: [iwl_stats_ntfy_per_link; IWL_FW_MAX_LINKS],
    pub per_phy: [iwl_stats_ntfy_per_phy; IWL_STATS_MAX_PHY_OPERATIONAL],
    pub per_sta: [iwl_stats_ntfy_per_sta; IWL_STATION_COUNT_MAX],
    pub /: *mut *mut } __packed; / STATISTICS_FW_NTFY_OPERATIONAL_API_S_VER_4,
//
// struct iwl_system_statistics_part1_notif_oper - part1 stats notification
//
// @time_stamp: time when the notification is sent from firmware
// @per_link: per link statistics &struct iwl_stats_ntfy_part1_per_link
// @per_phy_crc_error_stats: per phy crc error statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_system_statistics_part1_notif_oper {
    pub time_stamp: __le32,
    pub per_link: [iwl_stats_ntfy_part1_per_link; IWL_FW_MAX_LINKS],
    pub per_phy_crc_error_stats: [__le32; IWL_STATS_MAX_PHY_OPERATIONAL],
    pub /: *mut *mut } __packed; / STATISTICS_FW_NTFY_OPERATIONAL_PART1_API_S_VER_4,
//
// struct iwl_system_statistics_end_notif - statistics end notification
//
// @time_stamp: time when the notification is sent from firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_system_statistics_end_notif {
    pub time_stamp: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_FW_NTFY_END_API_S_VER_1,
//
// struct iwl_statistics_operational_ntfy - operational stats notification
//
// @hdr: general statistics header
// @flags: bitmap of possible notification structures
// @per_mac: per mac statistics, &struct iwl_stats_ntfy_per_mac
// @per_phy: per phy statistics, &struct iwl_stats_ntfy_per_phy_v1
// @per_sta: per sta statistics, &struct iwl_stats_ntfy_per_sta
// @rx_time: rx time
// @tx_time: usec the radio is transmitting.
// @on_time_rf: The total time in usec the RF is awake.
// @on_time_scan: usec the radio is awake due to scan.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_operational_ntfy {
    pub hdr: iwl_statistics_ntfy_hdr,
    pub flags: __le32,
    pub per_mac: [iwl_stats_ntfy_per_mac; MAC_INDEX_AUX],
    pub per_phy: [iwl_stats_ntfy_per_phy_v1; IWL_STATS_MAX_PHY_OPERATIONAL],
    pub per_sta: [iwl_stats_ntfy_per_sta; IWL_STATION_COUNT_MAX],
    pub rx_time: __le64,
    pub tx_time: __le64,
    pub on_time_rf: __le64,
    pub on_time_scan: __le64,
    pub /: *mut *mut } __packed; / STATISTICS_OPERATIONAL_NTFY_API_S_VER_15,
//
// struct iwl_statistics_operational_ntfy_ver_14 - operational stats notification
//
// @hdr: general statistics header
// @flags: bitmap of possible notification structures
// @mac_id: mac on which the beacon was received
// @beacon_filter_average_energy: Average energy [-dBm] of the 2
// antennas.
// @beacon_filter_reason: beacon filter reason
// @radio_temperature: radio temperature
// @air_time: air time
// @beacon_counter: all beacons (both filtered and not filtered)
// @beacon_average_energy: all beacons (both filtered and not
// filtered)
// @beacon_rssi_a: beacon RSSI on antenna A
// @beacon_rssi_b: beacon RSSI on antenna B
// @rx_bytes: per MAC RX byte count
// @rx_time: rx time
// @tx_time: usec the radio is transmitting.
// @on_time_rf: The total time in usec the RF is awake.
// @on_time_scan: usec the radio is awake due to scan.
// @average_energy: in fact it is minus the energy..
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_operational_ntfy_ver_14 {
    pub hdr: iwl_statistics_ntfy_hdr,
    pub flags: __le32,
    pub mac_id: __le32,
    pub beacon_filter_average_energy: __le32,
    pub beacon_filter_reason: __le32,
    pub radio_temperature: __le32,
    pub air_time: [__le32; MAC_INDEX_AUX],
    pub beacon_counter: [__le32; MAC_INDEX_AUX],
    pub beacon_average_energy: [__le32; MAC_INDEX_AUX],
    pub beacon_rssi_a: __le32,
    pub beacon_rssi_b: __le32,
    pub rx_bytes: [__le32; MAC_INDEX_AUX],
    pub rx_time: __le64,
    pub tx_time: __le64,
    pub on_time_rf: __le64,
    pub on_time_scan: __le64,
    pub average_energy: [__le32; IWL_STATION_COUNT_MAX],
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_OPERATIONAL_NTFY_API_S_VER_14,
//
// struct iwl_statistics_phy_ntfy - PHY statistics notification
//
// @hdr: general statistics header
// RX PHY related statistics
// @energy_and_config: ???
// @rssi_band: @31:24 rssiAllBand_B, 23:16 rssiInBand_B, 15:8
// rssiAllBand_A, 7:0 rssiInBand_A
// @agc_word: @31:16 agcWord_B, 15:0 agcWord_A
// @agc_gain: @19:10 agcGain_B, 9:0 agcGain_A
// @dfe_gain: @19:10 dfeGain_B, 9:0 dfeGain_A
// @snr_calc_main: @18:0 snrCalcMain
// @energy_calc_main: @18:0 energyCalcMain
// @snr_calc_aux: @18:0 snrCalcAux
// @dsp_dc_estim_a: @27:14 dspDcEstimQA, 13:0 dspDcEstimIA
// @dsp_dc_estim_b: @27:14 dspDcEstimQB, 13:0 dspDcEstimIB
// @ina_detec_type_and_ofdm_corr_comb: @31:31 inaDetectCckMrc,
// 30:27 inaDetectType, 26:0 ofdmCorrComb
// @cw_corr_comb: @26:0 cwCorrComb
// @rssi_comb: @25:0 rssiComb
// @auto_corr_cck: @23:12 autoCck, 11:00 crossCck
// @ofdm_fine_freq_and_pina_freq_err: @18:7 ofdmFineFreq, 6:0
// ofdmPinaFreqErr
// @snrm_evm_main: @31:0 snrmEvmMain
// @snrm_evm_aux: @31:0 snrmEvmAux
// @rx_rate: @31:0 rate
// TX PHY related statistics
// @per_chain_enums_and_dsp_atten_a: @perChainEnumsAndDspAtten
// (per version)
// @target_power_and_power_meas_a: @31:16 targetPower_A, 15:0
// powerMeasuredCalc_A
// @tx_config_as_i_and_ac_a: @31:16 txConfigAsI_A, 15:0
// txConfigAc_A
// @predist_dcq_and_dci_a: @31:16 predist_dci_A, 15:0
// predist_dcq_A
// @per_chain_enums_and_dsp_atten_b: @perChainEnumsAndDspAtten
// (per version)
// @target_power_and_power_meas_b: @31:16 targetPower_B, 15:0
// powerMeasuredCalc_B
// @tx_config_as_i_and_ac_b: @31:16 txConfigAsI_B, 15:0
// txConfigAc_B
// @predist_dcq_and_dci_b: @31:16 predist_dci_B, 15:0
// predist_dcq_B
// @tx_rate: @31:0 rate
// @tlc_backoff: @31:0 tlcBackoff
// @mpapd_calib_mode_mpapd_calib_type_a: @31:16
// mpapdCalibMode_A, 15:0 mpapdCalibType_A
// @psat_and_phy_power_limit_a: @31:16 psat_A, 15:0
// phyPowerLimit_A
// @sar_and_regulatory_power_limit_a: @31:16 sarPowerLimit_A,
// 15:0 regulatoryPowerLimit_A
// @mpapd_calib_mode_mpapd_calib_type_b: @31:16
// mpapdCalibMode_B, 15:0 mpapdCalibType_B
// @psat_and_phy_power_limit_b: @31:16 psat_B, 15:0
// phyPowerLimit_B
// @sar_and_regulatory_power_limit_b: @31:16 sarPowerLimit_B,
// 15:0 regulatoryPowerLimit_B
// @srd_and_driver_power_limits: @31:16 srdPowerLimit, 15:0
// driverPowerLimit
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_phy_ntfy {
    pub hdr: iwl_statistics_ntfy_hdr,
    pub energy_and_config: __le32,
    pub rssi_band: __le32,
    pub agc_word: __le32,
    pub agc_gain: __le32,
    pub dfe_gain: __le32,
    pub snr_calc_main: __le32,
    pub energy_calc_main: __le32,
    pub snr_calc_aux: __le32,
    pub dsp_dc_estim_a: __le32,
    pub dsp_dc_estim_b: __le32,
    pub ina_detec_type_and_ofdm_corr_comb: __le32,
    pub cw_corr_comb: __le32,
    pub rssi_comb: __le32,
    pub auto_corr_cck: __le32,
    pub ofdm_fine_freq_and_pina_freq_err: __le32,
    pub snrm_evm_main: __le32,
    pub snrm_evm_aux: __le32,
    pub rx_rate: __le32,
    pub per_chain_enums_and_dsp_atten_a: __le32,
    pub target_power_and_power_meas_a: __le32,
    pub tx_config_as_i_and_ac_a: __le32,
    pub predist_dcq_and_dci_a: __le32,
    pub per_chain_enums_and_dsp_atten_b: __le32,
    pub target_power_and_power_meas_b: __le32,
    pub tx_config_as_i_and_ac_b: __le32,
    pub predist_dcq_and_dci_b: __le32,
    pub tx_rate: __le32,
    pub tlc_backoff: __le32,
    pub mpapd_calib_mode_mpapd_calib_type_a: __le32,
    pub psat_and_phy_power_limit_a: __le32,
    pub sar_and_regulatory_power_limit_a: __le32,
    pub mpapd_calib_mode_mpapd_calib_type_b: __le32,
    pub psat_and_phy_power_limit_b: __le32,
    pub sar_and_regulatory_power_limit_b: __le32,
    pub srd_and_driver_power_limits: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_PHY_NTFY_API_S_VER_1,
//
// struct iwl_statistics_mac_ntfy - MAC statistics notification
//
// @hdr: general statistics header
// @bcast_filter_passed_per_mac: bcast filter passed per mac
// @bcast_filter_dropped_per_mac: bcast filter dropped per mac
// @bcast_filter_passed_per_filter: bcast filter passed per filter
// @bcast_filter_dropped_per_filter: bcast filter dropped per filter
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_mac_ntfy {
    pub hdr: iwl_statistics_ntfy_hdr,
    pub bcast_filter_passed_per_mac: [__le32; NUM_MAC_INDEX_CDB],
    pub bcast_filter_dropped_per_mac: [__le32; NUM_MAC_INDEX_CDB],
    pub bcast_filter_passed_per_filter: [__le32; MAX_BCAST_FILTER_NUM],
    pub bcast_filter_dropped_per_filter: [__le32; MAX_BCAST_FILTER_NUM],
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_MAC_NTFY_API_S_VER_1,
//
// struct iwl_statistics_rx_ntfy - RX statistics notification
//
// @hdr: general statistics header
// @rx_agg_mpdu_cnt: aggregation frame count (number of
// delimiters)
// @rx_agg_cnt: number of RX Aggregations
// @unsupported_mcs: number of PLCP headers that have rate which
// is unsupported by DSP
// @bogus_cts: CTS received when not expecting CTS
// @bogus_ack: ACK received when not expecting ACK
// @rx_byte_count: ???
// @rx_packet_count: ???
// @missed_beacons: ???
// @unresponded_rts: un-responded RTS, due to NAV not zero
// @rxe_frame_limit_overrun: RXE got frame limit overrun
// @sent_ba_rsp_cnt: BA response TX count
// @late_rx_handle: count the number of times the RX path was
// aborted due to late entry
// @num_bt_kills: ???
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_rx_ntfy {
    pub hdr: iwl_statistics_ntfy_hdr,
    pub rx_agg_mpdu_cnt: __le32,
    pub rx_agg_cnt: __le32,
    pub unsupported_mcs: __le32,
    pub bogus_cts: __le32,
    pub bogus_ack: __le32,
    pub rx_byte_count: [__le32; MAC_INDEX_AUX],
    pub rx_packet_count: [__le32; MAC_INDEX_AUX],
    pub missed_beacons: __le32,
    pub unresponded_rts: __le32,
    pub rxe_frame_limit_overrun: __le32,
    pub sent_ba_rsp_cnt: __le32,
    pub late_rx_handle: __le32,
    pub num_bt_kills: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_RX_NTFY_API_S_VER_1,
//
// struct iwl_statistics_tx_ntfy - TX statistics notification
//
// @hdr: general statistics header
// @cts_timeout: timeout when waiting for CTS
// @ack_timeout: timeout when waiting for ACK
// @dump_msdu_cnt: number of MSDUs that were dumped due to any
// reason
// @burst_abort_missing_next_frame_cnt: number of times a burst
// was aborted due to missing next frame bytes in txfifo
// number of times got timeout when waiting for CTS/ACK/BA and energy was
// detected just after sending the RTS/DATA. this statistics may help getting
// interesting indicators, like the likelihood of collision (so the benefit of
// protection may be estimated Vs. its cost). Or how many of the failures are
// due to collision and how many due to SNR.
// For Link-quality the CTS collision indication is more reliable then the ACK
// collision indication as the RTS frame is short and has more chance that the
// frame/s which caused the collision continue after the RTS was sent.
// @cts_timeout_collision: ???
// ACK/BA failed and energy as detected after DATA
// Note: to get the collision ratio need to:
// ackOrBaTimeoutCollision / (ack_timeout + ba_timeout)
// @ack_or_ba_timeout_collision: ???
// @ba_timeout: timeout when waiting for immediate BA response
// @ba_reschedule_frames: failed to get BA response and
// rescheduled all the non-ACKed frames
// gives the avarage number of frames inside aggregation
// @scd_query_agg_frame_cnt: ???
// @scd_query_no_agg: scheduler query prevented aggregation
// @scd_query_agg: scheduler query allowed aggregation
// @scd_query_mismatch: scheduler query inaccurate, either too
// short or too long
// @agg_terminated_underrun: aggregation was terminated due to
// underrun
// @agg_terminated_bt_prio_kill: aggregation was terminated due
// to BT
// @tx_kill_on_long_retry: count the tx frames dropped due to
// long retry limit (DATA frame failed)
// @tx_kill_on_short_retry: count the tx frames dropped due to
// short retry limit (RTS frame failed)
// TX deffer on energy. This counter is reset on each successful transmit.
// When timer exceed TX deffer limit than will be uCode assert.
// @tx_deffer_counter: ???
// @tx_deffer_base_time: Keep the time of the last successful
// transmit
// @tx_underrun: TX killed due to underrun
// @bt_defer: TX deferred due to BT priority, so probably TX was
// not started.
// @tx_kill_on_dsp_timeout: TX killed on DSP problem detected
// @tx_kill_on_immediate_quiet: TX killed due to immediate quiet
// @kill_ba_cnt: number of times sending BA failed
// @kill_ack_cnt: number of times sending ACK failed
// @kill_cts_cnt: number of times sending CTS failed
// @burst_terminated: Count burst or fragmentation termination
// occurrence
// @late_tx_vec_wr_cnt: ???
// TX is not sent because ucode failed to notify the TRM in SIFS-delta from
// ON_AIR deassertion.
// @late_rx2_tx_cnt: ???
// @scd_query_cnt: count the times SCD query was done to check
// for TX AGG
// @tx_frames_acked_in_agg: count the number of frames
// transmitted inside AGG and were successful
// @last_tx_ch_width_indx: ???
// number of deferred TX per channel width, 0 - 20, 1/2/3 - 40/80/160
// @rx_detected_per_ch_width: ???
// @success_per_ch_width: ???
// @fail_per_ch_width: ???
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_tx_ntfy {
    pub hdr: iwl_statistics_ntfy_hdr,
    pub cts_timeout: __le32,
    pub ack_timeout: __le32,
    pub dump_msdu_cnt: __le32,
    pub burst_abort_missing_next_frame_cnt: __le32,
    pub cts_timeout_collision: __le32,
    pub ack_or_ba_timeout_collision: __le32,
    pub ba_timeout: __le32,
    pub ba_reschedule_frames: __le32,
    pub scd_query_agg_frame_cnt: __le32,
    pub scd_query_no_agg: __le32,
    pub scd_query_agg: __le32,
    pub scd_query_mismatch: __le32,
    pub agg_terminated_underrun: __le32,
    pub agg_terminated_bt_prio_kill: __le32,
    pub tx_kill_on_long_retry: __le32,
    pub tx_kill_on_short_retry: __le32,
    pub tx_deffer_counter: __le32,
    pub tx_deffer_base_time: __le32,
    pub tx_underrun: __le32,
    pub bt_defer: __le32,
    pub tx_kill_on_dsp_timeout: __le32,
    pub tx_kill_on_immediate_quiet: __le32,
    pub kill_ba_cnt: __le32,
    pub kill_ack_cnt: __le32,
    pub kill_cts_cnt: __le32,
    pub burst_terminated: __le32,
    pub late_tx_vec_wr_cnt: __le32,
    pub late_rx2_tx_cnt: __le32,
    pub scd_query_cnt: __le32,
    pub tx_frames_acked_in_agg: __le32,
    pub last_tx_ch_width_indx: __le32,
    pub rx_detected_per_ch_width: [__le32; 4],
    pub success_per_ch_width: [__le32; 4],
    pub fail_per_ch_width: [__le32; 4],
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_TX_NTFY_API_S_VER_1,
//
// struct iwl_statistics_duration_ntfy - burst/duration statistics
//
// @hdr: general statistics header
// @cont_burst_chk_cnt: number of times continuation or
// fragmentation or bursting was checked
// @cont_burst_cnt: number of times continuation or fragmentation
// or bursting was successful
// @wait_for_silence_timeout_cnt: ???
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_duration_ntfy {
    pub hdr: iwl_statistics_ntfy_hdr,
    pub cont_burst_chk_cnt: __le32,
    pub cont_burst_cnt: __le32,
    pub wait_for_silence_timeout_cnt: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_DURATION_NTFY_API_S_VER_1,
//
// struct iwl_statistics_he_ntfy - HE statistics
//
// @hdr: general statistics header
// received HE frames
// @rx_siga_valid_cnt: rx HE SIG-A valid
// @rx_siga_invalid_cnt: rx HE SIG-A invalid
// received HE frames w/ valid Sig-A
// @rx_trig_based_frame_cnt: rx HE-TB (trig-based)
// @rx_su_frame_cnt: rx HE-SU
// @rx_sigb_invalid_cnt: rx (suspected) HE-MU w/ bad SIG-B
// @rx_our_bss_color_cnt: rx valid HE SIG-A w/ our BSS color
// @rx_other_bss_color_cnt: rx valid HE SIG-A w/ other BSS color
// @rx_zero_bss_color_cnt: ???
// received HE-MU frames w/ good Sig-B
// @rx_mu_for_us_cnt: match AID
// @rx_mu_not_for_us_cnt: no matched AID
// received HE-MU frames for us (w/ our AID)
// @rx_mu_nss_ar: 0 - SISO, 1 - MIMO2
// @rx_mu_mimo_cnt: full BW RU, compressed SIG-B
// @rx_mu_ru_bw_ar: MU alloc, MHz: 0 - 2, 1 - 5, 2 - 10, 3 - 20,
// 4 - 40, 5 - 80, 6 - 160
// received trigger frames
// @rx_trig_for_us_cnt: ???
// @rx_trig_not_for_us_cnt: ???
// trigger for us
// @rx_trig_with_cs_req_cnt: ???
// @rx_trig_type_ar: ???
// @rx_trig_in_agg_cnt: ???
// basic trigger for us allocations
// @rx_basic_trig_alloc_nss_ar: ???
// @rx_basic_trig_alloc_mu_mimo_cnt: ???
// @rx_basic_trig_alloc_ru_bw_ar: ???
// @rx_basic_trig_total_byte_cnt: ???
// trig-based TX
// @tx_trig_based_cs_req_fail_cnt: ???
// @tx_trig_based_sifs_ok_cnt: ???
// @tx_trig_based_sifs_fail_cnt: ???
// @tx_trig_based_byte_cnt: ???
// @tx_trig_based_pad_byte_cnt: ???
// @tx_trig_based_frame_cnt: ???
// @tx_trig_based_acked_frame_cnt: ???
// @tx_trig_based_ack_timeout_cnt: ???
// HE-SU TX
// @tx_su_frame_cnt: ???
// EDCA <--> MU-EDCA transitions
// @tx_edca_to_mu_edca_cnt: ???
// @tx_mu_edca_to_edca_by_timeout_cnt: ???
// @tx_mu_edca_to_edca_by_ack_fail_cnt: ???
// @tx_mu_edca_to_edca_by_small_alloc_cnt: ???
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_he_ntfy {
    pub hdr: iwl_statistics_ntfy_hdr,
    pub rx_siga_valid_cnt: __le32,
    pub rx_siga_invalid_cnt: __le32,
    pub rx_trig_based_frame_cnt: __le32,
    pub rx_su_frame_cnt: __le32,
    pub rx_sigb_invalid_cnt: __le32,
    pub rx_our_bss_color_cnt: __le32,
    pub rx_other_bss_color_cnt: __le32,
    pub rx_zero_bss_color_cnt: __le32,
    pub rx_mu_for_us_cnt: __le32,
    pub rx_mu_not_for_us_cnt: __le32,
    pub rx_mu_nss_ar: [__le32; 2],
    pub rx_mu_mimo_cnt: __le32,
    pub rx_mu_ru_bw_ar: [__le32; 7],
    pub rx_trig_for_us_cnt: __le32,
    pub rx_trig_not_for_us_cnt: __le32,
    pub rx_trig_with_cs_req_cnt: __le32,
    pub 1]: __le32 rx_trig_type_ar[8 +,
    pub rx_trig_in_agg_cnt: __le32,
    pub rx_basic_trig_alloc_nss_ar: [__le32; 2],
    pub rx_basic_trig_alloc_mu_mimo_cnt: __le32,
    pub rx_basic_trig_alloc_ru_bw_ar: [__le32; 7],
    pub rx_basic_trig_total_byte_cnt: __le32,
    pub tx_trig_based_cs_req_fail_cnt: __le32,
    pub tx_trig_based_sifs_ok_cnt: __le32,
    pub tx_trig_based_sifs_fail_cnt: __le32,
    pub tx_trig_based_byte_cnt: __le32,
    pub tx_trig_based_pad_byte_cnt: __le32,
    pub tx_trig_based_frame_cnt: __le32,
    pub tx_trig_based_acked_frame_cnt: __le32,
    pub tx_trig_based_ack_timeout_cnt: __le32,
    pub tx_su_frame_cnt: __le32,
    pub tx_edca_to_mu_edca_cnt: __le32,
    pub tx_mu_edca_to_edca_by_timeout_cnt: __le32,
    pub tx_mu_edca_to_edca_by_ack_fail_cnt: __le32,
    pub tx_mu_edca_to_edca_by_small_alloc_cnt: __le32,
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / STATISTICS_HE_NTFY_API_S_VER_1,
