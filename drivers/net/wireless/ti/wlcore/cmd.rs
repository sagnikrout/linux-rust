//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wlcore/cmd.h
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
// This file is part of wl1271
//
// Copyright (C) 1998-2009 Texas Instruments. All rights reserved.
// Copyright (C) 2009 Nokia Corporation
//
// Contact: Luciano Coelho <luciano.coelho@nokia.com>
//

extern "C" {
    pub fn wl12xx_cmd_role_disable(wl: *mut wl1271, role_id: *mut u8) -> c_int;
}
extern "C" {
    pub fn wl12xx_cmd_role_start_sta(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl12xx_cmd_role_stop_sta(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl12xx_cmd_role_start_ap(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl12xx_cmd_role_stop_ap(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl12xx_cmd_role_start_ibss(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl12xx_stop_dev(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl1271_cmd_test(wl: *mut wl1271, buf: *mut c_void, buf_len: usize, answer: u8) -> c_int;
}
extern "C" {
    pub fn wl1271_cmd_configure(wl: *mut wl1271, id: u16, buf: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn wl1271_cmd_data_path(wl: *mut wl1271, enable: bool) -> c_int;
}
extern "C" {
    pub fn wl12xx_cmd_build_null_data(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl1271_cmd_build_arp_rsp(wl: *mut wl1271, wlvif: *mut wl12xx_vif) -> c_int;
}
extern "C" {
    pub fn wl1271_build_qos_null_data(wl: *mut wl1271, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn wl12xx_cmd_set_default_wep_key(wl: *mut wl1271, id: u8, hlid: u8) -> c_int;
}
extern "C" {
    pub fn wl12xx_croc(wl: *mut wl1271, role_id: u8) -> c_int;
}
extern "C" {
    pub fn wlcore_cmd_regdomain_config_locked(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl12xx_cmd_config_fwlog(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl12xx_cmd_stop_fwlog(wl: *mut wl1271) -> c_int;
}
extern "C" {
    pub fn wl12xx_free_link(wl: *mut wl1271, wlvif: *mut wl12xx_vif, hlid: *mut u8);
}
extern "C" {
    pub fn wlcore_get_native_channel_type(nl_channel_type: u8) -> u8;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1271_commands {
    CMD_INTERROGATE	= 1, /* use this to read information elements */
    CMD_CONFIGURE	= 2, /* use this to write information elements */
    CMD_ENABLE_RX	= 3,
    CMD_ENABLE_TX	= 4,
    CMD_DISABLE_RX	= 5,
    CMD_DISABLE_TX	= 6,
    CMD_SCAN	= 7,
    CMD_STOP_SCAN	= 8,
    CMD_SET_KEYS	= 9,
    CMD_READ_MEMORY	= 10,
    CMD_WRITE_MEMORY	= 11,
    CMD_SET_TEMPLATE	= 12,
    CMD_TEST		= 13,
    CMD_NOISE_HIST		= 14,
    CMD_QUIET_ELEMENT_SET_STATE = 15,
    CMD_SET_BCN_MODE	= 16,

    CMD_MEASUREMENT		= 17,
    CMD_STOP_MEASUREMENT	= 18,
    CMD_SET_PS_MODE		= 19,
    CMD_CHANNEL_SWITCH	= 20,
    CMD_STOP_CHANNEL_SWICTH = 21,
    CMD_AP_DISCOVERY	= 22,
    CMD_STOP_AP_DISCOVERY	= 23,
    CMD_HEALTH_CHECK	= 24,
    CMD_DEBUG		= 25,
    CMD_TRIGGER_SCAN_TO	= 26,
    CMD_CONNECTION_SCAN_CFG	= 27,
    CMD_CONNECTION_SCAN_SSID_CFG	= 28,
    CMD_START_PERIODIC_SCAN	= 29,
    CMD_STOP_PERIODIC_SCAN	= 30,
    CMD_SET_PEER_STATE	= 31,
    CMD_REMAIN_ON_CHANNEL	= 32,
    CMD_CANCEL_REMAIN_ON_CHANNEL	= 33,
    CMD_CONFIG_FWLOGGER		= 34,
    CMD_START_FWLOGGER			= 35,
    CMD_STOP_FWLOGGER			= 36,

// Access point commands
    CMD_ADD_PEER		= 37,
    CMD_REMOVE_PEER		= 38,

// Role API
    CMD_ROLE_ENABLE		= 39,
    CMD_ROLE_DISABLE	= 40,
    CMD_ROLE_START		= 41,
    CMD_ROLE_STOP		= 42,

// DFS
    CMD_START_RADAR_DETECTION	= 43,
    CMD_STOP_RADAR_DETECTION	= 44,

// WIFI Direct
    CMD_WFD_START_DISCOVERY	= 45,
    CMD_WFD_STOP_DISCOVERY	= 46,
    CMD_WFD_ATTRIBUTE_CONFIG	= 47,
    CMD_GENERIC_CFG			= 48,
    CMD_NOP				= 49,

// start of 18xx specific commands
    CMD_DFS_CHANNEL_CONFIG		= 60,
    CMD_SMART_CONFIG_START		= 61,
    CMD_SMART_CONFIG_STOP		= 62,
    CMD_SMART_CONFIG_SET_GROUP_KEY	= 63,

    CMD_CAC_START			= 64,
    CMD_CAC_STOP			= 65,
    CMD_DFS_MASTER_RESTART		= 66,
    CMD_DFS_RADAR_DETECTION_DEBUG	= 67,

    MAX_COMMAND_ID = 0xFFFF,
}

pub const MAX_CMD_PARAMS: c_int = 572;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_templ {
    CMD_TEMPL_NULL_DATA = 0,
    CMD_TEMPL_BEACON,
    CMD_TEMPL_CFG_PROBE_REQ_2_4,
    CMD_TEMPL_CFG_PROBE_REQ_5,
    CMD_TEMPL_PROBE_RESPONSE,
    CMD_TEMPL_QOS_NULL_DATA,
    CMD_TEMPL_PS_POLL,
    CMD_TEMPL_KLV,
    CMD_TEMPL_DISCONNECT,
    CMD_TEMPL_APP_PROBE_REQ_2_4_LEGACY,
    CMD_TEMPL_APP_PROBE_REQ_5_LEGACY,
    CMD_TEMPL_BAR,           /* for firmware internal use only */
    CMD_TEMPL_CTS,           /*
// For CTS-to-self (FastCTS) mechanism
// for BT/WLAN coexistence (SoftGemini).
    CMD_TEMPL_AP_BEACON,
    CMD_TEMPL_AP_PROBE_RESPONSE,
    CMD_TEMPL_ARP_RSP,
    CMD_TEMPL_DEAUTH_AP,
    CMD_TEMPL_TEMPORARY,
    CMD_TEMPL_LINK_MEASUREMENT_REPORT,
    CMD_TEMPL_PROBE_REQ_2_4_PERIODIC,
    CMD_TEMPL_PROBE_REQ_5_PERIODIC,

    CMD_TEMPL_MAX = 0xff
}

// unit ms
pub const WL1271_COMMAND_TIMEOUT: c_int = 2000;
pub const WL1271_CMD_TEMPL_DFLT_SIZE: c_int = 252;
pub const WL1271_CMD_TEMPL_MAX_SIZE: c_int = 512;
pub const WL1271_EVENT_TIMEOUT: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_header {
    pub id: __le16,
    pub status: __le16,
    pub __packed: },
pub const WL1271_CMD_MAX_PARAMS: c_int = 572;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_command {
    pub header: wl1271_cmd_header,
    pub parameters: [u8; WL1271_CMD_MAX_PARAMS],
    pub __packed: },
}

pub const CMDMBOX_HEADER_LEN: c_int = 4;
pub const CMDMBOX_INFO_ELEM_HEADER_LEN: c_int = 4;
pub const WL1271_JOIN_CMD_CTRL_TX_FLUSH: c_uint = 0x80 /* Firmware flushes all Tx */;
pub const WL1271_JOIN_CMD_TX_SESSION_OFFSET: c_int = 1;
pub const WL1271_JOIN_CMD_BSS_TYPE_5GHZ: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_role_enable {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub role_type: u8,
    pub mac_address: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_role_disable {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_band {
    WLCORE_BAND_2_4GHZ		= 0,
    WLCORE_BAND_5GHZ		= 1,
    WLCORE_BAND_JAPAN_4_9_GHZ	= 2,
    WLCORE_BAND_DEFAULT		= WLCORE_BAND_2_4GHZ,
    WLCORE_BAND_INVALID		= 0x7E,
    WLCORE_BAND_MAX_RADIO		= 0x7F,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_channel_type {
    WLCORE_CHAN_NO_HT,
    WLCORE_CHAN_HT20,
    WLCORE_CHAN_HT40MINUS,
    WLCORE_CHAN_HT40PLUS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_role_start {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub band: u8,
    pub channel: u8,
// enum wlcore_channel_type
    pub channel_type: u8,
    pub hlid: u8,
    pub session: u8,
    pub padding_1: [u8; 54],
    pub device: } __packed,
// sta & p2p_cli use the same struct
    pub bssid: [u8; ETH_ALEN],
    pub /: *mut *mut u8 hlid; / data hlid,
    pub session: u8,
    pub /: *mut *mut __le32 remote_rates; / remote supported rates,
//
// The target uses this field to determine the rate at
// which to transmit control frame responses (such as
// ACK or CTS frames).
//
    pub basic_rate_set: __le32,
    pub /: *mut *mut __le32 local_rates; / local supported rates,
    pub ssid_type: u8,
    pub ssid_len: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub /: *mut *mut __le16 beacon_interval; / in TBTTs,
    pub sta: } __packed,
    pub bssid: [u8; ETH_ALEN],
    pub /: *mut *mut u8 hlid; / data hlid,
    pub dtim_interval: u8,
    pub /: *mut *mut __le32 remote_rates; / remote supported rates,
    pub basic_rate_set: __le32,
    pub /: *mut *mut __le32 local_rates; / local supported rates,
    pub ssid_type: u8,
    pub ssid_len: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub /: *mut *mut __le16 beacon_interval; / in TBTTs,
    pub padding_1: [u8; 4],
    pub ibss: } __packed,
// ap & p2p_go use the same struct
    pub /: *mut *mut __le16 aging_period; / in secs,
    pub /: *mut *mut u8 beacon_expiry; / in ms,
    pub bss_index: u8,
// The host link id for the AP's global queue
    pub global_hlid: u8,
// The host link id for the AP's broadcast queue
    pub broadcast_hlid: u8,
    pub /: *mut *mut __le16 beacon_interval; / in TBTTs,
    pub basic_rate_set: __le32,
    pub /: *mut *mut __le32 local_rates; / local supported rates,
    pub dtim_interval: u8,
    pub ssid_type: u8,
    pub ssid_len: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub reset_tsf: u8,
//
// ap supports wmm (note that there is additional
// per-sta wmm configuration)
//
    pub wmm: u8,
    pub bcast_session_id: u8,
    pub global_session_id: u8,
    pub padding_1: [u8; 1],
    pub ap: } __packed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_role_stop {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub /: *mut *mut u8 disc_type; / only STA and P2P_CLI,
    pub /: *mut *mut __le16 reason; / only STA and P2P_CLI,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_enabledisable_path {
    pub header: wl1271_cmd_header,
    pub channel: u8,
    pub padding: [u8; 3],
    pub __packed: },
pub const WL1271_RATE_AUTOMATIC: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_template_set {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub template_type: u8,
    pub len: __le16,
    pub /: *mut *mut u8 index; / relevant only for KLV_TEMPLATE type,
    pub padding: [u8; 3],
    pub enabled_rates: __le32,
    pub short_retry_limit: u8,
    pub long_retry_limit: u8,
    pub aflags: u8,
    pub reserved: u8,
    pub template_data: [u8; WL1271_CMD_TEMPL_MAX_SIZE],
    pub __packed: },
pub const TIM_ELE_ID: c_int = 5;
pub const PARTIAL_VBM_MAX: c_int = 251;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_tim {
    pub identity: u8,
    pub length: u8,
    pub dtim_count: u8,
    pub dtim_period: u8,
    pub bitmap_ctrl: u8,
    pub /: *mut *mut u8 pvb_field[PARTIAL_VBM_MAX]; / Partial Virtual Bitmap,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1271_cmd_ps_mode {
    STATION_AUTO_PS_MODE,   /* Dynamic Power Save */
    STATION_ACTIVE_MODE,
    STATION_POWER_SAVE_MODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_ps_params {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub /: *mut *mut *mut u8 ps_mode; / STATION_,
    pub auto_ps_timeout: u16,
    pub __packed: },
// HW encryption keys
pub const NUM_ACCESS_CATEGORIES_COPY: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1271_cmd_key_action {
    KEY_ADD_OR_REPLACE = 1,
    KEY_REMOVE         = 2,
    KEY_SET_ID         = 3,
    MAX_KEY_ACTION     = 0xffff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1271_cmd_lid_key_type {
    UNICAST_LID_TYPE     = 0,
    BROADCAST_LID_TYPE   = 1,
    WEP_DEFAULT_LID_TYPE = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1271_cmd_key_type {
    KEY_NONE = 0,
    KEY_WEP  = 1,
    KEY_TKIP = 2,
    KEY_AES  = 3,
    KEY_GEM  = 4,
    KEY_IGTK = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_set_keys {
    pub header: wl1271_cmd_header,
//
// Indicates whether the HLID is a unicast key set
// or broadcast key set. A special value 0xFF is
// used to indicate that the HLID is on WEP-default
// (multi-hlids). of type wl1271_cmd_lid_key_type.
//
    pub hlid: u8,
//
// In WEP-default network (hlid == 0xFF) used to
// indicate which network STA/IBSS/AP role should be
// changed
//
    pub lid_key_type: u8,
//
// Key ID - For TKIP and AES key types, this field
// indicates the value that should be inserted into
// the KeyID field of frames transmitted using this
// key entry. For broadcast keys the index use as a
// marker for TX/RX key.
// For WEP default network (HLID=0xFF), this field
// indicates the ID of the key to add or remove.
//
    pub key_id: u8,
    pub reserved_1: u8,
// key_action_e
    pub key_action: __le16,
// key size in bytes
    pub key_size: u8,
// key_type_e
    pub key_type: u8,
// This field holds the security key data to add to the STA table
    pub key: [u8; MAX_KEY_SIZE],
    pub ac_seq_num16: [__le16; NUM_ACCESS_CATEGORIES_COPY],
    pub ac_seq_num32: [__le32; NUM_ACCESS_CATEGORIES_COPY],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_test_header {
    pub id: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1271_channel_tune_bands {
    WL1271_CHANNEL_TUNE_BAND_2_4,
    WL1271_CHANNEL_TUNE_BAND_5,
    WL1271_CHANNEL_TUNE_BAND_4_9
}

pub const WL1271_PD_REFERENCE_POINT_BAND_B_G: c_int = 0;
//
// There are three types of disconnections:
//
// DISCONNECT_IMMEDIATE: the fw doesn't send any frames
// DISCONNECT_DEAUTH:    the fw generates a DEAUTH request with the reason
// we have passed
// DISCONNECT_DISASSOC:  the fw generates a DESASSOC request with the reason
// we have passed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1271_disconnect_type {
    DISCONNECT_IMMEDIATE,
    DISCONNECT_DEAUTH,
    DISCONNECT_DISASSOC
}

pub const WL1271_CMD_STA_STATE_CONNECTED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_set_peer_state {
    pub header: wl1271_cmd_header,
    pub hlid: u8,
    pub state: u8,
//
// wmm is relevant for sta role only.
// ap role configures the per-sta wmm params in
// the add_peer command.
//
    pub wmm: u8,
    pub padding: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_roc {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub channel: u8,
    pub band: u8,
    pub padding: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_croc {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub padding: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl12xx_ssid_type {
    WL12XX_SSID_TYPE_PUBLIC = 0,
    WL12XX_SSID_TYPE_HIDDEN = 1,
    WL12XX_SSID_TYPE_ANY = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1271_psd_type {
    WL1271_PSD_LEGACY = 0,
    WL1271_PSD_UPSD_TRIGGER = 1,
    WL1271_PSD_LEGACY_PSPOLL = 2,
    WL1271_PSD_SAPSD = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_add_peer {
    pub header: wl1271_cmd_header,
    pub addr: [u8; ETH_ALEN],
    pub hlid: u8,
    pub aid: u8,
    pub psd_type: [u8; NUM_ACCESS_CATEGORIES_COPY],
    pub supported_rates: __le32,
    pub bss_index: u8,
    pub sp_len: u8,
    pub wmm: u8,
    pub session_id: u8,
    pub role_id: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_remove_peer {
    pub header: wl1271_cmd_header,
    pub hlid: u8,
    pub reason_opcode: u8,
    pub send_deauth_flag: u8,
    pub role_id: u8,
    pub __packed: },
//
// Continuous mode - packets are transferred to the host periodically
// via the data path.
// On demand - Log messages are stored in a cyclic buffer in the
// firmware, and only transferred to the host when explicitly requested
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl12xx_fwlogger_log_mode {
    WL12XX_FWLOG_CONTINUOUS,
}

// Include/exclude timestamps from the log messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl12xx_fwlogger_timestamp {
    WL12XX_FWLOG_TIMESTAMP_DISABLED,
    WL12XX_FWLOG_TIMESTAMP_ENABLED
}

//
// Logs can be routed to the debug pinouts (where available), to the host bus
// (SDIO/SPI), or dropped
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl12xx_fwlogger_output {
    WL12XX_FWLOG_OUTPUT_NONE,
    WL12XX_FWLOG_OUTPUT_DBG_PINS,
    WL12XX_FWLOG_OUTPUT_HOST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_regdomain_dfs_config {
    pub header: wl1271_cmd_header,
    pub ch_bit_map1: __le32,
    pub ch_bit_map2: __le32,
    pub dfs_region: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wlcore_generic_cfg_feature {
    WLCORE_CFG_FEATURE_RADAR_DEBUG = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlcore_cmd_generic_cfg {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub feature: u8,
    pub enable: u8,
    pub value: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_config_fwlog {
    pub header: wl1271_cmd_header,
// See enum wl12xx_fwlogger_log_mode
    pub logger_mode: u8,
// Minimum log level threshold
    pub log_severity: u8,
// Include/exclude timestamps from the log messages
    pub timestamp: u8,
// See enum wl1271_fwlogger_output
    pub output: u8,
// Regulates the frequency of log messages
    pub threshold: u8,
    pub padding: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_start_fwlog {
    pub header: wl1271_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_stop_fwlog {
    pub header: wl1271_cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl12xx_cmd_stop_channel_switch {
    pub header: wl1271_cmd_header,
    pub role_id: u8,
    pub padding: [u8; 3],
    pub __packed: },
// Used to check radio status after calibration
pub const MAX_TLV_LENGTH: c_int = 500;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1271_cmd_cal_p2g {
    pub header: wl1271_cmd_header,
    pub test: wl1271_cmd_test_header,
    pub ver: __le32,
    pub len: __le16,
    pub buf: [u8; MAX_TLV_LENGTH],
    pub type: u8,
    pub padding: u8,
    pub radio_status: __le16,
    pub sub_band_mask: u8,
    pub padding2: u8,
    pub __packed: },
