//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/dvm/dev.h
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
// Copyright(c) 2003 - 2014, 2020, 2023 Intel Corporation. All rights reserved.
// Copyright (C) 2025 Intel Corporation
//
// Please use this file (dev.h) for driver implementation definitions.
// Please use commands.h for uCode API definitions.
//

// Macro flag: #define __iwl_dev_h__

// CT-KILL constants

// Default noise level to report when noise measurement is not available.
// This may be because we're:
// 1)  Not associated  no beacon statistics being sent to driver)
// 2)  Scanning (noise measurement does not apply to associated channel)
// Use default noise value of -127 ... this is below the range of measurable
// Rx dBm for all agn devices, so it can indicate "unmeasurable" to user.
// Also, -127 works better than 0 when averaging frames with/without
// noise info (e.g. averaging might be done in app); measured dBm values are
// always negative ... using a negative value as the default keeps all
// averages within an s8's (used in some apps) range of negative values.

//
// RTS threshold here is total size [2347] minus 4 FCS bytes
// Per spec:
// a value of 0 means RTS on all data/management packets
// a value > max MSDU size means no RTS
// else RTS for data/management frames where MPDU is larger
// than RTS value.
//

pub const IEEE80211_DATA_LEN: c_int = 2304;
pub const IEEE80211_4ADDR_LEN: c_int = 30;

pub const SUP_RATE_11A_MAX_NUM_CHANNELS: c_int = 8;
pub const SUP_RATE_11B_MAX_NUM_CHANNELS: c_int = 4;
pub const SUP_RATE_11G_MAX_NUM_CHANNELS: c_int = 12;
pub const IWL_SUPPORTED_RATES_IE_LEN: c_int = 8;
pub const IWL_INVALID_RATE: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub union iwl_ht_rate_supp {
    pub rates: u16,
    pub siso_rate: u8,
    pub mimo_rate: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ht_config {
    pub single_chain_sufficient: bool,
    pub /: *mut *mut ieee80211_smps_mode smps; / current smps mode,
}

// QoS structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_qos_info {
    pub qos_active: c_int,
    pub def_qos_parm: iwl_qosparam_cmd,
}

//
// enum iwl_agg_state - aggregation state
//
// The state machine of the BA agreement establishment / tear down.
// These states relate to a specific RA / TID.
//
// @IWL_AGG_OFF: aggregation is not used
// @IWL_AGG_STARTING: aggregation are starting (between start and oper)
// @IWL_AGG_ON: aggregation session is up
// @IWL_EMPTYING_HW_QUEUE_ADDBA: establishing a BA session - waiting for the
// HW queue to be empty from packets for this RA /TID.
// @IWL_EMPTYING_HW_QUEUE_DELBA: tearing down a BA session - waiting for the
// HW queue to be empty from packets for this RA /TID.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_agg_state {
    IWL_AGG_OFF = 0,
    IWL_AGG_STARTING,
    IWL_AGG_ON,
    IWL_EMPTYING_HW_QUEUE_ADDBA,
    IWL_EMPTYING_HW_QUEUE_DELBA,
}

//
// struct iwl_ht_agg - aggregation state machine
//
// This structs holds the states for the BA agreement establishment and tear
// down. It also holds the state during the BA session itself. This struct is
// duplicated for each RA / TID.
//
// @rate_n_flags: Rate at which Tx was attempted. Holds the data between the
// Tx response (REPLY_TX), and the block ack notification
// (REPLY_COMPRESSED_BA).
// @state: state of the BA agreement establishment / tear down.
// @txq_id: Tx queue used by the BA session
// @ssn: the first packet to be sent in AGG HW queue in Tx AGG start flow, or
// the first packet to be sent in legacy HW queue in Tx AGG stop flow.
// Basically when next_reclaimed reaches ssn, we can tell mac80211 that
// we are ready to finish the Tx AGG stop / start flow.
// @wait_for_ba: Expect block-ack before next Tx reply
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ht_agg {
    pub rate_n_flags: u32,
    pub state: iwl_agg_state,
    pub txq_id: u16,
    pub ssn: u16,
    pub wait_for_ba: bool,
}

//
// struct iwl_tid_data - one for each RA / TID
//
// This structs holds the states for each RA / TID.
//
// @seq_number: the next WiFi sequence number to use
// @next_reclaimed: the WiFi sequence number of the next packet to be acked.
// This is basically (last acked packet++).
// @agg: aggregation state machine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tid_data {
    pub seq_number: u16,
    pub next_reclaimed: u16,
    pub agg: iwl_ht_agg,
}

//
// Structure should be accessed with sta_lock held. When station addition
// is in progress (IWL_STA_UCODE_INPROGRESS) it is possible to access only
// the commands (iwl_addsta_cmd and iwl_link_quality_cmd) without sta_lock
// held.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_station_entry {
    pub sta: iwl_addsta_cmd,
    pub ctxid: u8 used,,
    pub lq: *mut iwl_link_quality_cmd,
}

//
// iwl_station_priv: Driver's private station information
//
// When mac80211 creates a station it reserves some space (hw->sta_data_size)
// in the structure for use by driver. This structure is places in that
// space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_station_priv {
    pub ctx: *mut iwl_rxon_context,
    pub lq_sta: iwl_lq_sta,
    pub pending_frames: core::sync::atomic::AtomicI32,
    pub client: bool,
    pub asleep: bool,
    pub max_agg_bufsize: u8,
    pub sta_id: u8,
}

//
// struct iwl_vif_priv - driver's private per-interface information
//
// When mac80211 allocates a virtual interface, it can allocate
// space for us to put data into.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_vif_priv {
    pub ctx: *mut iwl_rxon_context,
    pub ibss_bssid_sta_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sensitivity_ranges {
    pub min_nrg_cck: u16,
    pub nrg_th_cck: u16,
    pub nrg_th_ofdm: u16,
    pub auto_corr_min_ofdm: u16,
    pub auto_corr_min_ofdm_mrc: u16,
    pub auto_corr_min_ofdm_x1: u16,
    pub auto_corr_min_ofdm_mrc_x1: u16,
    pub auto_corr_max_ofdm: u16,
    pub auto_corr_max_ofdm_mrc: u16,
    pub auto_corr_max_ofdm_x1: u16,
    pub auto_corr_max_ofdm_mrc_x1: u16,
    pub auto_corr_max_cck: u16,
    pub auto_corr_max_cck_mrc: u16,
    pub auto_corr_min_cck: u16,
    pub auto_corr_min_cck_mrc: u16,
    pub barker_corr_th_min: u16,
    pub barker_corr_th_min_mrc: u16,
    pub nrg_th_cca: u16,
}

//
// Functions implemented in core module which are forward declared here
// for use by iwl-[4-5].c
//
// NOTE:  The implementation of these functions are not hardware specific
// which is why they are in the core module files.
//
// Naming convention --
// iwl_         <-- Is part of iwlwifi
// iwlXXXX_     <-- Hardware specific (implemented in iwl-XXXX.c for XXXX)
//
extern "C" {
    pub fn iwl_update_chain_flags(priv: *mut iwl_priv);
}
pub const IWL_OPERATION_MODE_AUTO: c_int = 0;
pub const IWL_OPERATION_MODE_HT_ONLY: c_int = 1;
pub const IWL_OPERATION_MODE_MIXED: c_int = 2;
pub const IWL_OPERATION_MODE_20MHZ: c_int = 3;

// Sensitivity and chain noise calibration
pub const INITIALIZATION_VALUE: c_uint = 0xFFFF;
pub const IWL_CAL_NUM_BEACONS: c_int = 16;
pub const MAXIMUM_ALLOWED_PATHLOSS: c_int = 15;
pub const CHAIN_NOISE_MAX_DELTA_GAIN_CODE: c_int = 3;
pub const MAX_FA_OFDM: c_int = 50;
pub const MIN_FA_OFDM: c_int = 5;
pub const MAX_FA_CCK: c_int = 50;
pub const MIN_FA_CCK: c_int = 5;
pub const AUTO_CORR_STEP_OFDM: c_int = 1;
pub const AUTO_CORR_STEP_CCK: c_int = 3;
pub const AUTO_CORR_MAX_TH_CCK: c_int = 160;
pub const NRG_DIFF: c_int = 2;
pub const NRG_STEP_CCK: c_int = 2;
pub const NRG_MARGIN: c_int = 8;
pub const MAX_NUMBER_CCK_NO_FA: c_int = 100;

pub const CHAIN_A: c_int = 0;
pub const CHAIN_B: c_int = 1;
pub const CHAIN_C: c_int = 2;
pub const CHAIN_NOISE_DELTA_GAIN_INIT_VAL: c_int = 4;
pub const ALL_BAND_FILTER: c_uint = 0xFF00;
pub const IN_BAND_FILTER: c_uint = 0xFF;
pub const MIN_AVERAGE_NOISE_MAX_VALUE: c_uint = 0xFFFFFFFF;
pub const NRG_NUM_PREV_STAT_L: c_int = 20;
pub const NUM_RX_CHAINS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwlagn_false_alarm_state {
    IWL_FA_TOO_MANY = 0,
    IWL_FA_TOO_FEW = 1,
    IWL_FA_GOOD_RANGE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwlagn_chain_noise_state {
    IWL_CHAIN_NOISE_ALIVE = 0,  /* must be 0 */
    IWL_CHAIN_NOISE_ACCUMULATE,
    IWL_CHAIN_NOISE_CALIBRATED,
    IWL_CHAIN_NOISE_DONE,
}

// Sensitivity calib data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sensitivity_data {
    pub auto_corr_ofdm: u32,
    pub auto_corr_ofdm_mrc: u32,
    pub auto_corr_ofdm_x1: u32,
    pub auto_corr_ofdm_mrc_x1: u32,
    pub auto_corr_cck: u32,
    pub auto_corr_cck_mrc: u32,
    pub last_bad_plcp_cnt_ofdm: u32,
    pub last_fa_cnt_ofdm: u32,
    pub last_bad_plcp_cnt_cck: u32,
    pub last_fa_cnt_cck: u32,
    pub nrg_curr_state: u32,
    pub nrg_prev_state: u32,
    pub nrg_value: [u32; 10],
    pub nrg_silence_rssi: [u8; NRG_NUM_PREV_STAT_L],
    pub nrg_silence_ref: u32,
    pub nrg_energy_idx: u32,
    pub nrg_silence_idx: u32,
    pub nrg_th_cck: u32,
    pub nrg_auto_corr_silence_diff: i32,
    pub num_in_cck_no_fa: u32,
    pub nrg_th_ofdm: u32,
    pub barker_corr_th_min: u16,
    pub barker_corr_th_min_mrc: u16,
    pub nrg_th_cca: u16,
}

// Chain noise (differential Rx gain) calib data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_chain_noise_data {
    pub active_chains: u32,
    pub chain_noise_a: u32,
    pub chain_noise_b: u32,
    pub chain_noise_c: u32,
    pub chain_signal_a: u32,
    pub chain_signal_b: u32,
    pub chain_signal_c: u32,
    pub beacon_count: u16,
    pub disconn_array: [u8; NUM_RX_CHAINS],
    pub delta_gain_code: [u8; NUM_RX_CHAINS],
    pub radio_write: u8,
    pub state: u8,
}

// reply_tx_statistics (for _agn devices)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reply_tx_error_statistics {
    pub pp_delay: u32,
    pub pp_few_bytes: u32,
    pub pp_bt_prio: u32,
    pub pp_quiet_period: u32,
    pub pp_calc_ttak: u32,
    pub int_crossed_retry: u32,
    pub short_limit: u32,
    pub long_limit: u32,
    pub fifo_underrun: u32,
    pub drain_flow: u32,
    pub rfkill_flush: u32,
    pub life_expire: u32,
    pub dest_ps: u32,
    pub host_abort: u32,
    pub bt_retry: u32,
    pub sta_invalid: u32,
    pub frag_drop: u32,
    pub tid_disable: u32,
    pub fifo_flush: u32,
    pub insuff_cf_poll: u32,
    pub fail_hw_drop: u32,
    pub sta_color_mismatch: u32,
    pub unknown: u32,
}

// reply_agg_tx_statistics (for _agn devices)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reply_agg_tx_error_statistics {
    pub underrun: u32,
    pub bt_prio: u32,
    pub few_bytes: u32,
    pub abort: u32,
    pub last_sent_ttl: u32,
    pub last_sent_try: u32,
    pub last_sent_bt_kill: u32,
    pub scd_query: u32,
    pub bad_crc32: u32,
    pub response: u32,
    pub dump_tx: u32,
    pub delay_tx: u32,
    pub unknown: u32,
}

//
// schedule the timer to wake up every UCODE_TRACE_PERIOD milliseconds
// to perform continuous uCode event logging operation if enabled
//

//
// iwl_event_log: current uCode event log position
//
// @ucode_trace: enable/disable ucode continuous trace timer
// @num_wraps: how many times the event buffer wraps
// @next_entry:  the entry just before the next one that uCode would fill
// @non_wraps_count: counter for no wrap detected when dump ucode events
// @wraps_once_count: counter for wrap once detected when dump ucode events
// @wraps_more_count: counter for wrap more than once detected
// when dump ucode events
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_event_log {
    pub ucode_trace: bool,
    pub num_wraps: u32,
    pub next_entry: u32,
    pub non_wraps_count: c_int,
    pub wraps_once_count: c_int,
    pub wraps_more_count: c_int,
}

// BT Antenna Coupling Threshold (dB)

// Firmware reload counter and Timestamp

pub const IWL_MAX_CONTINUE_RELOAD_CNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rf_reset {
    pub reset_request_count: c_int,
    pub reset_success_count: c_int,
    pub reset_reject_count: c_int,
    pub last_reset_jiffies: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_rxon_context_id {
    IWL_RXON_CTX_BSS,
    IWL_RXON_CTX_PAN,

    NUM_IWL_RXON_CTX
}

// extend beacon time format bit shifting
//
// for _agn devices
// bits 31:22 - extended
// bits 21:0  - interval
//
pub const IWLAGN_EXT_BEACON_TIME_POS: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rxon_context {
    pub vif: *mut ieee80211_vif,
    pub mcast_queue: u8,
    pub ac_to_queue: [u8; IEEE80211_NUM_ACS],
    pub ac_to_fifo: [u8; IEEE80211_NUM_ACS],
//
// We could use the vif to indicate active, but we
// also need it to be active during disabling when
// we already removed the vif for type setting.
//
    pub is_active: bool always_active,,
    pub ht_need_multiple_chains: bool,
    pub ctxid: iwl_rxon_context_id,
    pub exclusive_interface_modes: u32 interface_modes,,
    pub station_devtype: u8 unused_devtype, ap_devtype, ibss_devtype,,
//
// We declare this const so it can only be
// changed via explicit cast within the
// routines that actually update the physical
// hardware.
//
    pub active: iwl_rxon_cmd,
    pub staging: iwl_rxon_cmd,
    pub timing: iwl_rxon_time_cmd,
    pub qos_data: iwl_qos_info,
    pub ap_sta_id: u8 bcast_sta_id,,
    pub rxon_timing_cmd: u8 rxon_cmd, rxon_assoc_cmd,,
    pub qos_cmd: u8,
    pub wep_key_cmd: u8,
    pub wep_keys: [iwl_wep_key; WEP_KEYS_MAX],
    pub key_mapping_keys: u8,
    pub station_flags: __le32,
    pub beacon_int: c_int,
    pub non_gf_sta_present: bool,
    pub protection: u8,
    pub is_40mhz: bool enabled,,
    pub extension_chan_offset: u8,
    pub ht: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_type {
    IWL_SCAN_NORMAL,
    IWL_SCAN_RADIO_RESET,
}

//
// struct iwl_hw_params - HW parameters
//
// Holds the module parameters
//
// @tx_chains_num: Number of TX chains
// @rx_chains_num: Number of RX chains
// @ct_kill_threshold: temperature threshold - in hw dependent unit
// @ct_kill_exit_threshold: when to reeable the device - in hw dependent unit
// relevant for 1000, 6000 and up
// @struct iwl_sensitivity_ranges: range of sensitivity values
// @use_rts_for_aggregation: use rts/cts protection for HT traffic
// @sens: sensitivity ranges pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_hw_params {
    pub tx_chains_num: u8,
    pub rx_chains_num: u8,
    pub use_rts_for_aggregation: bool,
    pub ct_kill_threshold: u32,
    pub ct_kill_exit_threshold: u32,
    pub sens: *const iwl_sensitivity_ranges,
}

//
// struct iwl_dvm_bt_params - DVM specific BT (coex) parameters
// @advanced_bt_coexist: support advanced bt coexist
// @bt_init_traffic_load: specify initial bt traffic load
// @bt_prio_boost: default bt priority boost value
// @agg_time_limit: maximum number of uSec in aggregation
// @bt_sco_disable: uCode should not response to BT in SCO/ESCO mode
// @bt_session_2: indicates version 2 of the BT command is used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dvm_bt_params {
    pub advanced_bt_coexist: bool,
    pub bt_init_traffic_load: u8,
    pub bt_prio_boost: u32,
    pub agg_time_limit: u16,
    pub bt_sco_disable: bool,
    pub bt_session_2: bool,
}

//
// struct iwl_dvm_cfg - DVM firmware specific device configuration
// @set_hw_params: set hardware parameters
// @set_channel_switch: send channel switch command
// @nic_config: apply device specific configuration
// @temperature: read temperature
// @adv_thermal_throttle: support advance thermal throttle
// @support_ct_kill_exit: support ct kill exit condition
// @plcp_delta_threshold: plcp error rate threshold used to trigger
// radio tuning when there is a high receiving plcp error rate
// @chain_noise_scale: default chain noise scale used for gain computation
// @hd_v2: v2 of enhanced sensitivity value, used for 2000 series and up
// @no_idle_support: do not support idle mode
// @bt_params: pointer to BT parameters
// @need_temp_offset_calib: need to perform temperature offset calibration
// @no_xtal_calib: some devices do not need crystal calibration data,
// don't send it to those
// @temp_offset_v2: support v2 of temperature offset calibration
// @adv_pm: advanced power management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dvm_cfg {
    pub priv): *mut *mut void (set_hw_params)(struct iwl_priv,
    pub ch_switch): *mut ieee80211_channel_switch,
    pub priv): *mut *mut void (nic_config)(struct iwl_priv,
    pub priv): *mut *mut void (temperature)(struct iwl_priv,
    pub bt_params: *const iwl_dvm_bt_params,
    pub chain_noise_scale: i32,
    pub plcp_delta_threshold: u8,
    pub adv_thermal_throttle: bool,
    pub support_ct_kill_exit: bool,
    pub hd_v2: bool,
    pub no_idle_support: bool,
    pub need_temp_offset_calib: bool,
    pub no_xtal_calib: bool,
    pub temp_offset_v2: bool,
    pub adv_pm: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wipan_noa_data {
    pub rcu_head: rcu_head,
    pub length: u32,
    pub data: [u8; ],
}

// Calibration disabling bit mask

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_priv {
    pub trans: *mut iwl_trans,
    pub /: *mut *mut *mut device dev; / for debug prints only,
    pub cfg: *const iwl_rf_cfg,
    pub fw: *const iwl_fw,
    pub lib: *const iwl_dvm_cfg,
    pub status: c_ulong,
    pub sta_lock: spinlock_t,
    pub mutex: mutex,
    pub transport_queue_stop: c_ulong,
    pub passive_no_rx: bool,
pub const IWL_INVALID_MAC80211_QUEUE: c_uint = 0xff;
    pub queue_to_mac80211: [u8; IWL_MAX_HW_QUEUES],
    pub queue_stop_count: [core::sync::atomic::AtomicI32; IWL_MAX_HW_QUEUES],
    pub agg_q_alloc: [c_ulong; BITS_TO_LONGS(IWL_MAX_HW_QUEUES)],
// ieee device used by generic ieee processing code
    pub hw: *mut ieee80211_hw,
    pub napi: *mut napi_struct,
    pub calib_results: list_head,
    pub workqueue: *mut workqueue_struct,
    pub hw_params: iwl_hw_params,
    pub band: nl80211_band,
    pub valid_contexts: u8,
    pub rxb): *mut iwl_rx_cmd_buffer,
    pub notif_wait: iwl_notif_wait_data,
// spectrum measurement report caching
    pub measure_report: iwl_spectrum_notification,
    pub measurement_status: u8,
// ucode beacon time
    pub ucode_beacon_time: u32,
    pub missed_beacon_threshold: c_int,
// track IBSS manager (last beacon) status
    pub ibss_manager: u32,
// jiffies when last recovery from statistics was performed
    pub rx_statistics_jiffies: c_ulong,
// counters
    pub rx_handlers_stats: [u32; REPLY_MAX],
// rf reset
    pub rf_reset: iwl_rf_reset,
// firmware reload counter and timestamp
    pub reload_jiffies: c_ulong,
    pub reload_count: c_int,
    pub ucode_loaded: bool,
    pub plcp_delta_threshold: u8,
// thermal calibration
    pub /: *mut *mut s32 temperature; / Celsius,
    pub last_temperature: i32,
    pub noa_data: *mut iwl_wipan_noa_data __rcu,
// Scan related variables
    pub scan_start: c_ulong,
    pub scan_start_tsf: c_ulong,
    pub scan_cmd_size: usize,
    pub scan_cmd: *mut c_void,
    pub scan_band: nl80211_band,
    pub scan_request: *mut cfg80211_scan_request,
    pub scan_vif: *mut ieee80211_vif,
    pub scan_type: iwl_scan_type,
    pub scan_tx_ant: [u8; NUM_NL80211_BANDS],
    pub mgmt_tx_ant: u8,
// max number of station keys
    pub sta_key_max_num: u8,
    pub new_scan_threshold_behaviour: bool,
    pub wowlan: bool,
// EEPROM MAC addresses
    pub addresses: [mac_address; 2],
    pub contexts: [iwl_rxon_context; NUM_IWL_RXON_CTX],
    pub switch_channel: __le16,
    pub start_calib: u8,
    pub sensitivity_data: iwl_sensitivity_data,
    pub chain_noise_data: iwl_chain_noise_data,
    pub sensitivity_tbl: [__le16; HD_TABLE_SIZE],
    pub enhance_sensitivity_tbl: [__le16; ENHANCE_HD_TABLE_ENTRIES],
    pub current_ht_config: iwl_ht_config,
// Rate scaling data
    pub retry_rate: u8,
    pub activity_timer_active: c_int,
    pub power_data: iwl_power_mgr,
    pub thermal_throttle: iwl_tt_mgmt,
// station table variables
    pub num_stations: c_int,
    pub stations: [iwl_station_entry; IWLAGN_STATION_COUNT],
    pub ucode_key_table: c_ulong,
    pub tid_data: [iwl_tid_data; IWLAGN_STATION_COUNT][IWL_MAX_TID_COUNT],
    pub num_aux_in_flight: core::sync::atomic::AtomicI32,
    pub mac80211_registered: u8,
// Indication if ieee80211_ops->open has been called
    pub is_open: u8,
    pub iw_mode: nl80211_iftype,
// Last Rx'd beacon timestamp
    pub timestamp: u64,
    pub flag: __le32,
    pub common: statistics_general_common,
    pub rx_non_phy: statistics_rx_non_phy,
    pub rx_ofdm: statistics_rx_phy,
    pub rx_ofdm_ht: statistics_rx_ht_phy,
    pub rx_cck: statistics_rx_phy,
    pub tx: statistics_tx,

    pub bt_activity: statistics_bt_activity,
    pub accum_num_bt_kills: __le32 num_bt_kills,,

    pub lock: spinlock_t,
    pub statistics: },

    pub common: statistics_general_common,
    pub rx_non_phy: statistics_rx_non_phy,
    pub rx_ofdm: statistics_rx_phy,
    pub rx_ofdm_ht: statistics_rx_ht_phy,
    pub rx_cck: statistics_rx_phy,
    pub tx: statistics_tx,
    pub bt_activity: statistics_bt_activity,
    pub max_delta_stats: } accum_stats, delta_stats,,

//
// reporting the number of tids has AGG on. 0 means
// no AGGREGATION
//
    pub agg_tids_count: u8,
    pub last_phy_res: iwl_rx_phy_res,
    pub ampdu_ref: u32,
    pub last_phy_res_valid: bool,
//
// chain noise reset and gain commands are the
// two extra calibration commands follows the standard
// phy calibration commands
//
    pub phy_calib_chain_noise_reset_cmd: u8,
    pub phy_calib_chain_noise_gain_cmd: u8,
// counts reply_tx error
    pub reply_tx_stats: reply_tx_error_statistics,
    pub reply_agg_tx_stats: reply_agg_tx_error_statistics,
// bt coex
    pub bt_enable_flag: u8,
    pub bt_status: u8,
    pub last_bt_traffic_load: u8 bt_traffic_load,,
    pub bt_ch_announce: bool,
    pub bt_full_concurrent: bool,
    pub kill_ack_mask: __le32,
    pub kill_cts_mask: __le32,
    pub bt_valid: __le16,
    pub reduced_txpower: bool,
    pub bt_on_thresh: u16,
    pub bt_duration: u16,
    pub dynamic_frag_thresh: u16,
    pub bt_ci_compliance: u8,
    pub bt_traffic_change_work: work_struct,
    pub bt_enable_pspoll: bool,
    pub cur_rssi_ctx: *mut iwl_rxon_context,
    pub bt_is_sco: bool,
    pub restart: work_struct,
    pub scan_completed: work_struct,
    pub abort_scan: work_struct,
    pub beacon_update: work_struct,
    pub beacon_ctx: *mut iwl_rxon_context,
    pub beacon_skb: *mut sk_buff,
    pub beacon_cmd: *mut c_void,
    pub tt_work: work_struct,
    pub ct_enter: work_struct,
    pub ct_exit: work_struct,
    pub start_internal_scan: work_struct,
    pub tx_flush: work_struct,
    pub bt_full_concurrency: work_struct,
    pub bt_runtime_config: work_struct,
    pub scan_check: delayed_work,
// TX Power settings
    pub tx_power_user_lmt: i8,
    pub tx_power_next: i8,

// debugfs
    pub debugfs_dir: *mut dentry,
    pub dbgfs_sram_len: u32 dbgfs_sram_offset,,
    pub disable_ht40: bool,
    pub wowlan_sram: *mut c_void,

    pub nvm_data: *mut iwl_nvm_data,
// eeprom blob for debugfs
    pub eeprom_blob: *mut u8,
    pub eeprom_blob_size: usize,
    pub txpower_work: work_struct,
    pub calib_disabled: u32,
    pub run_time_calib_work: work_struct,
    pub statistics_periodic: timer_list,
    pub ucode_trace: timer_list,
    pub event_log: iwl_event_log,

    pub led: led_classdev,
    pub blink_off: unsigned long blink_on,,
    pub led_registered: bool,

// WoWLAN GTK rekey data
    pub kek: [u8 kck[NL80211_KCK_LEN],; NL80211_KEK_LEN],
    pub replay_ctr: __le64,
    pub last_seq_ctl: __le16,
    pub have_rekey_data: bool,

    pub wowlan_support: wiphy_wowlan_support,

// device_pointers: pointers to ucode event tables
    pub error_event_table: u32,
    pub log_event_table: u32,
    pub device_pointers: },
// indicator of loaded ucode image
    pub cur_ucode: iwl_ucode_type,
}

extern "C" {
    pub fn iwl_is_associated_ctx(_arg: &priv->contexts[ctxid]) -> return;
}
