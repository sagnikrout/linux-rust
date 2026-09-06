//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wil6210/wmi.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2018-2019, The Linux Foundation. All rights reserved.
// Copyright (c) 2012-2017 Qualcomm Atheros, Inc.
// Copyright (c) 2006-2012 Wilocity
//
// This file contains the definitions of the WMI protocol specified in the
// Wireless Module Interface (WMI) for the Qualcomm
// 60 GHz wireless solution.
// It includes definitions of all the commands and events.
// Commands are messages from the host to the WM.
// Events are messages from the WM to the host.
//
// This is an automatically generated file.
//

// DTYPE configuration array size
// must always be kept equal to (WMI_RF_DTYPE_LENGTH+1)
//

// ETYPE configuration array size
// must always be kept equal to
// (WMI_RF_ETYPE_LENGTH+WMI_RF_ETYPE_VAL_PER_RANGE)
//

// RX2TX configuration array size
// must always be kept equal to (WMI_RF_RX2TX_LENGTH+1)
//

// Qos configuration

// (WMI_QOS_MIN_DEFAULT_WEIGHT * WMI_QOS_VRING_SLOT_MAX_MS
// WMI_QOS_VRING_SLOT_MIN_MS)
//
pub const WMI_QOS_MAX_WEIGHT: c_int = 50;

// Mailbox interface
// used for commands and events
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_mid {
    MID_DEFAULT		= 0x00,
    FIRST_DBG_MID_ID	= 0x10,
    LAST_DBG_MID_ID		= 0xFE,
    MID_BROADCAST		= 0xFF,
}

// FW capability IDs
// Each ID maps to a bit in a 32-bit bitmask value provided by the FW to
// the host
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_fw_capability {
    WMI_FW_CAPABILITY_FTM				= 0,
    WMI_FW_CAPABILITY_PS_CONFIG			= 1,
    WMI_FW_CAPABILITY_RF_SECTORS			= 2,
    WMI_FW_CAPABILITY_MGMT_RETRY_LIMIT		= 3,
    WMI_FW_CAPABILITY_AP_SME_OFFLOAD_PARTIAL	= 4,
    WMI_FW_CAPABILITY_WMI_ONLY			= 5,
    WMI_FW_CAPABILITY_THERMAL_THROTTLING		= 7,
    WMI_FW_CAPABILITY_D3_SUSPEND			= 8,
    WMI_FW_CAPABILITY_LONG_RANGE			= 9,
    WMI_FW_CAPABILITY_FIXED_SCHEDULING		= 10,
    WMI_FW_CAPABILITY_MULTI_DIRECTED_OMNIS		= 11,
    WMI_FW_CAPABILITY_RSSI_REPORTING		= 12,
    WMI_FW_CAPABILITY_SET_SILENT_RSSI_TABLE		= 13,
    WMI_FW_CAPABILITY_LO_POWER_CALIB_FROM_OTP	= 14,
    WMI_FW_CAPABILITY_PNO				= 15,
    WMI_FW_CAPABILITY_CHANNEL_BONDING		= 17,
    WMI_FW_CAPABILITY_REF_CLOCK_CONTROL		= 18,
    WMI_FW_CAPABILITY_AP_SME_OFFLOAD_NONE		= 19,
    WMI_FW_CAPABILITY_MULTI_VIFS			= 20,
    WMI_FW_CAPABILITY_FT_ROAMING			= 21,
    WMI_FW_CAPABILITY_BACK_WIN_SIZE_64		= 22,
    WMI_FW_CAPABILITY_AMSDU				= 23,
    WMI_FW_CAPABILITY_RAW_MODE			= 24,
    WMI_FW_CAPABILITY_TX_REQ_EXT			= 25,
    WMI_FW_CAPABILITY_CHANNEL_4			= 26,
    WMI_FW_CAPABILITY_IPA				= 27,
    WMI_FW_CAPABILITY_TEMPERATURE_ALL_RF		= 30,
    WMI_FW_CAPABILITY_SPLIT_REKEY			= 31,
    WMI_FW_CAPABILITY_MAX,
}

// WMI_CMD_HDR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cmd_hdr {
    pub mid: u8,
    pub reserved: u8,
    pub command_id: __le16,
    pub fw_timestamp: __le32,
    pub __packed: },
// List of Commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_command_id {
    WMI_CONNECT_CMDID				= 0x01,
    WMI_DISCONNECT_CMDID				= 0x03,
    WMI_DISCONNECT_STA_CMDID			= 0x04,
    WMI_START_SCHED_SCAN_CMDID			= 0x05,
    WMI_STOP_SCHED_SCAN_CMDID			= 0x06,
    WMI_START_SCAN_CMDID				= 0x07,
    WMI_SET_BSS_FILTER_CMDID			= 0x09,
    WMI_SET_PROBED_SSID_CMDID			= 0x0A,
// deprecated
    WMI_SET_LISTEN_INT_CMDID			= 0x0B,
    WMI_FT_AUTH_CMDID				= 0x0C,
    WMI_FT_REASSOC_CMDID				= 0x0D,
    WMI_UPDATE_FT_IES_CMDID				= 0x0E,
    WMI_BCON_CTRL_CMDID				= 0x0F,
    WMI_ADD_CIPHER_KEY_CMDID			= 0x16,
    WMI_DELETE_CIPHER_KEY_CMDID			= 0x17,
    WMI_PCP_CONF_CMDID				= 0x18,
    WMI_SET_APPIE_CMDID				= 0x3F,
    WMI_SET_WSC_STATUS_CMDID			= 0x41,
    WMI_PXMT_RANGE_CFG_CMDID			= 0x42,
    WMI_PXMT_SNR2_RANGE_CFG_CMDID			= 0x43,
    WMI_RADAR_GENERAL_CONFIG_CMDID			= 0x100,
    WMI_RADAR_CONFIG_SELECT_CMDID			= 0x101,
    WMI_RADAR_PARAMS_CONFIG_CMDID			= 0x102,
    WMI_RADAR_SET_MODE_CMDID			= 0x103,
    WMI_RADAR_CONTROL_CMDID				= 0x104,
    WMI_RADAR_PCI_CONTROL_CMDID			= 0x105,
    WMI_MEM_READ_CMDID				= 0x800,
    WMI_MEM_WR_CMDID				= 0x801,
    WMI_ECHO_CMDID					= 0x803,
    WMI_DEEP_ECHO_CMDID				= 0x804,
    WMI_CONFIG_MAC_CMDID				= 0x805,
// deprecated
    WMI_CONFIG_PHY_DEBUG_CMDID			= 0x806,
    WMI_ADD_DEBUG_TX_PCKT_CMDID			= 0x808,
    WMI_PHY_GET_STATISTICS_CMDID			= 0x809,
// deprecated
    WMI_FS_TUNE_CMDID				= 0x80A,
// deprecated
    WMI_CORR_MEASURE_CMDID				= 0x80B,
    WMI_READ_RSSI_CMDID				= 0x80C,
    WMI_TEMP_SENSE_CMDID				= 0x80E,
    WMI_DC_CALIB_CMDID				= 0x80F,
// deprecated
    WMI_SEND_TONE_CMDID				= 0x810,
// deprecated
    WMI_IQ_TX_CALIB_CMDID				= 0x811,
// deprecated
    WMI_IQ_RX_CALIB_CMDID				= 0x812,
    WMI_SET_WORK_MODE_CMDID				= 0x815,
    WMI_LO_LEAKAGE_CALIB_CMDID			= 0x816,
    WMI_LO_POWER_CALIB_FROM_OTP_CMDID		= 0x817,
    WMI_SILENT_RSSI_CALIB_CMDID			= 0x81D,
// deprecated
    WMI_RF_RX_TEST_CMDID				= 0x81E,
    WMI_CFG_RX_CHAIN_CMDID				= 0x820,
    WMI_VRING_CFG_CMDID				= 0x821,
    WMI_BCAST_VRING_CFG_CMDID			= 0x822,
    WMI_RING_BA_EN_CMDID				= 0x823,
    WMI_RING_BA_DIS_CMDID				= 0x824,
    WMI_RCP_ADDBA_RESP_CMDID			= 0x825,
    WMI_RCP_DELBA_CMDID				= 0x826,
    WMI_SET_SSID_CMDID				= 0x827,
    WMI_GET_SSID_CMDID				= 0x828,
    WMI_SET_PCP_CHANNEL_CMDID			= 0x829,
    WMI_GET_PCP_CHANNEL_CMDID			= 0x82A,
    WMI_SW_TX_REQ_CMDID				= 0x82B,
// Event is shared between WMI_SW_TX_REQ_CMDID and
// WMI_SW_TX_REQ_EXT_CMDID
//
    WMI_SW_TX_REQ_EXT_CMDID				= 0x82C,
    WMI_MLME_PUSH_CMDID				= 0x835,
    WMI_BEAMFORMING_MGMT_CMDID			= 0x836,
    WMI_BF_TXSS_MGMT_CMDID				= 0x837,
    WMI_BF_SM_MGMT_CMDID				= 0x838,
    WMI_BF_RXSS_MGMT_CMDID				= 0x839,
    WMI_BF_TRIG_CMDID				= 0x83A,
    WMI_RCP_ADDBA_RESP_EDMA_CMDID			= 0x83B,
    WMI_LINK_MAINTAIN_CFG_WRITE_CMDID		= 0x842,
    WMI_LINK_MAINTAIN_CFG_READ_CMDID		= 0x843,
    WMI_SET_LINK_MONITOR_CMDID			= 0x845,
    WMI_SET_SECTORS_CMDID				= 0x849,
    WMI_MAINTAIN_PAUSE_CMDID			= 0x850,
    WMI_MAINTAIN_RESUME_CMDID			= 0x851,
    WMI_RS_MGMT_CMDID				= 0x852,
    WMI_RF_MGMT_CMDID				= 0x853,
    WMI_RF_XPM_READ_CMDID				= 0x856,
    WMI_RF_XPM_WRITE_CMDID				= 0x857,
    WMI_LED_CFG_CMDID				= 0x858,
    WMI_SET_CONNECT_SNR_THR_CMDID			= 0x85B,
    WMI_SET_ACTIVE_SILENT_RSSI_TABLE_CMDID		= 0x85C,
    WMI_RF_PWR_ON_DELAY_CMDID			= 0x85D,
    WMI_SET_HIGH_POWER_TABLE_PARAMS_CMDID		= 0x85E,
    WMI_FIXED_SCHEDULING_UL_CONFIG_CMDID		= 0x85F,
// Performance monitoring commands
    WMI_BF_CTRL_CMDID				= 0x862,
    WMI_NOTIFY_REQ_CMDID				= 0x863,
    WMI_GET_STATUS_CMDID				= 0x864,
    WMI_GET_RF_STATUS_CMDID				= 0x866,
    WMI_GET_BASEBAND_TYPE_CMDID			= 0x867,
    WMI_VRING_SWITCH_TIMING_CONFIG_CMDID		= 0x868,
    WMI_UNIT_TEST_CMDID				= 0x900,
    WMI_FLASH_READ_CMDID				= 0x902,
    WMI_FLASH_WRITE_CMDID				= 0x903,
// Power management
    WMI_TRAFFIC_SUSPEND_CMDID			= 0x904,
    WMI_TRAFFIC_RESUME_CMDID			= 0x905,
// P2P
    WMI_P2P_CFG_CMDID				= 0x910,
    WMI_PORT_ALLOCATE_CMDID				= 0x911,
    WMI_PORT_DELETE_CMDID				= 0x912,
    WMI_POWER_MGMT_CFG_CMDID			= 0x913,
    WMI_START_LISTEN_CMDID				= 0x914,
    WMI_START_SEARCH_CMDID				= 0x915,
    WMI_DISCOVERY_START_CMDID			= 0x916,
    WMI_DISCOVERY_STOP_CMDID			= 0x917,
    WMI_PCP_START_CMDID				= 0x918,
    WMI_PCP_STOP_CMDID				= 0x919,
    WMI_GET_PCP_FACTOR_CMDID			= 0x91B,
// Power Save Configuration Commands
    WMI_PS_DEV_PROFILE_CFG_CMDID			= 0x91C,
    WMI_RS_ENABLE_CMDID				= 0x91E,
    WMI_RS_CFG_EX_CMDID				= 0x91F,
    WMI_GET_DETAILED_RS_RES_EX_CMDID		= 0x920,
// deprecated
    WMI_RS_CFG_CMDID				= 0x921,
// deprecated
    WMI_GET_DETAILED_RS_RES_CMDID			= 0x922,
    WMI_AOA_MEAS_CMDID				= 0x923,
    WMI_BRP_SET_ANT_LIMIT_CMDID			= 0x924,
    WMI_SET_MGMT_RETRY_LIMIT_CMDID			= 0x930,
    WMI_GET_MGMT_RETRY_LIMIT_CMDID			= 0x931,
    WMI_NEW_STA_CMDID				= 0x935,
    WMI_DEL_STA_CMDID				= 0x936,
    WMI_SET_THERMAL_THROTTLING_CFG_CMDID		= 0x940,
    WMI_GET_THERMAL_THROTTLING_CFG_CMDID		= 0x941,
// Read Power Save profile type
    WMI_PS_DEV_PROFILE_CFG_READ_CMDID		= 0x942,
    WMI_TSF_SYNC_CMDID				= 0x973,
    WMI_TOF_SESSION_START_CMDID			= 0x991,
    WMI_TOF_GET_CAPABILITIES_CMDID			= 0x992,
    WMI_TOF_SET_LCR_CMDID				= 0x993,
    WMI_TOF_SET_LCI_CMDID				= 0x994,
    WMI_TOF_CFG_RESPONDER_CMDID			= 0x996,
    WMI_TOF_SET_TX_RX_OFFSET_CMDID			= 0x997,
    WMI_TOF_GET_TX_RX_OFFSET_CMDID			= 0x998,
    WMI_TOF_CHANNEL_INFO_CMDID			= 0x999,
    WMI_GET_RF_SECTOR_PARAMS_CMDID			= 0x9A0,
    WMI_SET_RF_SECTOR_PARAMS_CMDID			= 0x9A1,
    WMI_GET_SELECTED_RF_SECTOR_INDEX_CMDID		= 0x9A2,
    WMI_SET_SELECTED_RF_SECTOR_INDEX_CMDID		= 0x9A3,
    WMI_SET_RF_SECTOR_ON_CMDID			= 0x9A4,
    WMI_PRIO_TX_SECTORS_ORDER_CMDID			= 0x9A5,
    WMI_PRIO_TX_SECTORS_NUMBER_CMDID		= 0x9A6,
    WMI_PRIO_TX_SECTORS_SET_DEFAULT_CFG_CMDID	= 0x9A7,
// deprecated
    WMI_BF_CONTROL_CMDID				= 0x9AA,
    WMI_BF_CONTROL_EX_CMDID				= 0x9AB,
    WMI_TX_STATUS_RING_ADD_CMDID			= 0x9C0,
    WMI_RX_STATUS_RING_ADD_CMDID			= 0x9C1,
    WMI_TX_DESC_RING_ADD_CMDID			= 0x9C2,
    WMI_RX_DESC_RING_ADD_CMDID			= 0x9C3,
    WMI_BCAST_DESC_RING_ADD_CMDID			= 0x9C4,
    WMI_CFG_DEF_RX_OFFLOAD_CMDID			= 0x9C5,
    WMI_SCHEDULING_SCHEME_CMDID			= 0xA01,
    WMI_FIXED_SCHEDULING_CONFIG_CMDID		= 0xA02,
    WMI_ENABLE_FIXED_SCHEDULING_CMDID		= 0xA03,
    WMI_SET_MULTI_DIRECTED_OMNIS_CONFIG_CMDID	= 0xA04,
    WMI_SET_LONG_RANGE_CONFIG_CMDID			= 0xA05,
    WMI_GET_ASSOC_LIST_CMDID			= 0xA06,
    WMI_GET_CCA_INDICATIONS_CMDID			= 0xA07,
    WMI_SET_CCA_INDICATIONS_BI_AVG_NUM_CMDID	= 0xA08,
    WMI_INTERNAL_FW_IOCTL_CMDID			= 0xA0B,
    WMI_LINK_STATS_CMDID				= 0xA0C,
    WMI_SET_GRANT_MCS_CMDID				= 0xA0E,
    WMI_SET_AP_SLOT_SIZE_CMDID			= 0xA0F,
    WMI_SET_VRING_PRIORITY_WEIGHT_CMDID		= 0xA10,
    WMI_SET_VRING_PRIORITY_CMDID			= 0xA11,
    WMI_RBUFCAP_CFG_CMDID				= 0xA12,
    WMI_TEMP_SENSE_ALL_CMDID			= 0xA13,
    WMI_SET_MAC_ADDRESS_CMDID			= 0xF003,
    WMI_ABORT_SCAN_CMDID				= 0xF007,
    WMI_SET_PROMISCUOUS_MODE_CMDID			= 0xF041,
// deprecated
    WMI_GET_PMK_CMDID				= 0xF048,
    WMI_SET_PASSPHRASE_CMDID			= 0xF049,
// deprecated
    WMI_SEND_ASSOC_RES_CMDID			= 0xF04A,
// deprecated
    WMI_SET_ASSOC_REQ_RELAY_CMDID			= 0xF04B,
    WMI_MAC_ADDR_REQ_CMDID				= 0xF04D,
    WMI_FW_VER_CMDID				= 0xF04E,
    WMI_PMC_CMDID					= 0xF04F,
}

// WMI_CONNECT_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_network_type {
    WMI_NETTYPE_INFRA		= 0x01,
    WMI_NETTYPE_ADHOC		= 0x02,
    WMI_NETTYPE_ADHOC_CREATOR	= 0x04,
    WMI_NETTYPE_AP			= 0x10,
    WMI_NETTYPE_P2P			= 0x20,
// PCIE over 60g
    WMI_NETTYPE_WBE			= 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_dot11_auth_mode {
    WMI_AUTH11_OPEN		= 0x01,
    WMI_AUTH11_SHARED	= 0x02,
    WMI_AUTH11_LEAP		= 0x04,
    WMI_AUTH11_WSC		= 0x08,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_auth_mode {
    WMI_AUTH_NONE		= 0x01,
    WMI_AUTH_WPA		= 0x02,
    WMI_AUTH_WPA2		= 0x04,
    WMI_AUTH_WPA_PSK	= 0x08,
    WMI_AUTH_WPA2_PSK	= 0x10,
    WMI_AUTH_WPA_CCKM	= 0x20,
    WMI_AUTH_WPA2_CCKM	= 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_crypto_type {
    WMI_CRYPT_NONE		= 0x01,
    WMI_CRYPT_AES_GCMP	= 0x20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_connect_ctrl_flag_bits {
    WMI_CONNECT_ASSOC_POLICY_USER		= 0x01,
    WMI_CONNECT_SEND_REASSOC		= 0x02,
    WMI_CONNECT_IGNORE_WPA_GROUP_CIPHER	= 0x04,
    WMI_CONNECT_PROFILE_MATCH_DONE		= 0x08,
    WMI_CONNECT_IGNORE_AAC_BEACON		= 0x10,
    WMI_CONNECT_CSA_FOLLOW_BSS		= 0x20,
    WMI_CONNECT_DO_WPA_OFFLOAD		= 0x40,
    WMI_CONNECT_DO_NOT_DEAUTH		= 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_channel {
    WMI_CHANNEL_1	= 0x00,
    WMI_CHANNEL_2	= 0x01,
    WMI_CHANNEL_3	= 0x02,
    WMI_CHANNEL_4	= 0x03,
    WMI_CHANNEL_5	= 0x04,
    WMI_CHANNEL_6	= 0x05,
    WMI_CHANNEL_9	= 0x06,
    WMI_CHANNEL_10	= 0x07,
    WMI_CHANNEL_11	= 0x08,
    WMI_CHANNEL_12	= 0x09,
}

// WMI_CONNECT_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_connect_cmd {
    pub network_type: u8,
    pub dot11_auth_mode: u8,
    pub auth_mode: u8,
    pub pairwise_crypto_type: u8,
    pub pairwise_crypto_len: u8,
    pub group_crypto_type: u8,
    pub group_crypto_len: u8,
    pub ssid_len: u8,
    pub ssid: [u8; WMI_MAX_SSID_LEN],
// enum wmi_channel WMI_CHANNEL_1..WMI_CHANNEL_6; for EDMG this is
// the primary channel number
//
    pub channel: u8,
// enum wmi_channel WMI_CHANNEL_9..WMI_CHANNEL_12
    pub edmg_channel: u8,
    pub bssid: [u8; WMI_MAC_LEN],
    pub ctrl_flags: __le32,
    pub dst_mac: [u8; WMI_MAC_LEN],
    pub reserved1: [u8; 2],
    pub __packed: },
// WMI_DISCONNECT_STA_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_disconnect_sta_cmd {
    pub dst_mac: [u8; WMI_MAC_LEN],
    pub disconnect_reason: __le16,
    pub __packed: },

// WMI_SET_PASSPHRASE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_passphrase_cmd {
    pub ssid: [u8; WMI_MAX_SSID_LEN],
    pub passphrase: [u8; WMI_PASSPHRASE_LEN],
    pub ssid_len: u8,
    pub passphrase_len: u8,
    pub __packed: },
// WMI_ADD_CIPHER_KEY_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_key_usage {
    WMI_KEY_USE_PAIRWISE	= 0x00,
    WMI_KEY_USE_RX_GROUP	= 0x01,
    WMI_KEY_USE_TX_GROUP	= 0x02,
    WMI_KEY_USE_STORE_PTK	= 0x03,
    WMI_KEY_USE_APPLY_PTK	= 0x04,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_add_cipher_key_cmd {
    pub key_index: u8,
    pub key_type: u8,
// enum wmi_key_usage
    pub key_usage: u8,
    pub key_len: u8,
// key replay sequence counter
    pub key_rsc: [u8; 8],
    pub key: [u8; WMI_MAX_KEY_LEN],
// Additional Key Control information
    pub key_op_ctrl: u8,
    pub mac: [u8; WMI_MAC_LEN],
    pub __packed: },
// WMI_DELETE_CIPHER_KEY_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_delete_cipher_key_cmd {
    pub key_index: u8,
    pub mac: [u8; WMI_MAC_LEN],
    pub __packed: },
// WMI_START_SCAN_CMDID
//
// Start L1 scan operation
//
// Returned events:
// - WMI_RX_MGMT_PACKET_EVENTID - for every probe resp.
// - WMI_SCAN_COMPLETE_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_scan_type {
    WMI_ACTIVE_SCAN		= 0x00,
    WMI_SHORT_SCAN		= 0x01,
    WMI_PASSIVE_SCAN	= 0x02,
    WMI_DIRECT_SCAN		= 0x03,
    WMI_LONG_SCAN		= 0x04,
}

// WMI_START_SCAN_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_start_scan_cmd {
    pub direct_scan_mac_addr: [u8; WMI_MAC_LEN],
// run scan with discovery beacon. Relevant for ACTIVE scan only.
    pub discovery_mode: u8,
    pub reserved: u8,
// Max duration in the home channel(ms)
    pub dwell_time: __le32,
// Time interval between scans (ms)
    pub force_scan_interval: __le32,
// enum wmi_scan_type
    pub scan_type: u8,
// how many channels follow
    pub num_channels: u8,
// channels ID's:
// 0 - 58320 MHz
// 1 - 60480 MHz
// 2 - 62640 MHz
//
    pub channel: u8,
    pub reserved: u8,
    pub __counted_by(num_channels): } channel_list[],
    pub __packed: },

// WMI_START_SCHED_SCAN_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sched_scan_ssid_match {
    pub ssid_len: u8,
    pub ssid: [u8; WMI_MAX_SSID_LEN],
    pub rssi_threshold: i8,
// boolean
    pub add_ssid_to_probe: u8,
    pub reserved: u8,
    pub __packed: },
// WMI_START_SCHED_SCAN_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sched_scan_plan {
    pub interval_sec: __le16,
    pub num_of_iterations: __le16,
    pub __packed: },
// WMI_START_SCHED_SCAN_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_start_sched_scan_cmd {
    pub ssid_for_match: [wmi_sched_scan_ssid_match; WMI_MAX_PNO_SSID_NUM],
    pub num_of_ssids: u8,
    pub min_rssi_threshold: i8,
    pub channel_list: [u8; WMI_MAX_CHANNEL_NUM],
    pub num_of_channels: u8,
    pub reserved: u8,
    pub initial_delay_sec: __le16,
    pub scan_plans: [wmi_sched_scan_plan; WMI_MAX_PLANS_NUM],
    pub __packed: },
// WMI_FT_AUTH_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ft_auth_cmd {
    pub bssid: [u8; WMI_MAC_LEN],
// enum wmi_channel
    pub channel: u8,
// enum wmi_channel
    pub edmg_channel: u8,
    pub reserved: [u8; 4],
    pub __packed: },
// WMI_FT_REASSOC_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ft_reassoc_cmd {
    pub bssid: [u8; WMI_MAC_LEN],
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_UPDATE_FT_IES_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_update_ft_ies_cmd {
// Length of the FT IEs
    pub ie_len: __le16,
    pub reserved: [u8; 2],
    pub ie_info: [u8; ],
    pub __packed: },
// WMI_SET_PROBED_SSID_CMDID

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ssid_flag {
// disables entry
    WMI_SSID_FLAG_DISABLE	= 0x00,
// probes specified ssid
    WMI_SSID_FLAG_SPECIFIC	= 0x01,
// probes for any ssid
    WMI_SSID_FLAG_ANY	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_probed_ssid_cmd {
// 0 to MAX_PROBED_SSID_INDEX
    pub entry_index: u8,
// enum wmi_ssid_flag
    pub flag: u8,
    pub ssid_len: u8,
    pub ssid: [u8; WMI_MAX_SSID_LEN],
    pub __packed: },
// WMI_SET_APPIE_CMDID
// Add Application specified IE to a management frame
//

// Frame Types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_mgmt_frame_type {
    WMI_FRAME_BEACON	= 0x00,
    WMI_FRAME_PROBE_REQ	= 0x01,
    WMI_FRAME_PROBE_RESP	= 0x02,
    WMI_FRAME_ASSOC_REQ	= 0x03,
    WMI_FRAME_ASSOC_RESP	= 0x04,
    WMI_NUM_MGMT_FRAME	= 0x05,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_appie_cmd {
// enum wmi_mgmt_frame_type
    pub mgmt_frm_type: u8,
    pub reserved: u8,
// Length of the IE to be added to MGMT frame
    pub ie_len: __le16,
    pub ie_info: [u8; ],
    pub __packed: },
// WMI_PXMT_RANGE_CFG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pxmt_range_cfg_cmd {
    pub dst_mac: [u8; WMI_MAC_LEN],
    pub range: __le16,
    pub __packed: },
// WMI_PXMT_SNR2_RANGE_CFG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pxmt_snr2_range_cfg_cmd {
    pub snr2range_arr: [i8; 2],
    pub __packed: },
// WMI_RADAR_GENERAL_CONFIG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_general_config_cmd {
// Number of pulses (CIRs) in FW FIFO to initiate pulses transfer
// from FW to Host
//
    pub fifo_watermark: __le32,
// In unit of us, in the range [100, 1000000]
    pub t_burst: __le32,
// Valid in the range [1, 32768], 0xFFFF means infinite
    pub n_bursts: __le32,
// In unit of 330Mhz clk, in the range [4, 2000]*330
    pub t_pulse: __le32,
// In the range of [1,4096]
    pub n_pulses: __le16,
// Number of taps after cTap per CIR
    pub n_samples: __le16,
// Offset from the main tap (0 = zero-distance). In the range of [0,
// 255]
//
    pub first_sample_offset: u8,
// Number of Pulses to average, 1, 2, 4, 8
    pub pulses_to_avg: u8,
// Number of adjacent taps to average, 1, 2, 4, 8
    pub samples_to_avg: u8,
// The index to config general params
    pub general_index: u8,
    pub reserved: [u8; 4],
    pub __packed: },
// WMI_RADAR_CONFIG_SELECT_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_config_select_cmd {
// Select the general params index to use
    pub general_index: u8,
    pub reserved: [u8; 3],
// 0 means don't update burst_active_vector
    pub burst_active_vector: __le32,
// 0 means don't update pulse_active_vector
    pub pulse_active_vector: __le32,
    pub __packed: },
// WMI_RADAR_PARAMS_CONFIG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_params_config_cmd {
// The burst index selected to config
    pub burst_index: u8,
// 0-not active, 1-active
    pub burst_en: u8,
// The pulse index selected to config
    pub pulse_index: u8,
// 0-not active, 1-active
    pub pulse_en: u8,
// TX RF to use on current pulse
    pub tx_rfc_idx: u8,
    pub tx_sector: u8,
// Offset from calibrated value.(expected to be 0)(value is row in
// Gain-LUT, not dB)
//
    pub tx_rf_gain_comp: i8,
// expected to be 0
    pub tx_bb_gain_comp: i8,
// RX RF to use on current pulse
    pub rx_rfc_idx: u8,
    pub rx_sector: u8,
// Offset from calibrated value.(expected to be 0)(value is row in
// Gain-LUT, not dB)
//
    pub rx_rf_gain_comp: i8,
// Value in dB.(expected to be 0)
    pub rx_bb_gain_comp: i8,
// Offset from calibrated value.(expected to be 0)
    pub rx_timing_offset: i8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RADAR_SET_MODE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_set_mode_cmd {
// 0-disable/1-enable
    pub enable: u8,
// enum wmi_channel
    pub channel: u8,
// In the range of [0,7], 0xff means use default
    pub tx_rfc_idx: u8,
// In the range of [0,7], 0xff means use default
    pub rx_rfc_idx: u8,
    pub __packed: },
// WMI_RADAR_CONTROL_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_control_cmd {
// 0-stop/1-start
    pub start: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RADAR_PCI_CONTROL_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_pci_control_cmd {
// pcie host buffer start address
    pub base_addr: __le64,
// pcie host control block address
    pub control_block_addr: __le64,
// pcie host buffer size
    pub buffer_size: __le32,
    pub reserved: __le32,
    pub __packed: },
// WMI_RF_MGMT_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_rf_mgmt_type {
    WMI_RF_MGMT_W_DISABLE	= 0x00,
    WMI_RF_MGMT_W_ENABLE	= 0x01,
    WMI_RF_MGMT_GET_STATUS	= 0x02,
}

// WMI_BF_CONTROL_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_bf_triggers {
    WMI_BF_TRIGGER_RS_MCS1_TH_FAILURE		= 0x01,
    WMI_BF_TRIGGER_RS_MCS1_NO_BACK_FAILURE		= 0x02,
    WMI_BF_TRIGGER_MAX_CTS_FAILURE_IN_TXOP		= 0x04,
    WMI_BF_TRIGGER_MAX_BACK_FAILURE			= 0x08,
    WMI_BF_TRIGGER_FW				= 0x10,
    WMI_BF_TRIGGER_MAX_CTS_FAILURE_IN_KEEP_ALIVE	= 0x20,
    WMI_BF_TRIGGER_AOA				= 0x40,
    WMI_BF_TRIGGER_MAX_CTS_FAILURE_IN_UPM		= 0x80,
}

// WMI_RF_MGMT_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rf_mgmt_cmd {
    pub rf_mgmt_type: __le32,
    pub __packed: },
// WMI_CORR_MEASURE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_corr_measure_cmd {
    pub freq_mhz: __le32,
    pub length_samples: __le32,
    pub iterations: __le32,
    pub __packed: },
// WMI_SET_SSID_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_ssid_cmd {
    pub ssid_len: __le32,
    pub ssid: [u8; WMI_MAX_SSID_LEN],
    pub __packed: },
// WMI_SET_PCP_CHANNEL_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_pcp_channel_cmd {
    pub channel: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_BCON_CTRL_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcon_ctrl_cmd {
    pub bcon_interval: __le16,
    pub frag_num: __le16,
    pub ss_mask: __le64,
    pub network_type: u8,
    pub pcp_max_assoc_sta: u8,
    pub disable_sec_offload: u8,
    pub disable_sec: u8,
    pub hidden_ssid: u8,
    pub is_go: u8,
// A-BFT length override if non-0
    pub abft_len: u8,
    pub reserved: u8,
    pub __packed: },
// WMI_PORT_ALLOCATE_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_port_role {
    WMI_PORT_STA		= 0x00,
    WMI_PORT_PCP		= 0x01,
    WMI_PORT_AP		= 0x02,
    WMI_PORT_P2P_DEV	= 0x03,
    WMI_PORT_P2P_CLIENT	= 0x04,
    WMI_PORT_P2P_GO		= 0x05,
}

// WMI_PORT_ALLOCATE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_port_allocate_cmd {
    pub mac: [u8; WMI_MAC_LEN],
    pub port_role: u8,
    pub mid: u8,
    pub __packed: },
// WMI_PORT_DELETE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_port_delete_cmd {
    pub mid: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_TRAFFIC_SUSPEND_CMD wakeup trigger bit mask values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_wakeup_trigger {
    WMI_WAKEUP_TRIGGER_UCAST	= 0x01,
    WMI_WAKEUP_TRIGGER_BCAST	= 0x02,
}

// WMI_TRAFFIC_SUSPEND_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_traffic_suspend_cmd {
// Bit vector: bit[0] - wake on Unicast, bit[1] - wake on Broadcast
    pub wakeup_trigger: u8,
    pub __packed: },
// WMI_P2P_CFG_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_discovery_mode {
    WMI_DISCOVERY_MODE_NON_OFFLOAD	= 0x00,
    WMI_DISCOVERY_MODE_OFFLOAD	= 0x01,
    WMI_DISCOVERY_MODE_PEER2PEER	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_cfg_cmd {
// enum wmi_discovery_mode
    pub discovery_mode: u8,
    pub channel: u8,
// base to listen/search duration calculation
    pub bcon_interval: __le16,
    pub __packed: },
// WMI_POWER_MGMT_CFG_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_power_source_type {
    WMI_POWER_SOURCE_BATTERY	= 0x00,
    WMI_POWER_SOURCE_OTHER		= 0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_power_mgmt_cfg_cmd {
// enum wmi_power_source_type
    pub power_source: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_PCP_START_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ap_sme_offload_mode {
// Full AP SME in FW
    WMI_AP_SME_OFFLOAD_FULL		= 0x00,
// Probe AP SME in FW
    WMI_AP_SME_OFFLOAD_PARTIAL	= 0x01,
// AP SME in host
    WMI_AP_SME_OFFLOAD_NONE		= 0x02,
}

// WMI_PCP_START_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pcp_start_cmd {
    pub bcon_interval: __le16,
    pub pcp_max_assoc_sta: u8,
    pub hidden_ssid: u8,
    pub is_go: u8,
// enum wmi_channel WMI_CHANNEL_9..WMI_CHANNEL_12
    pub edmg_channel: u8,
    pub raw_mode: u8,
    pub reserved: [u8; 3],
// A-BFT length override if non-0
    pub abft_len: u8,
// enum wmi_ap_sme_offload_mode_e
    pub ap_sme_offload_mode: u8,
    pub network_type: u8,
// enum wmi_channel WMI_CHANNEL_1..WMI_CHANNEL_6; for EDMG this is
// the primary channel number
//
    pub channel: u8,
    pub disable_sec_offload: u8,
    pub disable_sec: u8,
    pub __packed: },
// WMI_SW_TX_REQ_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sw_tx_req_cmd {
    pub dst_mac: [u8; WMI_MAC_LEN],
    pub len: __le16,
    pub payload: [u8; ],
    pub __packed: },
// WMI_SW_TX_REQ_EXT_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sw_tx_req_ext_cmd {
    pub dst_mac: [u8; WMI_MAC_LEN],
    pub len: __le16,
    pub duration_ms: __le16,
// Channel to use, 0xFF for currently active channel
    pub channel: u8,
    pub reserved: [u8; 5],
    pub payload: [u8; ],
    pub __packed: },
// WMI_VRING_SWITCH_TIMING_CONFIG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vring_switch_timing_config_cmd {
// Set vring timing configuration:
//
// defined interval for vring switch
//
    pub interval_usec: __le32,
// vring inactivity threshold
    pub idle_th_usec: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sw_ring_cfg {
    pub ring_mem_base: __le64,
    pub ring_size: __le16,
    pub max_mpdu_size: __le16,
    pub __packed: },
// wmi_vring_cfg_schd
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vring_cfg_schd {
    pub priority: __le16,
    pub timeslot_us: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vring_cfg_encap_trans_type {
    WMI_VRING_ENC_TYPE_802_3	= 0x00,
    WMI_VRING_ENC_TYPE_NATIVE_WIFI	= 0x01,
    WMI_VRING_ENC_TYPE_NONE		= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vring_cfg_ds_cfg {
    WMI_VRING_DS_PBSS	= 0x00,
    WMI_VRING_DS_STATION	= 0x01,
    WMI_VRING_DS_AP		= 0x02,
    WMI_VRING_DS_ADDR4	= 0x03,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vring_cfg_nwifi_ds_trans_type {
    WMI_NWIFI_TX_TRANS_MODE_NO		= 0x00,
    WMI_NWIFI_TX_TRANS_MODE_AP2PBSS		= 0x01,
    WMI_NWIFI_TX_TRANS_MODE_STA2PBSS	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vring_cfg_schd_params_priority {
    WMI_SCH_PRIO_REGULAR	= 0x00,
    WMI_SCH_PRIO_HIGH	= 0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vring_cfg {
    pub tx_sw_ring: wmi_sw_ring_cfg,
// 0-23 vrings
    pub ringid: u8,
// Used for cid less than 8. For higher cid set
// CIDXTID_EXTENDED_CID_TID here and use cid and tid members instead
//
    pub cidxtid: u8,
    pub encap_trans_type: u8,
// 802.3 DS cfg
    pub ds_cfg: u8,
    pub nwifi_ds_trans_type: u8,
    pub mac_ctrl: u8,
    pub to_resolution: u8,
    pub agg_max_wsize: u8,
    pub schd_params: wmi_vring_cfg_schd,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub cid: u8,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub tid: u8,
// Update the vring's priority for Qos purpose. Set to
// WMI_QOS_DEFAULT_PRIORITY to use MID's QoS priority
//
    pub qos_priority: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vring_cfg_cmd_action {
    WMI_VRING_CMD_ADD	= 0x00,
    WMI_VRING_CMD_MODIFY	= 0x01,
    WMI_VRING_CMD_DELETE	= 0x02,
}

// WMI_VRING_CFG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vring_cfg_cmd {
    pub action: __le32,
    pub vring_cfg: wmi_vring_cfg,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcast_vring_cfg {
    pub tx_sw_ring: wmi_sw_ring_cfg,
// 0-23 vrings
    pub ringid: u8,
    pub encap_trans_type: u8,
// 802.3 DS cfg
    pub ds_cfg: u8,
    pub nwifi_ds_trans_type: u8,
    pub __packed: },
// WMI_BCAST_VRING_CFG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcast_vring_cfg_cmd {
    pub action: __le32,
    pub vring_cfg: wmi_bcast_vring_cfg,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_edma_ring_cfg {
    pub ring_mem_base: __le64,
// size in number of items
    pub ring_size: __le16,
    pub ring_id: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_rx_msg_type {
    WMI_RX_MSG_TYPE_COMPRESSED	= 0x00,
    WMI_RX_MSG_TYPE_EXTENDED	= 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ring_add_irq_mode {
// Backwards compatibility
// for DESC ring - interrupt disabled
// for STATUS ring - interrupt enabled
//
    WMI_RING_ADD_IRQ_MODE_BWC	= 0x00,
    WMI_RING_ADD_IRQ_MODE_DISABLE	= 0x01,
    WMI_RING_ADD_IRQ_MODE_ENABLE	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tx_status_ring_add_cmd {
    pub ring_cfg: wmi_edma_ring_cfg,
    pub irq_index: u8,
// wmi_ring_add_irq_mode
    pub irq_mode: u8,
    pub reserved: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rx_status_ring_add_cmd {
    pub ring_cfg: wmi_edma_ring_cfg,
    pub irq_index: u8,
// wmi_rx_msg_type
    pub rx_msg_type: u8,
    pub reserved: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cfg_def_rx_offload_cmd {
    pub max_msdu_size: __le16,
    pub max_rx_pl_per_desc: __le16,
    pub decap_trans_type: u8,
    pub l2_802_3_offload_ctrl: u8,
    pub l2_nwifi_offload_ctrl: u8,
    pub vlan_id: u8,
    pub nwifi_ds_trans_type: u8,
    pub l3_l4_ctrl: u8,
    pub reserved: [u8; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tx_desc_ring_add_cmd {
    pub ring_cfg: wmi_edma_ring_cfg,
    pub max_msdu_size: __le16,
// Correlated status ring (0-63)
    pub status_ring_id: u8,
    pub cid: u8,
    pub tid: u8,
    pub encap_trans_type: u8,
    pub mac_ctrl: u8,
    pub to_resolution: u8,
    pub agg_max_wsize: u8,
    pub irq_index: u8,
// wmi_ring_add_irq_mode
    pub irq_mode: u8,
    pub reserved: u8,
    pub schd_params: wmi_vring_cfg_schd,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rx_desc_ring_add_cmd {
    pub ring_cfg: wmi_edma_ring_cfg,
    pub irq_index: u8,
// 0-63 status rings
    pub status_ring_id: u8,
    pub reserved: [u8; 2],
    pub sw_tail_host_addr: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bcast_desc_ring_add_cmd {
    pub ring_cfg: wmi_edma_ring_cfg,
    pub max_msdu_size: __le16,
// Correlated status ring (0-63)
    pub status_ring_id: u8,
    pub encap_trans_type: u8,
    pub reserved: [u8; 4],
    pub __packed: },
// WMI_LO_POWER_CALIB_FROM_OTP_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_lo_power_calib_from_otp_cmd {
// index to read from OTP. zero based
    pub index: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_LO_POWER_CALIB_FROM_OTP_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_lo_power_calib_from_otp_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RING_BA_EN_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ring_ba_en_cmd {
    pub ring_id: u8,
    pub agg_max_wsize: u8,
    pub ba_timeout: __le16,
    pub amsdu: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RING_BA_DIS_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ring_ba_dis_cmd {
    pub ring_id: u8,
    pub reserved: u8,
    pub reason: __le16,
    pub __packed: },
// WMI_NOTIFY_REQ_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_notify_req_cmd {
    pub cid: u8,
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub interval_usec: __le32,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub miliseconds: u8,
    pub __packed: },
// WMI_CFG_RX_CHAIN_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sniffer_cfg_mode {
    WMI_SNIFFER_OFF	= 0x00,
    WMI_SNIFFER_ON	= 0x01,
}

// WMI_SILENT_RSSI_TABLE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_silent_rssi_table {
    RF_TEMPERATURE_CALIB_DEFAULT_DB		= 0x00,
    RF_TEMPERATURE_CALIB_HIGH_POWER_DB	= 0x01,
}

// WMI_SILENT_RSSI_STATUS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_silent_rssi_status {
    SILENT_RSSI_SUCCESS	= 0x00,
    SILENT_RSSI_FAILURE	= 0x01,
}

// WMI_SET_ACTIVE_SILENT_RSSI_TABLE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_active_silent_rssi_table_cmd {
// enum wmi_silent_rssi_table
    pub table: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sniffer_cfg_phy_info_mode {
    WMI_SNIFFER_PHY_INFO_DISABLED	= 0x00,
    WMI_SNIFFER_PHY_INFO_ENABLED	= 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sniffer_cfg_phy_support {
    WMI_SNIFFER_CP		= 0x00,
    WMI_SNIFFER_DP		= 0x01,
    WMI_SNIFFER_BOTH_PHYS	= 0x02,
}

// wmi_sniffer_cfg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sniffer_cfg {
// enum wmi_sniffer_cfg_mode
    pub mode: __le32,
// enum wmi_sniffer_cfg_phy_info_mode
    pub phy_info_mode: __le32,
// enum wmi_sniffer_cfg_phy_support
    pub phy_support: __le32,
    pub channel: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_cfg_rx_chain_cmd_action {
    WMI_RX_CHAIN_ADD	= 0x00,
    WMI_RX_CHAIN_DEL	= 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_cfg_rx_chain_cmd_decap_trans_type {
    WMI_DECAP_TYPE_802_3		= 0x00,
    WMI_DECAP_TYPE_NATIVE_WIFI	= 0x01,
    WMI_DECAP_TYPE_NONE		= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_cfg_rx_chain_cmd_nwifi_ds_trans_type {
    WMI_NWIFI_RX_TRANS_MODE_NO		= 0x00,
    WMI_NWIFI_RX_TRANS_MODE_PBSS2AP		= 0x01,
    WMI_NWIFI_RX_TRANS_MODE_PBSS2STA	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_cfg_rx_chain_cmd_reorder_type {
    WMI_RX_HW_REORDER	= 0x00,
    WMI_RX_SW_REORDER	= 0x01,
}

// WMI_CFG_RX_CHAIN_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cfg_rx_chain_cmd {
    pub action: __le32,
    pub rx_sw_ring: wmi_sw_ring_cfg,
    pub mid: u8,
    pub decap_trans_type: u8,
    pub l2_802_3_offload_ctrl: u8,
    pub l2_nwifi_offload_ctrl: u8,
    pub vlan_id: u8,
    pub nwifi_ds_trans_type: u8,
    pub l3_l4_ctrl: u8,
    pub ring_ctrl: u8,
    pub prefetch_thrsh: __le16,
    pub wb_thrsh: __le16,
    pub itr_value: __le32,
    pub host_thrsh: __le16,
    pub reorder_type: u8,
    pub reserved: u8,
    pub sniffer_cfg: wmi_sniffer_cfg,
    pub max_rx_pl_per_desc: __le16,
    pub __packed: },
// WMI_RCP_ADDBA_RESP_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rcp_addba_resp_cmd {
// Used for cid less than 8. For higher cid set
// CIDXTID_EXTENDED_CID_TID here and use cid and tid members instead
//
    pub cidxtid: u8,
    pub dialog_token: u8,
    pub status_code: __le16,
// ieee80211_ba_parameterset field to send
    pub ba_param_set: __le16,
    pub ba_timeout: __le16,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub cid: u8,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub tid: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_RCP_ADDBA_RESP_EDMA_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rcp_addba_resp_edma_cmd {
    pub cid: u8,
    pub tid: u8,
    pub dialog_token: u8,
    pub reserved: u8,
    pub status_code: __le16,
// ieee80211_ba_parameterset field to send
    pub ba_param_set: __le16,
    pub ba_timeout: __le16,
    pub status_ring_id: u8,
// wmi_cfg_rx_chain_cmd_reorder_type
    pub reorder_type: u8,
    pub __packed: },
// WMI_RCP_DELBA_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rcp_delba_cmd {
// Used for cid less than 8. For higher cid set
// CIDXTID_EXTENDED_CID_TID here and use cid and tid members instead
//
    pub cidxtid: u8,
    pub reserved: u8,
    pub reason: __le16,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub cid: u8,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub tid: u8,
    pub reserved2: [u8; 2],
    pub __packed: },
// WMI_RCP_ADDBA_REQ_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rcp_addba_req_cmd {
// Used for cid less than 8. For higher cid set
// CIDXTID_EXTENDED_CID_TID here and use cid and tid members instead
//
    pub cidxtid: u8,
    pub dialog_token: u8,
// ieee80211_ba_parameterset field as it received
    pub ba_param_set: __le16,
    pub ba_timeout: __le16,
// ieee80211_ba_seqstrl field as it received
    pub ba_seq_ctrl: __le16,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub cid: u8,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub tid: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_SET_MAC_ADDRESS_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_mac_address_cmd {
    pub mac: [u8; WMI_MAC_LEN],
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_ECHO_CMDID
// Check FW is alive
// Returned event: WMI_ECHO_RSP_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_echo_cmd {
    pub value: __le32,
    pub __packed: },
// WMI_DEEP_ECHO_CMDID
// Check FW and uCode is alive
// Returned event: WMI_DEEP_ECHO_RSP_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_deep_echo_cmd {
    pub value: __le32,
    pub __packed: },
// WMI_RF_PWR_ON_DELAY_CMDID
// set FW time parameters used through RF resetting
// RF reset consists of bringing its power down for a period of time, then
// bringing the power up
// Returned event: WMI_RF_PWR_ON_DELAY_RSP_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rf_pwr_on_delay_cmd {
// time in usec the FW waits after bringing the RF PWR down,
// set 0 for default
//
    pub down_delay_usec: __le16,
// time in usec the FW waits after bringing the RF PWR up,
// set 0 for default
//
    pub up_delay_usec: __le16,
    pub __packed: },
// WMI_SET_HIGH_POWER_TABLE_PARAMS_CMDID
// This API controls the Tx and Rx gain over temperature.
// It controls the Tx D-type, Rx D-type and Rx E-type amplifiers.
// It also controls the Tx gain index, by controlling the Rx to Tx gain index
// offset.
// The control is divided by 3 temperature values to 4 temperature ranges.
// Each parameter uses its own temperature values.
// Returned event: WMI_SET_HIGH_POWER_TABLE_PARAMS_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_high_power_table_params_cmd {
// Temperature range for Tx D-type parameters
    pub tx_dtype_temp: [u8; WMI_RF_DTYPE_LENGTH],
    pub reserved0: u8,
// Tx D-type values to be used for each temperature range
    pub tx_dtype_conf: [__le32; WMI_RF_DTYPE_CONF_LENGTH],
// Temperature range for Tx E-type parameters
    pub tx_etype_temp: [u8; WMI_RF_ETYPE_LENGTH],
    pub reserved1: u8,
// Tx E-type values to be used for each temperature range.
// The last 4 values of any range are the first 4 values of the next
// range and so on
//
    pub tx_etype_conf: [__le32; WMI_RF_ETYPE_CONF_LENGTH],
// Temperature range for Rx D-type parameters
    pub rx_dtype_temp: [u8; WMI_RF_DTYPE_LENGTH],
    pub reserved2: u8,
// Rx D-type values to be used for each temperature range
    pub rx_dtype_conf: [__le32; WMI_RF_DTYPE_CONF_LENGTH],
// Temperature range for Rx E-type parameters
    pub rx_etype_temp: [u8; WMI_RF_ETYPE_LENGTH],
    pub reserved3: u8,
// Rx E-type values to be used for each temperature range.
// The last 4 values of any range are the first 4 values of the next
// range and so on
//
    pub rx_etype_conf: [__le32; WMI_RF_ETYPE_CONF_LENGTH],
// Temperature range for rx_2_tx_offs parameters
    pub rx_2_tx_temp: [u8; WMI_RF_RX2TX_LENGTH],
    pub reserved4: u8,
// Rx to Tx gain index offset
    pub rx_2_tx_offs: [i8; WMI_RF_RX2TX_CONF_LENGTH],
    pub __packed: },
// WMI_FIXED_SCHEDULING_UL_CONFIG_CMDID
// This API sets rd parameter per mcs.
// Relevant only in Fixed Scheduling mode.
// Returned event: WMI_FIXED_SCHEDULING_UL_CONFIG_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_fixed_scheduling_ul_config_cmd {
// Use mcs -1 to set for every mcs
    pub mcs: i8,
// Number of frames with rd bit set in a single virtual slot
    pub rd_count_per_slot: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// CMD: WMI_RF_XPM_READ_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rf_xpm_read_cmd {
    pub rf_id: u8,
    pub reserved: [u8; 3],
// XPM bit start address in range [0,8191]bits - rounded by FW to
// multiple of 8bits
//
    pub xpm_bit_address: __le32,
    pub num_bytes: __le32,
    pub __packed: },
// CMD: WMI_RF_XPM_WRITE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rf_xpm_write_cmd {
    pub rf_id: u8,
    pub reserved0: [u8; 3],
// XPM bit start address in range [0,8191]bits - rounded by FW to
// multiple of 8bits
//
    pub xpm_bit_address: __le32,
    pub num_bytes: __le32,
// boolean flag indicating whether FW should verify the write
// operation
//
    pub verify: u8,
    pub reserved1: [u8; 3],
// actual size=num_bytes
    pub data_bytes: [u8; ],
    pub __packed: },
// Possible modes for temperature measurement
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_temperature_measure_mode {
    TEMPERATURE_USE_OLD_VALUE	= 0x01,
    TEMPERATURE_MEASURE_NOW		= 0x02,
}

// WMI_TEMP_SENSE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_temp_sense_cmd {
    pub measure_baseband_en: __le32,
    pub measure_rf_en: __le32,
    pub measure_mode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_pmc_op {
    WMI_PMC_ALLOCATE	= 0x00,
    WMI_PMC_RELEASE		= 0x01,
}

// WMI_PMC_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pmc_cmd {
// enum wmi_pmc_cmd_op_type
    pub op: u8,
    pub reserved: u8,
    pub ring_size: __le16,
    pub mem_base: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_aoa_meas_type {
    WMI_AOA_PHASE_MEAS	= 0x00,
    WMI_AOA_PHASE_AMP_MEAS	= 0x01,
}

// WMI_AOA_MEAS_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_aoa_meas_cmd {
    pub mac_addr: [u8; WMI_MAC_LEN],
// channels IDs:
// 0 - 58320 MHz
// 1 - 60480 MHz
// 2 - 62640 MHz
//
    pub channel: u8,
// enum wmi_aoa_meas_type
    pub aoa_meas_type: u8,
    pub meas_rf_mask: __le32,
    pub __packed: },
// WMI_SET_MGMT_RETRY_LIMIT_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_mgmt_retry_limit_cmd {
// MAC retransmit limit for mgmt frames
    pub mgmt_retry_limit: u8,
// alignment to 32b
    pub reserved: [u8; 3],
    pub __packed: },
// Zones: HIGH, MAX, CRITICAL

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tt_zone_limits {
// Above this temperature this zone is active
    pub temperature_high: u8,
// Below this temperature the adjacent lower zone is active
    pub temperature_low: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// Struct used for both configuration and status commands of thermal
// throttling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tt_data {
// Enable/Disable TT algorithm for baseband
    pub bb_enabled: u8,
    pub reserved0: [u8; 3],
// Define zones for baseband
    pub bb_zones: [wmi_tt_zone_limits; WMI_NUM_OF_TT_ZONES],
// Enable/Disable TT algorithm for radio
    pub rf_enabled: u8,
    pub reserved1: [u8; 3],
// Define zones for all radio chips
    pub rf_zones: [wmi_tt_zone_limits; WMI_NUM_OF_TT_ZONES],
    pub __packed: },
// WMI_SET_THERMAL_THROTTLING_CFG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_thermal_throttling_cfg_cmd {
// Command data
    pub tt_data: wmi_tt_data,
    pub __packed: },
// WMI_NEW_STA_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_new_sta_cmd {
    pub dst_mac: [u8; WMI_MAC_LEN],
    pub aid: u8,
    pub __packed: },
// WMI_DEL_STA_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_del_sta_cmd {
    pub dst_mac: [u8; WMI_MAC_LEN],
    pub disconnect_reason: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tof_burst_duration {
    WMI_TOF_BURST_DURATION_250_USEC		= 2,
    WMI_TOF_BURST_DURATION_500_USEC		= 3,
    WMI_TOF_BURST_DURATION_1_MSEC		= 4,
    WMI_TOF_BURST_DURATION_2_MSEC		= 5,
    WMI_TOF_BURST_DURATION_4_MSEC		= 6,
    WMI_TOF_BURST_DURATION_8_MSEC		= 7,
    WMI_TOF_BURST_DURATION_16_MSEC		= 8,
    WMI_TOF_BURST_DURATION_32_MSEC		= 9,
    WMI_TOF_BURST_DURATION_64_MSEC		= 10,
    WMI_TOF_BURST_DURATION_128_MSEC		= 11,
    WMI_TOF_BURST_DURATION_NO_PREFERENCES	= 15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tof_session_start_flags {
    WMI_TOF_SESSION_START_FLAG_SECURED	= 0x1,
    WMI_TOF_SESSION_START_FLAG_ASAP		= 0x2,
    WMI_TOF_SESSION_START_FLAG_LCI_REQ	= 0x4,
    WMI_TOF_SESSION_START_FLAG_LCR_REQ	= 0x8,
}

// WMI_TOF_SESSION_START_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ftm_dest_info {
    pub channel: u8,
// wmi_tof_session_start_flags_e
    pub flags: u8,
    pub initial_token: u8,
    pub num_of_ftm_per_burst: u8,
    pub num_of_bursts_exp: u8,
// wmi_tof_burst_duration_e
    pub burst_duration: u8,
// Burst Period indicate interval between two consecutive burst
// instances, in units of 100 ms
//
    pub burst_period: __le16,
    pub dst_mac: [u8; WMI_MAC_LEN],
    pub reserved: u8,
    pub num_burst_per_aoa_meas: u8,
    pub __packed: },
// WMI_TOF_SESSION_START_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_session_start_cmd {
    pub session_id: __le32,
    pub reserved1: u8,
    pub aoa_type: u8,
    pub num_of_dest: __le16,
    pub reserved: [u8; 4],
    pub ftm_dest_info: [wmi_ftm_dest_info; ],
    pub __packed: },
// WMI_TOF_CFG_RESPONDER_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_cfg_responder_cmd {
    pub enable: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tof_channel_info_report_type {
    WMI_TOF_CHANNEL_INFO_TYPE_CIR			= 0x1,
    WMI_TOF_CHANNEL_INFO_TYPE_RSSI			= 0x2,
    WMI_TOF_CHANNEL_INFO_TYPE_SNR			= 0x4,
    WMI_TOF_CHANNEL_INFO_TYPE_DEBUG_DATA		= 0x8,
    WMI_TOF_CHANNEL_INFO_TYPE_VENDOR_SPECIFIC	= 0x10,
}

// WMI_TOF_CHANNEL_INFO_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_channel_info_cmd {
// wmi_tof_channel_info_report_type_e
    pub channel_info_report_request: __le32,
    pub __packed: },
// WMI_TOF_SET_TX_RX_OFFSET_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_set_tx_rx_offset_cmd {
// TX delay offset
    pub tx_offset: __le32,
// RX delay offset
    pub rx_offset: __le32,
// Mask to define which RFs to configure. 0 means all RFs
    pub rf_mask: __le32,
// Offset to strongest tap of CIR
    pub precursor: __le32,
    pub __packed: },
// WMI_TOF_GET_TX_RX_OFFSET_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_get_tx_rx_offset_cmd {
// rf index to read offsets from
    pub rf_index: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_FIXED_SCHEDULING_CONFIG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_map_mcs_to_schd_params {
    pub mcs: u8,
// time in usec from start slot to start tx flow - default 15
    pub time_in_usec_before_initiate_tx: u8,
// RD enable - if yes consider RD according to STA mcs
    pub rd_enabled: u8,
    pub reserved: u8,
// time in usec from start slot to stop vring
    pub time_in_usec_to_stop_vring: __le16,
// timeout to force flush from start of slot
    pub flush_to_in_usec: __le16,
// per mcs the mac buffer limit size in bytes
    pub mac_buff_size_in_bytes: __le32,
    pub __packed: },
// WMI_FIXED_SCHEDULING_CONFIG_COMPLETE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_fixed_scheduling_config_complete_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// This value exists for backwards compatibility only.
// Do not use it in new commands.
// Use dynamic arrays where possible.
//

// WMI_FIXED_SCHEDULING_CONFIG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_fixed_scheduling_config_cmd {
// defaults in the SAS table
    pub mcs_to_schd_params_map: [wmi_map_mcs_to_schd_params; WMI_NUM_MCS],
// default 150 uSec
    pub max_sta_rd_ppdu_duration_in_usec: __le16,
// default 300 uSec
    pub max_sta_grant_ppdu_duration_in_usec: __le16,
// default 1000 uSec
    pub assoc_slot_duration_in_usec: __le16,
// default 360 uSec
    pub virtual_slot_duration_in_usec: __le16,
// each this field value slots start with grant frame to the station
// - default 2
//
    pub number_of_ap_slots_for_initiate_grant: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_ENABLE_FIXED_SCHEDULING_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_enable_fixed_scheduling_cmd {
    pub reserved: __le32,
    pub __packed: },
// WMI_ENABLE_FIXED_SCHEDULING_COMPLETE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_enable_fixed_scheduling_complete_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_MULTI_DIRECTED_OMNIS_CONFIG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_multi_directed_omnis_config_cmd {
// number of directed omnis at destination AP
    pub dest_ap_num_directed_omnis: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_MULTI_DIRECTED_OMNIS_CONFIG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_multi_directed_omnis_config_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RADAR_GENERAL_CONFIG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_general_config_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RADAR_CONFIG_SELECT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_config_select_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
// In unit of bytes
    pub fifo_size: __le32,
// In unit of bytes
    pub pulse_size: __le32,
    pub __packed: },
// WMI_RADAR_PARAMS_CONFIG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_params_config_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RADAR_SET_MODE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_set_mode_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RADAR_CONTROL_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_control_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RADAR_PCI_CONTROL_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_pci_control_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_LONG_RANGE_CONFIG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_long_range_config_cmd {
    pub reserved: __le32,
    pub __packed: },
// WMI_SET_LONG_RANGE_CONFIG_COMPLETE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_long_range_config_complete_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// payload max size is 1024 bytes: max event buffer size (1044) - WMI headers
// (16) - prev struct field size (4)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_internal_fw_ioctl_code {
    WMI_INTERNAL_FW_CODE_NONE	= 0x0,
    WMI_INTERNAL_FW_CODE_QCOM	= 0x1,
}

// WMI_INTERNAL_FW_IOCTL_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_internal_fw_ioctl_cmd {
// enum wmi_internal_fw_ioctl_code
    pub code: __le16,
    pub length: __le16,
// payload max size is WMI_MAX_IOCTL_PAYLOAD_SIZE
// Must be the last member of the struct
//
    pub payload: [__le32; ],
    pub __packed: },
// WMI_INTERNAL_FW_IOCTL_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_internal_fw_ioctl_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: u8,
    pub length: __le16,
// payload max size is WMI_MAX_IOCTL_REPLY_PAYLOAD_SIZE
// Must be the last member of the struct
//
    pub payload: [__le32; ],
    pub __packed: },
// WMI_INTERNAL_FW_EVENT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_internal_fw_event_event {
    pub id: __le16,
    pub length: __le16,
// payload max size is WMI_MAX_INTERNAL_EVENT_PAYLOAD_SIZE
// Must be the last member of the struct
//
    pub payload: [__le32; ],
    pub __packed: },
// WMI_SET_VRING_PRIORITY_WEIGHT_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_vring_priority_weight_cmd {
// Array of weights. Valid values are
// WMI_QOS_MIN_DEFAULT_WEIGHT...WMI_QOS_MAX_WEIGHT. Weight #0 is
// hard-coded WMI_QOS_MIN_WEIGHT. This array provide the weights
// #1..#3
//
    pub weight: [u8; 3],
    pub reserved: u8,
    pub __packed: },
// WMI_SET_VRING_PRIORITY_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vring_priority {
    pub vring_idx: u8,
// Weight index. Valid value is 0-3
    pub priority: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_SET_VRING_PRIORITY_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_vring_priority_cmd {
// number of entries in vring_priority. Set to
// WMI_QOS_SET_VIF_PRIORITY to update the VIF's priority, and there
// will be only one entry in vring_priority
//
    pub num_of_vrings: u8,
    pub reserved: [u8; 3],
    pub vring_priority: [wmi_vring_priority; ],
    pub __packed: },
// WMI_BF_CONTROL_CMDID - deprecated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bf_control_cmd {
// wmi_bf_triggers
    pub triggers: __le32,
    pub cid: u8,
// DISABLED = 0, ENABLED = 1 , DRY_RUN = 2
    pub txss_mode: u8,
// DISABLED = 0, ENABLED = 1, DRY_RUN = 2
    pub brp_mode: u8,
// Max cts threshold (correspond to
// WMI_BF_TRIGGER_MAX_CTS_FAILURE_IN_TXOP)
//
    pub bf_trigger_max_cts_failure_thr: u8,
// Max cts threshold in dense (correspond to
// WMI_BF_TRIGGER_MAX_CTS_FAILURE_IN_TXOP)
//
    pub bf_trigger_max_cts_failure_dense_thr: u8,
// Max b-ack threshold (correspond to
// WMI_BF_TRIGGER_MAX_BACK_FAILURE)
//
    pub bf_trigger_max_back_failure_thr: u8,
// Max b-ack threshold in dense (correspond to
// WMI_BF_TRIGGER_MAX_BACK_FAILURE)
//
    pub bf_trigger_max_back_failure_dense_thr: u8,
    pub reserved0: u8,
// Wrong sectors threshold
    pub wrong_sector_bis_thr: __le32,
// BOOL to enable/disable long term trigger
    pub long_term_enable: u8,
// 1 = Update long term thresholds from the long_term_mbps_th_tbl and
// long_term_trig_timeout_per_mcs arrays, 0 = Ignore
//
    pub long_term_update_thr: u8,
// Long term throughput threshold [Mbps]
    pub long_term_mbps_th_tbl: [u8; WMI_NUM_MCS],
    pub reserved1: u8,
// Long term timeout threshold table [msec]
    pub long_term_trig_timeout_per_mcs: [__le16; WMI_NUM_MCS],
    pub reserved2: [u8; 2],
    pub __packed: },
// BF configuration for each MCS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bf_control_ex_mcs {
// Long term throughput threshold [Mbps]
    pub long_term_mbps_th_tbl: u8,
    pub reserved: u8,
// Long term timeout threshold table [msec]
    pub long_term_trig_timeout_per_mcs: __le16,
    pub __packed: },
// WMI_BF_CONTROL_EX_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bf_control_ex_cmd {
// wmi_bf_triggers
    pub triggers: __le32,
// enum wmi_edmg_tx_mode
    pub tx_mode: u8,
// DISABLED = 0, ENABLED = 1 , DRY_RUN = 2
    pub txss_mode: u8,
// DISABLED = 0, ENABLED = 1, DRY_RUN = 2
    pub brp_mode: u8,
// Max cts threshold (correspond to
// WMI_BF_TRIGGER_MAX_CTS_FAILURE_IN_TXOP)
//
    pub bf_trigger_max_cts_failure_thr: u8,
// Max cts threshold in dense (correspond to
// WMI_BF_TRIGGER_MAX_CTS_FAILURE_IN_TXOP)
//
    pub bf_trigger_max_cts_failure_dense_thr: u8,
// Max b-ack threshold (correspond to
// WMI_BF_TRIGGER_MAX_BACK_FAILURE)
//
    pub bf_trigger_max_back_failure_thr: u8,
// Max b-ack threshold in dense (correspond to
// WMI_BF_TRIGGER_MAX_BACK_FAILURE)
//
    pub bf_trigger_max_back_failure_dense_thr: u8,
    pub reserved0: u8,
// Wrong sectors threshold
    pub wrong_sector_bis_thr: __le32,
// BOOL to enable/disable long term trigger
    pub long_term_enable: u8,
// 1 = Update long term thresholds from the long_term_mbps_th_tbl and
// long_term_trig_timeout_per_mcs arrays, 0 = Ignore
//
    pub long_term_update_thr: u8,
    pub each_mcs_cfg_size: u8,
    pub reserved1: u8,
// Configuration for each MCS
    pub each_mcs_cfg: [wmi_bf_control_ex_mcs; ],
    pub __packed: },
// WMI_LINK_STATS_CMD
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_link_stats_action {
    WMI_LINK_STATS_SNAPSHOT		= 0x00,
    WMI_LINK_STATS_PERIODIC		= 0x01,
    WMI_LINK_STATS_STOP_PERIODIC	= 0x02,
}

// WMI_LINK_STATS_EVENT record identifiers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_link_stats_record_type {
    WMI_LINK_STATS_TYPE_BASIC	= 0x01,
    WMI_LINK_STATS_TYPE_GLOBAL	= 0x02,
}

// WMI_LINK_STATS_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_stats_cmd {
// bitmask of required record types
// (wmi_link_stats_record_type_e)
//
    pub record_type_mask: __le32,
// 0xff for all cids
    pub cid: u8,
// wmi_link_stats_action_e
    pub action: u8,
    pub reserved: [u8; 6],
// range = 100 - 10000
    pub interval_msec: __le32,
    pub __packed: },
// WMI_SET_GRANT_MCS_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_grant_mcs_cmd {
    pub mcs: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_AP_SLOT_SIZE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_ap_slot_size_cmd {
    pub slot_size: __le32,
    pub __packed: },
// WMI_TEMP_SENSE_ALL_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_temp_sense_all_cmd {
    pub measure_baseband_en: u8,
    pub measure_rf_en: u8,
    pub measure_mode: u8,
    pub reserved: u8,
    pub __packed: },
// WMI Events
// List of Events (target to host)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_event_id {
    WMI_READY_EVENTID				= 0x1001,
    WMI_CONNECT_EVENTID				= 0x1002,
    WMI_DISCONNECT_EVENTID				= 0x1003,
    WMI_START_SCHED_SCAN_EVENTID			= 0x1005,
    WMI_STOP_SCHED_SCAN_EVENTID			= 0x1006,
    WMI_SCHED_SCAN_RESULT_EVENTID			= 0x1007,
    WMI_SCAN_COMPLETE_EVENTID			= 0x100A,
    WMI_REPORT_STATISTICS_EVENTID			= 0x100B,
    WMI_FT_AUTH_STATUS_EVENTID			= 0x100C,
    WMI_FT_REASSOC_STATUS_EVENTID			= 0x100D,
    WMI_LINK_MONITOR_EVENTID			= 0x100E,
    WMI_RADAR_GENERAL_CONFIG_EVENTID		= 0x1100,
    WMI_RADAR_CONFIG_SELECT_EVENTID			= 0x1101,
    WMI_RADAR_PARAMS_CONFIG_EVENTID			= 0x1102,
    WMI_RADAR_SET_MODE_EVENTID			= 0x1103,
    WMI_RADAR_CONTROL_EVENTID			= 0x1104,
    WMI_RADAR_PCI_CONTROL_EVENTID			= 0x1105,
    WMI_RD_MEM_RSP_EVENTID				= 0x1800,
    WMI_FW_READY_EVENTID				= 0x1801,
    WMI_EXIT_FAST_MEM_ACC_MODE_EVENTID		= 0x200,
    WMI_ECHO_RSP_EVENTID				= 0x1803,
    WMI_DEEP_ECHO_RSP_EVENTID			= 0x1804,
// deprecated
    WMI_FS_TUNE_DONE_EVENTID			= 0x180A,
// deprecated
    WMI_CORR_MEASURE_EVENTID			= 0x180B,
    WMI_READ_RSSI_EVENTID				= 0x180C,
    WMI_TEMP_SENSE_DONE_EVENTID			= 0x180E,
    WMI_DC_CALIB_DONE_EVENTID			= 0x180F,
// deprecated
    WMI_IQ_TX_CALIB_DONE_EVENTID			= 0x1811,
// deprecated
    WMI_IQ_RX_CALIB_DONE_EVENTID			= 0x1812,
    WMI_SET_WORK_MODE_DONE_EVENTID			= 0x1815,
    WMI_LO_LEAKAGE_CALIB_DONE_EVENTID		= 0x1816,
    WMI_LO_POWER_CALIB_FROM_OTP_EVENTID		= 0x1817,
    WMI_SILENT_RSSI_CALIB_DONE_EVENTID		= 0x181D,
// deprecated
    WMI_RF_RX_TEST_DONE_EVENTID			= 0x181E,
    WMI_CFG_RX_CHAIN_DONE_EVENTID			= 0x1820,
    WMI_VRING_CFG_DONE_EVENTID			= 0x1821,
    WMI_BA_STATUS_EVENTID				= 0x1823,
    WMI_RCP_ADDBA_REQ_EVENTID			= 0x1824,
    WMI_RCP_ADDBA_RESP_SENT_EVENTID			= 0x1825,
    WMI_DELBA_EVENTID				= 0x1826,
    WMI_GET_SSID_EVENTID				= 0x1828,
    WMI_GET_PCP_CHANNEL_EVENTID			= 0x182A,
// Event is shared between WMI_SW_TX_REQ_CMDID and
// WMI_SW_TX_REQ_EXT_CMDID
//
    WMI_SW_TX_COMPLETE_EVENTID			= 0x182B,
    WMI_BEAMFORMING_MGMT_DONE_EVENTID		= 0x1836,
    WMI_BF_TXSS_MGMT_DONE_EVENTID			= 0x1837,
    WMI_BF_RXSS_MGMT_DONE_EVENTID			= 0x1839,
    WMI_BF_TRIG_EVENTID				= 0x183A,
    WMI_RS_MGMT_DONE_EVENTID			= 0x1852,
    WMI_RF_MGMT_STATUS_EVENTID			= 0x1853,
    WMI_BF_SM_MGMT_DONE_EVENTID			= 0x1838,
    WMI_RX_MGMT_PACKET_EVENTID			= 0x1840,
    WMI_TX_MGMT_PACKET_EVENTID			= 0x1841,
    WMI_LINK_MAINTAIN_CFG_WRITE_DONE_EVENTID	= 0x1842,
    WMI_LINK_MAINTAIN_CFG_READ_DONE_EVENTID		= 0x1843,
    WMI_SET_LINK_MONITOR_EVENTID			= 0x1845,
    WMI_RF_XPM_READ_RESULT_EVENTID			= 0x1856,
    WMI_RF_XPM_WRITE_RESULT_EVENTID			= 0x1857,
    WMI_LED_CFG_DONE_EVENTID			= 0x1858,
    WMI_SET_SILENT_RSSI_TABLE_DONE_EVENTID		= 0x185C,
    WMI_RF_PWR_ON_DELAY_RSP_EVENTID			= 0x185D,
    WMI_SET_HIGH_POWER_TABLE_PARAMS_EVENTID		= 0x185E,
    WMI_FIXED_SCHEDULING_UL_CONFIG_EVENTID		= 0x185F,
// Performance monitoring events
    WMI_DATA_PORT_OPEN_EVENTID			= 0x1860,
    WMI_WBE_LINK_DOWN_EVENTID			= 0x1861,
    WMI_BF_CTRL_DONE_EVENTID			= 0x1862,
    WMI_NOTIFY_REQ_DONE_EVENTID			= 0x1863,
    WMI_GET_STATUS_DONE_EVENTID			= 0x1864,
    WMI_RING_EN_EVENTID				= 0x1865,
    WMI_GET_RF_STATUS_EVENTID			= 0x1866,
    WMI_GET_BASEBAND_TYPE_EVENTID			= 0x1867,
    WMI_VRING_SWITCH_TIMING_CONFIG_EVENTID		= 0x1868,
    WMI_UNIT_TEST_EVENTID				= 0x1900,
    WMI_FLASH_READ_DONE_EVENTID			= 0x1902,
    WMI_FLASH_WRITE_DONE_EVENTID			= 0x1903,
// Power management
    WMI_TRAFFIC_SUSPEND_EVENTID			= 0x1904,
    WMI_TRAFFIC_RESUME_EVENTID			= 0x1905,
// P2P
    WMI_P2P_CFG_DONE_EVENTID			= 0x1910,
    WMI_PORT_ALLOCATED_EVENTID			= 0x1911,
    WMI_PORT_DELETED_EVENTID			= 0x1912,
    WMI_LISTEN_STARTED_EVENTID			= 0x1914,
    WMI_SEARCH_STARTED_EVENTID			= 0x1915,
    WMI_DISCOVERY_STARTED_EVENTID			= 0x1916,
    WMI_DISCOVERY_STOPPED_EVENTID			= 0x1917,
    WMI_PCP_STARTED_EVENTID				= 0x1918,
    WMI_PCP_STOPPED_EVENTID				= 0x1919,
    WMI_PCP_FACTOR_EVENTID				= 0x191A,
// Power Save Configuration Events
    WMI_PS_DEV_PROFILE_CFG_EVENTID			= 0x191C,
    WMI_RS_ENABLE_EVENTID				= 0x191E,
    WMI_RS_CFG_EX_EVENTID				= 0x191F,
    WMI_GET_DETAILED_RS_RES_EX_EVENTID		= 0x1920,
// deprecated
    WMI_RS_CFG_DONE_EVENTID				= 0x1921,
// deprecated
    WMI_GET_DETAILED_RS_RES_EVENTID			= 0x1922,
    WMI_AOA_MEAS_EVENTID				= 0x1923,
    WMI_BRP_SET_ANT_LIMIT_EVENTID			= 0x1924,
    WMI_SET_MGMT_RETRY_LIMIT_EVENTID		= 0x1930,
    WMI_GET_MGMT_RETRY_LIMIT_EVENTID		= 0x1931,
    WMI_SET_THERMAL_THROTTLING_CFG_EVENTID		= 0x1940,
    WMI_GET_THERMAL_THROTTLING_CFG_EVENTID		= 0x1941,
// return the Power Save profile
    WMI_PS_DEV_PROFILE_CFG_READ_EVENTID		= 0x1942,
    WMI_TSF_SYNC_STATUS_EVENTID			= 0x1973,
    WMI_TOF_SESSION_END_EVENTID			= 0x1991,
    WMI_TOF_GET_CAPABILITIES_EVENTID		= 0x1992,
    WMI_TOF_SET_LCR_EVENTID				= 0x1993,
    WMI_TOF_SET_LCI_EVENTID				= 0x1994,
    WMI_TOF_FTM_PER_DEST_RES_EVENTID		= 0x1995,
    WMI_TOF_CFG_RESPONDER_EVENTID			= 0x1996,
    WMI_TOF_SET_TX_RX_OFFSET_EVENTID		= 0x1997,
    WMI_TOF_GET_TX_RX_OFFSET_EVENTID		= 0x1998,
    WMI_TOF_CHANNEL_INFO_EVENTID			= 0x1999,
    WMI_GET_RF_SECTOR_PARAMS_DONE_EVENTID		= 0x19A0,
    WMI_SET_RF_SECTOR_PARAMS_DONE_EVENTID		= 0x19A1,
    WMI_GET_SELECTED_RF_SECTOR_INDEX_DONE_EVENTID	= 0x19A2,
    WMI_SET_SELECTED_RF_SECTOR_INDEX_DONE_EVENTID	= 0x19A3,
    WMI_SET_RF_SECTOR_ON_DONE_EVENTID		= 0x19A4,
    WMI_PRIO_TX_SECTORS_ORDER_EVENTID		= 0x19A5,
    WMI_PRIO_TX_SECTORS_NUMBER_EVENTID		= 0x19A6,
    WMI_PRIO_TX_SECTORS_SET_DEFAULT_CFG_EVENTID	= 0x19A7,
// deprecated
    WMI_BF_CONTROL_EVENTID				= 0x19AA,
    WMI_BF_CONTROL_EX_EVENTID			= 0x19AB,
    WMI_TX_STATUS_RING_CFG_DONE_EVENTID		= 0x19C0,
    WMI_RX_STATUS_RING_CFG_DONE_EVENTID		= 0x19C1,
    WMI_TX_DESC_RING_CFG_DONE_EVENTID		= 0x19C2,
    WMI_RX_DESC_RING_CFG_DONE_EVENTID		= 0x19C3,
    WMI_CFG_DEF_RX_OFFLOAD_DONE_EVENTID		= 0x19C5,
    WMI_SCHEDULING_SCHEME_EVENTID			= 0x1A01,
    WMI_FIXED_SCHEDULING_CONFIG_COMPLETE_EVENTID	= 0x1A02,
    WMI_ENABLE_FIXED_SCHEDULING_COMPLETE_EVENTID	= 0x1A03,
    WMI_SET_MULTI_DIRECTED_OMNIS_CONFIG_EVENTID	= 0x1A04,
    WMI_SET_LONG_RANGE_CONFIG_COMPLETE_EVENTID	= 0x1A05,
    WMI_GET_ASSOC_LIST_RES_EVENTID			= 0x1A06,
    WMI_GET_CCA_INDICATIONS_EVENTID			= 0x1A07,
    WMI_SET_CCA_INDICATIONS_BI_AVG_NUM_EVENTID	= 0x1A08,
    WMI_INTERNAL_FW_EVENT_EVENTID			= 0x1A0A,
    WMI_INTERNAL_FW_IOCTL_EVENTID			= 0x1A0B,
    WMI_LINK_STATS_CONFIG_DONE_EVENTID		= 0x1A0C,
    WMI_LINK_STATS_EVENTID				= 0x1A0D,
    WMI_SET_GRANT_MCS_EVENTID			= 0x1A0E,
    WMI_SET_AP_SLOT_SIZE_EVENTID			= 0x1A0F,
    WMI_SET_VRING_PRIORITY_WEIGHT_EVENTID		= 0x1A10,
    WMI_SET_VRING_PRIORITY_EVENTID			= 0x1A11,
    WMI_RBUFCAP_CFG_EVENTID				= 0x1A12,
    WMI_TEMP_SENSE_ALL_DONE_EVENTID			= 0x1A13,
    WMI_SET_CHANNEL_EVENTID				= 0x9000,
    WMI_ASSOC_REQ_EVENTID				= 0x9001,
    WMI_EAPOL_RX_EVENTID				= 0x9002,
    WMI_MAC_ADDR_RESP_EVENTID			= 0x9003,
    WMI_FW_VER_EVENTID				= 0x9004,
    WMI_ACS_PASSIVE_SCAN_COMPLETE_EVENTID		= 0x9005,
    WMI_INTERNAL_FW_SET_CHANNEL			= 0x9006,
    WMI_COMMAND_NOT_SUPPORTED_EVENTID		= 0xFFFF,
}

// Events data structures
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_fw_status {
    WMI_FW_STATUS_SUCCESS	= 0x00,
    WMI_FW_STATUS_FAILURE	= 0x01,
}

// WMI_RF_MGMT_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_rf_status {
    WMI_RF_ENABLED		= 0x00,
    WMI_RF_DISABLED_HW	= 0x01,
    WMI_RF_DISABLED_SW	= 0x02,
    WMI_RF_DISABLED_HW_SW	= 0x03,
}

// WMI_RF_MGMT_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rf_mgmt_status_event {
    pub rf_status: __le32,
    pub __packed: },
// WMI_GET_STATUS_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_status_done_event {
    pub is_associated: __le32,
    pub cid: u8,
    pub reserved0: [u8; 3],
    pub bssid: [u8; WMI_MAC_LEN],
    pub channel: u8,
    pub reserved1: u8,
    pub network_type: u8,
    pub reserved2: [u8; 3],
    pub ssid_len: __le32,
    pub ssid: [u8; WMI_MAX_SSID_LEN],
    pub rf_status: __le32,
    pub is_secured: __le32,
    pub __packed: },
// WMI_FW_VER_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_fw_ver_event {
// FW image version
    pub fw_major: __le32,
    pub fw_minor: __le32,
    pub fw_subminor: __le32,
    pub fw_build: __le32,
// FW image build time stamp
    pub hour: __le32,
    pub minute: __le32,
    pub second: __le32,
    pub day: __le32,
    pub month: __le32,
    pub year: __le32,
// Boot Loader image version
    pub bl_major: __le32,
    pub bl_minor: __le32,
    pub bl_subminor: __le32,
    pub bl_build: __le32,
// The number of entries in the FW capabilities array
    pub fw_capabilities_len: u8,
    pub reserved: [u8; 3],
// FW capabilities info
// Must be the last member of the struct
//
    pub fw_capabilities: [__le32; ],
    pub __packed: },
// WMI_GET_RF_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rf_type {
    RF_UNKNOWN	= 0x00,
    RF_MARLON	= 0x01,
    RF_SPARROW	= 0x02,
    RF_TALYNA1	= 0x03,
    RF_TALYNA2	= 0x04,
}

// WMI_GET_RF_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum board_file_rf_type {
    BF_RF_MARLON	= 0x00,
    BF_RF_SPARROW	= 0x01,
    BF_RF_TALYNA1	= 0x02,
    BF_RF_TALYNA2	= 0x03,
}

// WMI_GET_RF_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rf_status {
    RF_OK			= 0x00,
    RF_NO_COMM		= 0x01,
    RF_WRONG_BOARD_FILE	= 0x02,
}

// WMI_GET_RF_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_rf_status_event {
// enum rf_type
    pub rf_type: __le32,
// attached RFs bit vector
    pub attached_rf_vector: __le32,
// enabled RFs bit vector
    pub enabled_rf_vector: __le32,
// enum rf_status, refers to enabled RFs
    pub rf_status: [u8; 32],
// enum board file RF type
    pub board_file_rf_type: __le32,
// board file platform type
    pub board_file_platform_type: __le32,
// board file version
    pub board_file_version: __le32,
// enabled XIFs bit vector
    pub enabled_xif_vector: __le32,
    pub reserved: __le32,
    pub __packed: },
// WMI_GET_BASEBAND_TYPE_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum baseband_type {
    BASEBAND_UNKNOWN	= 0x00,
    BASEBAND_SPARROW_M_A0	= 0x03,
    BASEBAND_SPARROW_M_A1	= 0x04,
    BASEBAND_SPARROW_M_B0	= 0x05,
    BASEBAND_SPARROW_M_C0	= 0x06,
    BASEBAND_SPARROW_M_D0	= 0x07,
    BASEBAND_TALYN_M_A0	= 0x08,
    BASEBAND_TALYN_M_B0	= 0x09,
}

// WMI_GET_BASEBAND_TYPE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_baseband_type_event {
// enum baseband_type
    pub baseband_type: __le32,
    pub __packed: },
// WMI_MAC_ADDR_RESP_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mac_addr_resp_event {
    pub mac: [u8; WMI_MAC_LEN],
    pub auth_mode: u8,
    pub crypt_mode: u8,
    pub offload_mode: __le32,
    pub __packed: },
// WMI_EAPOL_RX_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_eapol_rx_event {
    pub src_mac: [u8; WMI_MAC_LEN],
    pub eapol_len: __le16,
    pub eapol: [u8; ],
    pub __packed: },
// WMI_READY_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_phy_capability {
    WMI_11A_CAPABILITY		= 0x01,
    WMI_11G_CAPABILITY		= 0x02,
    WMI_11AG_CAPABILITY		= 0x03,
    WMI_11NA_CAPABILITY		= 0x04,
    WMI_11NG_CAPABILITY		= 0x05,
    WMI_11NAG_CAPABILITY		= 0x06,
    WMI_11AD_CAPABILITY		= 0x07,
    WMI_11N_CAPABILITY_OFFSET	= 0x03,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ready_event {
    pub sw_version: __le32,
    pub abi_version: __le32,
    pub mac: [u8; WMI_MAC_LEN],
// enum wmi_phy_capability
    pub phy_capability: u8,
    pub numof_additional_mids: u8,
// rfc read calibration result. 5..15
    pub rfc_read_calib_result: u8,
// Max associated STAs supported by FW in AP mode (default 0 means 8
// STA)
//
    pub max_assoc_sta: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_NOTIFY_REQ_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_notify_req_done_event {
// beamforming status, 0: fail; 1: OK; 2: retrying
    pub status: __le32,
    pub tsf: __le64,
    pub rssi: i8,
// enum wmi_edmg_tx_mode
    pub tx_mode: u8,
    pub reserved0: [u8; 2],
    pub tx_tpt: __le32,
    pub tx_goodput: __le32,
    pub rx_goodput: __le32,
    pub bf_mcs: __le16,
    pub my_rx_sector: __le16,
    pub my_tx_sector: __le16,
    pub other_rx_sector: __le16,
    pub other_tx_sector: __le16,
    pub range: __le16,
    pub sqi: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_CONNECT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_connect_event {
// enum wmi_channel WMI_CHANNEL_1..WMI_CHANNEL_6; for EDMG this is
// the primary channel number
//
    pub channel: u8,
// enum wmi_channel WMI_CHANNEL_9..WMI_CHANNEL_12
    pub edmg_channel: u8,
    pub bssid: [u8; WMI_MAC_LEN],
    pub listen_interval: __le16,
    pub beacon_interval: __le16,
    pub network_type: u8,
    pub reserved1: [u8; 3],
    pub beacon_ie_len: u8,
    pub assoc_req_len: u8,
    pub assoc_resp_len: u8,
    pub cid: u8,
    pub aid: u8,
    pub reserved2: [u8; 2],
// not in use
    pub assoc_info: [u8; ],
    pub __packed: },
// disconnect_reason
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_disconnect_reason {
    WMI_DIS_REASON_NO_NETWORK_AVAIL		= 0x01,
// bmiss
    WMI_DIS_REASON_LOST_LINK		= 0x02,
    WMI_DIS_REASON_DISCONNECT_CMD		= 0x03,
    WMI_DIS_REASON_BSS_DISCONNECTED		= 0x04,
    WMI_DIS_REASON_AUTH_FAILED		= 0x05,
    WMI_DIS_REASON_ASSOC_FAILED		= 0x06,
    WMI_DIS_REASON_NO_RESOURCES_AVAIL	= 0x07,
    WMI_DIS_REASON_CSERV_DISCONNECT		= 0x08,
    WMI_DIS_REASON_INVALID_PROFILE		= 0x0A,
    WMI_DIS_REASON_DOT11H_CHANNEL_SWITCH	= 0x0B,
    WMI_DIS_REASON_PROFILE_MISMATCH		= 0x0C,
    WMI_DIS_REASON_CONNECTION_EVICTED	= 0x0D,
    WMI_DIS_REASON_IBSS_MERGE		= 0x0E,
    WMI_DIS_REASON_HIGH_TEMPERATURE		= 0x0F,
}

// WMI_DISCONNECT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_disconnect_event {
// reason code, see 802.11 spec.
    pub protocol_reason_status: __le16,
// set if known
    pub bssid: [u8; WMI_MAC_LEN],
// see enum wmi_disconnect_reason
    pub disconnect_reason: u8,
// last assoc req may passed to host - not in used
    pub assoc_resp_len: u8,
// last assoc req may passed to host - not in used
    pub assoc_info: [u8; ],
    pub __packed: },
// WMI_SCAN_COMPLETE_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scan_status {
    WMI_SCAN_SUCCESS	= 0x00,
    WMI_SCAN_FAILED		= 0x01,
    WMI_SCAN_ABORTED	= 0x02,
    WMI_SCAN_REJECTED	= 0x03,
    WMI_SCAN_ABORT_REJECTED	= 0x04,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_scan_complete_event {
// enum scan_status
    pub status: __le32,
    pub __packed: },
// WMI_FT_AUTH_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ft_auth_status_event {
// enum wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub mac_addr: [u8; WMI_MAC_LEN],
    pub ie_len: __le16,
    pub ie_info: [u8; ],
    pub __packed: },
// WMI_FT_REASSOC_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ft_reassoc_status_event {
// enum wmi_fw_status
    pub status: u8,
// association id received from new AP
    pub aid: u8,
// enum wmi_channel
    pub channel: u8,
// enum wmi_channel
    pub edmg_channel: u8,
    pub mac_addr: [u8; WMI_MAC_LEN],
    pub beacon_ie_len: __le16,
    pub reassoc_req_ie_len: __le16,
    pub reassoc_resp_ie_len: __le16,
    pub reserved: [u8; 4],
    pub ie_info: [u8; ],
    pub __packed: },
// wmi_rx_mgmt_info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rx_mgmt_info {
    pub mcs: u8,
    pub rssi: i8,
    pub range: u8,
    pub sqi: u8,
    pub stype: __le16,
    pub status: __le16,
    pub len: __le32,
// Not resolved when == 0xFFFFFFFF == > Broadcast to all MIDS
    pub qid: u8,
// Not resolved when == 0xFFFFFFFF == > Broadcast to all MIDS
    pub mid: u8,
    pub cid: u8,
// From Radio MNGR
    pub channel: u8,
    pub __packed: },
// WMI_START_SCHED_SCAN_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_pno_result {
    WMI_PNO_SUCCESS			= 0x00,
    WMI_PNO_REJECT			= 0x01,
    WMI_PNO_INVALID_PARAMETERS	= 0x02,
    WMI_PNO_NOT_ENABLED		= 0x03,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_start_sched_scan_event {
// wmi_pno_result
    pub result: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_stop_sched_scan_event {
// wmi_pno_result
    pub result: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sched_scan_result_event {
    pub info: wmi_rx_mgmt_info,
    pub payload: [u8; ],
    pub __packed: },
// WMI_ACS_PASSIVE_SCAN_COMPLETE_EVENT
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_acs_info_bitmask {
    WMI_ACS_INFO_BITMASK_BEACON_FOUND	= 0x01,
    WMI_ACS_INFO_BITMASK_BUSY_TIME		= 0x02,
    WMI_ACS_INFO_BITMASK_TX_TIME		= 0x04,
    WMI_ACS_INFO_BITMASK_RX_TIME		= 0x08,
    WMI_ACS_INFO_BITMASK_NOISE		= 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scan_acs_info {
    pub channel: u8,
    pub beacon_found: u8,
// msec
    pub busy_time: __le16,
    pub tx_time: __le16,
    pub rx_time: __le16,
    pub noise: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_acs_passive_scan_complete_event {
    pub dwell_time: __le32,
// valid fields within channel info according to
// their appearance in struct order
//
    pub filled: __le16,
    pub num_scanned_channels: u8,
    pub reserved: u8,
    pub scan_info_list: [scan_acs_info; ],
    pub __packed: },
// WMI_BA_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_vring_ba_status {
    WMI_BA_AGREED			= 0x00,
    WMI_BA_NON_AGREED		= 0x01,
// BA_EN in middle of teardown flow
    WMI_BA_TD_WIP			= 0x02,
// BA_DIS or BA_EN in middle of BA SETUP flow
    WMI_BA_SETUP_WIP		= 0x03,
// BA_EN when the BA session is already active
    WMI_BA_SESSION_ACTIVE		= 0x04,
// BA_DIS when the BA session is not active
    WMI_BA_SESSION_NOT_ACTIVE	= 0x05,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ba_status_event {
// enum wmi_vring_ba_status
    pub status: __le16,
    pub reserved: [u8; 2],
    pub ringid: u8,
    pub agg_wsize: u8,
    pub ba_timeout: __le16,
    pub amsdu: u8,
    pub __packed: },
// WMI_DELBA_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_delba_event {
// Used for cid less than 8. For higher cid set
// CIDXTID_EXTENDED_CID_TID here and use cid and tid members instead
//
    pub cidxtid: u8,
    pub from_initiator: u8,
    pub reason: __le16,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub cid: u8,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub tid: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_VRING_CFG_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vring_cfg_done_event {
    pub ringid: u8,
    pub status: u8,
    pub reserved: [u8; 2],
    pub tx_vring_tail_ptr: __le32,
    pub __packed: },
// WMI_RCP_ADDBA_RESP_SENT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rcp_addba_resp_sent_event {
// Used for cid less than 8. For higher cid set
// CIDXTID_EXTENDED_CID_TID here and use cid and tid members instead
//
    pub cidxtid: u8,
    pub reserved: u8,
    pub status: __le16,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub cid: u8,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub tid: u8,
    pub reserved2: [u8; 2],
    pub __packed: },
// WMI_TX_STATUS_RING_CFG_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tx_status_ring_cfg_done_event {
    pub ring_id: u8,
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 2],
    pub ring_tail_ptr: __le32,
    pub __packed: },
// WMI_RX_STATUS_RING_CFG_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rx_status_ring_cfg_done_event {
    pub ring_id: u8,
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 2],
    pub ring_tail_ptr: __le32,
    pub __packed: },
// WMI_CFG_DEF_RX_OFFLOAD_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cfg_def_rx_offload_done_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_TX_DESC_RING_CFG_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tx_desc_ring_cfg_done_event {
    pub ring_id: u8,
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 2],
    pub ring_tail_ptr: __le32,
    pub __packed: },
// WMI_RX_DESC_RING_CFG_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rx_desc_ring_cfg_done_event {
    pub ring_id: u8,
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 2],
    pub ring_tail_ptr: __le32,
    pub __packed: },
// WMI_RCP_ADDBA_REQ_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rcp_addba_req_event {
// Used for cid less than 8. For higher cid set
// CIDXTID_EXTENDED_CID_TID here and use cid and tid members instead
//
    pub cidxtid: u8,
    pub dialog_token: u8,
// ieee80211_ba_parameterset as it received
    pub ba_param_set: __le16,
    pub ba_timeout: __le16,
// ieee80211_ba_seqstrl field as it received
    pub ba_seq_ctrl: __le16,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub cid: u8,
// Used when cidxtid = CIDXTID_EXTENDED_CID_TID
    pub tid: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_CFG_RX_CHAIN_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_cfg_rx_chain_done_event_status {
    WMI_CFG_RX_CHAIN_SUCCESS	= 0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cfg_rx_chain_done_event {
// V-Ring Tail pointer
    pub rx_ring_tail_ptr: __le32,
    pub status: __le32,
    pub __packed: },
// WMI_WBE_LINK_DOWN_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_wbe_link_down_event_reason {
    WMI_WBE_REASON_USER_REQUEST	= 0x00,
    WMI_WBE_REASON_RX_DISASSOC	= 0x01,
    WMI_WBE_REASON_BAD_PHY_LINK	= 0x02,
}

// WMI_WBE_LINK_DOWN_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_wbe_link_down_event {
    pub cid: u8,
    pub reserved: [u8; 3],
    pub reason: __le32,
    pub __packed: },
// WMI_DATA_PORT_OPEN_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_data_port_open_event {
    pub cid: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RING_EN_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ring_en_event {
    pub ring_index: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_GET_PCP_CHANNEL_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_pcp_channel_event {
    pub channel: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_P2P_CFG_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_cfg_done_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_PORT_ALLOCATED_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_port_allocated_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_PORT_DELETED_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_port_deleted_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_LISTEN_STARTED_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_listen_started_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SEARCH_STARTED_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_search_started_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_PCP_STARTED_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pcp_started_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_PCP_FACTOR_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pcp_factor_event {
    pub pcp_factor: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sw_tx_status {
    WMI_TX_SW_STATUS_SUCCESS		= 0x00,
    WMI_TX_SW_STATUS_FAILED_NO_RESOURCES	= 0x01,
    WMI_TX_SW_STATUS_FAILED_TX		= 0x02,
}

// WMI_SW_TX_COMPLETE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sw_tx_complete_event {
// enum wmi_sw_tx_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_CORR_MEASURE_EVENTID - deprecated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_corr_measure_event {
// signed
    pub i: __le32,
// signed
    pub q: __le32,
// signed
    pub image_i: __le32,
// signed
    pub image_q: __le32,
    pub __packed: },
// WMI_READ_RSSI_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_read_rssi_event {
    pub ina_rssi_adc_dbm: __le32,
    pub __packed: },
// WMI_GET_SSID_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_ssid_event {
    pub ssid_len: __le32,
    pub ssid: [u8; WMI_MAX_SSID_LEN],
    pub __packed: },
// EVENT: WMI_RF_XPM_READ_RESULT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rf_xpm_read_result_event {
// enum wmi_fw_status_e - success=0 or fail=1
    pub status: u8,
    pub reserved: [u8; 3],
// requested num_bytes of data
    pub data_bytes: [u8; ],
    pub __packed: },
// EVENT: WMI_RF_XPM_WRITE_RESULT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rf_xpm_write_result_event {
// enum wmi_fw_status_e - success=0 or fail=1
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_TX_MGMT_PACKET_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tx_mgmt_packet_event {
    pub payload): DECLARE_FLEX_ARRAY(u8,,
    pub __packed: },
// WMI_RX_MGMT_PACKET_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rx_mgmt_packet_event {
    pub info: wmi_rx_mgmt_info,
    pub payload: [u8; ],
    pub __packed: },
// WMI_ECHO_RSP_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_echo_rsp_event {
    pub echoed_value: __le32,
    pub __packed: },
// WMI_DEEP_ECHO_RSP_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_deep_echo_rsp_event {
    pub echoed_value: __le32,
    pub __packed: },
// WMI_RF_PWR_ON_DELAY_RSP_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rf_pwr_on_delay_rsp_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_HIGH_POWER_TABLE_PARAMS_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_high_power_table_params_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_FIXED_SCHEDULING_UL_CONFIG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_fixed_scheduling_ul_config_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_TEMP_SENSE_DONE_EVENTID
//
// Measure MAC and radio temperatures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_temp_sense_done_event {
// Temperature times 1000 (actual temperature will be achieved by
// dividing the value by 1000). When temperature cannot be read from
// device return WMI_INVALID_TEMPERATURE
//
    pub baseband_t1000: __le32,
// Temperature times 1000 (actual temperature will be achieved by
// dividing the value by 1000). When temperature cannot be read from
// device return WMI_INVALID_TEMPERATURE
//
    pub rf_t1000: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_hidden_ssid {
    WMI_HIDDEN_SSID_DISABLED	= 0x00,
    WMI_HIDDEN_SSID_SEND_EMPTY	= 0x10,
    WMI_HIDDEN_SSID_CLEAR		= 0xFE,
}

// WMI_LED_CFG_CMDID
//
// Configure LED On\Off\Blinking operation
//
// Returned events:
// - WMI_LED_CFG_DONE_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum led_mode {
    LED_DISABLE	= 0x00,
    LED_ENABLE	= 0x01,
}

// The names of the led as
// described on HW schemes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_led_id {
    WMI_LED_WLAN	= 0x00,
    WMI_LED_WPAN	= 0x01,
    WMI_LED_WWAN	= 0x02,
}

// Led polarity mode.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_led_polarity {
    LED_POLARITY_HIGH_ACTIVE	= 0x00,
    LED_POLARITY_LOW_ACTIVE		= 0x01,
}

// Combination of on and off
// creates the blinking period
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_led_blink_mode {
    pub blink_on: __le32,
    pub blink_off: __le32,
    pub __packed: },
// WMI_LED_CFG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_led_cfg_cmd {
// enum led_mode_e
    pub led_mode: u8,
// enum wmi_led_id_e
    pub id: u8,
// slow speed blinking combination
    pub slow_blink_cfg: wmi_led_blink_mode,
// medium speed blinking combination
    pub medium_blink_cfg: wmi_led_blink_mode,
// high speed blinking combination
    pub fast_blink_cfg: wmi_led_blink_mode,
// polarity of the led
    pub led_polarity: u8,
// reserved
    pub reserved: u8,
    pub __packed: },
// \WMI_SET_CONNECT_SNR_THR_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_connect_snr_thr_cmd {
    pub enable: u8,
    pub reserved: u8,
// 1/4 Db units
    pub omni_snr_thr: __le16,
// 1/4 Db units
    pub direct_snr_thr: __le16,
    pub __packed: },
// WMI_LED_CFG_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_led_cfg_done_event {
// led config status
    pub status: __le32,
    pub __packed: },
// Rate search parameters configuration per connection
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_cfg {
// The maximal allowed PER for each MCS
// MCS will be considered as failed if PER during RS is higher
//
    pub per_threshold: [u8; WMI_NUM_MCS],
// Number of MPDUs for each MCS
// this is the minimal statistic required to make an educated
// decision
//
    pub min_frame_cnt: [u8; WMI_NUM_MCS],
// stop threshold [0-100]
    pub stop_th: u8,
// MCS1 stop threshold [0-100]
    pub mcs1_fail_th: u8,
    pub max_back_failure_th: u8,
// Debug feature for disabling internal RS trigger (which is
// currently triggered by BF Done)
//
    pub dbg_disable_internal_trigger: u8,
    pub back_failure_mask: __le32,
    pub mcs_en_vec: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_edmg_tx_mode {
    WMI_TX_MODE_DMG			= 0x0,
    WMI_TX_MODE_EDMG_CB1		= 0x1,
    WMI_TX_MODE_EDMG_CB2		= 0x2,
    WMI_TX_MODE_EDMG_CB1_LONG_LDPC	= 0x3,
    WMI_TX_MODE_EDMG_CB2_LONG_LDPC	= 0x4,
    WMI_TX_MODE_MAX,
}

// Rate search parameters common configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_cfg_ex_common {
// enum wmi_edmg_tx_mode
    pub mode: u8,
// stop threshold [0-100]
    pub stop_th: u8,
// MCS1 stop threshold [0-100]
    pub mcs1_fail_th: u8,
    pub max_back_failure_th: u8,
// Debug feature for disabling internal RS trigger (which is
// currently triggered by BF Done)
//
    pub dbg_disable_internal_trigger: u8,
    pub reserved: [u8; 3],
    pub back_failure_mask: __le32,
    pub __packed: },
// Rate search parameters configuration per MCS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_cfg_ex_mcs {
// The maximal allowed PER for each MCS
// MCS will be considered as failed if PER during RS is higher
//
    pub per_threshold: u8,
// Number of MPDUs for each MCS
// this is the minimal statistic required to make an educated
// decision
//
    pub min_frame_cnt: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_RS_CFG_EX_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_cfg_ex_cmd {
// Configuration for all MCSs
    pub common_cfg: wmi_rs_cfg_ex_common,
    pub each_mcs_cfg_size: u8,
    pub reserved: [u8; 3],
// Configuration for each MCS
    pub each_mcs_cfg: [wmi_rs_cfg_ex_mcs; ],
    pub __packed: },
// WMI_RS_CFG_EX_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_cfg_ex_event {
// enum wmi_edmg_tx_mode
    pub mode: u8,
// enum wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_RS_ENABLE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_enable_cmd {
    pub cid: u8,
// enable or disable rate search
    pub rs_enable: u8,
    pub reserved: [u8; 2],
    pub mcs_en_vec: __le32,
    pub __packed: },
// WMI_RS_ENABLE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_enable_event {
// enum wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// Slot types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sched_scheme_slot_type {
    WMI_SCHED_SLOT_SP		= 0x0,
    WMI_SCHED_SLOT_CBAP		= 0x1,
    WMI_SCHED_SLOT_IDLE		= 0x2,
    WMI_SCHED_SLOT_ANNOUNCE_NO_ACK	= 0x3,
    WMI_SCHED_SLOT_DISCOVERY	= 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sched_scheme_slot_flags {
    WMI_SCHED_SCHEME_SLOT_PERIODIC	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sched_scheme_slot {
// in microsecond
    pub tbtt_offset: __le32,
// wmi_sched_scheme_slot_flags
    pub flags: u8,
// wmi_sched_scheme_slot_type
    pub type: u8,
// in microsecond
    pub duration: __le16,
// frame_exchange_sequence_duration
    pub tx_op: __le16,
// time in microseconds between two consecutive slots
// relevant only if flag WMI_SCHED_SCHEME_SLOT_PERIODIC set
//
    pub period: __le16,
// relevant only if flag WMI_SCHED_SCHEME_SLOT_PERIODIC set
// number of times to repeat allocation
//
    pub num_of_blocks: u8,
// relevant only if flag WMI_SCHED_SCHEME_SLOT_PERIODIC set
// every idle_period allocation will be idle
//
    pub idle_period: u8,
    pub src_aid: u8,
    pub dest_aid: u8,
    pub reserved: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sched_scheme_flags {
// should not be set when clearing scheduling scheme
    WMI_SCHED_SCHEME_ENABLE		= 0x01,
    WMI_SCHED_PROTECTED_SP		= 0x02,
// should be set only on first WMI fragment of scheme
    WMI_SCHED_FIRST			= 0x04,
// should be set only on last WMI fragment of scheme
    WMI_SCHED_LAST			= 0x08,
    WMI_SCHED_IMMEDIATE_START	= 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sched_scheme_advertisment {
// ESE is not advertised at all, STA has to be configured with WMI
// also
//
    WMI_ADVERTISE_ESE_DISABLED		= 0x0,
    WMI_ADVERTISE_ESE_IN_BEACON		= 0x1,
    WMI_ADVERTISE_ESE_IN_ANNOUNCE_FRAME	= 0x2,
}

// WMI_SCHEDULING_SCHEME_CMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_scheduling_scheme_cmd {
    pub serial_num: u8,
// wmi_sched_scheme_advertisment
    pub ese_advertisment: u8,
// wmi_sched_scheme_flags
    pub flags: __le16,
    pub num_allocs: u8,
    pub reserved: [u8; 3],
    pub start_tbtt: __le64,
// allocations list
    pub allocs: [wmi_sched_scheme_slot; WMI_SCHED_MAX_ALLOCS_PER_CMD],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sched_scheme_failure_type {
    WMI_SCHED_SCHEME_FAILURE_NO_ERROR		= 0x00,
    WMI_SCHED_SCHEME_FAILURE_OLD_START_TSF_ERR	= 0x01,
}

// WMI_SCHEDULING_SCHEME_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_scheduling_scheme_event {
// wmi_fw_status_e
    pub status: u8,
// serial number given in command
    pub serial_num: u8,
// wmi_sched_scheme_failure_type
    pub failure_type: u8,
// alignment to 32b
    pub reserved: [u8; 1],
    pub __packed: },
// WMI_RS_CFG_CMDID - deprecated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_cfg_cmd {
// connection id
    pub cid: u8,
// enable or disable rate search
    pub rs_enable: u8,
// rate search configuration
    pub rs_cfg: wmi_rs_cfg,
    pub __packed: },
// WMI_RS_CFG_DONE_EVENTID - deprecated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_cfg_done_event {
    pub cid: u8,
// enum wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_GET_DETAILED_RS_RES_CMDID - deprecated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_detailed_rs_res_cmd {
// connection id
    pub cid: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// RS results status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_rs_results_status {
    WMI_RS_RES_VALID	= 0x00,
    WMI_RS_RES_INVALID	= 0x01,
}

// Rate search results
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_results {
// number of sent MPDUs
    pub num_of_tx_pkt: [u8; WMI_NUM_MCS],
// number of non-acked MPDUs
    pub num_of_non_acked_pkt: [u8; WMI_NUM_MCS],
// RS timestamp
    pub tsf: __le32,
// RS selected MCS
    pub mcs: u8,
    pub __packed: },
// WMI_GET_DETAILED_RS_RES_EVENTID - deprecated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_detailed_rs_res_event {
    pub cid: u8,
// enum wmi_rs_results_status
    pub status: u8,
// detailed rs results
    pub rs_results: wmi_rs_results,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_GET_DETAILED_RS_RES_EX_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_detailed_rs_res_ex_cmd {
    pub cid: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// Rate search results
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_results_ex_common {
// RS timestamp
    pub tsf: __le32,
// RS selected MCS
    pub mcs: u8,
// enum wmi_edmg_tx_mode
    pub mode: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// Rate search results
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rs_results_ex_mcs {
// number of sent MPDUs
    pub num_of_tx_pkt: u8,
// number of non-acked MPDUs
    pub num_of_non_acked_pkt: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_GET_DETAILED_RS_RES_EX_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_detailed_rs_res_ex_event {
    pub cid: u8,
// enum wmi_rs_results_status
    pub status: u8,
    pub reserved0: [u8; 2],
    pub common_rs_results: wmi_rs_results_ex_common,
    pub each_mcs_results_size: u8,
    pub reserved1: [u8; 3],
// Results for each MCS
    pub each_mcs_results: [wmi_rs_results_ex_mcs; ],
    pub __packed: },
// BRP antenna limit mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_brp_ant_limit_mode {
// Disable BRP force antenna limit
    WMI_BRP_ANT_LIMIT_MODE_DISABLE		= 0x00,
// Define maximal antennas limit. Only effective antennas will be
// actually used
//
    WMI_BRP_ANT_LIMIT_MODE_EFFECTIVE	= 0x01,
// Force a specific number of antennas
    WMI_BRP_ANT_LIMIT_MODE_FORCE		= 0x02,
// number of BRP antenna limit modes
    WMI_BRP_ANT_LIMIT_MODES_NUM		= 0x03,
}

// WMI_BRP_SET_ANT_LIMIT_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_brp_set_ant_limit_cmd {
// connection id
    pub cid: u8,
// enum wmi_brp_ant_limit_mode
    pub limit_mode: u8,
// antenna limit count, 1-27
// disable_mode - ignored
// effective_mode - upper limit to number of antennas to be used
// force_mode - exact number of antennas to be used
//
    pub ant_limit: u8,
    pub reserved: u8,
    pub __packed: },
// WMI_BRP_SET_ANT_LIMIT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_brp_set_ant_limit_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_bf_type {
    WMI_BF_TYPE_SLS		= 0x00,
    WMI_BF_TYPE_BRP_RX	= 0x01,
}

// WMI_BF_TRIG_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bf_trig_cmd {
// enum wmi_bf_type - type of requested beamforming
    pub bf_type: u8,
// used only for WMI_BF_TYPE_BRP_RX
    pub cid: u8,
// used only for WMI_BF_TYPE_SLS
    pub dst_mac: [u8; WMI_MAC_LEN],
    pub reserved: [u8; 4],
    pub __packed: },
// WMI_BF_TRIG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bf_trig_event {
// enum wmi_fw_status
    pub status: u8,
    pub cid: u8,
    pub reserved: [u8; 2],
    pub __packed: },
// broadcast connection ID

// Types wmi_link_maintain_cfg presets for WMI_LINK_MAINTAIN_CFG_WRITE_CMD
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_link_maintain_cfg_type {
// AP/PCP default normal (non-FST) configuration settings
    WMI_LINK_MAINTAIN_CFG_TYPE_DEFAULT_NORMAL_AP	= 0x00,
// AP/PCP  default FST configuration settings
    WMI_LINK_MAINTAIN_CFG_TYPE_DEFAULT_FST_AP	= 0x01,
// STA default normal (non-FST) configuration settings
    WMI_LINK_MAINTAIN_CFG_TYPE_DEFAULT_NORMAL_STA	= 0x02,
// STA default FST configuration settings
    WMI_LINK_MAINTAIN_CFG_TYPE_DEFAULT_FST_STA	= 0x03,
// custom configuration settings
    WMI_LINK_MAINTAIN_CFG_TYPE_CUSTOM		= 0x04,
// number of defined configuration types
    WMI_LINK_MAINTAIN_CFG_TYPES_NUM			= 0x05,
}

// Response status codes for WMI_LINK_MAINTAIN_CFG_WRITE/READ commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_link_maintain_cfg_response_status {
// WMI_LINK_MAINTAIN_CFG_WRITE/READ command successfully accomplished
//
    WMI_LINK_MAINTAIN_CFG_RESPONSE_STATUS_OK		= 0x00,
// ERROR due to bad argument in WMI_LINK_MAINTAIN_CFG_WRITE/READ
// command request
//
    WMI_LINK_MAINTAIN_CFG_RESPONSE_STATUS_BAD_ARGUMENT	= 0x01,
}

// Link Loss and Keep Alive configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_maintain_cfg {
// link_loss_enable_detectors_vec
    pub link_loss_enable_detectors_vec: __le32,
// detectors check period usec
    pub check_link_loss_period_usec: __le32,
// max allowed tx ageing
    pub tx_ageing_threshold_usec: __le32,
// keep alive period for high SNR
    pub keep_alive_period_usec_high_snr: __le32,
// keep alive period for low SNR
    pub keep_alive_period_usec_low_snr: __le32,
// lower snr limit for keep alive period update
    pub keep_alive_snr_threshold_low_db: __le32,
// upper snr limit for keep alive period update
    pub keep_alive_snr_threshold_high_db: __le32,
// num of successive bad bcons causing link-loss
    pub bad_beacons_num_threshold: __le32,
// SNR limit for bad_beacons_detector
    pub bad_beacons_snr_threshold_db: __le32,
// timeout for disassoc response frame in uSec
    pub disconnect_timeout: __le32,
    pub __packed: },
// WMI_LINK_MAINTAIN_CFG_WRITE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_maintain_cfg_write_cmd {
// enum wmi_link_maintain_cfg_type_e - type of requested default
// configuration to be applied
//
    pub cfg_type: __le32,
// requested connection ID or WMI_LINK_MAINTAIN_CFG_CID_BROADCAST
    pub cid: __le32,
// custom configuration settings to be applied (relevant only if
// cfg_type==WMI_LINK_MAINTAIN_CFG_TYPE_CUSTOM)
//
    pub lm_cfg: wmi_link_maintain_cfg,
    pub __packed: },
// WMI_LINK_MAINTAIN_CFG_READ_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_maintain_cfg_read_cmd {
// connection ID which configuration settings are requested
    pub cid: __le32,
    pub __packed: },
// WMI_SET_LINK_MONITOR_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_link_monitor_cmd {
    pub rssi_hyst: u8,
    pub reserved: [u8; 12],
    pub rssi_thresholds_list_size: u8,
    pub __counted_by(rssi_thresholds_list_size): s8 rssi_thresholds_list[],
    pub __packed: },
// wmi_link_monitor_event_type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_link_monitor_event_type {
    WMI_LINK_MONITOR_NOTIF_RSSI_THRESHOLD_EVT	= 0x00,
    WMI_LINK_MONITOR_NOTIF_TX_ERR_EVT		= 0x01,
    WMI_LINK_MONITOR_NOTIF_THERMAL_EVT		= 0x02,
}

// WMI_SET_LINK_MONITOR_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_link_monitor_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_LINK_MONITOR_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_monitor_event {
// link_monitor_event_type
    pub type: u8,
    pub rssi_level: i8,
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_LINK_MAINTAIN_CFG_WRITE_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_maintain_cfg_write_done_event {
// requested connection ID
    pub cid: __le32,
// wmi_link_maintain_cfg_response_status_e - write status
    pub status: __le32,
    pub __packed: },
// \WMI_LINK_MAINTAIN_CFG_READ_DONE_EVENT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_maintain_cfg_read_done_event {
// requested connection ID
    pub cid: __le32,
// wmi_link_maintain_cfg_response_status_e - read status
    pub status: __le32,
// Retrieved configuration settings
    pub lm_cfg: wmi_link_maintain_cfg,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_traffic_suspend_status {
    WMI_TRAFFIC_SUSPEND_APPROVED			= 0x0,
    WMI_TRAFFIC_SUSPEND_REJECTED_LINK_NOT_IDLE	= 0x1,
    WMI_TRAFFIC_SUSPEND_REJECTED_DISCONNECT		= 0x2,
    WMI_TRAFFIC_SUSPEND_REJECTED_OTHER		= 0x3,
}

// WMI_TRAFFIC_SUSPEND_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_traffic_suspend_event {
// enum wmi_traffic_suspend_status_e
    pub status: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_traffic_resume_status {
    WMI_TRAFFIC_RESUME_SUCCESS	= 0x0,
    WMI_TRAFFIC_RESUME_FAILED	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_resume_trigger {
    WMI_RESUME_TRIGGER_UNKNOWN	= 0x0,
    WMI_RESUME_TRIGGER_HOST		= 0x1,
    WMI_RESUME_TRIGGER_UCAST_RX	= 0x2,
    WMI_RESUME_TRIGGER_BCAST_RX	= 0x4,
    WMI_RESUME_TRIGGER_WMI_EVT	= 0x8,
    WMI_RESUME_TRIGGER_DISCONNECT	= 0x10,
}

// WMI_TRAFFIC_RESUME_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_traffic_resume_event {
// enum wmi_traffic_resume_status
    pub status: u8,
    pub reserved: [u8; 3],
// enum wmi_resume_trigger bitmap
    pub resume_triggers: __le32,
    pub __packed: },
// Power Save command completion status codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ps_cfg_cmd_status {
    WMI_PS_CFG_CMD_STATUS_SUCCESS	= 0x00,
    WMI_PS_CFG_CMD_STATUS_BAD_PARAM	= 0x01,
// other error
    WMI_PS_CFG_CMD_STATUS_ERROR	= 0x02,
}

// Device Power Save Profiles
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ps_profile_type {
    WMI_PS_PROFILE_TYPE_DEFAULT		= 0x00,
    WMI_PS_PROFILE_TYPE_PS_DISABLED		= 0x01,
    WMI_PS_PROFILE_TYPE_MAX_PS		= 0x02,
    WMI_PS_PROFILE_TYPE_LOW_LATENCY_PS	= 0x03,
}

// WMI_PS_DEV_PROFILE_CFG_READ_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ps_dev_profile_cfg_read_cmd {
// reserved
    pub reserved: __le32,
    pub __packed: },
// WMI_PS_DEV_PROFILE_CFG_READ_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ps_dev_profile_cfg_read_event {
// wmi_ps_profile_type_e
    pub ps_profile: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_PS_DEV_PROFILE_CFG_CMDID
//
// Power save profile to be used by the device
//
// Returned event:
// - WMI_PS_DEV_PROFILE_CFG_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ps_dev_profile_cfg_cmd {
// wmi_ps_profile_type_e
    pub ps_profile: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_PS_DEV_PROFILE_CFG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ps_dev_profile_cfg_event {
// wmi_ps_cfg_cmd_status_e
    pub status: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ps_level {
    WMI_PS_LEVEL_DEEP_SLEEP		= 0x00,
    WMI_PS_LEVEL_SHALLOW_SLEEP	= 0x01,
// awake = all PS mechanisms are disabled
    WMI_PS_LEVEL_AWAKE		= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ps_deep_sleep_clk_level {
// 33k
    WMI_PS_DEEP_SLEEP_CLK_LEVEL_RTC		= 0x00,
// 10k
    WMI_PS_DEEP_SLEEP_CLK_LEVEL_OSC		= 0x01,
// @RTC Low latency
    WMI_PS_DEEP_SLEEP_CLK_LEVEL_RTC_LT	= 0x02,
    WMI_PS_DEEP_SLEEP_CLK_LEVEL_XTAL	= 0x03,
    WMI_PS_DEEP_SLEEP_CLK_LEVEL_SYSCLK	= 0x04,
// Not Applicable
    WMI_PS_DEEP_SLEEP_CLK_LEVEL_N_A		= 0xFF,
}

// Response by the FW to a D3 entry request
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ps_d3_resp_policy {
    WMI_PS_D3_RESP_POLICY_DEFAULT	= 0x00,
// debug -D3 req is always denied
    WMI_PS_D3_RESP_POLICY_DENIED	= 0x01,
// debug -D3 req is always approved
    WMI_PS_D3_RESP_POLICY_APPROVED	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_aoa_meas_status {
    WMI_AOA_MEAS_SUCCESS		= 0x00,
    WMI_AOA_MEAS_PEER_INCAPABLE	= 0x01,
    WMI_AOA_MEAS_FAILURE		= 0x02,
}

// WMI_AOA_MEAS_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_aoa_meas_event {
    pub mac_addr: [u8; WMI_MAC_LEN],
// channels IDs:
// 0 - 58320 MHz
// 1 - 60480 MHz
// 2 - 62640 MHz
//
    pub channel: u8,
// enum wmi_aoa_meas_type
    pub aoa_meas_type: u8,
// Measurements are from RFs, defined by the mask
    pub meas_rf_mask: __le32,
// enum wmi_aoa_meas_status
    pub meas_status: u8,
    pub reserved: u8,
// Length of meas_data in bytes
    pub length: __le16,
    pub meas_data: [u8; WMI_AOA_MAX_DATA_SIZE],
    pub __packed: },
// WMI_SET_MGMT_RETRY_LIMIT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_mgmt_retry_limit_event {
// enum wmi_fw_status
    pub status: u8,
// alignment to 32b
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_GET_MGMT_RETRY_LIMIT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_mgmt_retry_limit_event {
// MAC retransmit limit for mgmt frames
    pub mgmt_retry_limit: u8,
// alignment to 32b
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_TOF_GET_CAPABILITIES_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_get_capabilities_event {
    pub ftm_capability: u8,
// maximum supported number of destination to start TOF
    pub max_num_of_dest: u8,
// maximum supported number of measurements per burst
    pub max_num_of_meas_per_burst: u8,
    pub reserved: u8,
// maximum supported multi bursts
    pub max_multi_bursts_sessions: __le16,
// maximum supported FTM burst duration , wmi_tof_burst_duration_e
    pub max_ftm_burst_duration: __le16,
// AOA supported types
    pub aoa_supported_types: __le32,
    pub __packed: },
// WMI_SET_THERMAL_THROTTLING_CFG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_thermal_throttling_cfg_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_GET_THERMAL_THROTTLING_CFG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_thermal_throttling_cfg_event {
// Status data
    pub tt_data: wmi_tt_data,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tof_session_end_status {
    WMI_TOF_SESSION_END_NO_ERROR		= 0x00,
    WMI_TOF_SESSION_END_FAIL		= 0x01,
    WMI_TOF_SESSION_END_PARAMS_ERROR	= 0x02,
    WMI_TOF_SESSION_END_ABORTED		= 0x03,
    WMI_TOF_SESSION_END_BUSY		= 0x04,
}

// WMI_TOF_SESSION_END_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_session_end_event {
// FTM session ID
    pub session_id: __le32,
// wmi_tof_session_end_status_e
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_TOF_SET_LCI_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_set_lci_event {
// enum wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_TOF_SET_LCR_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_set_lcr_event {
// enum wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// Responder FTM Results
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_responder_ftm_res {
    pub t1: [u8; 6],
    pub t2: [u8; 6],
    pub t3: [u8; 6],
    pub t4: [u8; 6],
    pub tod_err: __le16,
    pub toa_err: __le16,
    pub tod_err_initiator: __le16,
    pub toa_err_initiator: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tof_ftm_per_dest_res_status {
    WMI_PER_DEST_RES_NO_ERROR		= 0x00,
    WMI_PER_DEST_RES_TX_RX_FAIL		= 0x01,
    WMI_PER_DEST_RES_PARAM_DONT_MATCH	= 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tof_ftm_per_dest_res_flags {
    WMI_PER_DEST_RES_REQ_START		= 0x01,
    WMI_PER_DEST_RES_BURST_REPORT_END	= 0x02,
    WMI_PER_DEST_RES_REQ_END		= 0x04,
    WMI_PER_DEST_RES_PARAM_UPDATE		= 0x08,
}

// WMI_TOF_FTM_PER_DEST_RES_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_ftm_per_dest_res_event {
// FTM session ID
    pub session_id: __le32,
// destination MAC address
    pub dst_mac: [u8; WMI_MAC_LEN],
// wmi_tof_ftm_per_dest_res_flags_e
    pub flags: u8,
// wmi_tof_ftm_per_dest_res_status_e
    pub status: u8,
// responder ASAP
    pub responder_asap: u8,
// responder number of FTM per burst
    pub responder_num_ftm_per_burst: u8,
// responder number of FTM burst exponent
    pub responder_num_ftm_bursts_exp: u8,
// responder burst duration ,wmi_tof_burst_duration_e
    pub responder_burst_duration: u8,
// responder burst period, indicate interval between two consecutive
// burst instances, in units of 100 ms
//
    pub responder_burst_period: __le16,
// receive burst counter
    pub bursts_cnt: __le16,
// tsf of responder start burst
    pub tsf_sync: __le32,
// actual received ftm per burst
    pub actual_ftm_per_burst: u8,
// Measurements are from RFs, defined by the mask
    pub meas_rf_mask: __le32,
    pub reserved0: [u8; 3],
    pub responder_ftm_res: [wmi_responder_ftm_res; ],
    pub __packed: },
// WMI_TOF_CFG_RESPONDER_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_cfg_responder_event {
// enum wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tof_channel_info_type {
    WMI_TOF_CHANNEL_INFO_AOA		= 0x00,
    WMI_TOF_CHANNEL_INFO_LCI		= 0x01,
    WMI_TOF_CHANNEL_INFO_LCR		= 0x02,
    WMI_TOF_CHANNEL_INFO_VENDOR_SPECIFIC	= 0x03,
    WMI_TOF_CHANNEL_INFO_CIR		= 0x04,
    WMI_TOF_CHANNEL_INFO_RSSI		= 0x05,
    WMI_TOF_CHANNEL_INFO_SNR		= 0x06,
    WMI_TOF_CHANNEL_INFO_DEBUG		= 0x07,
}

// WMI_TOF_CHANNEL_INFO_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_channel_info_event {
// FTM session ID
    pub session_id: __le32,
// destination MAC address
    pub dst_mac: [u8; WMI_MAC_LEN],
// wmi_tof_channel_info_type_e
    pub type: u8,
// data report length
    pub len: u8,
// data report payload
    pub report: [u8; ],
    pub __packed: },
// WMI_TOF_SET_TX_RX_OFFSET_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_set_tx_rx_offset_event {
// enum wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_TOF_GET_TX_RX_OFFSET_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tof_get_tx_rx_offset_event {
// enum wmi_fw_status
    pub status: u8,
// RF index used to read the offsets
    pub rf_index: u8,
    pub reserved1: [u8; 2],
// TX delay offset
    pub tx_offset: __le32,
// RX delay offset
    pub rx_offset: __le32,
// Offset to strongest tap of CIR
    pub precursor: __le32,
    pub __packed: },
// Result status codes for WMI commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_rf_sector_status {
    WMI_RF_SECTOR_STATUS_SUCCESS			= 0x00,
    WMI_RF_SECTOR_STATUS_BAD_PARAMETERS_ERROR	= 0x01,
    WMI_RF_SECTOR_STATUS_BUSY_ERROR			= 0x02,
    WMI_RF_SECTOR_STATUS_NOT_SUPPORTED_ERROR	= 0x03,
}

// Types of the RF sector (TX,RX)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_rf_sector_type {
    WMI_RF_SECTOR_TYPE_RX	= 0x00,
    WMI_RF_SECTOR_TYPE_TX	= 0x01,
}

// Content of RF Sector (six 32-bits registers)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rf_sector_info {
// Phase values for RF Chains[15-0] (2bits per RF chain)
    pub psh_hi: __le32,
// Phase values for RF Chains[31-16] (2bits per RF chain)
    pub psh_lo: __le32,
// ETYPE Bit0 for all RF chains[31-0] - bit0 of Edge amplifier gain
// index
//
    pub etype0: __le32,
// ETYPE Bit1 for all RF chains[31-0] - bit1 of Edge amplifier gain
// index
//
    pub etype1: __le32,
// ETYPE Bit2 for all RF chains[31-0] - bit2 of Edge amplifier gain
// index
//
    pub etype2: __le32,
// D-Type values (3bits each) for 8 Distribution amplifiers + X16
// switch bits
//
    pub dtype_swch_off: __le32,
    pub __packed: },

// WMI_GET_RF_SECTOR_PARAMS_CMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_rf_sector_params_cmd {
// Sector number to be retrieved
    pub sector_idx: __le16,
// enum wmi_rf_sector_type - type of requested RF sector
    pub sector_type: u8,
// bitmask vector specifying destination RF modules
    pub rf_modules_vec: u8,
    pub __packed: },
// \WMI_GET_RF_SECTOR_PARAMS_DONE_EVENT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_rf_sector_params_done_event {
// result status of WMI_GET_RF_SECTOR_PARAMS_CMD (enum
// wmi_rf_sector_status)
//
    pub status: u8,
// align next field to U64 boundary
    pub reserved: [u8; 7],
// TSF timestamp when RF sectors where retrieved
    pub tsf: __le64,
// Content of RF sector retrieved from each RF module
    pub sectors_info: [wmi_rf_sector_info; WMI_MAX_RF_MODULES_NUM],
    pub __packed: },
// WMI_SET_RF_SECTOR_PARAMS_CMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_rf_sector_params_cmd {
// Sector number to be retrieved
    pub sector_idx: __le16,
// enum wmi_rf_sector_type - type of requested RF sector
    pub sector_type: u8,
// bitmask vector specifying destination RF modules
    pub rf_modules_vec: u8,
// Content of RF sector to be written to each RF module
    pub sectors_info: [wmi_rf_sector_info; WMI_MAX_RF_MODULES_NUM],
    pub __packed: },
// \WMI_SET_RF_SECTOR_PARAMS_DONE_EVENT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_rf_sector_params_done_event {
// result status of WMI_SET_RF_SECTOR_PARAMS_CMD (enum
// wmi_rf_sector_status)
//
    pub status: u8,
    pub __packed: },
// WMI_GET_SELECTED_RF_SECTOR_INDEX_CMD - Get RF sector index selected by
// TXSS/BRP for communication with specified CID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_selected_rf_sector_index_cmd {
// Connection/Station ID in [0:7] range
    pub cid: u8,
// type of requested RF sector (enum wmi_rf_sector_type)
    pub sector_type: u8,
// align to U32 boundary
    pub reserved: [u8; 2],
    pub __packed: },
// \WMI_GET_SELECTED_RF_SECTOR_INDEX_DONE_EVENT - Returns retrieved RF sector
// index selected by TXSS/BRP for communication with specified CID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_selected_rf_sector_index_done_event {
// Retrieved sector index selected in TXSS (for TX sector request) or
// BRP (for RX sector request)
//
    pub sector_idx: __le16,
// result status of WMI_GET_SELECTED_RF_SECTOR_INDEX_CMD (enum
// wmi_rf_sector_status)
//
    pub status: u8,
// align next field to U64 boundary
    pub reserved: [u8; 5],
// TSF timestamp when result was retrieved
    pub tsf: __le64,
    pub __packed: },
// WMI_SET_SELECTED_RF_SECTOR_INDEX_CMD - Force RF sector index for
// communication with specified CID. Assumes that TXSS/BRP is disabled by
// other command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_selected_rf_sector_index_cmd {
// Connection/Station ID in [0:7] range
    pub cid: u8,
// type of requested RF sector (enum wmi_rf_sector_type)
    pub sector_type: u8,
// Forced sector index
    pub sector_idx: __le16,
    pub __packed: },
// \WMI_SET_SELECTED_RF_SECTOR_INDEX_DONE_EVENT - Success/Fail status for
// WMI_SET_SELECTED_RF_SECTOR_INDEX_CMD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_selected_rf_sector_index_done_event {
// result status of WMI_SET_SELECTED_RF_SECTOR_INDEX_CMD (enum
// wmi_rf_sector_status)
//
    pub status: u8,
// align to U32 boundary
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_RF_SECTOR_ON_CMD - Activates specified sector for specified rf
// modules
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_rf_sector_on_cmd {
// Sector index to be activated
    pub sector_idx: __le16,
// type of requested RF sector (enum wmi_rf_sector_type)
    pub sector_type: u8,
// bitmask vector specifying destination RF modules
    pub rf_modules_vec: u8,
    pub __packed: },
// \WMI_SET_RF_SECTOR_ON_DONE_EVENT - Success/Fail status for
// WMI_SET_RF_SECTOR_ON_CMD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_rf_sector_on_done_event {
// result status of WMI_SET_RF_SECTOR_ON_CMD (enum
// wmi_rf_sector_status)
//
    pub status: u8,
// align to U32 boundary
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sector_sweep_type {
    WMI_SECTOR_SWEEP_TYPE_TXSS		= 0x00,
    WMI_SECTOR_SWEEP_TYPE_BCON		= 0x01,
    WMI_SECTOR_SWEEP_TYPE_TXSS_AND_BCON	= 0x02,
    WMI_SECTOR_SWEEP_TYPE_NUM		= 0x03,
}

// WMI_PRIO_TX_SECTORS_ORDER_CMDID
//
// Set the order of TX sectors in TXSS and/or Beacon(AP).
//
// Returned event:
// - WMI_PRIO_TX_SECTORS_ORDER_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_prio_tx_sectors_order_cmd {
// tx sectors order to be applied, 0xFF for end of array
    pub tx_sectors_priority_array: [u8; MAX_NUM_OF_SECTORS],
// enum wmi_sector_sweep_type, TXSS and/or Beacon
    pub sector_sweep_type: u8,
// needed only for TXSS configuration
    pub cid: u8,
// alignment to 32b
    pub reserved: [u8; 2],
    pub __packed: },
// completion status codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_prio_tx_sectors_cmd_status {
    WMI_PRIO_TX_SECT_CMD_STATUS_SUCCESS	= 0x00,
    WMI_PRIO_TX_SECT_CMD_STATUS_BAD_PARAM	= 0x01,
// other error
    WMI_PRIO_TX_SECT_CMD_STATUS_ERROR	= 0x02,
}

// WMI_PRIO_TX_SECTORS_ORDER_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_prio_tx_sectors_order_event {
// enum wmi_prio_tx_sectors_cmd_status
    pub status: u8,
// alignment to 32b
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_prio_tx_sectors_num_cmd {
// [0-128], 0 = No changes
    pub beacon_number_of_sectors: u8,
// [0-128], 0 = No changes
    pub txss_number_of_sectors: u8,
// [0-8] needed only for TXSS configuration
    pub cid: u8,
    pub __packed: },
// WMI_PRIO_TX_SECTORS_NUMBER_CMDID
//
// Set the number of active sectors in TXSS and/or Beacon.
//
// Returned event:
// - WMI_PRIO_TX_SECTORS_NUMBER_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_prio_tx_sectors_number_cmd {
    pub active_sectors_num: wmi_prio_tx_sectors_num_cmd,
// alignment to 32b
    pub reserved: u8,
    pub __packed: },
// WMI_PRIO_TX_SECTORS_NUMBER_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_prio_tx_sectors_number_event {
// enum wmi_prio_tx_sectors_cmd_status
    pub status: u8,
// alignment to 32b
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_PRIO_TX_SECTORS_SET_DEFAULT_CFG_CMDID
//
// Set default sectors order and number (hard coded in board file)
// in TXSS and/or Beacon.
//
// Returned event:
// - WMI_PRIO_TX_SECTORS_SET_DEFAULT_CFG_EVENTID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_prio_tx_sectors_set_default_cfg_cmd {
// enum wmi_sector_sweep_type, TXSS and/or Beacon
    pub sector_sweep_type: u8,
// needed only for TXSS configuration
    pub cid: u8,
// alignment to 32b
    pub reserved: [u8; 2],
    pub __packed: },
// WMI_PRIO_TX_SECTORS_SET_DEFAULT_CFG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_prio_tx_sectors_set_default_cfg_event {
// enum wmi_prio_tx_sectors_cmd_status
    pub status: u8,
// alignment to 32b
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_SILENT_RSSI_TABLE_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_silent_rssi_table_done_event {
// enum wmi_silent_rssi_status
    pub status: __le32,
// enum wmi_silent_rssi_table
    pub table: __le32,
    pub __packed: },
// WMI_VRING_SWITCH_TIMING_CONFIG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_vring_switch_timing_config_event {
// enum wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_GET_ASSOC_LIST_RES_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_assoc_sta_info {
    pub mac: [u8; WMI_MAC_LEN],
    pub omni_index_address: u8,
    pub reserved: u8,
    pub __packed: },

// WMI_GET_ASSOC_LIST_RES_EVENTID
// Returns up to MAX_ASSOC_STA_LIST_SIZE associated STAs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_assoc_list_res_event {
    pub assoc_sta_list: [wmi_assoc_sta_info; WMI_GET_ASSOC_LIST_SIZE],
// STA count
    pub count: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_BF_CONTROL_EVENTID - deprecated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bf_control_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_BF_CONTROL_EX_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bf_control_ex_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_COMMAND_NOT_SUPPORTED_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_command_not_supported_event {
// device id
    pub mid: u8,
    pub reserved0: u8,
    pub command_id: __le16,
// for UT command only, otherwise reserved
    pub command_subtype: __le16,
    pub reserved1: __le16,
    pub __packed: },
// WMI_TSF_SYNC_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tsf_sync_cmd {
// The time interval to send announce frame in one BI
    pub interval_ms: u8,
// The mcs to send announce frame
    pub mcs: u8,
    pub reserved: [u8; 6],
    pub __packed: },
// WMI_TSF_SYNC_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_tsf_sync_status {
    WMI_TSF_SYNC_SUCCESS	= 0x00,
    WMI_TSF_SYNC_FAILED	= 0x01,
    WMI_TSF_SYNC_REJECTED	= 0x02,
}

// WMI_TSF_SYNC_STATUS_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tsf_sync_status_event {
// enum wmi_tsf_sync_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_GET_CCA_INDICATIONS_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_cca_indications_event {
// wmi_fw_status
    pub status: u8,
// CCA-Energy Detect in percentage over last BI (0..100)
    pub cca_ed_percent: u8,
// Averaged CCA-Energy Detect in percent over number of BIs (0..100)
    pub cca_ed_avg_percent: u8,
// NAV percent over last BI (0..100)
    pub nav_percent: u8,
// Averaged NAV percent over number of BIs (0..100)
    pub nav_avg_percent: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_CCA_INDICATIONS_BI_AVG_NUM_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_cca_indications_bi_avg_num_cmd {
// set the number of bis to average cca_ed (0..255)
    pub bi_number: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_CCA_INDICATIONS_BI_AVG_NUM_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_cca_indications_bi_avg_num_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_INTERNAL_FW_SET_CHANNEL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_internal_fw_set_channel_event {
    pub channel_num: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_LINK_STATS_CONFIG_DONE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_stats_config_done_event {
// wmi_fw_status_e
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_LINK_STATS_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_stats_event {
    pub tsf: __le64,
    pub payload_size: __le16,
    pub has_next: u8,
    pub reserved: [u8; 5],
// a stream of wmi_link_stats_record_s
    pub payload: [u8; ],
    pub __packed: },
// WMI_LINK_STATS_EVENT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_stats_record {
// wmi_link_stats_record_type_e
    pub record_type_id: u8,
    pub reserved: u8,
    pub record_size: __le16,
    pub record: [u8; ],
    pub __packed: },
// WMI_LINK_STATS_TYPE_BASIC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_stats_basic {
    pub cid: u8,
    pub rssi: i8,
    pub sqi: u8,
    pub bf_mcs: u8,
    pub per_average: u8,
    pub selected_rfc: u8,
    pub rx_effective_ant_num: u8,
    pub my_rx_sector: u8,
    pub my_tx_sector: u8,
    pub other_rx_sector: u8,
    pub other_tx_sector: u8,
    pub reserved: [u8; 7],
// 1/4 Db units
    pub snr: __le16,
    pub tx_tpt: __le32,
    pub tx_goodput: __le32,
    pub rx_goodput: __le32,
    pub bf_count: __le32,
    pub rx_bcast_frames: __le32,
    pub __packed: },
// WMI_LINK_STATS_TYPE_GLOBAL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_link_stats_global {
// all ack-able frames
    pub rx_frames: __le32,
// all ack-able frames
    pub tx_frames: __le32,
    pub rx_ba_frames: __le32,
    pub tx_ba_frames: __le32,
    pub tx_beacons: __le32,
    pub rx_mic_errors: __le32,
    pub rx_crc_errors: __le32,
    pub tx_fail_no_ack: __le32,
    pub reserved: [u8; 8],
    pub __packed: },
// WMI_SET_GRANT_MCS_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_grant_mcs_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_AP_SLOT_SIZE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_ap_slot_size_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_VRING_PRIORITY_WEIGHT_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_vring_priority_weight_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_SET_VRING_PRIORITY_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_vring_priority_event {
// wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_RADAR_PCI_CTRL_BLOCK struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_radar_pci_ctrl_block {
// last fw tail address index
    pub fw_tail_index: __le32,
// last SW head address index known to FW
    pub sw_head_index: __le32,
    pub last_wr_pulse_tsf_low: __le32,
    pub last_wr_pulse_count: __le32,
    pub last_wr_in_bytes: __le32,
    pub last_wr_pulse_id: __le32,
    pub last_wr_burst_id: __le32,
// When pre overflow detected, advance sw head in unit of pulses
    pub sw_head_inc: __le32,
    pub reserved: [__le32; 8],
    pub __packed: },
// WMI_RBUFCAP_CFG_CMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rbufcap_cfg_cmd {
    pub enable: u8,
    pub reserved: u8,
// RBUFCAP indicates rx space unavailable when number of rx
// descriptors drops below this threshold. Set 0 to use system
// default
//
    pub rx_desc_threshold: __le16,
    pub __packed: },
// WMI_RBUFCAP_CFG_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rbufcap_cfg_event {
// enum wmi_fw_status
    pub status: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// WMI_TEMP_SENSE_ALL_DONE_EVENTID
// Measure MAC and all radio temperatures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_temp_sense_all_done_event {
// enum wmi_fw_status
    pub status: u8,
// Bitmap of connected RFs
    pub rf_bitmap: u8,
    pub reserved: [u8; 2],
// Temperature times 1000 (actual temperature will be achieved by
// dividing the value by 1000). When temperature cannot be read from
// device return WMI_INVALID_TEMPERATURE
//
    pub rf_t1000: [__le32; WMI_MAX_XIF_PORTS_NUM],
// Temperature times 1000 (actual temperature will be achieved by
// dividing the value by 1000). When temperature cannot be read from
// device return WMI_INVALID_TEMPERATURE
//
    pub baseband_t1000: __le32,
    pub __packed: },
