//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wcn36xx/hal.h
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
// Copyright (c) 2013 Eugene Krasnikov <k.eugene.e@gmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// ---------------------------------------------------------------------------
pub const WCN36XX_HAL_VER_MAJOR: c_int = 1;
pub const WCN36XX_HAL_VER_MINOR: c_int = 4;
pub const WCN36XX_HAL_VER_VERSION: c_int = 1;
pub const WCN36XX_HAL_VER_REVISION: c_int = 2;
// This is to force compiler to use the maximum of an int ( 4 bytes )
pub const WCN36XX_HAL_MAX_ENUM_SIZE: c_uint = 0x7FFFFFFF;
pub const WCN36XX_HAL_MSG_TYPE_MAX_ENUM_SIZE: c_uint = 0x7FFF;
// Max no. of transmit categories
pub const STACFG_MAX_TC: c_int = 8;
// The maximum value of access category
pub const WCN36XX_HAL_MAX_AC: c_int = 4;
pub const WCN36XX_HAL_IPV4_ADDR_LEN: c_int = 4;
pub const WCN36XX_HAL_STA_INVALID_IDX: c_uint = 0xFF;
pub const WCN36XX_HAL_BSS_INVALID_IDX: c_uint = 0xFF;
// Default Beacon template size
pub const BEACON_TEMPLATE_SIZE: c_uint = 0x180;
// Minimum PVM size that the FW expects. See comment in smd.c for details.
pub const TIM_MIN_PVM_SIZE: c_int = 6;
// Param Change Bitmap sent to HAL

// dump command response Buffer size
pub const DUMPCMD_RSP_BUFFER: c_int = 100;
// version string max length (including NULL)
pub const WCN36XX_HAL_VERSION_LENGTH: c_int = 64;
// How many frames until we start a-mpdu TX session
pub const WCN36XX_AMPDU_START_THRESH: c_int = 20;
pub const WCN36XX_MAX_SCAN_SSIDS: c_int = 9;
pub const WCN36XX_MAX_SCAN_IE_LEN: c_int = 500;
// message types for messages exchanged between WDI and HAL
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_host_msg_type {
// Init/De-Init
    WCN36XX_HAL_START_REQ = 0,
    WCN36XX_HAL_START_RSP = 1,
    WCN36XX_HAL_STOP_REQ = 2,
    WCN36XX_HAL_STOP_RSP = 3,

// Scan
    WCN36XX_HAL_INIT_SCAN_REQ = 4,
    WCN36XX_HAL_INIT_SCAN_RSP = 5,
    WCN36XX_HAL_START_SCAN_REQ = 6,
    WCN36XX_HAL_START_SCAN_RSP = 7,
    WCN36XX_HAL_END_SCAN_REQ = 8,
    WCN36XX_HAL_END_SCAN_RSP = 9,
    WCN36XX_HAL_FINISH_SCAN_REQ = 10,
    WCN36XX_HAL_FINISH_SCAN_RSP = 11,

// HW STA configuration/deconfiguration
    WCN36XX_HAL_CONFIG_STA_REQ = 12,
    WCN36XX_HAL_CONFIG_STA_RSP = 13,
    WCN36XX_HAL_DELETE_STA_REQ = 14,
    WCN36XX_HAL_DELETE_STA_RSP = 15,
    WCN36XX_HAL_CONFIG_BSS_REQ = 16,
    WCN36XX_HAL_CONFIG_BSS_RSP = 17,
    WCN36XX_HAL_DELETE_BSS_REQ = 18,
    WCN36XX_HAL_DELETE_BSS_RSP = 19,

// Infra STA asscoiation
    WCN36XX_HAL_JOIN_REQ = 20,
    WCN36XX_HAL_JOIN_RSP = 21,
    WCN36XX_HAL_POST_ASSOC_REQ = 22,
    WCN36XX_HAL_POST_ASSOC_RSP = 23,

// Security
    WCN36XX_HAL_SET_BSSKEY_REQ = 24,
    WCN36XX_HAL_SET_BSSKEY_RSP = 25,
    WCN36XX_HAL_SET_STAKEY_REQ = 26,
    WCN36XX_HAL_SET_STAKEY_RSP = 27,
    WCN36XX_HAL_RMV_BSSKEY_REQ = 28,
    WCN36XX_HAL_RMV_BSSKEY_RSP = 29,
    WCN36XX_HAL_RMV_STAKEY_REQ = 30,
    WCN36XX_HAL_RMV_STAKEY_RSP = 31,

// Qos Related
    WCN36XX_HAL_ADD_TS_REQ = 32,
    WCN36XX_HAL_ADD_TS_RSP = 33,
    WCN36XX_HAL_DEL_TS_REQ = 34,
    WCN36XX_HAL_DEL_TS_RSP = 35,
    WCN36XX_HAL_UPD_EDCA_PARAMS_REQ = 36,
    WCN36XX_HAL_UPD_EDCA_PARAMS_RSP = 37,
    WCN36XX_HAL_ADD_BA_REQ = 38,
    WCN36XX_HAL_ADD_BA_RSP = 39,
    WCN36XX_HAL_DEL_BA_REQ = 40,
    WCN36XX_HAL_DEL_BA_RSP = 41,

    WCN36XX_HAL_CH_SWITCH_REQ = 42,
    WCN36XX_HAL_CH_SWITCH_RSP = 43,
    WCN36XX_HAL_SET_LINK_ST_REQ = 44,
    WCN36XX_HAL_SET_LINK_ST_RSP = 45,
    WCN36XX_HAL_GET_STATS_REQ = 46,
    WCN36XX_HAL_GET_STATS_RSP = 47,
    WCN36XX_HAL_UPDATE_CFG_REQ = 48,
    WCN36XX_HAL_UPDATE_CFG_RSP = 49,

    WCN36XX_HAL_MISSED_BEACON_IND = 50,
    WCN36XX_HAL_UNKNOWN_ADDR2_FRAME_RX_IND = 51,
    WCN36XX_HAL_MIC_FAILURE_IND = 52,
    WCN36XX_HAL_FATAL_ERROR_IND = 53,
    WCN36XX_HAL_SET_KEYDONE_MSG = 54,

// NV Interface
    WCN36XX_HAL_DOWNLOAD_NV_REQ = 55,
    WCN36XX_HAL_DOWNLOAD_NV_RSP = 56,

    WCN36XX_HAL_ADD_BA_SESSION_REQ = 57,
    WCN36XX_HAL_ADD_BA_SESSION_RSP = 58,
    WCN36XX_HAL_TRIGGER_BA_REQ = 59,
    WCN36XX_HAL_TRIGGER_BA_RSP = 60,
    WCN36XX_HAL_UPDATE_BEACON_REQ = 61,
    WCN36XX_HAL_UPDATE_BEACON_RSP = 62,
    WCN36XX_HAL_SEND_BEACON_REQ = 63,
    WCN36XX_HAL_SEND_BEACON_RSP = 64,

    WCN36XX_HAL_SET_BCASTKEY_REQ = 65,
    WCN36XX_HAL_SET_BCASTKEY_RSP = 66,
    WCN36XX_HAL_DELETE_STA_CONTEXT_IND = 67,
    WCN36XX_HAL_UPDATE_PROBE_RSP_TEMPLATE_REQ = 68,
    WCN36XX_HAL_UPDATE_PROBE_RSP_TEMPLATE_RSP = 69,

// PTT interface support
    WCN36XX_HAL_PROCESS_PTT_REQ = 70,
    WCN36XX_HAL_PROCESS_PTT_RSP = 71,

// BTAMP related events
    WCN36XX_HAL_SIGNAL_BTAMP_EVENT_REQ = 72,
    WCN36XX_HAL_SIGNAL_BTAMP_EVENT_RSP = 73,
    WCN36XX_HAL_TL_HAL_FLUSH_AC_REQ = 74,
    WCN36XX_HAL_TL_HAL_FLUSH_AC_RSP = 75,

    WCN36XX_HAL_ENTER_IMPS_REQ = 76,
    WCN36XX_HAL_EXIT_IMPS_REQ = 77,
    WCN36XX_HAL_ENTER_BMPS_REQ = 78,
    WCN36XX_HAL_EXIT_BMPS_REQ = 79,
    WCN36XX_HAL_ENTER_UAPSD_REQ = 80,
    WCN36XX_HAL_EXIT_UAPSD_REQ = 81,
    WCN36XX_HAL_UPDATE_UAPSD_PARAM_REQ = 82,
    WCN36XX_HAL_CONFIGURE_RXP_FILTER_REQ = 83,
    WCN36XX_HAL_ADD_BCN_FILTER_REQ = 84,
    WCN36XX_HAL_REM_BCN_FILTER_REQ = 85,
    WCN36XX_HAL_ADD_WOWL_BCAST_PTRN = 86,
    WCN36XX_HAL_DEL_WOWL_BCAST_PTRN = 87,
    WCN36XX_HAL_ENTER_WOWL_REQ = 88,
    WCN36XX_HAL_EXIT_WOWL_REQ = 89,
    WCN36XX_HAL_HOST_OFFLOAD_REQ = 90,
    WCN36XX_HAL_SET_RSSI_THRESH_REQ = 91,
    WCN36XX_HAL_GET_RSSI_REQ = 92,
    WCN36XX_HAL_SET_UAPSD_AC_PARAMS_REQ = 93,
    WCN36XX_HAL_CONFIGURE_APPS_CPU_WAKEUP_STATE_REQ = 94,

    WCN36XX_HAL_ENTER_IMPS_RSP = 95,
    WCN36XX_HAL_EXIT_IMPS_RSP = 96,
    WCN36XX_HAL_ENTER_BMPS_RSP = 97,
    WCN36XX_HAL_EXIT_BMPS_RSP = 98,
    WCN36XX_HAL_ENTER_UAPSD_RSP = 99,
    WCN36XX_HAL_EXIT_UAPSD_RSP = 100,
    WCN36XX_HAL_SET_UAPSD_AC_PARAMS_RSP = 101,
    WCN36XX_HAL_UPDATE_UAPSD_PARAM_RSP = 102,
    WCN36XX_HAL_CONFIGURE_RXP_FILTER_RSP = 103,
    WCN36XX_HAL_ADD_BCN_FILTER_RSP = 104,
    WCN36XX_HAL_REM_BCN_FILTER_RSP = 105,
    WCN36XX_HAL_SET_RSSI_THRESH_RSP = 106,
    WCN36XX_HAL_HOST_OFFLOAD_RSP = 107,
    WCN36XX_HAL_ADD_WOWL_BCAST_PTRN_RSP = 108,
    WCN36XX_HAL_DEL_WOWL_BCAST_PTRN_RSP = 109,
    WCN36XX_HAL_ENTER_WOWL_RSP = 110,
    WCN36XX_HAL_EXIT_WOWL_RSP = 111,
    WCN36XX_HAL_RSSI_NOTIFICATION_IND = 112,
    WCN36XX_HAL_GET_RSSI_RSP = 113,
    WCN36XX_HAL_CONFIGURE_APPS_CPU_WAKEUP_STATE_RSP = 114,

// 11k related events
    WCN36XX_HAL_SET_MAX_TX_POWER_REQ = 115,
    WCN36XX_HAL_SET_MAX_TX_POWER_RSP = 116,

// 11R related msgs
    WCN36XX_HAL_AGGR_ADD_TS_REQ = 117,
    WCN36XX_HAL_AGGR_ADD_TS_RSP = 118,

// P2P  WLAN_FEATURE_P2P
    WCN36XX_HAL_SET_P2P_GONOA_REQ = 119,
    WCN36XX_HAL_SET_P2P_GONOA_RSP = 120,

// WLAN Dump commands
    WCN36XX_HAL_DUMP_COMMAND_REQ = 121,
    WCN36XX_HAL_DUMP_COMMAND_RSP = 122,

// OEM_DATA FEATURE SUPPORT
    WCN36XX_HAL_START_OEM_DATA_REQ = 123,
    WCN36XX_HAL_START_OEM_DATA_RSP = 124,

// ADD SELF STA REQ and RSP
    WCN36XX_HAL_ADD_STA_SELF_REQ = 125,
    WCN36XX_HAL_ADD_STA_SELF_RSP = 126,

// DEL SELF STA SUPPORT
    WCN36XX_HAL_DEL_STA_SELF_REQ = 127,
    WCN36XX_HAL_DEL_STA_SELF_RSP = 128,

// Coex Indication
    WCN36XX_HAL_COEX_IND = 129,

// Tx Complete Indication
    WCN36XX_HAL_OTA_TX_COMPL_IND = 130,

// Host Suspend/resume messages
    WCN36XX_HAL_HOST_SUSPEND_IND = 131,
    WCN36XX_HAL_HOST_RESUME_REQ = 132,
    WCN36XX_HAL_HOST_RESUME_RSP = 133,

    WCN36XX_HAL_SET_TX_POWER_REQ = 134,
    WCN36XX_HAL_SET_TX_POWER_RSP = 135,
    WCN36XX_HAL_GET_TX_POWER_REQ = 136,
    WCN36XX_HAL_GET_TX_POWER_RSP = 137,

    WCN36XX_HAL_P2P_NOA_ATTR_IND = 138,

    WCN36XX_HAL_ENABLE_RADAR_DETECT_REQ = 139,
    WCN36XX_HAL_ENABLE_RADAR_DETECT_RSP = 140,
    WCN36XX_HAL_GET_TPC_REPORT_REQ = 141,
    WCN36XX_HAL_GET_TPC_REPORT_RSP = 142,
    WCN36XX_HAL_RADAR_DETECT_IND = 143,
    WCN36XX_HAL_RADAR_DETECT_INTR_IND = 144,
    WCN36XX_HAL_KEEP_ALIVE_REQ = 145,
    WCN36XX_HAL_KEEP_ALIVE_RSP = 146,

// PNO messages
    WCN36XX_HAL_SET_PREF_NETWORK_REQ = 147,
    WCN36XX_HAL_SET_PREF_NETWORK_RSP = 148,
    WCN36XX_HAL_SET_RSSI_FILTER_REQ = 149,
    WCN36XX_HAL_SET_RSSI_FILTER_RSP = 150,
    WCN36XX_HAL_UPDATE_SCAN_PARAM_REQ = 151,
    WCN36XX_HAL_UPDATE_SCAN_PARAM_RSP = 152,
    WCN36XX_HAL_PREF_NETW_FOUND_IND = 153,

    WCN36XX_HAL_SET_TX_PER_TRACKING_REQ = 154,
    WCN36XX_HAL_SET_TX_PER_TRACKING_RSP = 155,
    WCN36XX_HAL_TX_PER_HIT_IND = 156,

    WCN36XX_HAL_8023_MULTICAST_LIST_REQ = 157,
    WCN36XX_HAL_8023_MULTICAST_LIST_RSP = 158,

    WCN36XX_HAL_SET_PACKET_FILTER_REQ = 159,
    WCN36XX_HAL_SET_PACKET_FILTER_RSP = 160,
    WCN36XX_HAL_PACKET_FILTER_MATCH_COUNT_REQ = 161,
    WCN36XX_HAL_PACKET_FILTER_MATCH_COUNT_RSP = 162,
    WCN36XX_HAL_CLEAR_PACKET_FILTER_REQ = 163,
    WCN36XX_HAL_CLEAR_PACKET_FILTER_RSP = 164,

//
// This is temp fix. Should be removed once Host and Riva code is
// in sync.
//
    WCN36XX_HAL_INIT_SCAN_CON_REQ = 165,

    WCN36XX_HAL_SET_POWER_PARAMS_REQ = 166,
    WCN36XX_HAL_SET_POWER_PARAMS_RSP = 167,

    WCN36XX_HAL_TSM_STATS_REQ = 168,
    WCN36XX_HAL_TSM_STATS_RSP = 169,

// wake reason indication (WOW)
    WCN36XX_HAL_WAKE_REASON_IND = 170,

// GTK offload support
    WCN36XX_HAL_GTK_OFFLOAD_REQ = 171,
    WCN36XX_HAL_GTK_OFFLOAD_RSP = 172,
    WCN36XX_HAL_GTK_OFFLOAD_GETINFO_REQ = 173,
    WCN36XX_HAL_GTK_OFFLOAD_GETINFO_RSP = 174,

    WCN36XX_HAL_FEATURE_CAPS_EXCHANGE_REQ = 175,
    WCN36XX_HAL_FEATURE_CAPS_EXCHANGE_RSP = 176,
    WCN36XX_HAL_EXCLUDE_UNENCRYPTED_IND = 177,

    WCN36XX_HAL_SET_THERMAL_MITIGATION_REQ = 178,
    WCN36XX_HAL_SET_THERMAL_MITIGATION_RSP = 179,

    WCN36XX_HAL_UPDATE_VHT_OP_MODE_REQ = 182,
    WCN36XX_HAL_UPDATE_VHT_OP_MODE_RSP = 183,

    WCN36XX_HAL_P2P_NOA_START_IND = 184,

    WCN36XX_HAL_GET_ROAM_RSSI_REQ = 185,
    WCN36XX_HAL_GET_ROAM_RSSI_RSP = 186,

    WCN36XX_HAL_CLASS_B_STATS_IND = 187,
    WCN36XX_HAL_DEL_BA_IND = 188,
    WCN36XX_HAL_DHCP_START_IND = 189,
    WCN36XX_HAL_DHCP_STOP_IND = 190,

// Scan Offload(hw) APIs
    WCN36XX_HAL_START_SCAN_OFFLOAD_REQ = 204,
    WCN36XX_HAL_START_SCAN_OFFLOAD_RSP = 205,
    WCN36XX_HAL_STOP_SCAN_OFFLOAD_REQ = 206,
    WCN36XX_HAL_STOP_SCAN_OFFLOAD_RSP = 207,
    WCN36XX_HAL_UPDATE_CHANNEL_LIST_REQ = 208,
    WCN36XX_HAL_UPDATE_CHANNEL_LIST_RSP = 209,
    WCN36XX_HAL_SCAN_OFFLOAD_IND = 210,

    WCN36XX_HAL_AVOID_FREQ_RANGE_IND = 233,

    WCN36XX_HAL_PRINT_REG_INFO_IND = 259,

    WCN36XX_HAL_MSG_MAX = WCN36XX_HAL_MSG_TYPE_MAX_ENUM_SIZE
}

// Enumeration for Version
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_host_msg_version {
    WCN36XX_HAL_MSG_VERSION0 = 0,
    WCN36XX_HAL_MSG_VERSION1 = 1,
// define as 2 bytes data
    WCN36XX_HAL_MSG_WCNSS_CTRL_VERSION = 0x7FFF,
    WCN36XX_HAL_MSG_VERSION_MAX_FIELD = WCN36XX_HAL_MSG_WCNSS_CTRL_VERSION
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum driver_type {
    DRIVER_TYPE_PRODUCTION = 0,
    DRIVER_TYPE_MFG = 1,
    DRIVER_TYPE_DVT = 2,
    DRIVER_TYPE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_stop_type {
    HAL_STOP_TYPE_SYS_RESET,
    HAL_STOP_TYPE_SYS_DEEP_SLEEP,
    HAL_STOP_TYPE_RF_KILL,
    HAL_STOP_TYPE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_sys_mode {
    HAL_SYS_MODE_NORMAL,
    HAL_SYS_MODE_LEARN,
    HAL_SYS_MODE_SCAN,
    HAL_SYS_MODE_PROMISC,
    HAL_SYS_MODE_SUSPEND_LINK,
    HAL_SYS_MODE_ROAM_SCAN,
    HAL_SYS_MODE_ROAM_SUSPEND_LINK,
    HAL_SYS_MODE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_chan_bond_state {
// 20MHz IF bandwidth centered on IF carrier
    PHY_SINGLE_CHANNEL_CENTERED = 0,

// 40MHz IF bandwidth with lower 20MHz supporting the primary channel
    PHY_DOUBLE_CHANNEL_LOW_PRIMARY = 1,

// 40MHz IF bandwidth centered on IF carrier
    PHY_DOUBLE_CHANNEL_CENTERED = 2,

// 40MHz IF bandwidth with higher 20MHz supporting the primary ch
    PHY_DOUBLE_CHANNEL_HIGH_PRIMARY = 3,

// 20/40MHZ offset LOW 40/80MHZ offset CENTERED
    PHY_QUADRUPLE_CHANNEL_20MHZ_LOW_40MHZ_CENTERED = 4,

// 20/40MHZ offset CENTERED 40/80MHZ offset CENTERED
    PHY_QUADRUPLE_CHANNEL_20MHZ_CENTERED_40MHZ_CENTERED = 5,

// 20/40MHZ offset HIGH 40/80MHZ offset CENTERED
    PHY_QUADRUPLE_CHANNEL_20MHZ_HIGH_40MHZ_CENTERED = 6,

// 20/40MHZ offset LOW 40/80MHZ offset LOW
    PHY_QUADRUPLE_CHANNEL_20MHZ_LOW_40MHZ_LOW = 7,

// 20/40MHZ offset HIGH 40/80MHZ offset LOW
    PHY_QUADRUPLE_CHANNEL_20MHZ_HIGH_40MHZ_LOW = 8,

// 20/40MHZ offset LOW 40/80MHZ offset HIGH
    PHY_QUADRUPLE_CHANNEL_20MHZ_LOW_40MHZ_HIGH = 9,

// 20/40MHZ offset-HIGH 40/80MHZ offset HIGH
    PHY_QUADRUPLE_CHANNEL_20MHZ_HIGH_40MHZ_HIGH = 10,

    PHY_CHANNEL_BONDING_STATE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

// Spatial Multiplexing(SM) Power Save mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_ht_mimo_state {
// Static SM Power Save mode
    WCN36XX_HAL_HT_MIMO_PS_STATIC = 0,

// Dynamic SM Power Save mode
    WCN36XX_HAL_HT_MIMO_PS_DYNAMIC = 1,

// reserved
    WCN36XX_HAL_HT_MIMO_PS_NA = 2,

// SM Power Save disabled
    WCN36XX_HAL_HT_MIMO_PS_NO_LIMIT = 3,

    WCN36XX_HAL_HT_MIMO_PS_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

// each station added has a rate mode which specifies the sta attributes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sta_rate_mode {
    STA_TAURUS = 0,
    STA_TITAN,
    STA_POLARIS,
    STA_11b,
    STA_11bg,
    STA_11a,
    STA_11n,
    STA_11ac,
    STA_INVALID_RATE_MODE = WCN36XX_HAL_MAX_ENUM_SIZE
}

// 1,2,5.5,11
pub const WCN36XX_HAL_NUM_DSSS_RATES: c_int = 4;
// 6,9,12,18,24,36,48,54
pub const WCN36XX_HAL_NUM_OFDM_RATES: c_int = 8;
// 72,96,108
pub const WCN36XX_HAL_NUM_POLARIS_RATES: c_int = 3;
pub const WCN36XX_HAL_MAC_MAX_SUPPORTED_MCS_SET: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_bss_type {
    WCN36XX_HAL_INFRASTRUCTURE_MODE,

// Added for softAP support
    WCN36XX_HAL_INFRA_AP_MODE,

    WCN36XX_HAL_IBSS_MODE,

// Added for BT-AMP support
    WCN36XX_HAL_BTAMP_STA_MODE,

// Added for BT-AMP support
    WCN36XX_HAL_BTAMP_AP_MODE,

    WCN36XX_HAL_AUTO_MODE,

    WCN36XX_HAL_DONOT_USE_BSS_TYPE = WCN36XX_HAL_MAX_ENUM_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_nw_type {
    WCN36XX_HAL_11A_NW_TYPE,
    WCN36XX_HAL_11B_NW_TYPE,
    WCN36XX_HAL_11G_NW_TYPE,
    WCN36XX_HAL_11N_NW_TYPE,
    WCN36XX_HAL_DONOT_USE_NW_TYPE = WCN36XX_HAL_MAX_ENUM_SIZE
}

pub const WCN36XX_HAL_MAC_RATESET_EID_MAX: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_ht_operating_mode {
// No Protection
    WCN36XX_HAL_HT_OP_MODE_PURE,

// Overlap Legacy device present, protection is optional
    WCN36XX_HAL_HT_OP_MODE_OVERLAP_LEGACY,

// No legacy device, but 20 MHz HT present
    WCN36XX_HAL_HT_OP_MODE_NO_LEGACY_20MHZ_HT,

// Protection is required
    WCN36XX_HAL_HT_OP_MODE_MIXED,

    WCN36XX_HAL_HT_OP_MODE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

// Encryption type enum used with peer
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ani_ed_type {
    WCN36XX_HAL_ED_NONE,
    WCN36XX_HAL_ED_WEP40,
    WCN36XX_HAL_ED_WEP104,
    WCN36XX_HAL_ED_TKIP,
    WCN36XX_HAL_ED_CCMP,
    WCN36XX_HAL_ED_WPI,
    WCN36XX_HAL_ED_AES_128_CMAC,
    WCN36XX_HAL_ED_NOT_IMPLEMENTED = WCN36XX_HAL_MAX_ENUM_SIZE
}

pub const WLAN_MAX_KEY_RSC_LEN: c_int = 16;
pub const WLAN_WAPI_KEY_RSC_LEN: c_int = 16;
// MAX key length when ULA is used
pub const WCN36XX_HAL_MAC_MAX_KEY_LENGTH: c_int = 32;
pub const WCN36XX_HAL_MAC_MAX_NUM_OF_DEFAULT_KEYS: c_int = 4;
//
// Enum to specify whether key is used for TX only, RX only or both.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ani_key_direction {
    WCN36XX_HAL_TX_ONLY,
    WCN36XX_HAL_RX_ONLY,
    WCN36XX_HAL_TX_RX,
    WCN36XX_HAL_TX_DEFAULT,
    WCN36XX_HAL_DONOT_USE_KEY_DIRECTION = WCN36XX_HAL_MAX_ENUM_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ani_wep_type {
    WCN36XX_HAL_WEP_STATIC,
    WCN36XX_HAL_WEP_DYNAMIC,
    WCN36XX_HAL_WEP_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_link_state {

    WCN36XX_HAL_LINK_IDLE_STATE = 0,
    WCN36XX_HAL_LINK_PREASSOC_STATE = 1,
    WCN36XX_HAL_LINK_POSTASSOC_STATE = 2,
    WCN36XX_HAL_LINK_AP_STATE = 3,
    WCN36XX_HAL_LINK_IBSS_STATE = 4,

// BT-AMP Case
    WCN36XX_HAL_LINK_BTAMP_PREASSOC_STATE = 5,
    WCN36XX_HAL_LINK_BTAMP_POSTASSOC_STATE = 6,
    WCN36XX_HAL_LINK_BTAMP_AP_STATE = 7,
    WCN36XX_HAL_LINK_BTAMP_STA_STATE = 8,

// Reserved for HAL Internal Use
    WCN36XX_HAL_LINK_LEARN_STATE = 9,
    WCN36XX_HAL_LINK_SCAN_STATE = 10,
    WCN36XX_HAL_LINK_FINISH_SCAN_STATE = 11,
    WCN36XX_HAL_LINK_INIT_CAL_STATE = 12,
    WCN36XX_HAL_LINK_FINISH_CAL_STATE = 13,
    WCN36XX_HAL_LINK_LISTEN_STATE = 14,

    WCN36XX_HAL_LINK_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_stats_mask {
    HAL_SUMMARY_STATS_INFO = 0x00000001,
    HAL_GLOBAL_CLASS_A_STATS_INFO = 0x00000002,
    HAL_GLOBAL_CLASS_B_STATS_INFO = 0x00000004,
    HAL_GLOBAL_CLASS_C_STATS_INFO = 0x00000008,
    HAL_GLOBAL_CLASS_D_STATS_INFO = 0x00000010,
    HAL_PER_STA_STATS_INFO = 0x00000020
}

// BT-AMP events type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_amp_event_type {
    BTAMP_EVENT_CONNECTION_START,
    BTAMP_EVENT_CONNECTION_STOP,
    BTAMP_EVENT_CONNECTION_TERMINATED,

// This and beyond are invalid values
    BTAMP_EVENT_TYPE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE,
}

// PE Statistics
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pe_stats_mask {
    PE_SUMMARY_STATS_INFO = 0x00000001,
    PE_GLOBAL_CLASS_A_STATS_INFO = 0x00000002,
    PE_GLOBAL_CLASS_B_STATS_INFO = 0x00000004,
    PE_GLOBAL_CLASS_C_STATS_INFO = 0x00000008,
    PE_GLOBAL_CLASS_D_STATS_INFO = 0x00000010,
    PE_PER_STA_STATS_INFO = 0x00000020,

// This and beyond are invalid values
    PE_STATS_TYPE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

//
// Configuration Parameter IDs
//
pub const WCN36XX_HAL_CFG_STA_ID: c_int = 0;
pub const WCN36XX_HAL_CFG_CURRENT_TX_ANTENNA: c_int = 1;
pub const WCN36XX_HAL_CFG_CURRENT_RX_ANTENNA: c_int = 2;
pub const WCN36XX_HAL_CFG_LOW_GAIN_OVERRIDE: c_int = 3;
pub const WCN36XX_HAL_CFG_POWER_STATE_PER_CHAIN: c_int = 4;
pub const WCN36XX_HAL_CFG_CAL_PERIOD: c_int = 5;
pub const WCN36XX_HAL_CFG_CAL_CONTROL: c_int = 6;
pub const WCN36XX_HAL_CFG_PROXIMITY: c_int = 7;
pub const WCN36XX_HAL_CFG_NETWORK_DENSITY: c_int = 8;
pub const WCN36XX_HAL_CFG_MAX_MEDIUM_TIME: c_int = 9;
pub const WCN36XX_HAL_CFG_MAX_MPDUS_IN_AMPDU: c_int = 10;
pub const WCN36XX_HAL_CFG_RTS_THRESHOLD: c_int = 11;
pub const WCN36XX_HAL_CFG_SHORT_RETRY_LIMIT: c_int = 12;
pub const WCN36XX_HAL_CFG_LONG_RETRY_LIMIT: c_int = 13;
pub const WCN36XX_HAL_CFG_FRAGMENTATION_THRESHOLD: c_int = 14;
pub const WCN36XX_HAL_CFG_DYNAMIC_THRESHOLD_ZERO: c_int = 15;
pub const WCN36XX_HAL_CFG_DYNAMIC_THRESHOLD_ONE: c_int = 16;
pub const WCN36XX_HAL_CFG_DYNAMIC_THRESHOLD_TWO: c_int = 17;
pub const WCN36XX_HAL_CFG_FIXED_RATE: c_int = 18;
pub const WCN36XX_HAL_CFG_RETRYRATE_POLICY: c_int = 19;
pub const WCN36XX_HAL_CFG_RETRYRATE_SECONDARY: c_int = 20;
pub const WCN36XX_HAL_CFG_RETRYRATE_TERTIARY: c_int = 21;
pub const WCN36XX_HAL_CFG_FORCE_POLICY_PROTECTION: c_int = 22;
pub const WCN36XX_HAL_CFG_FIXED_RATE_MULTICAST_24GHZ: c_int = 23;
pub const WCN36XX_HAL_CFG_FIXED_RATE_MULTICAST_5GHZ: c_int = 24;
pub const WCN36XX_HAL_CFG_DEFAULT_RATE_INDEX_24GHZ: c_int = 25;
pub const WCN36XX_HAL_CFG_DEFAULT_RATE_INDEX_5GHZ: c_int = 26;
pub const WCN36XX_HAL_CFG_MAX_BA_SESSIONS: c_int = 27;
pub const WCN36XX_HAL_CFG_PS_DATA_INACTIVITY_TIMEOUT: c_int = 28;
pub const WCN36XX_HAL_CFG_PS_ENABLE_BCN_FILTER: c_int = 29;
pub const WCN36XX_HAL_CFG_PS_ENABLE_RSSI_MONITOR: c_int = 30;
pub const WCN36XX_HAL_CFG_NUM_BEACON_PER_RSSI_AVERAGE: c_int = 31;
pub const WCN36XX_HAL_CFG_STATS_PERIOD: c_int = 32;
pub const WCN36XX_HAL_CFG_CFP_MAX_DURATION: c_int = 33;
pub const WCN36XX_HAL_CFG_FRAME_TRANS_ENABLED: c_int = 34;
pub const WCN36XX_HAL_CFG_DTIM_PERIOD: c_int = 35;
pub const WCN36XX_HAL_CFG_EDCA_WMM_ACBK: c_int = 36;
pub const WCN36XX_HAL_CFG_EDCA_WMM_ACBE: c_int = 37;
pub const WCN36XX_HAL_CFG_EDCA_WMM_ACVO: c_int = 38;
pub const WCN36XX_HAL_CFG_EDCA_WMM_ACVI: c_int = 39;
pub const WCN36XX_HAL_CFG_BA_THRESHOLD_HIGH: c_int = 40;
pub const WCN36XX_HAL_CFG_MAX_BA_BUFFERS: c_int = 41;
pub const WCN36XX_HAL_CFG_RPE_POLLING_THRESHOLD: c_int = 42;
pub const WCN36XX_HAL_CFG_RPE_AGING_THRESHOLD_FOR_AC0_REG: c_int = 43;
pub const WCN36XX_HAL_CFG_RPE_AGING_THRESHOLD_FOR_AC1_REG: c_int = 44;
pub const WCN36XX_HAL_CFG_RPE_AGING_THRESHOLD_FOR_AC2_REG: c_int = 45;
pub const WCN36XX_HAL_CFG_RPE_AGING_THRESHOLD_FOR_AC3_REG: c_int = 46;
pub const WCN36XX_HAL_CFG_NO_OF_ONCHIP_REORDER_SESSIONS: c_int = 47;
pub const WCN36XX_HAL_CFG_PS_LISTEN_INTERVAL: c_int = 48;
pub const WCN36XX_HAL_CFG_PS_HEART_BEAT_THRESHOLD: c_int = 49;
pub const WCN36XX_HAL_CFG_PS_NTH_BEACON_FILTER: c_int = 50;
pub const WCN36XX_HAL_CFG_PS_MAX_PS_POLL: c_int = 51;
pub const WCN36XX_HAL_CFG_PS_MIN_RSSI_THRESHOLD: c_int = 52;
pub const WCN36XX_HAL_CFG_PS_RSSI_FILTER_PERIOD: c_int = 53;
pub const WCN36XX_HAL_CFG_PS_BROADCAST_FRAME_FILTER_ENABLE: c_int = 54;
pub const WCN36XX_HAL_CFG_PS_IGNORE_DTIM: c_int = 55;
pub const WCN36XX_HAL_CFG_PS_ENABLE_BCN_EARLY_TERM: c_int = 56;
pub const WCN36XX_HAL_CFG_DYNAMIC_PS_POLL_VALUE: c_int = 57;
pub const WCN36XX_HAL_CFG_PS_NULLDATA_AP_RESP_TIMEOUT: c_int = 58;
pub const WCN36XX_HAL_CFG_TELE_BCN_WAKEUP_EN: c_int = 59;
pub const WCN36XX_HAL_CFG_TELE_BCN_TRANS_LI: c_int = 60;
pub const WCN36XX_HAL_CFG_TELE_BCN_TRANS_LI_IDLE_BCNS: c_int = 61;
pub const WCN36XX_HAL_CFG_TELE_BCN_MAX_LI: c_int = 62;
pub const WCN36XX_HAL_CFG_TELE_BCN_MAX_LI_IDLE_BCNS: c_int = 63;
pub const WCN36XX_HAL_CFG_TX_PWR_CTRL_ENABLE: c_int = 64;
pub const WCN36XX_HAL_CFG_VALID_RADAR_CHANNEL_LIST: c_int = 65;
pub const WCN36XX_HAL_CFG_TX_POWER_24_20: c_int = 66;
pub const WCN36XX_HAL_CFG_TX_POWER_24_40: c_int = 67;
pub const WCN36XX_HAL_CFG_TX_POWER_50_20: c_int = 68;
pub const WCN36XX_HAL_CFG_TX_POWER_50_40: c_int = 69;
pub const WCN36XX_HAL_CFG_MCAST_BCAST_FILTER_SETTING: c_int = 70;
pub const WCN36XX_HAL_CFG_BCN_EARLY_TERM_WAKEUP_INTERVAL: c_int = 71;
pub const WCN36XX_HAL_CFG_MAX_TX_POWER_2_4: c_int = 72;
pub const WCN36XX_HAL_CFG_MAX_TX_POWER_5: c_int = 73;
pub const WCN36XX_HAL_CFG_INFRA_STA_KEEP_ALIVE_PERIOD: c_int = 74;
pub const WCN36XX_HAL_CFG_ENABLE_CLOSE_LOOP: c_int = 75;
pub const WCN36XX_HAL_CFG_BTC_EXECUTION_MODE: c_int = 76;
pub const WCN36XX_HAL_CFG_BTC_DHCP_BT_SLOTS_TO_BLOCK: c_int = 77;
pub const WCN36XX_HAL_CFG_BTC_A2DP_DHCP_BT_SUB_INTERVALS: c_int = 78;
pub const WCN36XX_HAL_CFG_PS_TX_INACTIVITY_TIMEOUT: c_int = 79;
pub const WCN36XX_HAL_CFG_WCNSS_API_VERSION: c_int = 80;
pub const WCN36XX_HAL_CFG_AP_KEEPALIVE_TIMEOUT: c_int = 81;
pub const WCN36XX_HAL_CFG_GO_KEEPALIVE_TIMEOUT: c_int = 82;
pub const WCN36XX_HAL_CFG_ENABLE_MC_ADDR_LIST: c_int = 83;
pub const WCN36XX_HAL_CFG_BTC_STATIC_LEN_INQ_BT: c_int = 84;
pub const WCN36XX_HAL_CFG_BTC_STATIC_LEN_PAGE_BT: c_int = 85;
pub const WCN36XX_HAL_CFG_BTC_STATIC_LEN_CONN_BT: c_int = 86;
pub const WCN36XX_HAL_CFG_BTC_STATIC_LEN_LE_BT: c_int = 87;
pub const WCN36XX_HAL_CFG_BTC_STATIC_LEN_INQ_WLAN: c_int = 88;
pub const WCN36XX_HAL_CFG_BTC_STATIC_LEN_PAGE_WLAN: c_int = 89;
pub const WCN36XX_HAL_CFG_BTC_STATIC_LEN_CONN_WLAN: c_int = 90;
pub const WCN36XX_HAL_CFG_BTC_STATIC_LEN_LE_WLAN: c_int = 91;
pub const WCN36XX_HAL_CFG_BTC_DYN_MAX_LEN_BT: c_int = 92;
pub const WCN36XX_HAL_CFG_BTC_DYN_MAX_LEN_WLAN: c_int = 93;
pub const WCN36XX_HAL_CFG_BTC_MAX_SCO_BLOCK_PERC: c_int = 94;
pub const WCN36XX_HAL_CFG_BTC_DHCP_PROT_ON_A2DP: c_int = 95;
pub const WCN36XX_HAL_CFG_BTC_DHCP_PROT_ON_SCO: c_int = 96;
pub const WCN36XX_HAL_CFG_ENABLE_UNICAST_FILTER: c_int = 97;
pub const WCN36XX_HAL_CFG_MAX_ASSOC_LIMIT: c_int = 98;
pub const WCN36XX_HAL_CFG_ENABLE_LPWR_IMG_TRANSITION: c_int = 99;
pub const WCN36XX_HAL_CFG_ENABLE_MCC_ADAPTIVE_SCHEDULER: c_int = 100;
pub const WCN36XX_HAL_CFG_ENABLE_DETECT_PS_SUPPORT: c_int = 101;
pub const WCN36XX_HAL_CFG_AP_LINK_MONITOR_TIMEOUT: c_int = 102;
pub const WCN36XX_HAL_CFG_BTC_DWELL_TIME_MULTIPLIER: c_int = 103;
pub const WCN36XX_HAL_CFG_ENABLE_TDLS_OXYGEN_MODE: c_int = 104;
pub const WCN36XX_HAL_CFG_ENABLE_NAT_KEEP_ALIVE_FILTER: c_int = 105;
pub const WCN36XX_HAL_CFG_ENABLE_SAP_OBSS_PROT: c_int = 106;
pub const WCN36XX_HAL_CFG_PSPOLL_DATA_RECEP_TIMEOUT: c_int = 107;
pub const WCN36XX_HAL_CFG_TDLS_PUAPSD_BUFFER_STA_CAPABLE: c_int = 108;
pub const WCN36XX_HAL_CFG_TDLS_PUAPSD_MASK: c_int = 109;
pub const WCN36XX_HAL_CFG_TDLS_PUAPSD_INACTIVITY_TIME: c_int = 110;
pub const WCN36XX_HAL_CFG_TDLS_PUAPSD_RX_FRAME_THRESHOLD: c_int = 111;
pub const WCN36XX_HAL_CFG_ANTENNA_DIVERSITY: c_int = 112;
pub const WCN36XX_HAL_CFG_ATH_DISABLE: c_int = 113;
pub const WCN36XX_HAL_CFG_FLEXCONNECT_POWER_FACTOR: c_int = 114;
pub const WCN36XX_HAL_CFG_ENABLE_ADAPTIVE_RX_DRAIN: c_int = 115;
pub const WCN36XX_HAL_CFG_TDLS_OFF_CHANNEL_CAPABLE: c_int = 116;
pub const WCN36XX_HAL_CFG_MWS_COEX_V1_WAN_FREQ: c_int = 117;
pub const WCN36XX_HAL_CFG_MWS_COEX_V1_WLAN_FREQ: c_int = 118;
pub const WCN36XX_HAL_CFG_MWS_COEX_V1_CONFIG: c_int = 119;
pub const WCN36XX_HAL_CFG_MWS_COEX_V1_CONFIG2: c_int = 120;
pub const WCN36XX_HAL_CFG_MWS_COEX_V2_WAN_FREQ: c_int = 121;
pub const WCN36XX_HAL_CFG_MWS_COEX_V2_WLAN_FREQ: c_int = 122;
pub const WCN36XX_HAL_CFG_MWS_COEX_V2_CONFIG: c_int = 123;
pub const WCN36XX_HAL_CFG_MWS_COEX_V2_CONFIG2: c_int = 124;
pub const WCN36XX_HAL_CFG_MWS_COEX_V3_WAN_FREQ: c_int = 125;
pub const WCN36XX_HAL_CFG_MWS_COEX_V3_WLAN_FREQ: c_int = 126;
pub const WCN36XX_HAL_CFG_MWS_COEX_V3_CONFIG: c_int = 127;
pub const WCN36XX_HAL_CFG_MWS_COEX_V3_CONFIG2: c_int = 128;
pub const WCN36XX_HAL_CFG_MWS_COEX_V4_WAN_FREQ: c_int = 129;
pub const WCN36XX_HAL_CFG_MWS_COEX_V4_WLAN_FREQ: c_int = 130;
pub const WCN36XX_HAL_CFG_MWS_COEX_V4_CONFIG: c_int = 131;
pub const WCN36XX_HAL_CFG_MWS_COEX_V4_CONFIG2: c_int = 132;
pub const WCN36XX_HAL_CFG_MWS_COEX_V5_WAN_FREQ: c_int = 133;
pub const WCN36XX_HAL_CFG_MWS_COEX_V5_WLAN_FREQ: c_int = 134;
pub const WCN36XX_HAL_CFG_MWS_COEX_V5_CONFIG: c_int = 135;
pub const WCN36XX_HAL_CFG_MWS_COEX_V5_CONFIG2: c_int = 136;
pub const WCN36XX_HAL_CFG_MWS_COEX_V6_WAN_FREQ: c_int = 137;
pub const WCN36XX_HAL_CFG_MWS_COEX_V6_WLAN_FREQ: c_int = 138;
pub const WCN36XX_HAL_CFG_MWS_COEX_V6_CONFIG: c_int = 139;
pub const WCN36XX_HAL_CFG_MWS_COEX_V6_CONFIG2: c_int = 140;
pub const WCN36XX_HAL_CFG_MWS_COEX_V7_WAN_FREQ: c_int = 141;
pub const WCN36XX_HAL_CFG_MWS_COEX_V7_WLAN_FREQ: c_int = 142;
pub const WCN36XX_HAL_CFG_MWS_COEX_V7_CONFIG: c_int = 143;
pub const WCN36XX_HAL_CFG_MWS_COEX_V7_CONFIG2: c_int = 144;
pub const WCN36XX_HAL_CFG_MWS_COEX_V8_WAN_FREQ: c_int = 145;
pub const WCN36XX_HAL_CFG_MWS_COEX_V8_WLAN_FREQ: c_int = 146;
pub const WCN36XX_HAL_CFG_MWS_COEX_V8_CONFIG: c_int = 147;
pub const WCN36XX_HAL_CFG_MWS_COEX_V8_CONFIG2: c_int = 148;
pub const WCN36XX_HAL_CFG_MWS_COEX_V9_WAN_FREQ: c_int = 149;
pub const WCN36XX_HAL_CFG_MWS_COEX_V9_WLAN_FREQ: c_int = 150;
pub const WCN36XX_HAL_CFG_MWS_COEX_V9_CONFIG: c_int = 151;
pub const WCN36XX_HAL_CFG_MWS_COEX_V9_CONFIG2: c_int = 152;
pub const WCN36XX_HAL_CFG_MWS_COEX_V10_WAN_FREQ: c_int = 153;
pub const WCN36XX_HAL_CFG_MWS_COEX_V10_WLAN_FREQ: c_int = 154;
pub const WCN36XX_HAL_CFG_MWS_COEX_V10_CONFIG: c_int = 155;
pub const WCN36XX_HAL_CFG_MWS_COEX_V10_CONFIG2: c_int = 156;
pub const WCN36XX_HAL_CFG_MWS_COEX_MODEM_BACKOFF: c_int = 157;
pub const WCN36XX_HAL_CFG_MWS_COEX_CONFIG1: c_int = 158;
pub const WCN36XX_HAL_CFG_MWS_COEX_CONFIG2: c_int = 159;
pub const WCN36XX_HAL_CFG_MWS_COEX_CONFIG3: c_int = 160;
pub const WCN36XX_HAL_CFG_MWS_COEX_CONFIG4: c_int = 161;
pub const WCN36XX_HAL_CFG_MWS_COEX_CONFIG5: c_int = 162;
pub const WCN36XX_HAL_CFG_MWS_COEX_CONFIG6: c_int = 163;
pub const WCN36XX_HAL_CFG_SAR_POWER_BACKOFF: c_int = 164;
pub const WCN36XX_HAL_CFG_GO_LINK_MONITOR_TIMEOUT: c_int = 165;
pub const WCN36XX_HAL_CFG_BTC_STATIC_OPP_WLAN_ACTIVE_WLAN_LEN: c_int = 166;
pub const WCN36XX_HAL_CFG_BTC_STATIC_OPP_WLAN_ACTIVE_BT_LEN: c_int = 167;
pub const WCN36XX_HAL_CFG_BTC_SAP_STATIC_OPP_ACTIVE_WLAN_LEN: c_int = 168;
pub const WCN36XX_HAL_CFG_BTC_SAP_STATIC_OPP_ACTIVE_BT_LEN: c_int = 169;
pub const WCN36XX_HAL_CFG_RMC_FIXED_RATE: c_int = 170;
pub const WCN36XX_HAL_CFG_ASD_PROBE_INTERVAL: c_int = 171;
pub const WCN36XX_HAL_CFG_ASD_TRIGGER_THRESHOLD: c_int = 172;
pub const WCN36XX_HAL_CFG_ASD_RTT_RSSI_HYST_THRESHOLD: c_int = 173;
pub const WCN36XX_HAL_CFG_BTC_CTS2S_ON_STA_DURING_SCO: c_int = 174;
pub const WCN36XX_HAL_CFG_SHORT_PREAMBLE: c_int = 175;
pub const WCN36XX_HAL_CFG_SHORT_SLOT_TIME: c_int = 176;
pub const WCN36XX_HAL_CFG_DELAYED_BA: c_int = 177;
pub const WCN36XX_HAL_CFG_IMMEDIATE_BA: c_int = 178;
pub const WCN36XX_HAL_CFG_DOT11_MODE: c_int = 179;
pub const WCN36XX_HAL_CFG_HT_CAPS: c_int = 180;
pub const WCN36XX_HAL_CFG_AMPDU_PARAMS: c_int = 181;
pub const WCN36XX_HAL_CFG_TX_BF_INFO: c_int = 182;
pub const WCN36XX_HAL_CFG_ASC_CAP_INFO: c_int = 183;
pub const WCN36XX_HAL_CFG_EXT_HT_CAPS: c_int = 184;
pub const WCN36XX_HAL_CFG_QOS_ENABLED: c_int = 185;
pub const WCN36XX_HAL_CFG_WME_ENABLED: c_int = 186;
pub const WCN36XX_HAL_CFG_WSM_ENABLED: c_int = 187;
pub const WCN36XX_HAL_CFG_WMM_ENABLED: c_int = 188;
pub const WCN36XX_HAL_CFG_UAPSD_PER_AC_BITMASK: c_int = 189;
pub const WCN36XX_HAL_CFG_MCS_RATES: c_int = 190;
pub const WCN36XX_HAL_CFG_VHT_CAPS: c_int = 191;
pub const WCN36XX_HAL_CFG_VHT_RX_SUPP_MCS: c_int = 192;
pub const WCN36XX_HAL_CFG_VHT_TX_SUPP_MCS: c_int = 193;
pub const WCN36XX_HAL_CFG_RA_FILTER_ENABLE: c_int = 194;
pub const WCN36XX_HAL_CFG_RA_RATE_LIMIT_INTERVAL: c_int = 195;
pub const WCN36XX_HAL_CFG_BTC_FATAL_HID_NSNIFF_BLK: c_int = 196;
pub const WCN36XX_HAL_CFG_BTC_CRITICAL_HID_NSNIFF_BLK: c_int = 197;
pub const WCN36XX_HAL_CFG_BTC_DYN_A2DP_TX_QUEUE_THOLD: c_int = 198;
pub const WCN36XX_HAL_CFG_BTC_DYN_OPP_TX_QUEUE_THOLD: c_int = 199;
pub const WCN36XX_HAL_CFG_LINK_FAIL_TIMEOUT: c_int = 200;
pub const WCN36XX_HAL_CFG_MAX_UAPSD_CONSEC_SP: c_int = 201;
pub const WCN36XX_HAL_CFG_MAX_UAPSD_CONSEC_RX_CNT: c_int = 202;
pub const WCN36XX_HAL_CFG_MAX_UAPSD_CONSEC_TX_CNT: c_int = 203;
pub const WCN36XX_HAL_CFG_MAX_UAPSD_CONSEC_RX_CNT_MEAS_WINDOW: c_int = 204;
pub const WCN36XX_HAL_CFG_MAX_UAPSD_CONSEC_TX_CNT_MEAS_WINDOW: c_int = 205;
pub const WCN36XX_HAL_CFG_MAX_PSPOLL_IN_WMM_UAPSD_PS_MODE: c_int = 206;
pub const WCN36XX_HAL_CFG_MAX_UAPSD_INACTIVITY_INTERVALS: c_int = 207;
pub const WCN36XX_HAL_CFG_ENABLE_DYNAMIC_WMMPS: c_int = 208;
pub const WCN36XX_HAL_CFG_BURST_MODE_BE_TXOP_VALUE: c_int = 209;
pub const WCN36XX_HAL_CFG_ENABLE_DYNAMIC_RA_START_RATE: c_int = 210;
pub const WCN36XX_HAL_CFG_BTC_FAST_WLAN_CONN_PREF: c_int = 211;
pub const WCN36XX_HAL_CFG_ENABLE_RTSCTS_HTVHT: c_int = 212;
pub const WCN36XX_HAL_CFG_BTC_STATIC_OPP_WLAN_IDLE_WLAN_LEN: c_int = 213;
pub const WCN36XX_HAL_CFG_BTC_STATIC_OPP_WLAN_IDLE_BT_LEN: c_int = 214;
pub const WCN36XX_HAL_CFG_LINK_FAIL_TX_CNT: c_int = 215;
pub const WCN36XX_HAL_CFG_TOGGLE_ARP_BDRATES: c_int = 216;
pub const WCN36XX_HAL_CFG_OPTIMIZE_CA_EVENT: c_int = 217;
pub const WCN36XX_HAL_CFG_EXT_SCAN_CONC_MODE: c_int = 218;
pub const WCN36XX_HAL_CFG_BAR_WAKEUP_HOST_DISABLE: c_int = 219;
pub const WCN36XX_HAL_CFG_SAR_BOFFSET_CORRECTION_ENABLE: c_int = 220;
pub const WCN36XX_HAL_CFG_UNITS_OF_BCN_WAIT_TIME: c_int = 221;
pub const WCN36XX_HAL_CFG_CONS_BCNMISS_COUNT: c_int = 222;
pub const WCN36XX_HAL_CFG_BTC_DISABLE_WLAN_LINK_CRITICAL: c_int = 223;
pub const WCN36XX_HAL_CFG_DISABLE_SCAN_DURING_SCO: c_int = 224;
pub const WCN36XX_HAL_CFG_TRIGGER_NULLFRAME_BEFORE_HB: c_int = 225;
pub const WCN36XX_HAL_CFG_ENABLE_POWERSAVE_OFFLOAD: c_int = 226;
pub const WCN36XX_HAL_CFG_MAX_PARAMS: c_int = 227;
// Specify the starting bitrate, 11B and 11A/G rates can be specified in
// multiples of 0.5 So for 5.5 mbps => 11. for MCS 0 - 7 rates, Bit 7 should
// set to 1 and Bit 0-6 represent the MCS index. so for MCS2 => 130.
// Any invalid non-zero value or unsupported rate will set the start rate
// to 6 mbps.
//
pub const WCN36XX_HAL_CFG_ENABLE_DYNAMIC_RA_START_RATE: c_int = 210;
// Message definitons - All the messages below need to be packed
// Definition for HAL API Version.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcnss_wlan_version {
    pub revision: u8,
    pub version: u8,
    pub minor: u8,
    pub major: u8,
    pub __packed: },
// Definition for Encryption Keys
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_keys {
    pub id: u8,
// 0 for multicast
    pub unicast: u8,
    pub direction: ani_key_direction,
// Usage is unknown
    pub rsc: [u8; WLAN_MAX_KEY_RSC_LEN],
// =1 for authenticator,=0 for supplicant
    pub pae_role: u8,
    pub length: u16,
    pub key: [u8; WCN36XX_HAL_MAC_MAX_KEY_LENGTH],
    pub __packed: },
//
// set_sta_key_params Moving here since it is shared by
// configbss/setstakey msgs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_sta_key_params {
// STA Index
    pub sta_index: u16,
// Encryption Type used with peer
    pub enc_type: ani_ed_type,
// STATIC/DYNAMIC - valid only for WEP
    pub wep_type: ani_wep_type,
// Default WEP key, valid only for static WEP, must between 0 and 3.
    pub def_wep_idx: u8,
// valid only for non-static WEP encyrptions
    pub key: [wcn36xx_hal_keys; WCN36XX_HAL_MAC_MAX_NUM_OF_DEFAULT_KEYS],
//
// Control for Replay Count, 1= Single TID based replay count on Tx
// 0 = Per TID based replay count on TX
//
    pub single_tid_rc: u8,
    pub __packed: },
// 4-byte control message header used by HAL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_msg_header {
    pub msg_type:16: wcn36xx_hal_host_msg_type,
    pub msg_version:16: wcn36xx_hal_host_msg_version,
    pub len: u32,
    pub __packed: },
// Config format required by HAL for each CFG item
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_cfg {
// Cfg Id. The Id required by HAL is exported by HAL
// in shared header file between UMAC and HAL.
    pub id: u16,
// Length of the Cfg. This parameter is used to go to next cfg
// in the TLV format.
    pub len: u16,
// Padding bytes for unaligned address's
    pub pad_bytes: u16,
// Reserve bytes for making cfgVal to align address
    pub reserve: u16,
// Following the uCfgLen field there should be a 'uCfgLen' bytes
// containing the uCfgValue ; u8 uCfgValue[uCfgLen]
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_start_parameters {
// Drive Type - Production or FTM etc
    pub type: driver_type,
// Length of the config buffer
    pub len: u32,
// Following this there is a TLV formatted buffer of length
// "len" bytes containing all config values.
// The TLV is expected to be formatted like this:
// 0           15            31           31+CFG_LEN-1        length-1
// |   CFG_ID   |   CFG_LEN   |   CFG_BODY    |  CFG_ID  |......|
//
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_start_req_msg {
// config buffer must start in TLV format just here
    pub header: wcn36xx_hal_msg_header,
    pub params: wcn36xx_hal_mac_start_parameters,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_start_rsp_params {
// success or failure
    pub status: u16,
// Max number of STA supported by the device
    pub stations: u8,
// Max number of BSS supported by the device
    pub bssids: u8,
// API Version
    pub version: wcnss_wlan_version,
// CRM build information
    pub crm_version: [u8; WCN36XX_HAL_VERSION_LENGTH],
// hardware/chipset/misc version information
    pub wlan_version: [u8; WCN36XX_HAL_VERSION_LENGTH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_start_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
    pub start_rsp_params: wcn36xx_hal_mac_start_rsp_params,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_stop_req_params {
// The reason for which the device is being stopped
    pub reason: wcn36xx_hal_stop_type,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_stop_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub stop_req_params: wcn36xx_hal_mac_stop_req_params,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_stop_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_update_cfg_req_msg {
//
// Note: The length specified in tHalUpdateCfgReqMsg messages should be
// header.msgLen = sizeof(tHalUpdateCfgReqMsg) + uConfigBufferLen
//
    pub header: wcn36xx_hal_msg_header,
// Length of the config buffer. Allows UMAC to update multiple CFGs
    pub len: u32,
//
// Following this there is a TLV formatted buffer of length
// "uConfigBufferLen" bytes containing all config values.
// The TLV is expected to be formatted like this:
// 0           15            31           31+CFG_LEN-1        length-1
// |   CFG_ID   |   CFG_LEN   |   CFG_BODY    |  CFG_ID  |......|
//
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_update_cfg_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
// Frame control field format (2 bytes)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_frame_ctl {

    pub subType:4: u8,
    pub type:2: u8,
    pub protVer:2: u8,
    pub order:1: u8,
    pub wep:1: u8,
    pub moreData:1: u8,
    pub powerMgmt:1: u8,
    pub retry:1: u8,
    pub moreFrag:1: u8,
    pub fromDS:1: u8,
    pub toDS:1: u8,

    pub protVer:2: u8,
    pub type:2: u8,
    pub subType:4: u8,
    pub toDS:1: u8,
    pub fromDS:1: u8,
    pub moreFrag:1: u8,
    pub retry:1: u8,
    pub powerMgmt:1: u8,
    pub moreData:1: u8,
    pub wep:1: u8,
    pub order:1: u8,

}

// Sequence control field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_seq_ctl {
    pub fragNum:4: u8,
    pub seqNumLo:4: u8,
    pub seqNumHi:8: u8,
}

// Management header format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_mgmt_hdr {
    pub fc: wcn36xx_hal_mac_frame_ctl,
    pub durationLo: u8,
    pub durationHi: u8,
    pub da: [u8; 6],
    pub sa: [u8; 6],
    pub bssId: [u8; 6],
    pub seqControl: wcn36xx_hal_mac_seq_ctl,
}

// FIXME: pronto v1 apparently has 4
pub const WCN36XX_HAL_NUM_BSSID: c_int = 2;
// Scan Entry to hold active BSS idx's
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_scan_entry {
    pub bss_index: [u8; WCN36XX_HAL_NUM_BSSID],
    pub active_bss_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_init_scan_req_msg {
    pub header: wcn36xx_hal_msg_header,
// LEARN - AP Role
    pub mode: wcn36xx_hal_sys_mode,
// BSSID of the BSS
    pub bssid: [u8; ETH_ALEN],
// Whether BSS needs to be notified
    pub notify: u8,
// Kind of frame to be used for notifying the BSS (Data Null, QoS
// Null, or CTS to Self). Must always be a valid frame type.
    pub frame_type: u8,
// UMAC has the option of passing the MAC frame to be used for
// notifying the BSS. If non-zero, HAL will use the MAC frame
// buffer pointed to by macMgmtHdr. If zero, HAL will generate the
// appropriate MAC frame based on frameType.
    pub frame_len: u8,
// Following the framelength there is a MAC frame buffer if
// frameLength is non-zero.
    pub mac_mgmt_hdr: wcn36xx_hal_mac_mgmt_hdr,
// Entry to hold number of active BSS idx's
    pub scan_entry: wcn36xx_hal_scan_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_init_scan_con_req_msg {
    pub header: wcn36xx_hal_msg_header,
// LEARN - AP Role
    pub mode: wcn36xx_hal_sys_mode,
// BSSID of the BSS
    pub bssid: [u8; ETH_ALEN],
// Whether BSS needs to be notified
    pub notify: u8,
// Kind of frame to be used for notifying the BSS (Data Null, QoS
// Null, or CTS to Self). Must always be a valid frame type.
    pub frame_type: u8,
// UMAC has the option of passing the MAC frame to be used for
// notifying the BSS. If non-zero, HAL will use the MAC frame
// buffer pointed to by macMgmtHdr. If zero, HAL will generate the
// appropriate MAC frame based on frameType.
    pub frame_length: u8,
// Following the framelength there is a MAC frame buffer if
// frameLength is non-zero.
    pub mac_mgmt_hdr: wcn36xx_hal_mac_mgmt_hdr,
// Entry to hold number of active BSS idx's
    pub scan_entry: wcn36xx_hal_scan_entry,
// Single NoA usage in Scanning
    pub use_noa: u8,
// Indicates the scan duration (in ms)
    pub scan_duration: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_init_scan_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_start_scan_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Indicates the channel to scan
    pub scan_channel: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_start_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub start_tsf: [u32; 2],
    pub tx_mgmt_power: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_end_scan_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Indicates the channel to stop scanning. Not used really. But
// retained for symmetry with "start Scan" message. It can also
// help in error check if needed.
    pub scan_channel: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_end_scan_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_finish_scan_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Identifies the operational state of the AP/STA
// LEARN - AP Role SCAN - STA Role
    pub mode: wcn36xx_hal_sys_mode,
// Operating channel to tune to.
    pub oper_channel: u8,
// Channel Bonding state If 20/40 MHz is operational, this will
// indicate the 40 MHz extension channel in combination with the
// control channel
    pub cb_state: phy_chan_bond_state,
// BSSID of the BSS
    pub bssid: [u8; ETH_ALEN],
// Whether BSS needs to be notified
    pub notify: u8,
// Kind of frame to be used for notifying the BSS (Data Null, QoS
// Null, or CTS to Self). Must always be a valid frame type.
    pub frame_type: u8,
// UMAC has the option of passing the MAC frame to be used for
// notifying the BSS. If non-zero, HAL will use the MAC frame
// buffer pointed to by macMgmtHdr. If zero, HAL will generate the
// appropriate MAC frame based on frameType.
    pub frame_length: u8,
// Following the framelength there is a MAC frame buffer if
// frameLength is non-zero.
    pub mac_mgmt_hdr: wcn36xx_hal_mac_mgmt_hdr,
// Entry to hold number of active BSS idx's
    pub scan_entry: wcn36xx_hal_scan_entry,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_finish_scan_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_scan_type {
    WCN36XX_HAL_SCAN_TYPE_PASSIVE = 0x00,
    WCN36XX_HAL_SCAN_TYPE_ACTIVE = WCN36XX_HAL_MAX_ENUM_SIZE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_ssid {
    pub length: u8,
    pub ssid: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_start_scan_offload_req_msg {
    pub header: wcn36xx_hal_msg_header,
// BSSIDs hot list
    pub num_bssid: u8,
    pub bssids: [u8; 4][ETH_ALEN],
// Directed probe-requests will be sent for listed SSIDs (max 10)
    pub num_ssid: u8,
    pub ssids: [wcn36xx_hal_mac_ssid; 10],
// Report AP with hidden ssid
    pub scan_hidden: u8,
// Self MAC address
    pub mac: [u8; ETH_ALEN],
// BSS type
    pub bss_type: wcn36xx_hal_bss_type,
// Scan type
    pub scan_type: wcn36xx_hal_scan_type,
// Minimum scanning time on each channel (ms)
    pub min_ch_time: u32,
// Maximum scanning time on each channel
    pub max_ch_time: u32,
// Is a p2p search
    pub p2p_search: u8,
// Channels to scan
    pub num_channel: u8,
    pub channels: [u8; 80],
// IE field
    pub ie_len: u16,
    pub ie: [u8; WCN36XX_MAX_SCAN_IE_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_start_scan_offload_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_scan_offload_ind_type {
// Scan has been started
    WCN36XX_HAL_SCAN_IND_STARTED = 0x01,
// Scan has been completed
    WCN36XX_HAL_SCAN_IND_COMPLETED = 0x02,
// Moved to foreign channel
    WCN36XX_HAL_SCAN_IND_FOREIGN_CHANNEL = 0x08,
// scan request has been dequeued
    WCN36XX_HAL_SCAN_IND_DEQUEUED = 0x10,
// preempted by other high priority scan
    WCN36XX_HAL_SCAN_IND_PREEMPTED = 0x20,
// scan start failed
    WCN36XX_HAL_SCAN_IND_FAILED = 0x40,
// scan restarted
    WCN36XX_HAL_SCAN_IND_RESTARTED = 0x80,
    WCN36XX_HAL_SCAN_IND_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_scan_offload_ind {
    pub header: wcn36xx_hal_msg_header,
    pub type: u32,
    pub channel_mhz: u32,
    pub scan_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_stop_scan_offload_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_stop_scan_offload_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
pub const WCN36XX_HAL_CHAN_REG1_MIN_PWR_MASK: c_uint = 0x000000ff;
pub const WCN36XX_HAL_CHAN_REG1_MAX_PWR_MASK: c_uint = 0x0000ff00;
pub const WCN36XX_HAL_CHAN_REG1_REG_PWR_MASK: c_uint = 0x00ff0000;
pub const WCN36XX_HAL_CHAN_REG1_CLASS_ID_MASK: c_uint = 0xff000000;
pub const WCN36XX_HAL_CHAN_REG2_ANT_GAIN_MASK: c_uint = 0x000000ff;

pub const WCN36XX_HAL_CHAN_INFO_PHY_11A: c_int = 0;
pub const WCN36XX_HAL_CHAN_INFO_PHY_11BG: c_int = 1;
pub const WCN36XX_HAL_DEFAULT_ANT_GAIN: c_int = 6;
pub const WCN36XX_HAL_DEFAULT_MIN_POWER: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_channel_param {
    pub mhz: u32,
    pub band_center_freq1: u32,
    pub band_center_freq2: u32,
    pub channel_info: u32,
    pub reg_info_1: u32,
    pub reg_info_2: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_update_channel_list_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub num_channel: u8,
    pub channels: [wcn36xx_hal_channel_param; 80],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_rate_index {
    HW_RATE_INDEX_1MBPS	= 0x82,
    HW_RATE_INDEX_2MBPS	= 0x84,
    HW_RATE_INDEX_5_5MBPS	= 0x8B,
    HW_RATE_INDEX_6MBPS	= 0x0C,
    HW_RATE_INDEX_9MBPS	= 0x12,
    HW_RATE_INDEX_11MBPS	= 0x96,
    HW_RATE_INDEX_12MBPS	= 0x18,
    HW_RATE_INDEX_18MBPS	= 0x24,
    HW_RATE_INDEX_24MBPS	= 0x30,
    HW_RATE_INDEX_36MBPS	= 0x48,
    HW_RATE_INDEX_48MBPS	= 0x60,
    HW_RATE_INDEX_54MBPS	= 0x6C
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_supported_rates {
//
// For Self STA Entry: this represents Self Mode.
// For Peer Stations, this represents the mode of the peer.
// On Station:
//
// --this mode is updated when PE adds the Self Entry.
//
// -- OR when PE sends 'ADD_BSS' message and station context in BSS
// is used to indicate the mode of the AP.
//
// ON AP:
//
// -- this mode is updated when PE sends 'ADD_BSS' and Sta entry
// for that BSS is used to indicate the self mode of the AP.
//
// -- OR when a station is associated, PE sends 'ADD_STA' message
// with this mode updated.
//
    pub op_rate_mode: sta_rate_mode,
// 11b, 11a and aniLegacyRates are IE rates which gives rate in
// unit of 500Kbps
    pub dsss_rates: [u16; WCN36XX_HAL_NUM_DSSS_RATES],
    pub ofdm_rates: [u16; WCN36XX_HAL_NUM_OFDM_RATES],
    pub legacy_rates: [u16; WCN36XX_HAL_NUM_POLARIS_RATES],
    pub reserved: u16,
// Taurus only supports 26 Titan Rates(no ESF/concat Rates will be
// supported) First 26 bits are reserved for those Titan rates and
// the last 4 bits(bit28-31) for Taurus, 2(bit26-27) bits are
// reserved.
// Titan and Taurus Rates
    pub enhanced_rate_bitmap: u32,
//
// 0-76 bits used, remaining reserved
// bits 0-15 and 32 should be set.
//
    pub supported_mcs_set: [u8; WCN36XX_HAL_MAC_MAX_SUPPORTED_MCS_SET],
//
// RX Highest Supported Data Rate defines the highest data
// rate that the STA is able to receive, in unites of 1Mbps.
// This value is derived from "Supported MCS Set field" inside
// the HT capability element.
//
    pub rx_highest_data_rate: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_sta_params {
// BSSID of STA
    pub bssid: [u8; ETH_ALEN],
// ASSOC ID, as assigned by UMAC
    pub aid: u16,
// STA entry Type: 0 - Self, 1 - Other/Peer, 2 - BSSID, 3 - BCAST
    pub type: u8,
// Short Preamble Supported.
    pub short_preamble_supported: u8,
// MAC Address of STA
    pub mac: [u8; ETH_ALEN],
// Listen interval of the STA
    pub listen_interval: u16,
// Support for 11e/WMM
    pub wmm_enabled: u8,
// 11n HT capable STA
    pub ht_capable: u8,
// TX Width Set: 0 - 20 MHz only, 1 - 20/40 MHz
    pub tx_channel_width_set: u8,
// RIFS mode 0 - NA, 1 - Allowed
    pub rifs_mode: u8,
// L-SIG TXOP Protection mechanism
    pub lsig_txop_protection: u8,
// Max Ampdu Size supported by STA. TPE programming.
    pub max_ampdu_size: u8,
// Max Ampdu density. Used by RA.  3 : 0~7 : 2^(11nAMPDUdensity -4)
    pub max_ampdu_density: u8,
// Max AMSDU size 1 : 3839 bytes, 0 : 7935 bytes
    pub max_amsdu_size: u8,
// Short GI support for 40Mhz packets
    pub sgi_40mhz: u8,
// Short GI support for 20Mhz packets
    pub sgi_20Mhz: u8,
// TODO move this parameter to the end for 3680
// These rates are the intersection of peer and self capabilities.
    pub supported_rates: wcn36xx_hal_supported_rates,
// Robust Management Frame (RMF) enabled/disabled
    pub rmf: u8,
// The unicast encryption type in the association
    pub encrypt_type: u32,
// HAL should update the existing STA entry, if this flag is set. UMAC
    pub action: u8,
// U-APSD Flags: 1b per AC.  Encoded as follows:
    pub uapsd: u8,
// Max SP Length
    pub max_sp_len: u8,
// 11n Green Field preamble support
    pub green_field_capable: u8,
// MIMO Power Save mode
    pub mimo_ps: wcn36xx_hal_ht_mimo_state,
// Delayed BA Support
    pub delayed_ba_support: u8,
// Max AMPDU duration in 32us
    pub max_ampdu_duration: u8,
// HT STA should set it to 1 if it is enabled in BSS. HT STA should
// set it to 0 if AP does not support it. This indication is sent
// to HAL and HAL uses this flag to pickup up appropriate 40Mhz
// rates.
    pub dsss_cck_mode_40mhz: u8,
// Valid STA Idx when action=Update. Set to 0xFF when invalid!
// Retained for backward compalibity with existing HAL code
    pub sta_index: u8,
// BSSID of BSS to which station is associated. Set to 0xFF when
// invalid. Retained for backward compalibity with existing HAL
// code
    pub bssid_index: u8,
    pub p2p: u8,
// TODO add this parameter for 3680.
// Reserved to align next field on a dword boundary
// u8 reserved;
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_sta_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub sta_params: wcn36xx_hal_config_sta_params,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_supported_rates_v1 {
// For Self STA Entry: this represents Self Mode.
// For Peer Stations, this represents the mode of the peer.
// On Station:
//
// --this mode is updated when PE adds the Self Entry.
//
// -- OR when PE sends 'ADD_BSS' message and station context in BSS
// is used to indicate the mode of the AP.
//
// ON AP:
//
// -- this mode is updated when PE sends 'ADD_BSS' and Sta entry
// for that BSS is used to indicate the self mode of the AP.
//
// -- OR when a station is associated, PE sends 'ADD_STA' message
// with this mode updated.
//
    pub op_rate_mode: sta_rate_mode,
// 11b, 11a and aniLegacyRates are IE rates which gives rate in
// unit of 500Kbps
//
    pub dsss_rates: [u16; WCN36XX_HAL_NUM_DSSS_RATES],
    pub ofdm_rates: [u16; WCN36XX_HAL_NUM_OFDM_RATES],
    pub legacy_rates: [u16; WCN36XX_HAL_NUM_POLARIS_RATES],
    pub reserved: u16,
// Taurus only supports 26 Titan Rates(no ESF/concat Rates will be
// supported) First 26 bits are reserved for those Titan rates and
// the last 4 bits(bit28-31) for Taurus, 2(bit26-27) bits are
// reserved
// Titan and Taurus Rates
//
    pub enhanced_rate_bitmap: u32,
// 0-76 bits used, remaining reserved
// bits 0-15 and 32 should be set.
//
    pub supported_mcs_set: [u8; WCN36XX_HAL_MAC_MAX_SUPPORTED_MCS_SET],
// RX Highest Supported Data Rate defines the highest data
// rate that the STA is able to receive, in unites of 1Mbps.
// This value is derived from "Supported MCS Set field" inside
// the HT capability element.
//
    pub rx_highest_data_rate: u16,
// Indicates the Maximum MCS that can be received for each spatial
// stream.
//
    pub vht_rx_mcs_map: u16,
// Indicates the highest VHT data rate that the STA is able to
// receive.
//
    pub vht_rx_highest_data_rate: u16,
// Indicates the Maximum MCS that can be transmitted for each spatial
// stream.
//
    pub vht_tx_mcs_map: u16,
// Indicates the highest VHT data rate that the STA is able to
// transmit.
//
    pub vht_tx_highest_data_rate: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_sta_params_v1 {
// BSSID of STA
    pub bssid: [u8; ETH_ALEN],
// ASSOC ID, as assigned by UMAC
    pub aid: u16,
// STA entry Type: 0 - Self, 1 - Other/Peer, 2 - BSSID, 3 - BCAST
    pub type: u8,
// Short Preamble Supported.
    pub short_preamble_supported: u8,
// MAC Address of STA
    pub mac: [u8; ETH_ALEN],
// Listen interval of the STA
    pub listen_interval: u16,
// Support for 11e/WMM
    pub wmm_enabled: u8,
// 11n HT capable STA
    pub ht_capable: u8,
// TX Width Set: 0 - 20 MHz only, 1 - 20/40 MHz
    pub tx_channel_width_set: u8,
// RIFS mode 0 - NA, 1 - Allowed
    pub rifs_mode: u8,
// L-SIG TXOP Protection mechanism
    pub lsig_txop_protection: u8,
// Max Ampdu Size supported by STA. TPE programming.
    pub max_ampdu_size: u8,
// Max Ampdu density. Used by RA.  3 : 0~7 : 2^(11nAMPDUdensity -4)
    pub max_ampdu_density: u8,
// Max AMSDU size 1 : 3839 bytes, 0 : 7935 bytes
    pub max_amsdu_size: u8,
// Short GI support for 40Mhz packets
    pub sgi_40mhz: u8,
// Short GI support for 20Mhz packets
    pub sgi_20Mhz: u8,
// Robust Management Frame (RMF) enabled/disabled
    pub rmf: u8,
// The unicast encryption type in the association
    pub encrypt_type: u32,
// HAL should update the existing STA entry, if this flag is set. UMAC
    pub action: u8,
// U-APSD Flags: 1b per AC.  Encoded as follows:
    pub uapsd: u8,
// Max SP Length
    pub max_sp_len: u8,
// 11n Green Field preamble support
    pub green_field_capable: u8,
// MIMO Power Save mode
    pub mimo_ps: wcn36xx_hal_ht_mimo_state,
// Delayed BA Support
    pub delayed_ba_support: u8,
// Max AMPDU duration in 32us
    pub max_ampdu_duration: u8,
// HT STA should set it to 1 if it is enabled in BSS. HT STA should
// set it to 0 if AP does not support it. This indication is sent
// to HAL and HAL uses this flag to pickup up appropriate 40Mhz
// rates.
    pub dsss_cck_mode_40mhz: u8,
// Valid STA Idx when action=Update. Set to 0xFF when invalid!
// Retained for backward compalibity with existing HAL code
    pub sta_index: u8,
// BSSID of BSS to which station is associated. Set to 0xFF when
// invalid. Retained for backward compalibity with existing HAL
// code
    pub bssid_index: u8,
    pub p2p: u8,
// Reserved to align next field on a dword boundary
    pub ht_ldpc_enabled:1: u8,
    pub vht_ldpc_enabled:1: u8,
    pub vht_tx_bf_enabled:1: u8,
    pub vht_tx_mu_beamformee_capable:1: u8,
    pub reserved:4: u8,
// These rates are the intersection of peer and self capabilities.
    pub supported_rates: wcn36xx_hal_supported_rates_v1,
    pub vht_capable: u8,
    pub vht_tx_channel_width_set: u8,
    pub __packed: },
pub const WCN36XX_DIFF_STA_PARAMS_V1_NOVHT: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_sta_req_msg_v1 {
    pub header: wcn36xx_hal_msg_header,
    pub sta_params: wcn36xx_hal_config_sta_params_v1,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_sta_rsp_params {
// success or failure
    pub status: u32,
// Station index; valid only when 'status' field value SUCCESS
    pub sta_index: u8,
// BSSID Index of BSS to which the station is associated
    pub bssid_index: u8,
// DPU Index for PTK
    pub dpu_index: u8,
// DPU Index for GTK
    pub bcast_dpu_index: u8,
// DPU Index for IGTK
    pub bcast_mgmt_dpu_idx: u8,
// PTK DPU signature
    pub uc_ucast_sig: u8,
// GTK DPU isignature
    pub uc_bcast_sig: u8,
// IGTK DPU signature
    pub uc_mgmt_sig: u8,
    pub p2p: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_sta_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
    pub params: config_sta_rsp_params,
    pub __packed: },
// Delete STA Request message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_delete_sta_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Index of STA to delete
    pub sta_index: u8,
    pub __packed: },
// Delete STA Response message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_delete_sta_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
// Index of STA deleted
    pub sta_id: u8,
    pub __packed: },
// 12 Bytes long because this structure can be used to represent rate and
// extended rate set IEs. The parser assume this to be at least 12
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rate_set {
    pub num_rates: u8,
    pub rate: [u8; WCN36XX_HAL_MAC_RATESET_EID_MAX],
    pub __packed: },
// access category record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_aci_aifsn {

    pub rsvd:1: u8,
    pub aci:2: u8,
    pub acm:1: u8,
    pub aifsn:4: u8,

    pub aifsn:4: u8,
    pub acm:1: u8,
    pub aci:2: u8,
    pub rsvd:1: u8,

    pub __packed: },
// contention window size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_mac_cw {

    pub max:4: u8,
    pub min:4: u8,

    pub min:4: u8,
    pub max:4: u8,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_edca_param_record {
    pub aci: wcn36xx_hal_aci_aifsn,
    pub cw: wcn36xx_hal_mac_cw,
    pub txop_limit: u16,
    pub __packed: },
// Concurrency role. These are generic IDs that identify the various roles
// in the software system.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_con_mode {
    WCN36XX_HAL_STA_MODE = 0,

// to support softAp mode . This is misleading.
    It means AP MODE only. */
    WCN36XX_HAL_STA_SAP_MODE = 1,

    WCN36XX_HAL_P2P_CLIENT_MODE,
    WCN36XX_HAL_P2P_GO_MODE,
    WCN36XX_HAL_MONITOR_MODE,
}

// This is a bit pattern to be set for each mode
// bit 0 - sta mode
// bit 1 - ap mode
// bit 2 - p2p client mode
// bit 3 - p2p go mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_concurrency_mode {
    HAL_STA = 1,
    HAL_SAP = 2,

// to support sta, softAp  mode . This means STA+AP mode
    HAL_STA_SAP = 3,

    HAL_P2P_CLIENT = 4,
    HAL_P2P_GO = 8,
    HAL_MAX_CONCURRENCY_PERSONA = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_bss_params {
// BSSID
    pub bssid: [u8; ETH_ALEN],
// Self Mac Address
    pub self_mac_addr: [u8; ETH_ALEN],
// BSS type
    pub bss_type: wcn36xx_hal_bss_type,
// Operational Mode: AP =0, STA = 1
    pub oper_mode: u8,
// Network Type
    pub nw_type: wcn36xx_hal_nw_type,
// Used to classify PURE_11G/11G_MIXED to program MTU
    pub short_slot_time_supported: u8,
// Co-exist with 11a STA
    pub lla_coexist: u8,
// Co-exist with 11b STA
    pub llb_coexist: u8,
// Co-exist with 11g STA
    pub llg_coexist: u8,
// Coexistence with 11n STA
    pub ht20_coexist: u8,
// Non GF coexist flag
    pub lln_non_gf_coexist: u8,
// TXOP protection support
    pub lsig_tx_op_protection_full_support: u8,
// RIFS mode
    pub rifs_mode: u8,
// Beacon Interval in TU
    pub beacon_interval: u16,
// DTIM period
    pub dtim_period: u8,
// TX Width Set: 0 - 20 MHz only, 1 - 20/40 MHz
    pub tx_channel_width_set: u8,
// Operating channel
    pub oper_channel: u8,
// Extension channel for channel bonding
    pub ext_channel: u8,
// Reserved to align next field on a dword boundary
    pub reserved: u8,
// TODO move sta to the end for 3680
// Context of the station being added in HW
// Add a STA entry for "itself" -
//
// On AP  - Add the AP itself in an "STA context"
//
// On STA - Add the AP to which this STA is joining in an
// "STA context"
//
    pub sta: wcn36xx_hal_config_sta_params,
// SSID of the BSS
    pub ssid: wcn36xx_hal_mac_ssid,
// HAL should update the existing BSS entry, if this flag is set.
// UMAC will set this flag in case of reassoc, where we want to
// resue the old BSSID and still return success 0 = Add, 1 =
// Update
    pub action: u8,
// MAC Rate Set
    pub rateset: wcn36xx_hal_rate_set,
// Enable/Disable HT capabilities of the BSS
    pub ht: u8,
// Enable/Disable OBSS protection
    pub obss_prot_enabled: u8,
// RMF enabled/disabled
    pub rmf: u8,
// HT Operating Mode operating mode of the 802.11n STA
    pub ht_oper_mode: wcn36xx_hal_ht_operating_mode,
// Dual CTS Protection: 0 - Unused, 1 - Used
    pub dual_cts_protection: u8,
// Probe Response Max retries
    pub max_probe_resp_retry_limit: u8,
// To Enable Hidden ssid
    pub hidden_ssid: u8,
// To Enable Disable FW Proxy Probe Resp
    pub proxy_probe_resp: u8,
// Boolean to indicate if EDCA params are valid. UMAC might not
// have valid EDCA params or might not desire to apply EDCA params
// during config BSS. 0 implies Not Valid ; Non-Zero implies
// valid
    pub edca_params_valid: u8,
// EDCA Parameters for Best Effort Access Category
    pub acbe: wcn36xx_hal_edca_param_record,
// EDCA Parameters forBackground Access Category
    pub acbk: wcn36xx_hal_edca_param_record,
// EDCA Parameters for Video Access Category
    pub acvi: wcn36xx_hal_edca_param_record,
// EDCA Parameters for Voice Access Category
    pub acvo: wcn36xx_hal_edca_param_record,
// Ext Bss Config Msg if set
    pub ext_set_sta_key_param_valid: u8,
// SetStaKeyParams for ext bss msg
    pub ext_set_sta_key_param: wcn36xx_hal_set_sta_key_params,
// Persona for the BSS can be STA,AP,GO,CLIENT value same as enum
// wcn36xx_hal_con_mode
    pub wcn36xx_hal_persona: u8,
    pub spectrum_mgt_enable: u8,
// HAL fills in the tx power used for mgmt frames in txMgmtPower
    pub tx_mgmt_power: i8,
// maxTxPower has max power to be used after applying the power
// constraint if any
    pub max_tx_power: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_bss_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bss_params: wcn36xx_hal_config_bss_params,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_bss_params_v1 {
// BSSID
    pub bssid: [u8; ETH_ALEN],
// Self Mac Address
    pub self_mac_addr: [u8; ETH_ALEN],
// BSS type
    pub bss_type: wcn36xx_hal_bss_type,
// Operational Mode: AP =0, STA = 1
    pub oper_mode: u8,
// Network Type
    pub nw_type: wcn36xx_hal_nw_type,
// Used to classify PURE_11G/11G_MIXED to program MTU
    pub short_slot_time_supported: u8,
// Co-exist with 11a STA
    pub lla_coexist: u8,
// Co-exist with 11b STA
    pub llb_coexist: u8,
// Co-exist with 11g STA
    pub llg_coexist: u8,
// Coexistence with 11n STA
    pub ht20_coexist: u8,
// Non GF coexist flag
    pub lln_non_gf_coexist: u8,
// TXOP protection support
    pub lsig_tx_op_protection_full_support: u8,
// RIFS mode
    pub rifs_mode: u8,
// Beacon Interval in TU
    pub beacon_interval: u16,
// DTIM period
    pub dtim_period: u8,
// TX Width Set: 0 - 20 MHz only, 1 - 20/40 MHz
    pub tx_channel_width_set: u8,
// Operating channel
    pub oper_channel: u8,
// Extension channel for channel bonding
    pub ext_channel: u8,
// Reserved to align next field on a dword boundary
    pub reserved: u8,
// SSID of the BSS
    pub ssid: wcn36xx_hal_mac_ssid,
// HAL should update the existing BSS entry, if this flag is set.
// UMAC will set this flag in case of reassoc, where we want to
// resue the old BSSID and still return success 0 = Add, 1 =
// Update
    pub action: u8,
// MAC Rate Set
    pub rateset: wcn36xx_hal_rate_set,
// Enable/Disable HT capabilities of the BSS
    pub ht: u8,
// Enable/Disable OBSS protection
    pub obss_prot_enabled: u8,
// RMF enabled/disabled
    pub rmf: u8,
// HT Operating Mode operating mode of the 802.11n STA
    pub ht_oper_mode: wcn36xx_hal_ht_operating_mode,
// Dual CTS Protection: 0 - Unused, 1 - Used
    pub dual_cts_protection: u8,
// Probe Response Max retries
    pub max_probe_resp_retry_limit: u8,
// To Enable Hidden ssid
    pub hidden_ssid: u8,
// To Enable Disable FW Proxy Probe Resp
    pub proxy_probe_resp: u8,
// Boolean to indicate if EDCA params are valid. UMAC might not
// have valid EDCA params or might not desire to apply EDCA params
// during config BSS. 0 implies Not Valid ; Non-Zero implies
// valid
    pub edca_params_valid: u8,
// EDCA Parameters for Best Effort Access Category
    pub acbe: wcn36xx_hal_edca_param_record,
// EDCA Parameters forBackground Access Category
    pub acbk: wcn36xx_hal_edca_param_record,
// EDCA Parameters for Video Access Category
    pub acvi: wcn36xx_hal_edca_param_record,
// EDCA Parameters for Voice Access Category
    pub acvo: wcn36xx_hal_edca_param_record,
// Ext Bss Config Msg if set
    pub ext_set_sta_key_param_valid: u8,
// SetStaKeyParams for ext bss msg
    pub ext_set_sta_key_param: wcn36xx_hal_set_sta_key_params,
// Persona for the BSS can be STA,AP,GO,CLIENT value same as enum
// wcn36xx_hal_con_mode
    pub wcn36xx_hal_persona: u8,
    pub spectrum_mgt_enable: u8,
// HAL fills in the tx power used for mgmt frames in txMgmtPower
    pub tx_mgmt_power: i8,
// maxTxPower has max power to be used after applying the power
// constraint if any
    pub max_tx_power: i8,
// Context of the station being added in HW
// Add a STA entry for "itself" -
//
// On AP  - Add the AP itself in an "STA context"
//
// On STA - Add the AP to which this STA is joining in an
// "STA context"
//
    pub sta: wcn36xx_hal_config_sta_params_v1,
    pub vht_capable: u8,
    pub vht_tx_channel_width_set: u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_bss_req_msg_v1 {
    pub header: wcn36xx_hal_msg_header,
    pub bss_params: wcn36xx_hal_config_bss_params_v1,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_bss_rsp_params {
// Success or Failure
    pub status: u32,
// BSS index allocated by HAL
    pub bss_index: u8,
// DPU descriptor index for PTK
    pub dpu_desc_index: u8,
// PTK DPU signature
    pub ucast_dpu_signature: u8,
// DPU descriptor index for GTK
    pub bcast_dpu_desc_indx: u8,
// GTK DPU signature
    pub bcast_dpu_signature: u8,
// DPU descriptor for IGTK
    pub mgmt_dpu_desc_index: u8,
// IGTK DPU signature
    pub mgmt_dpu_signature: u8,
// Station Index for BSS entry
    pub bss_sta_index: u8,
// Self station index for this BSS
    pub bss_self_sta_index: u8,
// Bcast station for buffering bcast frames in AP role
    pub bss_bcast_sta_idx: u8,
// MAC Address of STA(PEER/SELF) in staContext of configBSSReq
    pub mac: [u8; ETH_ALEN],
// HAL fills in the tx power used for mgmt frames in this field.
    pub tx_mgmt_power: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_config_bss_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bss_rsp_params: wcn36xx_hal_config_bss_rsp_params,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_delete_bss_req_msg {
    pub header: wcn36xx_hal_msg_header,
// BSS index to be deleted
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_delete_bss_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// Success or Failure
    pub status: u32,
// BSS index that has been deleted
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_join_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Indicates the BSSID to which STA is going to associate
    pub bssid: [u8; ETH_ALEN],
// Indicates the channel to switch to.
    pub channel: u8,
// Self STA MAC
    pub self_sta_mac_addr: [u8; ETH_ALEN],
// Local power constraint
    pub local_power_constraint: u8,
// Secondary channel offset
    pub secondary_channel_offset: phy_chan_bond_state,
// link State
    pub link_state: wcn36xx_hal_link_state,
// Max TX power
    pub max_tx_power: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_join_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
// HAL fills in the tx power used for mgmt frames in this field
    pub tx_mgmt_power: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct post_assoc_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub sta_params: wcn36xx_hal_config_sta_params,
    pub bss_params: wcn36xx_hal_config_bss_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct post_assoc_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
    pub sta_rsp_params: config_sta_rsp_params,
    pub bss_rsp_params: wcn36xx_hal_config_bss_rsp_params,
}

// This is used to create a set of WEP keys for a given BSS.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_bss_key_req_msg {
    pub header: wcn36xx_hal_msg_header,
// BSS Index of the BSS
    pub bss_idx: u8,
// Encryption Type used with peer
    pub enc_type: ani_ed_type,
// Number of keys
    pub num_keys: u8,
// Array of keys.
    pub keys: [wcn36xx_hal_keys; WCN36XX_HAL_MAC_MAX_NUM_OF_DEFAULT_KEYS],
// Control for Replay Count, 1= Single TID based replay count on Tx
// 0 = Per TID based replay count on TX
    pub single_tid_rc: u8,
    pub __packed: },
// tagged version of set bss key
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_bss_key_req_msg_tagged {
    pub Msg: wcn36xx_hal_set_bss_key_req_msg,
    pub tag: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_bss_key_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
//
// This is used  configure the key information on a given station.
// When the sec_type is WEP40 or WEP104, the def_wep_idx is used to locate
// a preconfigured key from a BSS the station associated with; otherwise
// a new key descriptor is created based on the key field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_sta_key_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub set_sta_key_params: wcn36xx_hal_set_sta_key_params,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_sta_key_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_remove_bss_key_req_msg {
    pub header: wcn36xx_hal_msg_header,
// BSS Index of the BSS
    pub bss_idx: u8,
// Encryption Type used with peer
    pub enc_type: ani_ed_type,
// Key Id
    pub key_id: u8,
// STATIC/DYNAMIC. Used in Nullifying in Key Descriptors for
// Static/Dynamic keys
    pub wep_type: ani_wep_type,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_remove_bss_key_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
//
// This is used by PE to Remove the key information on a given station.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_remove_sta_key_req_msg {
    pub header: wcn36xx_hal_msg_header,
// STA Index
    pub sta_idx: u16,
// Encryption Type used with peer
    pub enc_type: ani_ed_type,
// Key Id
    pub key_id: u8,
// Whether to invalidate the Broadcast key or Unicast key. In case
// of WEP, the same key is used for both broadcast and unicast.
    pub unicast: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_remove_sta_key_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },

pub const OEM_DATA_REQ_SIZE: c_int = 134;

pub const OEM_DATA_RSP_SIZE: c_int = 1968;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct start_oem_data_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub status: u32,
    pub self_mac_addr: tSirMacAddr,
    pub oem_data_req: [u8; OEM_DATA_REQ_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct start_oem_data_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
    pub oem_data_rsp: [u8; OEM_DATA_RSP_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_switch_channel_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Channel number
    pub channel_number: u8,
// Local power constraint
    pub local_power_constraint: u8,
// Secondary channel offset
    pub secondary_channel_offset: phy_chan_bond_state,
// HAL fills in the tx power used for mgmt frames in this field.
    pub tx_mgmt_power: u8,
// Max TX power
    pub max_tx_power: u8,
// Self STA MAC
    pub self_sta_mac_addr: [u8; ETH_ALEN],
// VO WIFI comment: BSSID needed to identify session. As the
// request has power constraints, this should be applied only to
// that session Since MTU timing and EDCA are sessionized, this
// struct needs to be sessionized and bssid needs to be out of the
// VOWifi feature flag V IMP: Keep bssId field at the end of this
// msg. It is used to mantain backward compatbility by way of
// ignoring if using new host/old FW or old host/new FW since it is
// at the end of this struct
//
    pub bssid: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_switch_channel_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// Status
    pub status: u32,
// Channel number - same as in request
    pub channel_number: u8,
// HAL fills in the tx power used for mgmt frames in this field
    pub tx_mgmt_power: u8,
// BSSID needed to identify session - same as in request
    pub bssid: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_process_ptt_msg_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Actual FTM Command body
    pub ptt_msg: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_process_ptt_msg_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// FTM Command response status
    pub ptt_msg_resp_status: u32,
// Actual FTM Command body
    pub ptt_msg: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_edca_params_req_msg {
    pub header: wcn36xx_hal_msg_header,
// BSS Index
    pub bss_index: u16,
// Best Effort
    pub acbe: wcn36xx_hal_edca_param_record,
// Background
    pub acbk: wcn36xx_hal_edca_param_record,
// Video
    pub acvi: wcn36xx_hal_edca_param_record,
// Voice
    pub acvo: wcn36xx_hal_edca_param_record,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_edca_params_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_stats_params {
// Index of STA to which the statistics
    pub sta_index: u16,
// Encryption mode
    pub enc_mode: u8,
// status
    pub status: u32,
// Statistics
    pub send_blocks: u32,
    pub recv_blocks: u32,
    pub replays: u32,
    pub mic_error_cnt: u8,
    pub prot_excl_cnt: u32,
    pub format_err_cnt: u16,
    pub un_decryptable_cnt: u16,
    pub decrypt_err_cnt: u32,
    pub decrypt_ok_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_stats_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Valid STA Idx for per STA stats request
    pub sta_id: u32,
// Categories of stats requested as specified in eHalStatsMask
    pub stats_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ani_summary_stats_info {
// Total number of packets(per AC) that were successfully
// transmitted with retries
    pub retry_cnt: [u32; 4],
// The number of MSDU packets and MMPDU frames per AC that the
// 802.11 station successfully transmitted after more than one
// retransmission attempt
    pub multiple_retry_cnt: [u32; 4],
// Total number of packets(per AC) that were successfully
// transmitted (with and without retries, including multi-cast,
// broadcast)
    pub tx_frm_cnt: [u32; 4],
// Total number of packets that were successfully received (after
// appropriate filter rules including multi-cast, broadcast)
    pub rx_frm_cnt: u32,
// Total number of duplicate frames received successfully
    pub frm_dup_cnt: u32,
// Total number packets(per AC) failed to transmit
    pub fail_cnt: [u32; 4],
// Total number of RTS/CTS sequence failures for transmission of a
// packet
    pub rts_fail_cnt: u32,
// Total number packets failed transmit because of no ACK from the
// remote entity
    pub ack_fail_cnt: u32,
// Total number of RTS/CTS sequence success for transmission of a
// packet
    pub rts_succ_cnt: u32,
// The sum of the receive error count and dropped-receive-buffer
// error count. HAL will provide this as a sum of (FCS error) +
// (Fail get BD/PDU in HW)
    pub rx_discard_cnt: u32,
//
// The receive error count. HAL will provide the RxP FCS error
// global counter.
    pub rx_error_cnt: u32,
// The sum of the transmit-directed byte count, transmit-multicast
// byte count and transmit-broadcast byte count. HAL will sum TPE
// UC/MC/BCAST global counters to provide this.
    pub tx_byte_cnt: u32,
}

// defines tx_rate_flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_rate_info {
// Legacy rates
    HAL_TX_RATE_LEGACY = 0x1,

// HT20 rates
    HAL_TX_RATE_HT20 = 0x2,

// HT40 rates
    HAL_TX_RATE_HT40 = 0x4,

// Rate with Short guard interval
    HAL_TX_RATE_SGI = 0x8,

// Rate with Long guard interval
    HAL_TX_RATE_LGI = 0x10,

// VHT rates
    HAL_TX_RATE_VHT20  = 0x20,
    HAL_TX_RATE_VHT40  = 0x40,
    HAL_TX_RATE_VHT80  = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ani_global_class_a_stats_info {
// The number of MPDU frames received by the 802.11 station for
// MSDU packets or MMPDU frames
    pub rx_frag_cnt: u32,
// The number of MPDU frames received by the 802.11 station for
// MSDU packets or MMPDU frames when a promiscuous packet filter
// was enabled
    pub promiscuous_rx_frag_cnt: u32,
// The receiver input sensitivity referenced to a FER of 8% at an
// MPDU length of 1024 bytes at the antenna connector. Each element
// of the array shall correspond to a supported rate and the order
// shall be the same as the supporteRates parameter.
    pub rx_input_sensitivity: u32,
// The maximum transmit power in dBm upto one decimal. for eg: if
// it is 10.5dBm, the value would be 105
    pub max_pwr: u32,
// Number of times the receiver failed to synchronize with the
// incoming signal after detecting the sync in the preamble of the
// transmitted PLCP protocol data unit.
    pub sync_fail_cnt: u32,
// Legacy transmit rate, in units of 500 kbit/sec, for the most
// recently transmitted frame
    pub tx_rate: u32,
// mcs index for HT20 and HT40 rates
    pub mcs_index: u32,
// to differentiate between HT20 and HT40 rates; short and long
// guard interval
    pub tx_rate_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ani_global_security_stats {
// The number of unencrypted received MPDU frames that the MAC
// layer discarded when the IEEE 802.11 dot11ExcludeUnencrypted
// management information base (MIB) object is enabled
    pub rx_wep_unencrypted_frm_cnt: u32,
// The number of received MSDU packets that the 802.11 station
// discarded because of MIC failures
    pub rx_mic_fail_cnt: u32,
// The number of encrypted MPDU frames that the 802.11 station
// failed to decrypt because of a TKIP ICV error
    pub tkip_icv_err: u32,
// The number of received MPDU frames that the 802.11 discarded
// because of an invalid AES-CCMP format
    pub aes_ccmp_format_err: u32,
// The number of received MPDU frames that the 802.11 station
// discarded because of the AES-CCMP replay protection procedure
    pub aes_ccmp_replay_cnt: u32,
// The number of received MPDU frames that the 802.11 station
// discarded because of errors detected by the AES-CCMP decryption
// algorithm
    pub aes_ccmp_decrpt_err: u32,
// The number of encrypted MPDU frames received for which a WEP
// decryption key was not available on the 802.11 station
    pub wep_undecryptable_cnt: u32,
// The number of encrypted MPDU frames that the 802.11 station
// failed to decrypt because of a WEP ICV error
    pub wep_icv_err: u32,
// The number of received encrypted packets that the 802.11 station
// successfully decrypted
    pub rx_decrypt_succ_cnt: u32,
// The number of encrypted packets that the 802.11 station failed
// to decrypt
    pub rx_decrypt_fail_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ani_global_class_b_stats_info {
    pub uc_stats: ani_global_security_stats,
    pub mc_bc_stats: ani_global_security_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ani_global_class_c_stats_info {
// This counter shall be incremented for a received A-MSDU frame
// with the stations MAC address in the address 1 field or an
// A-MSDU frame with a group address in the address 1 field
    pub rx_amsdu_cnt: u32,
// This counter shall be incremented when the MAC receives an AMPDU
// from the PHY
    pub rx_ampdu_cnt: u32,
// This counter shall be incremented when a Frame is transmitted
// only on the primary channel
    pub tx_20_frm_cnt: u32,
// This counter shall be incremented when a Frame is received only
// on the primary channel
    pub rx_20_frm_cnt: u32,
// This counter shall be incremented by the number of MPDUs
// received in the A-MPDU when an A-MPDU is received
    pub rx_mpdu_in_ampdu_cnt: u32,
// This counter shall be incremented when an MPDU delimiter has a
// CRC error when this is the first CRC error in the received AMPDU
// or when the previous delimiter has been decoded correctly
    pub ampdu_delimiter_crc_err: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ani_per_sta_stats_info {
// The number of MPDU frames that the 802.11 station transmitted
// and acknowledged through a received 802.11 ACK frame
    pub tx_frag_cnt: [u32; 4],
// This counter shall be incremented when an A-MPDU is transmitted
    pub tx_ampdu_cnt: u32,
// This counter shall increment by the number of MPDUs in the AMPDU
// when an A-MPDU is transmitted
    pub tx_mpdu_in_ampdu_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_stats_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// Success or Failure
    pub status: u32,
// STA Idx
    pub sta_index: u32,
// Categories of STATS being returned as per eHalStatsMask
    pub stats_mask: u32,
// message type is same as the request type
    pub msg_type: u16,
// length of the entire request, includes the pStatsBuf length too
    pub msg_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_link_state_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bssid: [u8; ETH_ALEN],
    pub state: wcn36xx_hal_link_state,
    pub self_mac_addr: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_link_state_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

// TSPEC Params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_ts_info_tfc {

    pub ackPolicy:2: u16,
    pub userPrio:3: u16,
    pub psb:1: u16,
    pub aggregation:1: u16,
    pub accessPolicy:2: u16,
    pub direction:2: u16,
    pub tsid:4: u16,
    pub trafficType:1: u16,

    pub trafficType:1: u16,
    pub tsid:4: u16,
    pub direction:2: u16,
    pub accessPolicy:2: u16,
    pub aggregation:1: u16,
    pub psb:1: u16,
    pub userPrio:3: u16,
    pub ackPolicy:2: u16,

}

// Flag to schedule the traffic type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_ts_info_sch {

    pub rsvd:7: u8,
    pub schedule:1: u8,

    pub schedule:1: u8,
    pub rsvd:7: u8,

}

// Traffic and scheduling info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_ts_info {
    pub traffic: wcn36xx_hal_ts_info_tfc,
    pub schedule: wcn36xx_hal_ts_info_sch,
}

// Information elements
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_tspec_ie {
    pub type: u8,
    pub length: u8,
    pub ts_info: wcn36xx_hal_ts_info,
    pub nom_msdu_size: u16,
    pub max_msdu_size: u16,
    pub min_svc_interval: u32,
    pub max_svc_interval: u32,
    pub inact_interval: u32,
    pub suspend_interval: u32,
    pub svc_start_time: u32,
    pub min_data_rate: u32,
    pub mean_data_rate: u32,
    pub peak_data_rate: u32,
    pub max_burst_sz: u32,
    pub delay_bound: u32,
    pub min_phy_rate: u32,
    pub surplus_bw: u16,
    pub medium_time: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct add_ts_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Station Index
    pub sta_index: u16,
// TSPEC handler uniquely identifying a TSPEC for a STA in a BSS
    pub tspec_index: u16,
// To program TPE with required parameters
    pub tspec: wcn36xx_hal_tspec_ie,
// U-APSD Flags: 1b per AC.  Encoded as follows:
    pub uapsd: u8,
// These parameters are for all the access categories
// Service Interval
    pub service_interval: [u32; WCN36XX_HAL_MAX_AC],
// Suspend Interval
    pub suspend_interval: [u32; WCN36XX_HAL_MAX_AC],
// Delay Interval
    pub delay_interval: [u32; WCN36XX_HAL_MAX_AC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct add_rs_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct del_ts_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Station Index
    pub sta_index: u16,
// TSPEC identifier uniquely identifying a TSPEC for a STA in a BSS
    pub tspec_index: u16,
// To lookup station id using the mac address
    pub bssid: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct del_ts_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

// End of TSpec Parameters
// Start of BLOCK ACK related Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_add_ba_session_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Station Index
    pub sta_index: u16,
// Peer MAC Address
    pub mac_addr: [u8; ETH_ALEN],
// ADDBA Action Frame dialog token
    pub dialog_token: u8,
// TID for which the BA is being setup
    pub tid: u8,
// 0 - Delayed BA (Not supported)
    pub policy: u8,
// Indicates the number of buffers for this TID (baTID)
    pub buffer_size: u16,
// BA timeout in TU's 0 means no timeout will occur
    pub timeout: u16,
// b0..b3 - Fragment Number - Always set to 0
    pub ssn: u16,
// ADDBA direction
    pub direction: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_add_ba_session_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
// Dialog token
    pub dialog_token: u8,
// TID for which the BA session has been setup
    pub ba_tid: u8,
// BA Buffer Size allocated for the current BA session
    pub ba_buffer_size: u8,
    pub ba_session_id: u8,
// Reordering Window buffer
    pub win_size: u8,
// Station Index to id the sta
    pub sta_index: u8,
// Starting Sequence Number
    pub ssn: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_add_ba_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Session Id
    pub session_id: u8,
// Reorder Window Size
    pub win_size: u8,
// Old FW 1.2.2.4 does not support this

    pub reordering_done_on_chip: u8,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_add_ba_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
// Dialog token
    pub dialog_token: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct add_ba_info {
    pub ba_enable:1: u16,
    pub starting_seq_num:12: u16,
    pub reserved:3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_trigger_ba_rsp_candidate {
    pub sta_addr: [u8; ETH_ALEN],
    pub ba_info: [add_ba_info; STACFG_MAX_TC],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_trigger_ba_req_candidate {
    pub sta_index: u8,
    pub tid_bitmap: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_trigger_ba_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Session Id
    pub session_id: u8,
// baCandidateCnt is followed by trigger BA
// Candidate List(tTriggerBaCandidate)
//
    pub candidate_cnt: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_trigger_ba_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// TO SUPPORT BT-AMP
    pub bssid: [u8; ETH_ALEN],
// success or failure
    pub status: u32,
// baCandidateCnt is followed by trigger BA
// Rsp Candidate List(tTriggerRspBaCandidate)
//
    pub candidate_cnt: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_del_ba_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Station Index
    pub sta_index: u16,
// TID for which the BA session is being deleted
    pub tid: u8,
// DELBA direction
    pub direction: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_del_ba_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_stats_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Traffic Id
    pub tid: u8,
    pub bssid: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsm_stats_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
// Uplink Packet Queue delay
    pub uplink_pkt_queue_delay: u16,
// Uplink Packet Queue delay histogram
    pub uplink_pkt_queue_delay_hist: [u16; 4],
// Uplink Packet Transmit delay
    pub uplink_pkt_tx_delay: u32,
// Uplink Packet loss
    pub uplink_pkt_loss: u16,
// Uplink Packet count
    pub uplink_pkt_count: u16,
// Roaming count
    pub roaming_count: u8,
// Roaming Delay
    pub roaming_delay: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_key_done_msg {
    pub header: wcn36xx_hal_msg_header,
// bssid of the keys
    pub bssidx: u8,
    pub enc_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_nv_img_download_req_msg {
// Note: The length specified in wcn36xx_hal_nv_img_download_req_msg
// messages should be
// header.len = sizeof(wcn36xx_hal_nv_img_download_req_msg) +
// nv_img_buffer_size
    pub header: wcn36xx_hal_msg_header,
// Fragment sequence number of the NV Image. Note that NV Image
// might not fit into one message due to size limitation of the SMD
// channel FIFO. UMAC can hence choose to chop the NV blob into
// multiple fragments starting with seqeunce number 0, 1, 2 etc.
// The last fragment MUST be indicated by marking the
// isLastFragment field to 1. Note that all the NV blobs would be
// concatenated together by HAL without any padding bytes in
// between.
    pub frag_number: u16,
// Is this the last fragment? When set to 1 it indicates that no
// more fragments will be sent by UMAC and HAL can concatenate all
// the NV blobs rcvd & proceed with the parsing. HAL would generate
// a WCN36XX_HAL_DOWNLOAD_NV_RSP to the WCN36XX_HAL_DOWNLOAD_NV_REQ
// after it receives each fragment
    pub last_fragment: u16,
// NV Image size (number of bytes)
    pub nv_img_buffer_size: u32,
// Following the 'nv_img_buffer_size', there should be
// nv_img_buffer_size bytes of NV Image i.e.
// u8[nv_img_buffer_size]
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_nv_img_download_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// Success or Failure. HAL would generate a
// WCN36XX_HAL_DOWNLOAD_NV_RSP after each fragment
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_nv_store_ind {
// Note: The length specified in tHalNvStoreInd messages should be
// header.msgLen = sizeof(tHalNvStoreInd) + nvBlobSize
    pub header: wcn36xx_hal_msg_header,
// NV Item
    pub table_id: u32,
// Size of NV Blob
    pub nv_blob_size: u32,
// Following the 'nvBlobSize', there should be nvBlobSize bytes of
// NV blob i.e. u8[nvBlobSize]
}

// End of Block Ack Related Parameters
pub const WCN36XX_HAL_CIPHER_SEQ_CTR_SIZE: c_int = 6;
// Definition for MIC failure indication MAC reports this each time a MIC
// failure occures on Rx TKIP packet
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mic_failure_ind_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bssid: [u8; ETH_ALEN],
// address used to compute MIC
    pub src_addr: [u8; ETH_ALEN],
// transmitter address
    pub ta_addr: [u8; ETH_ALEN],
    pub dst_addr: [u8; ETH_ALEN],
    pub multicast: u8,
// first byte of IV
    pub iv1: u8,
// second byte of IV
    pub key_id: u8,
// sequence number
    pub tsc: [u8; WCN36XX_HAL_CIPHER_SEQ_CTR_SIZE],
// receive address
    pub rx_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_vht_op_mode_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub op_mode: u16,
    pub sta_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_vht_op_mode_params_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_beacon_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bss_index: u8,
// shortPreamble mode. HAL should update all the STA rates when it
// receives this message
    pub short_preamble: u8,
// short Slot time.
    pub short_slot_time: u8,
// Beacon Interval
    pub beacon_interval: u16,
// Protection related
    pub lla_coexist: u8,
    pub llb_coexist: u8,
    pub llg_coexist: u8,
    pub ht20_coexist: u8,
    pub lln_non_gf_coexist: u8,
    pub lsig_tx_op_protection_full_support: u8,
    pub rifs_mode: u8,
    pub param_change_bitmap: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct update_beacon_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_send_beacon_req_msg {
    pub header: wcn36xx_hal_msg_header,
// length of the template + 6. Only qcom knows why
    pub beacon_length6: u32,
// length of the template.
    pub beacon_length: u32,
// Beacon data.
    pub sizeof(u32)]: u8 beacon[BEACON_TEMPLATE_SIZE -,
    pub bssid: [u8; ETH_ALEN],
// TIM IE offset from the beginning of the template.
    pub tim_ie_offset: u32,
// P2P IE offset from the begining of the template
    pub p2p_ie_offset: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct send_beacon_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enable_radar_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bssid: [u8; ETH_ALEN],
    pub channel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enable_radar_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// Link Parameters
    pub bssid: [u8; ETH_ALEN],
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radar_detect_intr_ind_msg {
    pub header: wcn36xx_hal_msg_header,
    pub radar_det_channel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radar_detect_ind_msg {
    pub header: wcn36xx_hal_msg_header,
// channel number in which the RADAR detected
    pub channel_number: u8,
// RADAR pulse width in usecond
    pub radar_pulse_width: u16,
// Number of RADAR pulses
    pub num_radar_pulse: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_get_tpc_report_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub sta: [u8; ETH_ALEN],
    pub dialog_token: u8,
    pub txpower: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_get_tpc_report_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_send_probe_resp_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub probe_resp_template: [u8; BEACON_TEMPLATE_SIZE],
    pub probe_resp_template_len: u32,
    pub proxy_probe_req_valid_ie_bmap: [u32; 8],
    pub bssid: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct send_probe_resp_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct send_unknown_frame_rx_ind_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_delete_sta_context_ind_msg {
    pub header: wcn36xx_hal_msg_header,
    pub aid: u16,
    pub sta_id: u16,
// TO SUPPORT BT-AMP
    pub bssid: [u8; ETH_ALEN],
// HAL copies bssid from the sta table.
    pub addr2: [u8; ETH_ALEN],
// To unify the keepalive / unknown A2 / tim-based disa
    pub reason_code: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct indicate_del_sta {
    pub header: wcn36xx_hal_msg_header,
    pub aid: u8,
    pub sta_index: u8,
    pub bss_index: u8,
    pub reason_code: u8,
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_amp_event_msg {
    pub header: wcn36xx_hal_msg_header,
    pub btAmpEventType: bt_amp_event_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bt_amp_event_rsp {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tl_hal_flush_ac_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Station Index. originates from HAL
    pub sta_id: u8,
// TID for which the transmit queue is being flushed
    pub tid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tl_hal_flush_ac_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// Station Index. originates from HAL
    pub sta_id: u8,
// TID for which the transmit queue is being flushed
    pub tid: u8,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_enter_imps_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_exit_imps_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_enter_bmps_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bss_index: u8,
// TBTT value derived from the last beacon

    pub tbtt: u64,

    pub dtim_count: u8,
// DTIM period given to HAL during association may not be valid, if
// association is based on ProbeRsp instead of beacon.
    pub dtim_period: u8,
// For CCX and 11R Roaming
    pub rssi_filter_period: u32,
    pub num_beacon_per_rssi_average: u32,
    pub rssi_filter_enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_exit_bmps_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub send_data_null: u8,
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_missed_beacon_ind_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bss_index: u8,
    pub __packed: },
// Beacon Filtering data structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct beacon_filter_ie {
    pub element_id: u8,
    pub check_ie_presence: u8,
    pub offset: u8,
    pub value: u8,
    pub bitmask: u8,
    pub ref: u8,
    pub __packed: },
pub const WCN36XX_FILTER_CAPABILITY_MASK: c_uint = 0x73cf;
pub const WCN36XX_FILTER_IE_DS_CHANNEL_MASK: c_uint = 0x00;
pub const WCN36XX_FILTER_IE_ERP_FILTER_MASK: c_uint = 0xF8;
pub const WCN36XX_FILTER_IE_EDCA_FILTER_MASK: c_uint = 0xF0;
pub const WCN36XX_FILTER_IE_QOS_FILTER_MASK: c_uint = 0xF0;
pub const WCN36XX_FILTER_IE_CHANNEL_SWITCH_MASK: c_uint = 0x00;
pub const WCN36XX_FILTER_IE_HT_BYTE0_FILTER_MASK: c_uint = 0x00;
pub const WCN36XX_FILTER_IE_HT_BYTE1_FILTER_MASK: c_uint = 0xF8;
pub const WCN36XX_FILTER_IE_HT_BYTE2_FILTER_MASK: c_uint = 0xEB;
pub const WCN36XX_FILTER_IE_HT_BYTE5_FILTER_MASK: c_uint = 0xFD;
pub const WCN36XX_FILTER_IE_PWR_CONSTRAINT_MASK: c_uint = 0x00;
pub const WCN36XX_FILTER_IE_OPMODE_NOTIF_MASK: c_uint = 0x00;
pub const WCN36XX_FILTER_IE_VHTOP_CHWIDTH_MASK: c_uint = 0xFC;
pub const WCN36XX_FILTER_IE_RSN_MASK: c_uint = 0x00;
pub const WCN36XX_FILTER_IE_VENDOR_MASK: c_uint = 0x00;
// The above structure would be followed by multiple of below mentioned
// structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_add_bcn_filter_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub capability_info: u16,
    pub capability_mask: u16,
    pub beacon_interval: u16,
    pub ie_num: u16,
    pub bss_index: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rem_bcn_filter_req {
    pub header: wcn36xx_hal_msg_header,
    pub ie_Count: u8,
    pub rem_ie_id: [u8; 1],
    pub __packed: },
pub const WCN36XX_HAL_IPV4_ARP_REPLY_OFFLOAD: c_int = 0;
pub const WCN36XX_HAL_IPV6_NEIGHBOR_DISCOVERY_OFFLOAD: c_int = 1;
pub const WCN36XX_HAL_IPV6_NS_OFFLOAD: c_int = 2;
pub const WCN36XX_HAL_IPV6_ADDR_LEN: c_int = 16;
pub const WCN36XX_HAL_OFFLOAD_DISABLE: c_int = 0;
pub const WCN36XX_HAL_OFFLOAD_ENABLE: c_int = 1;
pub const WCN36XX_HAL_OFFLOAD_BCAST_FILTER_ENABLE: c_uint = 0x2;
pub const WCN36XX_HAL_OFFLOAD_MCAST_FILTER_ENABLE: c_uint = 0x4;

pub const WCN36XX_HAL_IPV6_OFFLOAD_ADDR_MAX: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_ns_offload_params {
    pub src_ipv6_addr: [u8; WCN36XX_HAL_IPV6_ADDR_LEN],
    pub self_ipv6_addr: [u8; WCN36XX_HAL_IPV6_ADDR_LEN],
// Only support 2 possible Network Advertisement IPv6 address
    pub target_ipv6_addr1: [u8; WCN36XX_HAL_IPV6_ADDR_LEN],
    pub target_ipv6_addr2: [u8; WCN36XX_HAL_IPV6_ADDR_LEN],
    pub self_addr: [u8; ETH_ALEN],
    pub src_ipv6_addr_valid:1: u8,
    pub target_ipv6_addr1_valid:1: u8,
    pub target_ipv6_addr2_valid:1: u8,
    pub reserved1:5: u8,
// make it DWORD aligned
    pub reserved2: u8,
// slot index for this offload
    pub slot_index: u32,
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_host_offload_req {
    pub offload_type: u8,
// enable or disable
    pub enable: u8,
    pub host_ipv4_addr: [u8; 4],
    pub host_ipv6_addr: [u8; WCN36XX_HAL_IPV6_ADDR_LEN],
    pub u: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_host_offload_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub host_offload_params: wcn36xx_hal_host_offload_req,
    pub ns_offload_params: wcn36xx_hal_ns_offload_params,
    pub __packed: },
// Packet Types.
pub const WCN36XX_HAL_KEEP_ALIVE_NULL_PKT: c_int = 1;
pub const WCN36XX_HAL_KEEP_ALIVE_UNSOLICIT_ARP_RSP: c_int = 2;
// Enable or disable keep alive
pub const WCN36XX_HAL_KEEP_ALIVE_DISABLE: c_int = 0;
pub const WCN36XX_HAL_KEEP_ALIVE_ENABLE: c_int = 1;

// Keep Alive request.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_keep_alive_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub packet_type: u8,
    pub time_period: u32,
    pub host_ipv4_addr: [u8; WCN36XX_HAL_IPV4_ADDR_LEN],
    pub dest_ipv4_addr: [u8; WCN36XX_HAL_IPV4_ADDR_LEN],
    pub dest_addr: [u8; ETH_ALEN],
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rssi_threshold_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub threshold1:8: i8,
    pub threshold2:8: i8,
    pub threshold3:8: i8,
    pub thres1_pos_notify:1: u8,
    pub thres1_neg_notify:1: u8,
    pub thres2_pos_notify:1: u8,
    pub thres2_neg_notify:1: u8,
    pub thres3_pos_notify:1: u8,
    pub thres3_neg_notify:1: u8,
    pub reserved10:2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_enter_uapsd_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bk_delivery:1: u8,
    pub be_delivery:1: u8,
    pub vi_delivery:1: u8,
    pub vo_delivery:1: u8,
    pub bk_trigger:1: u8,
    pub be_trigger:1: u8,
    pub vi_trigger:1: u8,
    pub vo_trigger:1: u8,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_exit_uapsd_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bss_index: u8,
}

pub const WCN36XX_HAL_WOWL_BCAST_PATTERN_MAX_SIZE: c_int = 128;
pub const WCN36XX_HAL_WOWL_BCAST_MAX_NUM_PATTERNS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_wowl_add_bcast_ptrn_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Pattern ID
    pub id: u8,
// Pattern byte offset from beginning of the 802.11 packet to start
// of the wake-up pattern
    pub byte_Offset: u8,
// Non-Zero Pattern size
    pub size: u8,
// Pattern
    pub pattern: [u8; WCN36XX_HAL_WOWL_BCAST_PATTERN_MAX_SIZE],
// Non-zero pattern mask size
    pub mask_size: u8,
// Pattern mask
    pub mask: [u8; WCN36XX_HAL_WOWL_BCAST_PATTERN_MAX_SIZE],
// Extra pattern
    pub extra: [u8; WCN36XX_HAL_WOWL_BCAST_PATTERN_MAX_SIZE],
// Extra pattern mask
    pub mask_extra: [u8; WCN36XX_HAL_WOWL_BCAST_PATTERN_MAX_SIZE],
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_wow_del_bcast_ptrn_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Pattern ID of the wakeup pattern to be deleted
    pub id: u8,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_wowl_enter_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Enables/disables magic packet filtering
    pub magic_packet_enable: u8,
// Magic pattern
    pub magic_pattern: [u8; ETH_ALEN],
// Enables/disables packet pattern filtering in firmware. Enabling
// this flag enables broadcast pattern matching in Firmware. If
// unicast pattern matching is also desired,
// ucUcastPatternFilteringEnable flag must be set tot true as well
//
    pub pattern_filtering_enable: u8,
// Enables/disables unicast packet pattern filtering. This flag
// specifies whether we want to do pattern match on unicast packets
// as well and not just broadcast packets. This flag has no effect
// if the ucPatternFilteringEnable (main controlling flag) is set
// to false
//
    pub ucast_pattern_filtering_enable: u8,
// This configuration is valid only when magicPktEnable=1. It
// requests hardware to wake up when it receives the Channel Switch
// Action Frame.
//
    pub wow_channel_switch_receive: u8,
// This configuration is valid only when magicPktEnable=1. It
// requests hardware to wake up when it receives the
// Deauthentication Frame.
//
    pub wow_deauth_receive: u8,
// This configuration is valid only when magicPktEnable=1. It
// requests hardware to wake up when it receives the Disassociation
// Frame.
//
    pub wow_disassoc_receive: u8,
// This configuration is valid only when magicPktEnable=1. It
// requests hardware to wake up when it has missed consecutive
// beacons. This is a hardware register configuration (NOT a
// firmware configuration).
//
    pub wow_max_missed_beacons: u8,
// This configuration is valid only when magicPktEnable=1. This is
// a timeout value in units of microsec. It requests hardware to
// unconditionally wake up after it has stayed in WoWLAN mode for
// some time. Set 0 to disable this feature.
//
    pub wow_max_sleep: u8,
// This configuration directs the WoW packet filtering to look for
// EAP-ID requests embedded in EAPOL frames and use this as a wake
// source.
//
    pub wow_eap_id_request_enable: u8,
// This configuration directs the WoW packet filtering to look for
// EAPOL-4WAY requests and use this as a wake source.
//
    pub wow_eapol_4way_enable: u8,
// This configuration allows a host wakeup on an network scan
// offload match.
//
    pub wow_net_scan_offload_match: u8,
// This configuration allows a host wakeup on any GTK rekeying
// error.
//
    pub wow_gtk_rekey_error: u8,
// This configuration allows a host wakeup on BSS connection loss.
//
    pub wow_bss_connection_loss: u8,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_wowl_exit_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_get_rssi_req_msg {
    pub header: wcn36xx_hal_msg_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_get_roam_rssi_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Valid STA Idx for per STA stats request
    pub sta_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_uapsd_ac_params_req_msg {
    pub header: wcn36xx_hal_msg_header,
// STA index
    pub sta_idx: u8,
// Access Category
    pub ac: u8,
// User Priority
    pub up: u8,
// Service Interval
    pub service_interval: u32,
// Suspend Interval
    pub suspend_interval: u32,
// Delay Interval
    pub delay_interval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_configure_rxp_filter_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub set_mcst_bcst_filter_setting: u8,
    pub set_mcst_bcst_filter: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_enter_imps_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_exit_imps_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_enter_bmps_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_exit_bmps_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_enter_uapsd_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_exit_uapsd_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rssi_notification_ind_msg {
    pub header: wcn36xx_hal_msg_header,
    pub rssi_thres1_pos_cross:1: u32,
    pub rssi_thres1_neg_cross:1: u32,
    pub rssi_thres2_pos_cross:1: u32,
    pub rssi_thres2_neg_cross:1: u32,
    pub rssi_thres3_pos_cross:1: u32,
    pub rssi_thres3_neg_cross:1: u32,
    pub avg_rssi:8: u32,
    pub reserved:18: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_get_rssio_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub rssi: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_get_roam_rssi_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub sta_id: u8,
    pub rssi: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_wowl_enter_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_wowl_exit_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_add_bcn_filter_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rem_bcn_filter_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_add_wowl_bcast_ptrn_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_del_wowl_bcast_ptrn_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_host_offload_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_keep_alive_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_rssi_thresh_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_uapsd_ac_params_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_configure_rxp_filter_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_max_tx_pwr_req {
    pub header: wcn36xx_hal_msg_header,
// BSSID is needed to identify which session issued this request.
// As the request has power constraints, this should be applied
// only to that session
    pub bssid: [u8; ETH_ALEN],
    pub self_addr: [u8; ETH_ALEN],
// In request, power == MaxTx power to be used.
    pub power: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_max_tx_pwr_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// power == tx power used for management frames
    pub power: u8,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_tx_pwr_req_msg {
    pub header: wcn36xx_hal_msg_header,
// TX Power in milli watts
    pub tx_power: u32,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_tx_pwr_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_tx_pwr_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub sta_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_tx_pwr_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
// TX Power in milli watts
    pub tx_power: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_p2p_gonoa_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub opp_ps: u8,
    pub ct_window: u32,
    pub count: u8,
    pub duration: u32,
    pub interval: u32,
    pub single_noa_duration: u32,
    pub ps_selection: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_p2p_gonoa_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_add_sta_self_req {
    pub header: wcn36xx_hal_msg_header,
    pub self_addr: [u8; ETH_ALEN],
    pub status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_add_sta_self_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
// Self STA Index
    pub self_sta_index: u8,
// DPU Index (IGTK, PTK, GTK all same)
    pub dpu_index: u8,
// DPU Signature
    pub dpu_signature: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_del_sta_self_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub self_addr: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_del_sta_self_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub self_addr: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggr_add_ts_req {
    pub header: wcn36xx_hal_msg_header,
// Station Index
    pub sta_idx: u16,
// TSPEC handler uniquely identifying a TSPEC for a STA in a BSS.
// This will carry the bitmap with the bit positions representing
// different AC.s
    pub tspec_index: u16,
// Tspec info per AC To program TPE with required parameters
    pub tspec: [wcn36xx_hal_tspec_ie; WCN36XX_HAL_MAX_AC],
// U-APSD Flags: 1b per AC.  Encoded as follows:
    pub uapsd: u8,
// These parameters are for all the access categories
// Service Interval
    pub service_interval: [u32; WCN36XX_HAL_MAX_AC],
// Suspend Interval
    pub suspend_interval: [u32; WCN36XX_HAL_MAX_AC],
// Delay Interval
    pub delay_interval: [u32; WCN36XX_HAL_MAX_AC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggr_add_ts_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status0: u32,
// FIXME PRIMA for future use for 11R
    pub status1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_configure_apps_cpu_wakeup_state_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub is_apps_cpu_awake: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_configure_apps_cpu_wakeup_state_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_dump_cmd_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub arg1: u32,
    pub arg2: u32,
    pub arg3: u32,
    pub arg4: u32,
    pub arg5: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_dump_cmd_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
// Length of the responce message
    pub rsp_length: u32,
// FIXME: Currently considering the responce will be less than
// 100bytes
    pub rsp_buffer: [u8; DUMPCMD_RSP_BUFFER],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coex_ind_msg {
    pub header: wcn36xx_hal_msg_header,
// Coex Indication Type
    pub type: u32,
// Coex Indication Data
    pub data: [u32; WLAN_COEX_IND_DATA_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_tx_compl_ind_msg {
    pub header: wcn36xx_hal_msg_header,
// Tx Complete Indication Success or Failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_wlan_host_suspend_ind_msg {
    pub header: wcn36xx_hal_msg_header,
    pub configured_mcst_bcst_filter_setting: u32,
    pub active_session_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_wlan_exclude_unencrpted_ind_msg {
    pub header: wcn36xx_hal_msg_header,
    pub dot11_exclude_unencrypted: u8,
    pub bssid: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct noa_attr_ind_msg {
    pub header: wcn36xx_hal_msg_header,
    pub index: u8,
    pub opp_ps_flag: u8,
    pub ctwin: u16,
    pub noa1_interval_count: u16,
    pub bss_index: u16,
    pub noa1_duration: u32,
    pub noa1_interval: u32,
    pub noa1_starttime: u32,
    pub noa2_interval_count: u16,
    pub reserved2: u16,
    pub noa2_duration: u32,
    pub noa2_interval: u32,
    pub noa2_start_time: u32,
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct noa_start_ind_msg {
    pub header: wcn36xx_hal_msg_header,
    pub status: u32,
    pub bss_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_wlan_host_resume_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub configured_mcst_bcst_filter_setting: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_host_resume_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_del_ba_ind_msg {
    pub header: wcn36xx_hal_msg_header,
    pub sta_idx: u16,
// Peer MAC Address, whose BA session has timed out
    pub peer_addr: [u8; ETH_ALEN],
// TID for which a BA session timeout is being triggered
    pub ba_tid: u8,
// DELBA direction
// 1 - Originator
// 0 - Recipient
//
    pub direction: u8,
    pub reason_code: u32,
// TO SUPPORT BT-AMP
    pub bssid: [u8; ETH_ALEN],
}

// PNO Messages
// Max number of channels that a network can be found on
pub const WCN36XX_HAL_PNO_MAX_NETW_CHANNELS: c_int = 26;
// Max number of channels that a network can be found on
pub const WCN36XX_HAL_PNO_MAX_NETW_CHANNELS_EX: c_int = 60;
// Maximum numbers of networks supported by PNO
pub const WCN36XX_HAL_PNO_MAX_SUPP_NETWORKS: c_int = 16;
// The number of scan time intervals that can be programmed into PNO
pub const WCN36XX_HAL_PNO_MAX_SCAN_TIMERS: c_int = 10;
// Maximum size of the probe template
pub const WCN36XX_HAL_PNO_MAX_PROBE_SIZE: c_int = 450;
// Type of PNO enabling:
//
// Immediate - scanning will start immediately and PNO procedure will be
// repeated based on timer
//
// Suspend - scanning will start at suspend
//
// Resume - scanning will start on system resume
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pno_mode {
    PNO_MODE_IMMEDIATE,
    PNO_MODE_ON_SUSPEND,
    PNO_MODE_ON_RESUME,
    PNO_MODE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

// Authentication type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum auth_type {
    AUTH_TYPE_ANY = 0,
    AUTH_TYPE_OPEN_SYSTEM = 1,

// Upper layer authentication types
    AUTH_TYPE_WPA = 2,
    AUTH_TYPE_WPA_PSK = 3,

    AUTH_TYPE_RSN = 4,
    AUTH_TYPE_RSN_PSK = 5,
    AUTH_TYPE_FT_RSN = 6,
    AUTH_TYPE_FT_RSN_PSK = 7,
    AUTH_TYPE_WAPI_WAI_CERTIFICATE = 8,
    AUTH_TYPE_WAPI_WAI_PSK = 9,

    AUTH_TYPE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

// Encryption type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ed_type {
    ED_ANY = 0,
    ED_NONE = 1,
    ED_WEP = 2,
    ED_TKIP = 3,
    ED_CCMP = 4,
    ED_WPI = 5,

    ED_TYPE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

// SSID broadcast  type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssid_bcast_type {
    BCAST_UNKNOWN = 0,
    BCAST_NORMAL = 1,
    BCAST_HIDDEN = 2,

    BCAST_TYPE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE
}

// The network description for which PNO will have to look for
#[repr(C)]
#[derive(Copy, Clone)]
pub struct network_type {
// SSID of the BSS
    pub ssid: wcn36xx_hal_mac_ssid,
// Authentication type for the network
    pub authentication: auth_type,
// Encryption type for the network
    pub encryption: ed_type,
// Indicate the channel on which the Network can be found 0 - if
// all channels
    pub channel_count: u8,
    pub channels: [u8; WCN36XX_HAL_PNO_MAX_NETW_CHANNELS],
// Indicates the RSSI threshold for the network to be considered
    pub rssi_threshold: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_timer {
// How much it should wait
    pub value: u32,
// How many times it should repeat that wait value 0 - keep using
// this timer until PNO is disabled
    pub repeat: u32,
// e.g: 2 3 4 0 - it will wait 2s between consecutive scans for 3
// times - after that it will wait 4s between consecutive scans
// until disabled
}

// The network parameters to be sent to the PNO algorithm
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_timers_type {
// set to 0 if you wish for PNO to use its default telescopic timer
    pub count: u8,
// A set value represents the amount of time that PNO will wait
// between two consecutive scan procedures If the desired is for a
// uniform timer that fires always at the exact same interval - one
// single value is to be set If there is a desire for a more
// complex - telescopic like timer multiple values can be set -
// once PNO reaches the end of the array it will continue scanning
// at intervals presented by the last value
    pub values: [scan_timer; WCN36XX_HAL_PNO_MAX_SCAN_TIMERS],
}

// Preferred network list request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_pref_netw_list_req {
    pub header: wcn36xx_hal_msg_header,
// Enable PNO
    pub enable: u32,
// Immediate,  On Suspend,   On Resume
    pub mode: pno_mode,
// Number of networks sent for PNO
    pub networks_count: u32,
// The networks that PNO needs to look for
    pub networks: [network_type; WCN36XX_HAL_PNO_MAX_SUPP_NETWORKS],
// The scan timers required for PNO
    pub scan_timers: scan_timers_type,
// Probe template for 2.4GHz band
    pub band_24g_probe_size: u16,
    pub band_24g_probe_template: [u8; WCN36XX_HAL_PNO_MAX_PROBE_SIZE],
// Probe template for 5GHz band
    pub band_5g_probe_size: u16,
    pub band_5g_probe_template: [u8; WCN36XX_HAL_PNO_MAX_PROBE_SIZE],
}

// The network description for which PNO will have to look for
#[repr(C)]
#[derive(Copy, Clone)]
pub struct network_type_new {
// SSID of the BSS
    pub ssid: wcn36xx_hal_mac_ssid,
// Authentication type for the network
    pub authentication: auth_type,
// Encryption type for the network
    pub encryption: ed_type,
// SSID broadcast type, normal, hidden or unknown
    pub bcast_network_type: ssid_bcast_type,
// Indicate the channel on which the Network can be found 0 - if
// all channels
    pub channel_count: u8,
    pub channels: [u8; WCN36XX_HAL_PNO_MAX_NETW_CHANNELS],
// Indicates the RSSI threshold for the network to be considered
    pub rssi_threshold: u8,
}

// Preferred network list request new
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_pref_netw_list_req_new {
    pub header: wcn36xx_hal_msg_header,
// Enable PNO
    pub enable: u32,
// Immediate,  On Suspend,   On Resume
    pub mode: pno_mode,
// Number of networks sent for PNO
    pub networks_count: u32,
// The networks that PNO needs to look for
    pub networks: [network_type_new; WCN36XX_HAL_PNO_MAX_SUPP_NETWORKS],
// The scan timers required for PNO
    pub scan_timers: scan_timers_type,
// Probe template for 2.4GHz band
    pub band_24g_probe_size: u16,
    pub band_24g_probe_template: [u8; WCN36XX_HAL_PNO_MAX_PROBE_SIZE],
// Probe template for 5GHz band
    pub band_5g_probe_size: u16,
    pub band_5g_probe_template: [u8; WCN36XX_HAL_PNO_MAX_PROBE_SIZE],
}

// Preferred network list response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_pref_netw_list_resp {
    pub header: wcn36xx_hal_msg_header,
// status of the request - just to indicate that PNO has
// acknowledged the request and will start scanning
    pub status: u32,
}

// Preferred network found indication
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pref_netw_found_ind {
    pub header: wcn36xx_hal_msg_header,
// Network that was found with the highest RSSI
    pub ssid: wcn36xx_hal_mac_ssid,
// Indicates the RSSI
    pub rssi: u8,
}

// RSSI Filter request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_rssi_filter_req {
    pub header: wcn36xx_hal_msg_header,
// RSSI Threshold
    pub rssi_threshold: u8,
}

// Set RSSI filter resp
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_rssi_filter_resp {
    pub header: wcn36xx_hal_msg_header,
// status of the request
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_tx_per_tracking_req_msg {
    pub header: wcn36xx_hal_msg_header,
// 0: disable, 1:enable
    pub tx_per_tracking_enable: u8,
// Check period, unit is sec.
    pub tx_per_tracking_period: u8,
// (Fail TX packet)/(Total TX packet) ratio, the unit is 10%.
    pub tx_per_tracking_ratio: u8,
// A watermark of check number, once the tx packet exceed this
// number, we do the check, default is 5
    pub tx_per_tracking_watermark: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_tx_per_tracking_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_per_hit_ind_msg {
    pub header: wcn36xx_hal_msg_header,
}

// Packet Filtering Definitions Begin
pub const WCN36XX_HAL_PROTOCOL_DATA_LEN: c_int = 8;
pub const WCN36XX_HAL_MAX_NUM_MULTICAST_ADDRESS: c_int = 240;
pub const WCN36XX_HAL_MAX_NUM_FILTERS: c_int = 20;
pub const WCN36XX_HAL_MAX_CMP_PER_FILTER: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_receive_packet_filter_type {
    HAL_RCV_FILTER_TYPE_INVALID,
    HAL_RCV_FILTER_TYPE_FILTER_PKT,
    HAL_RCV_FILTER_TYPE_BUFFER_PKT,
    HAL_RCV_FILTER_TYPE_MAX_ENUM_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_rcv_pkt_flt_protocol_type {
    HAL_FILTER_PROTO_TYPE_INVALID,
    HAL_FILTER_PROTO_TYPE_MAC,
    HAL_FILTER_PROTO_TYPE_ARP,
    HAL_FILTER_PROTO_TYPE_IPV4,
    HAL_FILTER_PROTO_TYPE_IPV6,
    HAL_FILTER_PROTO_TYPE_UDP,
    HAL_FILTER_PROTO_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_rcv_pkt_flt_cmp_flag_type {
    HAL_FILTER_CMP_TYPE_INVALID,
    HAL_FILTER_CMP_TYPE_EQUAL,
    HAL_FILTER_CMP_TYPE_MASK_EQUAL,
    HAL_FILTER_CMP_TYPE_NOT_EQUAL,
    HAL_FILTER_CMP_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rcv_pkt_filter_params {
    pub protocol_layer: u8,
    pub cmp_flag: u8,
// Length of the data to compare
    pub data_length: u16,
// from start of the respective frame header
    pub data_offset: u8,
// Reserved field
    pub reserved: u8,
// Data to compare
    pub compare_data: [u8; WCN36XX_HAL_PROTOCOL_DATA_LEN],
// Mask to be applied on the received packet data before compare
    pub data_mask: [u8; WCN36XX_HAL_PROTOCOL_DATA_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_sessionized_rcv_pkt_filter_cfg_type {
    pub id: u8,
    pub type: u8,
    pub params_count: u8,
    pub coleasce_time: u32,
    pub bss_index: u8,
    pub params: [wcn36xx_hal_rcv_pkt_filter_params; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_rcv_pkt_filter_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub id: u8,
    pub type: u8,
    pub params_count: u8,
    pub coalesce_time: u32,
    pub params: [wcn36xx_hal_rcv_pkt_filter_params; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rcv_flt_mc_addr_list_type {
// from start of the respective frame header
    pub data_offset: u8,
    pub mc_addr_count: u32,
    pub mc_addr: [u8; WCN36XX_HAL_MAX_NUM_MULTICAST_ADDRESS][ETH_ALEN],
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_pkt_filter_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rcv_flt_pkt_match_cnt_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rcv_flt_pkt_match_cnt {
    pub id: u8,
    pub match_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rcv_flt_pkt_match_cnt_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// Success or Failure
    pub status: u32,
    pub match_count: u32,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rcv_flt_pkt_clear_param {
// only valid for response message
    pub status: u32,
    pub id: u8,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rcv_flt_pkt_clear_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub param: wcn36xx_hal_rcv_flt_pkt_clear_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rcv_flt_pkt_clear_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
    pub param: wcn36xx_hal_rcv_flt_pkt_clear_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rcv_flt_pkt_set_mc_list_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub mc_addr_list: wcn36xx_hal_rcv_flt_mc_addr_list_type,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_rcv_flt_pkt_set_mc_list_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
    pub status: u32,
    pub bss_index: u8,
}

// Packet Filtering Definitions End
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_power_params_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Ignore DTIM
    pub ignore_dtim: u32,
// DTIM Period
    pub dtim_period: u32,
// Listen Interval
    pub listen_interval: u32,
// Broadcast Multicast Filter
    pub bcast_mcast_filter: u32,
// Beacon Early Termination
    pub enable_bet: u32,
// Beacon Early Termination Interval
    pub bet_interval: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_set_power_params_resp {
    pub header: wcn36xx_hal_msg_header,
// status of the request
    pub status: u32,
    pub __packed: },
pub const WCN36XX_HAL_CAPS_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_feat_caps_msg {
    pub header: wcn36xx_hal_msg_header,
    pub feat_caps: [u32; WCN36XX_HAL_CAPS_SIZE],
    pub __packed: },
// status codes to help debug rekey failures
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gtk_rekey_status {
    WCN36XX_HAL_GTK_REKEY_STATUS_SUCCESS = 0,

// rekey detected, but not handled
    WCN36XX_HAL_GTK_REKEY_STATUS_NOT_HANDLED = 1,

// MIC check error on M1
    WCN36XX_HAL_GTK_REKEY_STATUS_MIC_ERROR = 2,

// decryption error on M1
    WCN36XX_HAL_GTK_REKEY_STATUS_DECRYPT_ERROR = 3,

// M1 replay detected
    WCN36XX_HAL_GTK_REKEY_STATUS_REPLAY_ERROR = 4,

// missing GTK key descriptor in M1
    WCN36XX_HAL_GTK_REKEY_STATUS_MISSING_KDE = 5,

// missing iGTK key descriptor in M1
    WCN36XX_HAL_GTK_REKEY_STATUS_MISSING_IGTK_KDE = 6,

// key installation error
    WCN36XX_HAL_GTK_REKEY_STATUS_INSTALL_ERROR = 7,

// iGTK key installation error
    WCN36XX_HAL_GTK_REKEY_STATUS_IGTK_INSTALL_ERROR = 8,

// GTK rekey M2 response TX error
    WCN36XX_HAL_GTK_REKEY_STATUS_RESP_TX_ERROR = 9,

// non-specific general error
    WCN36XX_HAL_GTK_REKEY_STATUS_GEN_ERROR = 255
}

// wake reason types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wake_reason_type {
    WCN36XX_HAL_WAKE_REASON_NONE = 0,

// magic packet match
    WCN36XX_HAL_WAKE_REASON_MAGIC_PACKET = 1,

// host defined pattern match
    WCN36XX_HAL_WAKE_REASON_PATTERN_MATCH = 2,

// EAP-ID frame detected
    WCN36XX_HAL_WAKE_REASON_EAPID_PACKET = 3,

// start of EAPOL 4-way handshake detected
    WCN36XX_HAL_WAKE_REASON_EAPOL4WAY_PACKET = 4,

// network scan offload match
    WCN36XX_HAL_WAKE_REASON_NETSCAN_OFFL_MATCH = 5,

// GTK rekey status wakeup (see status)
    WCN36XX_HAL_WAKE_REASON_GTK_REKEY_STATUS = 6,

// BSS connection lost
    WCN36XX_HAL_WAKE_REASON_BSS_CONN_LOST = 7,
}

//
// Wake reason indication
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_wake_reason_ind {
    pub header: wcn36xx_hal_msg_header,
// see tWakeReasonType
    pub reason: u32,
// argument specific to the reason type
    pub reason_arg: u32,
// length of optional data stored in this message, in case HAL
// truncates the data (i.e. data packets) this length will be less
// than the actual length
    pub stored_data_len: u32,
// actual length of data
    pub actual_data_len: u32,
// variable length start of data (length == storedDataLen) see
// specific wake type
    pub data_start: [u8; 1],
    pub bss_index:8: u32,
    pub reserved:24: u32,
}

pub const WCN36XX_HAL_GTK_KEK_BYTES: c_int = 16;
pub const WCN36XX_HAL_GTK_KCK_BYTES: c_int = 16;

pub const GTK_SET_BSS_KEY_TAG: c_uint = 0x1234AA55;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_gtk_offload_req_msg {
    pub header: wcn36xx_hal_msg_header,
// optional flags
    pub flags: u32,
// Key confirmation key
    pub kck: [u8; WCN36XX_HAL_GTK_KCK_BYTES],
// key encryption key
    pub kek: [u8; WCN36XX_HAL_GTK_KEK_BYTES],
// replay counter
    pub key_replay_counter: u64,
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_gtk_offload_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
    pub bss_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_gtk_offload_get_info_req_msg {
    pub header: wcn36xx_hal_msg_header,
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_gtk_offload_get_info_rsp_msg {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
// last rekey status when the rekey was offloaded
    pub last_rekey_status: u32,
// current replay counter value
    pub key_replay_counter: u64,
// total rekey attempts
    pub total_rekey_count: u32,
// successful GTK rekeys
    pub gtk_rekey_count: u32,
// successful iGTK rekeys
    pub igtk_rekey_count: u32,
    pub bss_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dhcp_info {
// Indicates the device mode which indicates about the DHCP activity
    pub device_mode: u8,
    pub addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dhcp_ind_status {
    pub header: wcn36xx_hal_msg_header,
// success or failure
    pub status: u32,
}

//
// Thermal Mitigation mode of operation.
//
// WCN36XX_HAL_THERMAL_MITIGATION_MODE_0 - Based on AMPDU disabling aggregation
//
// WCN36XX_HAL_THERMAL_MITIGATION_MODE_1 - Based on AMPDU disabling aggregation
// and reducing transmit power
//
// WCN36XX_HAL_THERMAL_MITIGATION_MODE_2 - Not supported
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_thermal_mitigation_mode_type {
    HAL_THERMAL_MITIGATION_MODE_INVALID = -1,
    HAL_THERMAL_MITIGATION_MODE_0,
    HAL_THERMAL_MITIGATION_MODE_1,
    HAL_THERMAL_MITIGATION_MODE_2,
    HAL_THERMAL_MITIGATION_MODE_MAX = WCN36XX_HAL_MAX_ENUM_SIZE,
}

//
// Thermal Mitigation level.
// Note the levels are incremental i.e WCN36XX_HAL_THERMAL_MITIGATION_LEVEL_2 =
// WCN36XX_HAL_THERMAL_MITIGATION_LEVEL_0 +
// WCN36XX_HAL_THERMAL_MITIGATION_LEVEL_1
//
// WCN36XX_HAL_THERMAL_MITIGATION_LEVEL_0 - lowest level of thermal mitigation.
// This level indicates normal mode of operation
//
// WCN36XX_HAL_THERMAL_MITIGATION_LEVEL_1 - 1st level of thermal mitigation
//
// WCN36XX_HAL_THERMAL_MITIGATION_LEVEL_2 - 2nd level of thermal mitigation
//
// WCN36XX_HAL_THERMAL_MITIGATION_LEVEL_3 - 3rd level of thermal mitigation
//
// WCN36XX_HAL_THERMAL_MITIGATION_LEVEL_4 - 4th level of thermal mitigation
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wcn36xx_hal_thermal_mitigation_level_type {
    HAL_THERMAL_MITIGATION_LEVEL_INVALID = -1,
    HAL_THERMAL_MITIGATION_LEVEL_0,
    HAL_THERMAL_MITIGATION_LEVEL_1,
    HAL_THERMAL_MITIGATION_LEVEL_2,
    HAL_THERMAL_MITIGATION_LEVEL_3,
    HAL_THERMAL_MITIGATION_LEVEL_4,
    HAL_THERMAL_MITIGATION_LEVEL_MAX = WCN36XX_HAL_MAX_ENUM_SIZE,
}

// WCN36XX_HAL_SET_THERMAL_MITIGATION_REQ
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_thermal_mitigation_req_msg {
    pub header: wcn36xx_hal_msg_header,
// Thermal Mitigation Operation Mode
    pub mode: wcn36xx_hal_thermal_mitigation_mode_type,
// Thermal Mitigation Level
    pub level: wcn36xx_hal_thermal_mitigation_level_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_thermal_mitigation_resp {
    pub header: wcn36xx_hal_msg_header,
// status of the request
    pub status: u32,
}

// Per STA Class B Statistics. Class B statistics are STA TX/RX stats
// provided to FW from Host via periodic messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stats_class_b_ind {
    pub header: wcn36xx_hal_msg_header,
// Duration over which this stats was collected
    pub duration: u32,
// Per STA Stats
// TX stats
    pub tx_bytes_pushed: u32,
    pub tx_packets_pushed: u32,
// RX stats
    pub rx_bytes_rcvd: u32,
    pub rx_packets_rcvd: u32,
    pub rx_time_total: u32,
}

// WCN36XX_HAL_PRINT_REG_INFO_IND
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcn36xx_hal_print_reg_info_ind {
    pub header: wcn36xx_hal_msg_header,
    pub count: u32,
    pub scenario: u32,
    pub reason: u32,
    pub addr: u32,
    pub value: u32,
    pub regs: [}; ],
    pub __packed: },
