//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/rsi/rsi_mgmt.h
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


//
// Copyright (c) 2014 Redpine Signals Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const MAX_MGMT_PKT_SIZE: c_int = 512;
pub const RSI_NEEDED_HEADROOM: c_int = 84;
pub const RSI_RCV_BUFFER_LEN: c_int = 2000;
pub const RSI_11B_MODE: c_int = 0;

pub const RETRY_COUNT: c_int = 8;
pub const RETRY_LONG: c_int = 4;
pub const RETRY_SHORT: c_int = 7;
pub const WMM_SHORT_SLOT_TIME: c_int = 9;
pub const SIFS_DURATION: c_int = 16;
pub const EAPOL4_PACKET_LEN: c_uint = 0x85;
pub const KEY_TYPE_CLEAR: c_int = 0;
pub const RSI_PAIRWISE_KEY: c_int = 1;
pub const RSI_GROUP_KEY: c_int = 2;
// EPPROM_READ_ADDRESS
pub const WLAN_MAC_EEPROM_ADDR: c_int = 40;
pub const WLAN_MAC_MAGIC_WORD_LEN: c_uint = 0x01;
pub const WLAN_HOST_MODE_LEN: c_uint = 0x04;
pub const WLAN_FW_VERSION_LEN: c_uint = 0x08;
pub const MAGIC_WORD: c_uint = 0x5A;
pub const WLAN_EEPROM_RFTYPE_ADDR: c_int = 424;
// WOWLAN RESUME WAKEUP TYPES

pub const WOW_MAX_FILTERS_PER_LIST: c_int = 16;
pub const WOW_PATTERN_SIZE: c_int = 256;
// Receive Frame Types
pub const RSI_RX_DESC_MSG_TYPE_OFFSET: c_int = 2;
pub const TA_CONFIRM_TYPE: c_uint = 0x01;
pub const RX_DOT11_MGMT: c_uint = 0x02;
pub const TX_STATUS_IND: c_uint = 0x04;
pub const BEACON_EVENT_IND: c_uint = 0x08;
pub const EAPOL4_CONFIRM: c_int = 1;
pub const PROBEREQ_CONFIRM: c_int = 2;
pub const CARD_READY_IND: c_uint = 0x00;
pub const SLEEP_NOTIFY_IND: c_uint = 0x06;
pub const RSI_TX_STATUS_TYPE: c_int = 15;
pub const RSI_TX_STATUS: c_int = 12;
pub const RSI_DELETE_PEER: c_uint = 0x0;
pub const RSI_ADD_PEER: c_uint = 0x1;
pub const START_AMPDU_AGGR: c_uint = 0x1;
pub const STOP_AMPDU_AGGR: c_uint = 0x0;
pub const INTERNAL_MGMT_PKT: c_uint = 0x99;
pub const PUT_BBP_RESET: c_int = 0;
pub const BBP_REG_WRITE: c_int = 0;

pub const FULL40M_ENABLE: c_uint = 0x6;
pub const RSI_LMAC_CLOCK_80MHZ: c_uint = 0x1;

pub const RX_BA_INDICATION: c_int = 1;
pub const RSI_TBL_SZ: c_int = 40;
pub const MAX_RETRIES: c_int = 8;
pub const RSI_IFTYPE_STATION: c_int = 0;
pub const STD_RATE_MCS7: c_uint = 0x07;
pub const STD_RATE_MCS6: c_uint = 0x06;
pub const STD_RATE_MCS5: c_uint = 0x05;
pub const STD_RATE_MCS4: c_uint = 0x04;
pub const STD_RATE_MCS3: c_uint = 0x03;
pub const STD_RATE_MCS2: c_uint = 0x02;
pub const STD_RATE_MCS1: c_uint = 0x01;
pub const STD_RATE_MCS0: c_uint = 0x00;
pub const STD_RATE_54: c_uint = 0x6c;
pub const STD_RATE_48: c_uint = 0x60;
pub const STD_RATE_36: c_uint = 0x48;
pub const STD_RATE_24: c_uint = 0x30;
pub const STD_RATE_18: c_uint = 0x24;
pub const STD_RATE_12: c_uint = 0x18;
pub const STD_RATE_11: c_uint = 0x16;
pub const STD_RATE_09: c_uint = 0x12;
pub const STD_RATE_06: c_uint = 0x0C;
pub const STD_RATE_5_5: c_uint = 0x0B;
pub const STD_RATE_02: c_uint = 0x04;
pub const STD_RATE_01: c_uint = 0x02;
pub const RSI_RF_TYPE: c_int = 1;
pub const RSI_RATE_00: c_uint = 0x00;
pub const RSI_RATE_1: c_uint = 0x0;
pub const RSI_RATE_2: c_uint = 0x2;
pub const RSI_RATE_5_5: c_uint = 0x4;
pub const RSI_RATE_11: c_uint = 0x6;
pub const RSI_RATE_6: c_uint = 0x8b;
pub const RSI_RATE_9: c_uint = 0x8f;
pub const RSI_RATE_12: c_uint = 0x8a;
pub const RSI_RATE_18: c_uint = 0x8e;
pub const RSI_RATE_24: c_uint = 0x89;
pub const RSI_RATE_36: c_uint = 0x8d;
pub const RSI_RATE_48: c_uint = 0x88;
pub const RSI_RATE_54: c_uint = 0x8c;
pub const RSI_RATE_MCS0: c_uint = 0x100;
pub const RSI_RATE_MCS1: c_uint = 0x101;
pub const RSI_RATE_MCS2: c_uint = 0x102;
pub const RSI_RATE_MCS3: c_uint = 0x103;
pub const RSI_RATE_MCS4: c_uint = 0x104;
pub const RSI_RATE_MCS5: c_uint = 0x105;
pub const RSI_RATE_MCS6: c_uint = 0x106;
pub const RSI_RATE_MCS7: c_uint = 0x107;
pub const RSI_RATE_MCS7_SG: c_uint = 0x307;
pub const RSI_RATE_AUTO: c_uint = 0xffff;
pub const BW_20MHZ: c_int = 0;
pub const BW_40MHZ: c_int = 1;
pub const EP_2GHZ_20MHZ: c_int = 0;
pub const EP_2GHZ_40MHZ: c_int = 1;
pub const EP_5GHZ_20MHZ: c_int = 2;
pub const EP_5GHZ_40MHZ: c_int = 3;
pub const SIFS_TX_11N_VALUE: c_int = 580;
pub const SIFS_TX_11B_VALUE: c_int = 346;
pub const SHORT_SLOT_VALUE: c_int = 360;
pub const LONG_SLOT_VALUE: c_int = 640;
pub const OFDM_ACK_TOUT_VALUE: c_int = 2720;
pub const CCK_ACK_TOUT_VALUE: c_int = 9440;
pub const LONG_PREAMBLE: c_uint = 0x0000;
pub const SHORT_PREAMBLE: c_uint = 0x0001;

pub const ANTENNA_SEL_INT: c_uint = 0x02 /* RF_OUT_2 / Integerated */;
pub const ANTENNA_SEL_UFL: c_uint = 0x03 /* RF_OUT_1 / U.FL */;
pub const ANTENNA_MASK_VALUE: c_uint = 0x00ff;
pub const ANTENNA_SEL_TYPE: c_int = 1;
// Rx filter word definitions

pub const RSI_MPDU_DENSITY: c_uint = 0x8;

pub const RSI_BEACON_INTERVAL: c_int = 200;
pub const RSI_DTIM_COUNT: c_int = 2;

pub const RSI_PS_ENABLE: c_int = 1;
pub const RSI_PS_DISABLE: c_int = 0;
pub const RSI_DEEP_SLEEP: c_int = 1;
pub const RSI_CONNECTED_SLEEP: c_int = 2;
pub const RSI_SLEEP_REQUEST: c_int = 1;
pub const RSI_WAKEUP_REQUEST: c_int = 2;

pub const RSI_DESC_VAP_ID_MASK: c_uint = 0xC000u;
pub const RSI_DESC_VAP_ID_OFST: c_int = 14;

pub const RSI_DATA_DESC_NORMAL_FRAME: c_uint = 0x00;

pub const RSI_MAX_TX_AGGR_FRMS: c_int = 8;
pub const RSI_MAX_RX_AGGR_FRMS: c_int = 8;
pub const RSI_MAX_SCAN_SSIDS: c_int = 16;
pub const RSI_MAX_SCAN_IE_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opmode {
    RSI_OPMODE_UNSUPPORTED = -1,
    RSI_OPMODE_AP = 0,
    RSI_OPMODE_STA,
    RSI_OPMODE_P2P_GO,
    RSI_OPMODE_P2P_CLIENT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vap_status {
    VAP_ADD = 1,
    VAP_DELETE = 2,
    VAP_UPDATE = 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum peer_type {
    PEER_TYPE_AP,
    PEER_TYPE_STA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sta_notify_events {
    STA_CONNECTED = 0,
    STA_DISCONNECTED,
    STA_TX_ADDBA_DONE,
    STA_TX_DELBA,
    STA_RX_ADDBA_DONE,
    STA_RX_DELBA
}

// Send Frames Types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_frame_type {
    TX_DOT11_MGMT,
    RESET_MAC_REQ,
    RADIO_CAPABILITIES,
    BB_PROG_VALUES_REQUEST,
    RF_PROG_VALUES_REQUEST,
    WAKEUP_SLEEP_REQUEST,
    SCAN_REQUEST,
    TSF_UPDATE,
    PEER_NOTIFY,
    BLOCK_HW_QUEUE,
    SET_KEY_REQ,
    AUTO_RATE_IND,
    BOOTUP_PARAMS_REQUEST,
    VAP_CAPABILITIES,
    EEPROM_READ,
    EEPROM_WRITE,
    GPIO_PIN_CONFIG ,
    SET_RX_FILTER,
    AMPDU_IND,
    STATS_REQUEST_FRAME,
    BB_BUF_PROG_VALUES_REQ,
    BBP_PROG_IN_TA,
    BG_SCAN_PARAMS,
    BG_SCAN_PROBE_REQ,
    CW_MODE_REQ,
    PER_CMD_PKT,
    ANT_SEL_FRAME = 0x20,
    VAP_DYNAMIC_UPDATE = 0x27,
    COMMON_DEV_CONFIG = 0x28,
    RADIO_PARAMS_UPDATE = 0x29,
    WOWLAN_CONFIG_PARAMS = 0x2B,
    FEATURES_ENABLE = 0x33,
    WOWLAN_WAKEUP_REASON = 0xc5
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_mac_frame {
    pub desc_word: [__le16; 8],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_xtended_desc {
    pub confirm_frame_type: u8,
    pub retry_cnt: u8,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_cmd_desc_dword0 {
    pub len_qno: __le16,
    pub frame_type: u8,
    pub misc_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_cmd_desc_dword1 {
    pub xtend_desc_size: u8,
    pub reserved1: u8,
    pub reserved2: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_cmd_desc_dword2 {
    pub /: *mut *mut __le32 pkt_info; / Packet specific data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_cmd_desc_dword3 {
    pub token: __le16,
    pub qid_tid: u8,
    pub sta_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_cmd_desc {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub desc_dword1: rsi_cmd_desc_dword1,
    pub desc_dword2: rsi_cmd_desc_dword2,
    pub desc_dword3: rsi_cmd_desc_dword3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_boot_params {
    pub desc_word: [__le16; 8],
    pub bootup_params: bootup_params,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_boot_params_9116 {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub desc_dword1: rsi_cmd_desc_dword1,
    pub desc_dword2: rsi_cmd_desc_dword2,
    pub reserved: __le16,
    pub umac_clk: __le16,
    pub bootup_params: bootup_params_9116,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_peer_notify {
    pub desc: rsi_cmd_desc,
    pub mac_addr: [u8; 6],
    pub command: __le16,
    pub mpdu_density: __le16,
    pub reserved: __le16,
    pub sta_flags: __le32,
    pub __packed: },
// Aggregation params flags
pub const RSI_AGGR_PARAMS_TID_MASK: c_uint = 0xf;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_aggr_params {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub desc_dword1: rsi_cmd_desc_dword0,
    pub seq_start: __le16,
    pub baw_size: __le16,
    pub token: __le16,
    pub aggr_params: u8,
    pub peer_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_bb_rf_prog {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub reserved1: __le16,
    pub rf_power_mode: u8,
    pub reserved2: u8,
    pub endpoint: u8,
    pub reserved3: u8,
    pub reserved4: __le16,
    pub reserved5: __le16,
    pub flags: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_chan_config {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub desc_dword1: rsi_cmd_desc_dword1,
    pub channel_number: u8,
    pub antenna_gain_offset_2g: u8,
    pub antenna_gain_offset_5g: u8,
    pub channel_width: u8,
    pub tx_power: __le16,
    pub region_rftype: u8,
    pub flags: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_vap_caps {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub reserved1: u8,
    pub status: u8,
    pub reserved2: __le16,
    pub vif_type: u8,
    pub channel_bw: u8,
    pub antenna_info: __le16,
    pub token: __le16,
    pub radioid_macid: u8,
    pub vap_id: u8,
    pub mac_addr: [u8; 6],
    pub keep_alive_period: __le16,
    pub bssid: [u8; 6],
    pub reserved4: __le16,
    pub flags: __le32,
    pub frag_threshold: __le16,
    pub rts_threshold: __le16,
    pub default_mgmt_rate: __le32,
    pub default_ctrl_rate: __le16,
    pub ctrl_rate_flags: __le16,
    pub default_data_rate: __le32,
    pub beacon_interval: __le16,
    pub dtim_period: __le16,
    pub beacon_miss_threshold: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_ant_sel_frame {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub reserved: u8,
    pub sub_frame_type: u8,
    pub ant_value: __le16,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_dynamic_s {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub desc_dword1: rsi_cmd_desc_dword1,
    pub desc_dword2: rsi_cmd_desc_dword2,
    pub desc_dword3: rsi_cmd_desc_dword3,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct framebody {
    pub data_rate: __le16,
    pub mgmt_rate: __le16,
    pub keep_alive_period: __le16,
    pub frame_body: },
    pub __packed: },
// Key descriptor flags

pub const RSI_KEY_ID_MASK: c_uint = 0xC0;
pub const RSI_KEY_ID_OFFSET: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_set_key {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub desc_dword1: rsi_cmd_desc_dword1,
    pub key_desc: __le16,
    pub bpn: __le32,
    pub sta_id: u8,
    pub vap_id: u8,
    pub key: [u8; 4][32],
    pub tx_mic_key: [u8; 8],
    pub rx_mic_key: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_auto_rate {
    pub desc: rsi_cmd_desc,
    pub failure_limit: __le16,
    pub initial_boundary: __le16,
    pub max_threshold_limt: __le16,
    pub num_supported_rates: __le16,
    pub aarf_rssi: __le16,
    pub moderate_rate_inx: __le16,
    pub collision_tolerance: __le16,
    pub supported_rates: [__le16; 40],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_block_unblock_data {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub xtend_desc_size: u8,
    pub host_quiet_info: u8,
    pub reserved: __le16,
    pub block_q_bitmap: __le16,
    pub unblock_q_bitmap: __le16,
    pub token: __le16,
    pub flush_q_bitmap: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qos_params {
    pub cont_win_min_q: __le16,
    pub cont_win_max_q: __le16,
    pub aifsn_val_q: __le16,
    pub txop_q: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_radio_caps {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub desc_dword1: rsi_cmd_desc_dword0,
    pub channel_num: u8,
    pub rf_model: u8,
    pub ppe_ack_rate: __le16,
    pub mode_11j: __le16,
    pub radio_cfg_info: u8,
    pub radio_info: u8,
    pub qos_params: [qos_params; MAX_HW_QUEUES],
    pub num_11n_rates: u8,
    pub num_11ac_rates: u8,
    pub gcpd_per_rate: [__le16; 20],
    pub sifs_tx_11n: __le16,
    pub sifs_tx_11b: __le16,
    pub slot_rx_11n: __le16,
    pub ofdm_ack_tout: __le16,
    pub cck_ack_tout: __le16,
    pub preamble_type: __le16,
    pub __packed: },
// ULP GPIO flags

// SOC GPIO flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_config_vals {
    pub len_qno: __le16,
    pub pkt_type: u8,
    pub misc_flags: u8,
    pub reserved1: [__le16; 6],
    pub lp_ps_handshake: u8,
    pub ulp_ps_handshake: u8,
    pub handshake,: *mut *mut u8 sleep_config_params; / 0 for no,
// 1 for GPIO based handshake,
// 2 packet handshake
//
    pub unused_ulp_gpio: u8,
    pub unused_soc_gpio_bitmap: __le32,
    pub ext_pa_or_bt_coex_en: u8,
    pub opermode: u8,
    pub wlan_rf_pwr_mode: u8,
    pub bt_rf_pwr_mode: u8,
    pub zigbee_rf_pwr_mode: u8,
    pub driver_mode: u8,
    pub region_code: u8,
    pub antenna_sel_val: u8,
    pub reserved2: [u8; 16],
    pub __packed: },
// Packet info flags
pub const RSI_EEPROM_HDR_SIZE_OFFSET: c_int = 8;
pub const RSI_EEPROM_HDR_SIZE_MASK: c_uint = 0x300;
pub const RSI_EEPROM_LEN_OFFSET: c_int = 20;
pub const RSI_EEPROM_LEN_MASK: c_uint = 0xFFF00000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_eeprom_read_frame {
    pub len_qno: __le16,
    pub pkt_type: u8,
    pub misc_flags: u8,
    pub pkt_info: __le32,
    pub eeprom_offset: __le32,
    pub delay_ms: __le16,
    pub reserved3: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_request_ps {
    pub desc: rsi_cmd_desc,
    pub ps_sleep: ps_sleep_params,
    pub ps_mimic_support: u8,
    pub ps_uapsd_acs: u8,
    pub ps_uapsd_wakeup_period: u8,
    pub reserved: u8,
    pub ps_listen_interval: __le32,
    pub ps_dtim_interval_duration: __le32,
    pub ps_num_dtim_intervals: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_wowlan_req {
    pub desc: rsi_cmd_desc,
    pub sourceid: [u8; ETH_ALEN],
    pub wow_flags: u16,
    pub host_sleep_status: u16,
    pub __packed: },
pub const RSI_START_BGSCAN: c_int = 1;
pub const RSI_STOP_BGSCAN: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_bgscan_config {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub reserved: __le64,
    pub reserved1: __le32,
    pub bgscan_threshold: __le16,
    pub roam_threshold: __le16,
    pub bgscan_periodicity: __le16,
    pub num_bgscan_channels: u8,
    pub two_probe: u8,
    pub active_scan_duration: __le16,
    pub passive_scan_duration: __le16,
    pub channels2scan: [__le16; MAX_BGSCAN_CHANNELS_DUAL_BAND],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_bgscan_probe {
    pub desc_dword0: rsi_cmd_desc_dword0,
    pub reserved: __le64,
    pub reserved1: __le32,
    pub mgmt_rate: __le16,
    pub flags: __le16,
    pub def_chan: __le16,
    pub channel_scan_time: __le16,
    pub probe_req_length: __le16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_wlan_9116_features {
    pub desc: rsi_cmd_desc,
    pub pll_mode: u8,
    pub rf_type: u8,
    pub wireless_mode: u8,
    pub enable_ppe: u8,
    pub afe_type: u8,
    pub reserved1: u8,
    pub reserved2: __le16,
    pub feature_enable: __le32,
}

// addr = cpu_to_le16(len | ((qno & 7) << 12));
extern "C" {
    pub fn rsi_handle_card_ready(common: *mut rsi_common, msg: *mut u8) -> c_int;
}
extern "C" {
    pub fn rsi_mgmt_pkt_recv(common: *mut rsi_common, msg: *mut u8) -> c_int;
}
extern "C" {
    pub fn rsi_send_vap_dynamic_update(common: *mut rsi_common) -> c_int;
}
extern "C" {
    pub fn rsi_send_block_unblock_frame(common: *mut rsi_common, event: bool) -> c_int;
}
extern "C" {
    pub fn rsi_indicate_pkt_to_os(common: *mut rsi_common, skb: *mut sk_buff);
}
extern "C" {
    pub fn rsi_mac80211_attach(common: *mut rsi_common) -> c_int;
}
extern "C" {
    pub fn rsi_is_cipher_wep(common: *mut rsi_common) -> bool;
}
extern "C" {
    pub fn rsi_core_qos_processor(common: *mut rsi_common);
}
extern "C" {
    pub fn rsi_core_xmit(common: *mut rsi_common, skb: *mut sk_buff);
}
extern "C" {
    pub fn rsi_send_mgmt_pkt(common: *mut rsi_common, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn rsi_send_data_pkt(common: *mut rsi_common, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn rsi_band_check(common: *mut rsi_common, chan: *mut ieee80211_channel) -> c_int;
}
extern "C" {
    pub fn rsi_send_rx_filter_frame(common: *mut rsi_common, rx_filter_word: u16) -> c_int;
}
extern "C" {
    pub fn rsi_send_radio_params_update(common: *mut rsi_common) -> c_int;
}
extern "C" {
    pub fn rsi_set_antenna(common: *mut rsi_common, antenna: u8) -> c_int;
}

extern "C" {
    pub fn init_bgscan_params(common: *mut rsi_common);
}
extern "C" {
    pub fn rsi_send_bgscan_params(common: *mut rsi_common, enable: c_int) -> c_int;
}
