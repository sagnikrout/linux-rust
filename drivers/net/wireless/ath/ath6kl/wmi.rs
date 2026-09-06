//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath6kl/wmi.h
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
// Copyright (c) 2010-2011 Atheros Communications Inc.
// Copyright (c) 2011-2012 Qualcomm Atheros, Inc.
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
// This file contains the definitions of the WMI protocol specified in the
// Wireless Module Interface (WMI).  It includes definitions of all the
// commands and events. Commands are messages from the host to the WM.
// Events and Replies are messages from the WM to the host.
//

pub const HTC_PROTOCOL_VERSION: c_uint = 0x0002;
pub const WMI_PROTOCOL_VERSION: c_uint = 0x0002;
pub const WMI_CONTROL_MSG_MAX_LEN: c_int = 256;

pub const IP_ETHERTYPE: c_uint = 0x0800;
pub const WMI_IMPLICIT_PSTREAM: c_uint = 0xFF;
pub const WMI_MAX_THINSTREAM: c_int = 15;
pub const SSID_IE_LEN_INDEX: c_int = 13;
// Host side link management data structures
pub const SIG_QUALITY_THRESH_LVLS: c_int = 6;

pub const A_BAND_24GHZ: c_int = 0;
pub const A_BAND_5GHZ: c_int = 1;
pub const ATH6KL_NUM_BANDS: c_int = 2;
// in ms
pub const WMI_IMPLICIT_PSTREAM_INACTIVITY_INT: c_int = 5000;
//
// There are no signed versions of __le16 and __le32, so for a temporary
// solution come up with our own version. The idea is from fs/ntfs/types.h.
//
// Use a_ prefix so that it doesn't conflict if we get proper support to
// linux/types.h.
//
pub type a_sle16 = __s16 ;
pub type a_sle32 = __s32 ;
extern "C" {
    pub fn le32_to_cpu(val: ( __le32)) -> return;
}
extern "C" {
    pub fn le16_to_cpu(val: ( __le16)) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_threshold_params {
    pub upper_threshold: [i16; SIG_QUALITY_UPPER_THRESH_LVLS],
    pub lower_threshold: [i16; SIG_QUALITY_LOWER_THRESH_LVLS],
    pub upper_threshold_valid_count: u32,
    pub lower_threshold_valid_count: u32,
    pub polling_interval: u32,
    pub weight: u8,
    pub last_rssi: u8,
    pub last_rssi_poll_event: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_data_sync_bufs {
    pub traffic_class: u8,
    pub skb: *mut sk_buff,
}

// WMM stream classes
pub const WMM_NUM_AC: c_int = 4;

pub const WMI_VOICE_USER_PRIORITY: c_uint = 0x7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi {
    pub stream_exist_for_ac: [u16; WMM_NUM_AC],
    pub fat_pipe_exist: u8,
    pub parent_dev: *mut ath6kl,
    pub pwr_mode: u8,
// protects fat_pipe_exist and stream_exist_for_ac
    pub lock: spinlock_t,
    pub ep_id: htc_endpoint_id,
    pub is_wmm_enabled: bool,
    pub traffic_class: u8,
    pub is_probe_ssid: bool,
    pub last_mgmt_tx_frame: *mut u8,
    pub last_mgmt_tx_frame_len: usize,
    pub last_mgmt_tx_cookie: u64,
    pub saved_pwr_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_app_area {
    pub wmi_protocol_ver: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_msg_type {
    DATA_MSGTYPE = 0x0,
    CNTL_MSGTYPE,
    SYNC_MSGTYPE,
    OPT_MSGTYPE,
}

//
// Macros for operating on WMI_DATA_HDR (info) field
//
pub const WMI_DATA_HDR_MSG_TYPE_MASK: c_uint = 0x03;
pub const WMI_DATA_HDR_MSG_TYPE_SHIFT: c_int = 0;
pub const WMI_DATA_HDR_UP_MASK: c_uint = 0x07;
pub const WMI_DATA_HDR_UP_SHIFT: c_int = 2;
// In AP mode, the same bit (b5) is used to indicate Power save state in
// the Rx dir and More data bit state in the tx direction.
//
pub const WMI_DATA_HDR_PS_MASK: c_uint = 0x1;
pub const WMI_DATA_HDR_PS_SHIFT: c_int = 5;
pub const WMI_DATA_HDR_MORE: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_data_hdr_data_type {
    WMI_DATA_HDR_DATA_TYPE_802_3 = 0,
    WMI_DATA_HDR_DATA_TYPE_802_11,

// used to be used for the PAL
    WMI_DATA_HDR_DATA_TYPE_ACL,
}

// Bitmap of data header flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_data_hdr_flags {
    WMI_DATA_HDR_FLAGS_MORE = 0x1,
    WMI_DATA_HDR_FLAGS_EOSP = 0x2,
    WMI_DATA_HDR_FLAGS_UAPSD = 0x4,
}

pub const WMI_DATA_HDR_DATA_TYPE_MASK: c_uint = 0x3;
pub const WMI_DATA_HDR_DATA_TYPE_SHIFT: c_int = 6;
// Macros for operating on WMI_DATA_HDR (info2) field
pub const WMI_DATA_HDR_SEQNO_MASK: c_uint = 0xFFF;
pub const WMI_DATA_HDR_SEQNO_SHIFT: c_int = 0;
pub const WMI_DATA_HDR_AMSDU_MASK: c_uint = 0x1;
pub const WMI_DATA_HDR_AMSDU_SHIFT: c_int = 12;
pub const WMI_DATA_HDR_META_MASK: c_uint = 0x7;
pub const WMI_DATA_HDR_META_SHIFT: c_int = 13;
pub const WMI_DATA_HDR_PAD_BEFORE_DATA_MASK: c_uint = 0xFF;
pub const WMI_DATA_HDR_PAD_BEFORE_DATA_SHIFT: c_uint = 0x8;
// Macros for operating on WMI_DATA_HDR (info3) field
pub const WMI_DATA_HDR_IF_IDX_MASK: c_uint = 0xF;
pub const WMI_DATA_HDR_TRIG: c_uint = 0x10;
pub const WMI_DATA_HDR_EOSP: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_data_hdr {
    pub rssi: i8,
//
// usage of 'info' field(8-bit):
//
// b1:b0       - WMI_MSG_TYPE
// b4:b3:b2    - UP(tid)
// b5          - Used in AP mode.
// More-data in tx dir, PS in rx.
// b7:b6       - Dot3 header(0),
// Dot11 Header(1),
// ACL data(2)
//
    pub info: u8,
//
// usage of 'info2' field(16-bit):
//
// b11:b0       - seq_no
// b12          - A-MSDU?
// b15:b13      - META_DATA_VERSION 0 - 7
//
    pub info2: __le16,
//
// usage of info3, 16-bit:
// b3:b0	- Interface index
// b4		- uAPSD trigger in rx & EOSP in tx
// b15:b5	- Reserved
//
    pub info3: __le16,
    pub __packed: },
    pub WMI_DATA_HDR_UP_MASK: return (dhdr->info >> WMI_DATA_HDR_UP_SHIFT) &,
    pub WMI_DATA_HDR_UP_SHIFT): dhdr->info &= ~(WMI_DATA_HDR_UP_MASK <<,
    pub WMI_DATA_HDR_UP_SHIFT: dhdr->info |= usr_pri <<,
    pub data_type: u8,
    pub WMI_DATA_HDR_DATA_TYPE_802_11): return (data_type ==,
    pub WMI_DATA_HDR_IF_IDX_MASK: return le16_to_cpu(dhdr->info3) &,
// Tx meta version definitions
pub const WMI_MAX_TX_META_SZ: c_int = 12;
pub const WMI_META_VERSION_1: c_uint = 0x01;
pub const WMI_META_VERSION_2: c_uint = 0x02;
// Flag to signal to FW to calculate TCP checksum
pub const WMI_META_V2_FLAG_CSUM_OFFLOAD: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tx_meta_v1 {
// packet ID to identify the tx request
    pub pkt_id: u8,
// rate policy to be used for the tx of this frame
    pub rate_plcy_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tx_meta_v2 {
//
// Offset from start of the WMI header for csum calculation to
// begin.
//
    pub csum_start: u8,
// offset from start of WMI header where final csum goes
    pub csum_dest: u8,
// no of bytes over which csum is calculated
    pub csum_flags: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rx_meta_v1 {
    pub status: u8,
// rate index mapped to rate at which this packet was received.
    pub rix: u8,
// rssi of packet
    pub rssi: u8,
// rf channel during packet reception
    pub channel: u8,
    pub flags: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rx_meta_v2 {
    pub csum: __le16,
// bit 0 set -partial csum valid bit 1 set -test mode
    pub csum_flags: u8,
    pub __packed: },
pub const WMI_CMD_HDR_IF_ID_MASK: c_uint = 0xF;
// Control Path
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cmd_hdr {
    pub cmd_id: __le16,
// info1 - 16 bits
// b03:b00 - id
// b15:b04 - unused
    pub info1: __le16,
// for alignment
    pub reserved: __le16,
    pub __packed: },
    pub WMI_CMD_HDR_IF_ID_MASK: return le16_to_cpu(chdr->info1) &,
// List of WMI commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_cmd_id {
    WMI_CONNECT_CMDID = 0x0001,
    WMI_RECONNECT_CMDID,
    WMI_DISCONNECT_CMDID,
    WMI_SYNCHRONIZE_CMDID,
    WMI_CREATE_PSTREAM_CMDID,
    WMI_DELETE_PSTREAM_CMDID,
// WMI_START_SCAN_CMDID is to be deprecated. Use
// WMI_BEGIN_SCAN_CMDID instead. The new cmd supports P2P mgmt
// operations using station interface.
//
    WMI_START_SCAN_CMDID,
    WMI_SET_SCAN_PARAMS_CMDID,
    WMI_SET_BSS_FILTER_CMDID,
    WMI_SET_PROBED_SSID_CMDID,	/* 10 */
    WMI_SET_LISTEN_INT_CMDID,
    WMI_SET_BMISS_TIME_CMDID,
    WMI_SET_DISC_TIMEOUT_CMDID,
    WMI_GET_CHANNEL_LIST_CMDID,
    WMI_SET_BEACON_INT_CMDID,
    WMI_GET_STATISTICS_CMDID,
    WMI_SET_CHANNEL_PARAMS_CMDID,
    WMI_SET_POWER_MODE_CMDID,
    WMI_SET_IBSS_PM_CAPS_CMDID,
    WMI_SET_POWER_PARAMS_CMDID,	/* 20 */
    WMI_SET_POWERSAVE_TIMERS_POLICY_CMDID,
    WMI_ADD_CIPHER_KEY_CMDID,
    WMI_DELETE_CIPHER_KEY_CMDID,
    WMI_ADD_KRK_CMDID,
    WMI_DELETE_KRK_CMDID,
    WMI_SET_PMKID_CMDID,
    WMI_SET_TX_PWR_CMDID,
    WMI_GET_TX_PWR_CMDID,
    WMI_SET_ASSOC_INFO_CMDID,
    WMI_ADD_BAD_AP_CMDID,		/* 30 */
    WMI_DELETE_BAD_AP_CMDID,
    WMI_SET_TKIP_COUNTERMEASURES_CMDID,
    WMI_RSSI_THRESHOLD_PARAMS_CMDID,
    WMI_TARGET_ERROR_REPORT_BITMASK_CMDID,
    WMI_SET_ACCESS_PARAMS_CMDID,
    WMI_SET_RETRY_LIMITS_CMDID,
    WMI_SET_OPT_MODE_CMDID,
    WMI_OPT_TX_FRAME_CMDID,
    WMI_SET_VOICE_PKT_SIZE_CMDID,
    WMI_SET_MAX_SP_LEN_CMDID,	/* 40 */
    WMI_SET_ROAM_CTRL_CMDID,
    WMI_GET_ROAM_TBL_CMDID,
    WMI_GET_ROAM_DATA_CMDID,
    WMI_ENABLE_RM_CMDID,
    WMI_SET_MAX_OFFHOME_DURATION_CMDID,
    WMI_EXTENSION_CMDID,	/* Non-wireless extensions */
    WMI_SNR_THRESHOLD_PARAMS_CMDID,
    WMI_LQ_THRESHOLD_PARAMS_CMDID,
    WMI_SET_LPREAMBLE_CMDID,
    WMI_SET_RTS_CMDID,		/* 50 */
    WMI_CLR_RSSI_SNR_CMDID,
    WMI_SET_FIXRATES_CMDID,
    WMI_GET_FIXRATES_CMDID,
    WMI_SET_AUTH_MODE_CMDID,
    WMI_SET_REASSOC_MODE_CMDID,
    WMI_SET_WMM_CMDID,
    WMI_SET_WMM_TXOP_CMDID,
    WMI_TEST_CMDID,

// COEX AR6002 only
    WMI_SET_BT_STATUS_CMDID,
    WMI_SET_BT_PARAMS_CMDID,	/* 60 */

    WMI_SET_KEEPALIVE_CMDID,
    WMI_GET_KEEPALIVE_CMDID,
    WMI_SET_APPIE_CMDID,
    WMI_GET_APPIE_CMDID,
    WMI_SET_WSC_STATUS_CMDID,

// Wake on Wireless
    WMI_SET_HOST_SLEEP_MODE_CMDID,
    WMI_SET_WOW_MODE_CMDID,
    WMI_GET_WOW_LIST_CMDID,
    WMI_ADD_WOW_PATTERN_CMDID,
    WMI_DEL_WOW_PATTERN_CMDID,	/* 70 */

    WMI_SET_FRAMERATES_CMDID,
    WMI_SET_AP_PS_CMDID,
    WMI_SET_QOS_SUPP_CMDID,
    WMI_SET_IE_CMDID,

// WMI_THIN_RESERVED_... mark the start and end
// values for WMI_THIN_RESERVED command IDs. These
// command IDs can be found in wmi_thin.h
    WMI_THIN_RESERVED_START = 0x8000,
    WMI_THIN_RESERVED_END = 0x8fff,

// Developer commands starts at 0xF000
    WMI_SET_BITRATE_CMDID = 0xF000,
    WMI_GET_BITRATE_CMDID,
    WMI_SET_WHALPARAM_CMDID,
    WMI_SET_MAC_ADDRESS_CMDID,
    WMI_SET_AKMP_PARAMS_CMDID,
    WMI_SET_PMKID_LIST_CMDID,
    WMI_GET_PMKID_LIST_CMDID,
    WMI_ABORT_SCAN_CMDID,
    WMI_SET_TARGET_EVENT_REPORT_CMDID,

// Unused
    WMI_UNUSED1,
    WMI_UNUSED2,

// AP mode commands
    WMI_AP_HIDDEN_SSID_CMDID,
    WMI_AP_SET_NUM_STA_CMDID,
    WMI_AP_ACL_POLICY_CMDID,
    WMI_AP_ACL_MAC_LIST_CMDID,
    WMI_AP_CONFIG_COMMIT_CMDID,
    WMI_AP_SET_MLME_CMDID,
    WMI_AP_SET_PVB_CMDID,
    WMI_AP_CONN_INACT_CMDID,
    WMI_AP_PROT_SCAN_TIME_CMDID,
    WMI_AP_SET_COUNTRY_CMDID,
    WMI_AP_SET_DTIM_CMDID,
    WMI_AP_MODE_STAT_CMDID,

    WMI_SET_IP_CMDID,
    WMI_SET_PARAMS_CMDID,
    WMI_SET_MCAST_FILTER_CMDID,
    WMI_DEL_MCAST_FILTER_CMDID,

    WMI_ALLOW_AGGR_CMDID,
    WMI_ADDBA_REQ_CMDID,
    WMI_DELBA_REQ_CMDID,
    WMI_SET_HT_CAP_CMDID,
    WMI_SET_HT_OP_CMDID,
    WMI_SET_TX_SELECT_RATES_CMDID,
    WMI_SET_TX_SGI_PARAM_CMDID,
    WMI_SET_RATE_POLICY_CMDID,

    WMI_HCI_CMD_CMDID,
    WMI_RX_FRAME_FORMAT_CMDID,
    WMI_SET_THIN_MODE_CMDID,
    WMI_SET_BT_WLAN_CONN_PRECEDENCE_CMDID,

    WMI_AP_SET_11BG_RATESET_CMDID,
    WMI_SET_PMK_CMDID,
    WMI_MCAST_FILTER_CMDID,

// COEX CMDID AR6003
    WMI_SET_BTCOEX_FE_ANT_CMDID,
    WMI_SET_BTCOEX_COLOCATED_BT_DEV_CMDID,
    WMI_SET_BTCOEX_SCO_CONFIG_CMDID,
    WMI_SET_BTCOEX_A2DP_CONFIG_CMDID,
    WMI_SET_BTCOEX_ACLCOEX_CONFIG_CMDID,
    WMI_SET_BTCOEX_BTINQUIRY_PAGE_CONFIG_CMDID,
    WMI_SET_BTCOEX_DEBUG_CMDID,
    WMI_SET_BTCOEX_BT_OPERATING_STATUS_CMDID,
    WMI_GET_BTCOEX_STATS_CMDID,
    WMI_GET_BTCOEX_CONFIG_CMDID,

    WMI_SET_DFS_ENABLE_CMDID,	/* F034 */
    WMI_SET_DFS_MINRSSITHRESH_CMDID,
    WMI_SET_DFS_MAXPULSEDUR_CMDID,
    WMI_DFS_RADAR_DETECTED_CMDID,

// P2P commands
    WMI_P2P_SET_CONFIG_CMDID,	/* F038 */
    WMI_WPS_SET_CONFIG_CMDID,
    WMI_SET_REQ_DEV_ATTR_CMDID,
    WMI_P2P_FIND_CMDID,
    WMI_P2P_STOP_FIND_CMDID,
    WMI_P2P_GO_NEG_START_CMDID,
    WMI_P2P_LISTEN_CMDID,

    WMI_CONFIG_TX_MAC_RULES_CMDID,	/* F040 */
    WMI_SET_PROMISCUOUS_MODE_CMDID,
    WMI_RX_FRAME_FILTER_CMDID,
    WMI_SET_CHANNEL_CMDID,

// WAC commands
    WMI_ENABLE_WAC_CMDID,
    WMI_WAC_SCAN_REPLY_CMDID,
    WMI_WAC_CTRL_REQ_CMDID,
    WMI_SET_DIV_PARAMS_CMDID,

    WMI_GET_PMK_CMDID,
    WMI_SET_PASSPHRASE_CMDID,
    WMI_SEND_ASSOC_RES_CMDID,
    WMI_SET_ASSOC_REQ_RELAY_CMDID,

// ACS command, consists of sub-commands
    WMI_ACS_CTRL_CMDID,
    WMI_SET_EXCESS_TX_RETRY_THRES_CMDID,
    WMI_SET_TBD_TIME_CMDID, /*added for wmiconfig command for TBD */

// Pktlog cmds
    WMI_PKTLOG_ENABLE_CMDID,
    WMI_PKTLOG_DISABLE_CMDID,

// More P2P Cmds
    WMI_P2P_GO_NEG_REQ_RSP_CMDID,
    WMI_P2P_GRP_INIT_CMDID,
    WMI_P2P_GRP_FORMATION_DONE_CMDID,
    WMI_P2P_INVITE_CMDID,
    WMI_P2P_INVITE_REQ_RSP_CMDID,
    WMI_P2P_PROV_DISC_REQ_CMDID,
    WMI_P2P_SET_CMDID,

    WMI_GET_RFKILL_MODE_CMDID,
    WMI_SET_RFKILL_MODE_CMDID,
    WMI_AP_SET_APSD_CMDID,
    WMI_AP_APSD_BUFFERED_TRAFFIC_CMDID,

    WMI_P2P_SDPD_TX_CMDID, /* F05C */
    WMI_P2P_STOP_SDPD_CMDID,
    WMI_P2P_CANCEL_CMDID,
// Ultra low power store / recall commands
    WMI_STORERECALL_CONFIGURE_CMDID,
    WMI_STORERECALL_RECALL_CMDID,
    WMI_STORERECALL_HOST_READY_CMDID,
    WMI_FORCE_TARGET_ASSERT_CMDID,

    WMI_SET_PROBED_SSID_EX_CMDID,
    WMI_SET_NETWORK_LIST_OFFLOAD_CMDID,
    WMI_SET_ARP_NS_OFFLOAD_CMDID,
    WMI_ADD_WOW_EXT_PATTERN_CMDID,
    WMI_GTK_OFFLOAD_OP_CMDID,
    WMI_REMAIN_ON_CHNL_CMDID,
    WMI_CANCEL_REMAIN_ON_CHNL_CMDID,
// WMI_SEND_ACTION_CMDID is to be deprecated. Use
// WMI_SEND_MGMT_CMDID instead. The new cmd supports P2P mgmt
// operations using station interface.
//
    WMI_SEND_ACTION_CMDID,
    WMI_PROBE_REQ_REPORT_CMDID,
    WMI_DISABLE_11B_RATES_CMDID,
    WMI_SEND_PROBE_RESPONSE_CMDID,
    WMI_GET_P2P_INFO_CMDID,
    WMI_AP_JOIN_BSS_CMDID,

    WMI_SMPS_ENABLE_CMDID,
    WMI_SMPS_CONFIG_CMDID,
    WMI_SET_RATECTRL_PARM_CMDID,
// LPL specific commands
    WMI_LPL_FORCE_ENABLE_CMDID,
    WMI_LPL_SET_POLICY_CMDID,
    WMI_LPL_GET_POLICY_CMDID,
    WMI_LPL_GET_HWSTATE_CMDID,
    WMI_LPL_SET_PARAMS_CMDID,
    WMI_LPL_GET_PARAMS_CMDID,

    WMI_SET_BUNDLE_PARAM_CMDID,

// GreenTx specific commands

    WMI_GREENTX_PARAMS_CMDID,

    WMI_RTT_MEASREQ_CMDID,
    WMI_RTT_CAPREQ_CMDID,
    WMI_RTT_STATUSREQ_CMDID,

// WPS Commands
    WMI_WPS_START_CMDID,
    WMI_GET_WPS_STATUS_CMDID,

// More P2P commands
    WMI_SET_NOA_CMDID,
    WMI_GET_NOA_CMDID,
    WMI_SET_OPPPS_CMDID,
    WMI_GET_OPPPS_CMDID,
    WMI_ADD_PORT_CMDID,
    WMI_DEL_PORT_CMDID,

// 802.11w cmd
    WMI_SET_RSN_CAP_CMDID,
    WMI_GET_RSN_CAP_CMDID,
    WMI_SET_IGTK_CMDID,

    WMI_RX_FILTER_COALESCE_FILTER_OP_CMDID,
    WMI_RX_FILTER_SET_FRAME_TEST_LIST_CMDID,

    WMI_SEND_MGMT_CMDID,
    WMI_BEGIN_SCAN_CMDID,

    WMI_SET_BLACK_LIST,
    WMI_SET_MCASTRATE,

    WMI_STA_BMISS_ENHANCE_CMDID,

    WMI_SET_REGDOMAIN_CMDID,

    WMI_SET_RSSI_FILTER_CMDID,

    WMI_SET_KEEP_ALIVE_EXT,

    WMI_VOICE_DETECTION_ENABLE_CMDID,

    WMI_SET_TXE_NOTIFY_CMDID,

    WMI_SET_RECOVERY_TEST_PARAMETER_CMDID, /*0xf094*/

    WMI_ENABLE_SCHED_SCAN_CMDID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_mgmt_frame_type {
    WMI_FRAME_BEACON = 0,
    WMI_FRAME_PROBE_REQ,
    WMI_FRAME_PROBE_RESP,
    WMI_FRAME_ASSOC_REQ,
    WMI_FRAME_ASSOC_RESP,
    WMI_NUM_MGMT_FRAME
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ie_field_type {
    WMI_RSN_IE_CAPB	= 0x1,
    WMI_IE_FULL	= 0xFF,  /* indicates full IE */
}

// WMI_CONNECT_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum network_type {
    INFRA_NETWORK = 0x01,
    ADHOC_NETWORK = 0x02,
    ADHOC_CREATOR = 0x04,
    AP_NETWORK = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum network_subtype {
    SUBTYPE_NONE,
    SUBTYPE_BT,
    SUBTYPE_P2PDEV,
    SUBTYPE_P2PCLIENT,
    SUBTYPE_P2PGO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dot11_auth_mode {
    OPEN_AUTH = 0x01,
    SHARED_AUTH = 0x02,

// different from IEEE_AUTH_MODE definitions
    LEAP_AUTH = 0x04,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum auth_mode {
    NONE_AUTH = 0x01,
    WPA_AUTH = 0x02,
    WPA2_AUTH = 0x04,
    WPA_PSK_AUTH = 0x08,
    WPA2_PSK_AUTH = 0x10,
    WPA_AUTH_CCKM = 0x20,
    WPA2_AUTH_CCKM = 0x40,
}

pub const WMI_MAX_KEY_INDEX: c_int = 3;
pub const WMI_MAX_KEY_LEN: c_int = 32;
//
// NB: these values are ordered carefully; there are lots of
// implications in any reordering.  In particular beware
// that 4 is not used to avoid conflicting with IEEE80211_F_PRIVACY.
//
pub const ATH6KL_CIPHER_WEP: c_int = 0;
pub const ATH6KL_CIPHER_TKIP: c_int = 1;
pub const ATH6KL_CIPHER_AES_OCB: c_int = 2;
pub const ATH6KL_CIPHER_AES_CCM: c_int = 3;
pub const ATH6KL_CIPHER_CKIP: c_int = 5;
pub const ATH6KL_CIPHER_CCKM_KRK: c_int = 6;

//
// 802.11 rate set.
//

pub const ATH_OUI_TYPE: c_uint = 0x01;
pub const WPA_OUI_TYPE: c_uint = 0x01;
pub const WMM_PARAM_OUI_SUBTYPE: c_uint = 0x01;
pub const WMM_OUI_TYPE: c_uint = 0x02;
pub const WSC_OUT_TYPE: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_connect_ctrl_flags_bits {
    CONNECT_ASSOC_POLICY_USER = 0x0001,
    CONNECT_SEND_REASSOC = 0x0002,
    CONNECT_IGNORE_WPAx_GROUP_CIPHER = 0x0004,
    CONNECT_PROFILE_MATCH_DONE = 0x0008,
    CONNECT_IGNORE_AAC_BEACON = 0x0010,
    CONNECT_CSA_FOLLOW_BSS = 0x0020,
    CONNECT_DO_WPA_OFFLOAD = 0x0040,
    CONNECT_DO_NOT_DEAUTH = 0x0080,
    CONNECT_WPS_FLAG = 0x0100,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_connect_cmd {
    pub nw_type: u8,
    pub dot11_auth_mode: u8,
    pub auth_mode: u8,
    pub prwise_crypto_type: u8,
    pub prwise_crypto_len: u8,
    pub grp_crypto_type: u8,
    pub grp_crypto_len: u8,
    pub ssid_len: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ch: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub ctrl_flags: __le32,
    pub nw_subtype: u8,
    pub __packed: },
// WMI_RECONNECT_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_reconnect_cmd {
// channel hint
    pub channel: __le16,
// mandatory if set
    pub bssid: [u8; ETH_ALEN],
    pub __packed: },
// WMI_ADD_CIPHER_KEY_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum key_usage {
    PAIRWISE_USAGE = 0x00,
    GROUP_USAGE = 0x01,

// default Tx Key - static WEP only
    TX_USAGE = 0x02,
}

//
// Bit Flag
// Bit 0 - Initialise TSC - default is Initialize
//
pub const KEY_OP_INIT_TSC: c_uint = 0x01;
pub const KEY_OP_INIT_RSC: c_uint = 0x02;
// default initialise the TSC & RSC
pub const KEY_OP_INIT_VAL: c_uint = 0x03;
pub const KEY_OP_VALID_MASK: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_add_cipher_key_cmd {
    pub key_index: u8,
    pub key_type: u8,
// enum key_usage
    pub key_usage: u8,
    pub key_len: u8,
// key replay sequence counter
    pub key_rsc: [u8; 8],
    pub key: [u8; WLAN_MAX_KEY_LEN],
// additional key control info
    pub key_op_ctrl: u8,
    pub key_mac_addr: [u8; ETH_ALEN],
    pub __packed: },
// WMI_DELETE_CIPHER_KEY_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_delete_cipher_key_cmd {
    pub key_index: u8,
    pub __packed: },
pub const WMI_KRK_LEN: c_int = 16;
// WMI_ADD_KRK_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_add_krk_cmd {
    pub krk: [u8; WMI_KRK_LEN],
    pub __packed: },
// WMI_SETPMKID_CMDID
pub const WMI_PMKID_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmkid_enable_flg {
    PMKID_DISABLE = 0,
    PMKID_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_setpmkid_cmd {
    pub bssid: [u8; ETH_ALEN],
// enum pmkid_enable_flg
    pub enable: u8,
    pub pmkid: [u8; WMI_PMKID_LEN],
    pub __packed: },
// WMI_START_SCAN_CMD
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_scan_type {
    WMI_LONG_SCAN = 0,
    WMI_SHORT_SCAN = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_supp_rates {
    pub nrates: u8,
    pub rates: [u8; ATH6KL_RATE_MAXSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_begin_scan_cmd {
    pub force_fg_scan: __le32,
// for legacy cisco AP compatibility
    pub is_legacy: __le32,
// max duration in the home channel(msec)
    pub home_dwell_time: __le32,
// time interval between scans (msec)
    pub force_scan_intvl: __le32,
// no CCK rates
    pub no_cck: __le32,
// enum wmi_scan_type
    pub scan_type: u8,
// Supported rates to advertise in the probe request frames
    pub supp_rates: [wmi_supp_rates; ATH6KL_NUM_BANDS],
// how many channels follow
    pub num_ch: u8,
// channels in Mhz
    pub ch_list: [__le16; ],
    pub __packed: },
// wmi_start_scan_cmd is to be deprecated. Use
// wmi_begin_scan_cmd instead. The new structure supports P2P mgmt
// operations using station interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_start_scan_cmd {
    pub force_fg_scan: __le32,
// for legacy cisco AP compatibility
    pub is_legacy: __le32,
// max duration in the home channel(msec)
    pub home_dwell_time: __le32,
// time interval between scans (msec)
    pub force_scan_intvl: __le32,
// enum wmi_scan_type
    pub scan_type: u8,
// how many channels follow
    pub num_ch: u8,
// channels in Mhz
    pub ch_list: [__le16; ],
    pub __packed: },
//
// Warning: scan control flag value of 0xFF is used to disable
// all flags in WMI_SCAN_PARAMS_CMD. Do not add any more
// flags here
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_scan_ctrl_flags_bits {
// set if can scan in the connect cmd
    CONNECT_SCAN_CTRL_FLAGS = 0x01,

// set if scan for the SSID it is already connected to
    SCAN_CONNECTED_CTRL_FLAGS = 0x02,

// set if enable active scan
    ACTIVE_SCAN_CTRL_FLAGS = 0x04,

// set if enable roam scan when bmiss and lowrssi
    ROAM_SCAN_CTRL_FLAGS = 0x08,

// set if follows customer BSSINFO reporting rule
    REPORT_BSSINFO_CTRL_FLAGS = 0x10,

// if disabled, target doesn't scan after a disconnect event
    ENABLE_AUTO_CTRL_FLAGS = 0x20,

//
// Scan complete event with canceled status will be generated when
// a scan is prempted before it gets completed.
//
    ENABLE_SCAN_ABORT_EVENT = 0x40
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_scan_params_cmd {
// sec
    pub fg_start_period: __le16,
// sec
    pub fg_end_period: __le16,
// sec
    pub bg_period: __le16,
// msec
    pub maxact_chdwell_time: __le16,
// msec
    pub pas_chdwell_time: __le16,
// how many shorts scan for one long
    pub short_scan_ratio: u8,
    pub scan_ctrl_flags: u8,
// msec
    pub minact_chdwell_time: __le16,
// max active scans per ssid
    pub maxact_scan_per_ssid: __le16,
// msecs
    pub max_dfsch_act_time: __le32,
    pub __packed: },
// WMI_ENABLE_SCHED_SCAN_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_enable_sched_scan_cmd {
    pub enable: u8,
    pub __packed: },
// WMI_SET_BSS_FILTER_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_bss_filter {
// no beacons forwarded
    NONE_BSS_FILTER = 0x0,

// all beacons forwarded
    ALL_BSS_FILTER,

// only beacons matching profile
    PROFILE_FILTER,

// all but beacons matching profile
    ALL_BUT_PROFILE_FILTER,

// only beacons matching current BSS
    CURRENT_BSS_FILTER,

// all but beacons matching BSS
    ALL_BUT_BSS_FILTER,

// beacons matching probed ssid
    PROBED_SSID_FILTER,

// beacons matching matched ssid
    MATCHED_SSID_FILTER,

// marker only
    LAST_BSS_FILTER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bss_filter_cmd {
// see, enum wmi_bss_filter
    pub bss_filter: u8,
// for alignment
    pub reserved1: u8,
// for alignment
    pub reserved2: __le16,
    pub ie_mask: __le32,
    pub __packed: },
// WMI_SET_PROBED_SSID_CMDID
pub const MAX_PROBED_SSIDS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ssid_flag {
// disables entry
    DISABLE_SSID_FLAG = 0,

// probes specified ssid
    SPECIFIC_SSID_FLAG = 0x01,

// probes for any ssid
    ANY_SSID_FLAG = 0x02,

// match for ssid
    MATCH_SSID_FLAG = 0x08,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_probed_ssid_cmd {
// 0 to MAX_PROBED_SSIDS - 1
    pub entry_index: u8,
// see, enum wmi_ssid_flg
    pub flag: u8,
    pub ssid_len: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub __packed: },
//
// WMI_SET_LISTEN_INT_CMDID
// The Listen interval is between 15 and 3000 TUs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_listen_int_cmd {
    pub listen_intvl: __le16,
    pub num_beacons: __le16,
    pub __packed: },
// WMI_SET_BMISS_TIME_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bmiss_time_cmd {
    pub bmiss_time: __le16,
    pub num_beacons: __le16,
}

// WMI_STA_ENHANCE_BMISS_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sta_bmiss_enhance_cmd {
    pub enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_regdomain_cmd {
    pub length: u8,
    pub iso_name: [u8; 2],
    pub __packed: },
// WMI_SET_POWER_MODE_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_power_mode {
    REC_POWER = 0x01,
    MAX_PERF_POWER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_power_mode_cmd {
// see, enum wmi_power_mode
    pub pwr_mode: u8,
    pub __packed: },
//
// Policy to determine whether power save failure event should be sent to
// host during scanning
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_save_fail_event_policy {
    SEND_POWER_SAVE_FAIL_EVENT_ALWAYS = 1,
    IGNORE_PS_FAIL_DURING_SCAN = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_power_params_cmd {
// msec
    pub idle_period: __le16,
    pub pspoll_number: __le16,
    pub dtim_policy: __le16,
    pub tx_wakeup_policy: __le16,
    pub num_tx_to_wakeup: __le16,
    pub ps_fail_event_policy: __le16,
    pub __packed: },
//
// Ratemask for below modes should be passed
// to WMI_SET_TX_SELECT_RATES_CMDID.
// AR6003 has 32 bit mask for each modes.
// First 12 bits for legacy rates, 13 to 20
// bits for HT 20 rates and 21 to 28 bits for
// HT 40 rates
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_mode_phy {
    WMI_RATES_MODE_11A = 0,
    WMI_RATES_MODE_11G,
    WMI_RATES_MODE_11B,
    WMI_RATES_MODE_11GONLY,
    WMI_RATES_MODE_11A_HT20,
    WMI_RATES_MODE_11G_HT20,
    WMI_RATES_MODE_11A_HT40,
    WMI_RATES_MODE_11G_HT40,
    WMI_RATES_MODE_MAX
}

// WMI_SET_TX_SELECT_RATES_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_tx_select_rates32_cmd {
    pub ratemask: [__le32; WMI_RATES_MODE_MAX],
    pub __packed: },
// WMI_SET_TX_SELECT_RATES_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_tx_select_rates64_cmd {
    pub ratemask: [__le64; WMI_RATES_MODE_MAX],
    pub __packed: },
// WMI_SET_DISC_TIMEOUT_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_disc_timeout_cmd {
// seconds
    pub discon_timeout: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dir_type {
    UPLINK_TRAFFIC = 0,
    DNLINK_TRAFFIC = 1,
    BIDIR_TRAFFIC = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum voiceps_cap_type {
    DISABLE_FOR_THIS_AC = 0,
    ENABLE_FOR_THIS_AC = 1,
    ENABLE_FOR_ALL_AC = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum traffic_type {
    TRAFFIC_TYPE_APERIODIC = 0,
    TRAFFIC_TYPE_PERIODIC = 1,
}

// WMI_SYNCHRONIZE_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sync_cmd {
    pub data_sync_map: u8,
    pub __packed: },
// WMI_CREATE_PSTREAM_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_create_pstream_cmd {
// msec
    pub min_service_int: __le32,
// msec
    pub max_service_int: __le32,
// msec
    pub inactivity_int: __le32,
// msec
    pub suspension_int: __le32,
    pub service_start_time: __le32,
// in bps
    pub min_data_rate: __le32,
// in bps
    pub mean_data_rate: __le32,
// in bps
    pub peak_data_rate: __le32,
    pub max_burst_size: __le32,
    pub delay_bound: __le32,
// in bps
    pub min_phy_rate: __le32,
    pub sba: __le32,
    pub medium_time: __le32,
// in octets
    pub nominal_msdu: __le16,
// in octets
    pub max_msdu: __le16,
    pub traffic_class: u8,
// see, enum dir_type
    pub traffic_direc: u8,
    pub rx_queue_num: u8,
// see, enum traffic_type
    pub traffic_type: u8,
// see, enum voiceps_cap_type
    pub voice_psc_cap: u8,
    pub tsid: u8,
// 802.1D user priority
    pub user_pri: u8,
// nominal phy rate
    pub nominal_phy: u8,
    pub __packed: },
// WMI_DELETE_PSTREAM_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_delete_pstream_cmd {
    pub tx_queue_num: u8,
    pub rx_queue_num: u8,
    pub traffic_direc: u8,
    pub traffic_class: u8,
    pub tsid: u8,
    pub __packed: },
// WMI_SET_CHANNEL_PARAMS_CMDID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_phy_mode {
    WMI_11A_MODE = 0x1,
    WMI_11G_MODE = 0x2,
    WMI_11AG_MODE = 0x3,
    WMI_11B_MODE = 0x4,
    WMI_11GONLY_MODE = 0x5,
    WMI_11G_HT20	= 0x6,
}

pub const WMI_MAX_CHANNELS: c_int = 32;
//
// WMI_RSSI_THRESHOLD_PARAMS_CMDID
// Setting the polltime to 0 would disable polling. Threshold values are
// in the ascending order, and should agree to:
// (lowThreshold_lowerVal < lowThreshold_upperVal < highThreshold_lowerVal
// < highThreshold_upperVal)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rssi_threshold_params_cmd {
// polling time as a factor of LI
    pub poll_time: __le32,
// lowest of upper
    pub thresh_above1_val: a_sle16,
    pub thresh_above2_val: a_sle16,
    pub thresh_above3_val: a_sle16,
    pub thresh_above4_val: a_sle16,
    pub thresh_above5_val: a_sle16,
// highest of upper
    pub thresh_above6_val: a_sle16,
// lowest of below
    pub thresh_below1_val: a_sle16,
    pub thresh_below2_val: a_sle16,
    pub thresh_below3_val: a_sle16,
    pub thresh_below4_val: a_sle16,
    pub thresh_below5_val: a_sle16,
// highest of below
    pub thresh_below6_val: a_sle16,
// "alpha"
    pub weight: u8,
    pub reserved: [u8; 3],
    pub __packed: },
//
// WMI_SNR_THRESHOLD_PARAMS_CMDID
// Setting the polltime to 0 would disable polling.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_snr_threshold_params_cmd {
// polling time as a factor of LI
    pub poll_time: __le32,
// "alpha"
    pub weight: u8,
// lowest of upper
    pub thresh_above1_val: u8,
    pub thresh_above2_val: u8,
    pub thresh_above3_val: u8,
// highest of upper
    pub thresh_above4_val: u8,
// lowest of below
    pub thresh_below1_val: u8,
    pub thresh_below2_val: u8,
    pub thresh_below3_val: u8,
// highest of below
    pub thresh_below4_val: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// Don't report BSSs with signal (RSSI) below this threshold
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_rssi_filter_cmd {
    pub rssi: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_preamble_policy {
    WMI_IGNORE_BARKER_IN_ERP = 0,
    WMI_FOLLOW_BARKER_IN_ERP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_lpreamble_cmd {
    pub status: u8,
    pub preamble_policy: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_rts_cmd {
    pub threshold: __le16,
    pub __packed: },
// WMI_SET_TX_PWR_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_tx_pwr_cmd {
// in dbM units
    pub dbM: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tx_pwr_reply {
// in dbM units
    pub dbM: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_report_sleep_state_event {
    pub sleep_state: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_report_sleep_status {
    WMI_REPORT_SLEEP_STATUS_IS_DEEP_SLEEP = 0,
    WMI_REPORT_SLEEP_STATUS_IS_AWAKE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_event_report_config {
// default
    DISCONN_EVT_IN_RECONN = 0,

    NO_DISCONN_EVT_IN_RECONN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mcast_filter_cmd {
    pub mcast_all_enable: u8,
    pub __packed: },
pub const ATH6KL_MCAST_FILTER_MAC_ADDR_SIZE: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_mcast_filter_add_del_cmd {
    pub mcast_mac: [u8; ATH6KL_MCAST_FILTER_MAC_ADDR_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_htcap_cmd {
    pub band: u8,
    pub ht_enable: u8,
    pub ht40_supported: u8,
    pub ht20_sgi: u8,
    pub ht40_sgi: u8,
    pub intolerant_40mhz: u8,
    pub max_ampdu_len_exp: u8,
    pub __packed: },
// Command Replies
// WMI_GET_CHANNEL_LIST_CMDID reply
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_channel_list_reply {
    pub reserved: u8,
// number of channels in reply
    pub num_ch: u8,
// channel in Mhz
    pub ch_list: [__le16; ],
    pub __packed: },
// List of Events (target to host)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_event_id {
    WMI_READY_EVENTID = 0x1001,
    WMI_CONNECT_EVENTID,
    WMI_DISCONNECT_EVENTID,
    WMI_BSSINFO_EVENTID,
    WMI_CMDERROR_EVENTID,
    WMI_REGDOMAIN_EVENTID,
    WMI_PSTREAM_TIMEOUT_EVENTID,
    WMI_NEIGHBOR_REPORT_EVENTID,
    WMI_TKIP_MICERR_EVENTID,
    WMI_SCAN_COMPLETE_EVENTID,	/* 0x100a */
    WMI_REPORT_STATISTICS_EVENTID,
    WMI_RSSI_THRESHOLD_EVENTID,
    WMI_ERROR_REPORT_EVENTID,
    WMI_OPT_RX_FRAME_EVENTID,
    WMI_REPORT_ROAM_TBL_EVENTID,
    WMI_EXTENSION_EVENTID,
    WMI_CAC_EVENTID,
    WMI_SNR_THRESHOLD_EVENTID,
    WMI_LQ_THRESHOLD_EVENTID,
    WMI_TX_RETRY_ERR_EVENTID,	/* 0x1014 */
    WMI_REPORT_ROAM_DATA_EVENTID,
    WMI_TEST_EVENTID,
    WMI_APLIST_EVENTID,
    WMI_GET_WOW_LIST_EVENTID,
    WMI_GET_PMKID_LIST_EVENTID,
    WMI_CHANNEL_CHANGE_EVENTID,
    WMI_PEER_NODE_EVENTID,
    WMI_PSPOLL_EVENTID,
    WMI_DTIMEXPIRY_EVENTID,
    WMI_WLAN_VERSION_EVENTID,
    WMI_SET_PARAMS_REPLY_EVENTID,
    WMI_ADDBA_REQ_EVENTID,		/*0x1020 */
    WMI_ADDBA_RESP_EVENTID,
    WMI_DELBA_REQ_EVENTID,
    WMI_TX_COMPLETE_EVENTID,
    WMI_HCI_EVENT_EVENTID,
    WMI_ACL_DATA_EVENTID,
    WMI_REPORT_SLEEP_STATE_EVENTID,
    WMI_REPORT_BTCOEX_STATS_EVENTID,
    WMI_REPORT_BTCOEX_CONFIG_EVENTID,
    WMI_GET_PMK_EVENTID,

// DFS Events
    WMI_DFS_HOST_ATTACH_EVENTID,
    WMI_DFS_HOST_INIT_EVENTID,
    WMI_DFS_RESET_DELAYLINES_EVENTID,
    WMI_DFS_RESET_RADARQ_EVENTID,
    WMI_DFS_RESET_AR_EVENTID,
    WMI_DFS_RESET_ARQ_EVENTID,
    WMI_DFS_SET_DUR_MULTIPLIER_EVENTID,
    WMI_DFS_SET_BANGRADAR_EVENTID,
    WMI_DFS_SET_DEBUGLEVEL_EVENTID,
    WMI_DFS_PHYERR_EVENTID,

// CCX Evants
    WMI_CCX_RM_STATUS_EVENTID,

// P2P Events
    WMI_P2P_GO_NEG_RESULT_EVENTID,

    WMI_WAC_SCAN_DONE_EVENTID,
    WMI_WAC_REPORT_BSS_EVENTID,
    WMI_WAC_START_WPS_EVENTID,
    WMI_WAC_CTRL_REQ_REPLY_EVENTID,

    WMI_REPORT_WMM_PARAMS_EVENTID,
    WMI_WAC_REJECT_WPS_EVENTID,

// More P2P Events
    WMI_P2P_GO_NEG_REQ_EVENTID,
    WMI_P2P_INVITE_REQ_EVENTID,
    WMI_P2P_INVITE_RCVD_RESULT_EVENTID,
    WMI_P2P_INVITE_SENT_RESULT_EVENTID,
    WMI_P2P_PROV_DISC_RESP_EVENTID,
    WMI_P2P_PROV_DISC_REQ_EVENTID,

// RFKILL Events
    WMI_RFKILL_STATE_CHANGE_EVENTID,
    WMI_RFKILL_GET_MODE_CMD_EVENTID,

    WMI_P2P_START_SDPD_EVENTID,
    WMI_P2P_SDPD_RX_EVENTID,

    WMI_SET_HOST_SLEEP_MODE_CMD_PROCESSED_EVENTID = 0x1047,

    WMI_THIN_RESERVED_START_EVENTID = 0x8000,
// Events in this range are reserved for thinmode
    WMI_THIN_RESERVED_END_EVENTID = 0x8fff,

    WMI_SET_CHANNEL_EVENTID,
    WMI_ASSOC_REQ_EVENTID,

// Generic ACS event
    WMI_ACS_EVENTID,
    WMI_STORERECALL_STORE_EVENTID,
    WMI_WOW_EXT_WAKE_EVENTID,
    WMI_GTK_OFFLOAD_STATUS_EVENTID,
    WMI_NETWORK_LIST_OFFLOAD_EVENTID,
    WMI_REMAIN_ON_CHNL_EVENTID,
    WMI_CANCEL_REMAIN_ON_CHNL_EVENTID,
    WMI_TX_STATUS_EVENTID,
    WMI_RX_PROBE_REQ_EVENTID,
    WMI_P2P_CAPABILITIES_EVENTID,
    WMI_RX_ACTION_EVENTID,
    WMI_P2P_INFO_EVENTID,

// WPS Events
    WMI_WPS_GET_STATUS_EVENTID,
    WMI_WPS_PROFILE_EVENTID,

// more P2P events
    WMI_NOA_INFO_EVENTID,
    WMI_OPPPS_INFO_EVENTID,
    WMI_PORT_STATUS_EVENTID,

// 802.11w
    WMI_GET_RSN_CAP_EVENTID,

    WMI_TXE_NOTIFY_EVENTID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ready_event_2 {
    pub sw_version: __le32,
    pub abi_version: __le32,
    pub mac_addr: [u8; ETH_ALEN],
    pub phy_cap: u8,
    pub __packed: },
// WMI_PHY_CAPABILITY
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_phy_cap {
    WMI_11A_CAP = 0x01,
    WMI_11G_CAP = 0x02,
    WMI_11AG_CAP = 0x03,
    WMI_11AN_CAP = 0x04,
    WMI_11GN_CAP = 0x05,
    WMI_11AGN_CAP = 0x06,
}

// Connect Event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_connect_event {
    pub ch: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub listen_intvl: __le16,
    pub beacon_intvl: __le16,
    pub nw_type: __le32,
    pub sta: },
    pub aid: u8,
    pub phymode: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub auth: u8,
    pub keymgmt: u8,
    pub cipher: __le16,
    pub apsd_info: u8,
    pub unused: [u8; 3],
    pub ap_sta: },
    pub ch: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub unused: [u8; 8],
    pub ap_bss: },
    pub u: },
    pub beacon_ie_len: u8,
    pub assoc_req_len: u8,
    pub assoc_resp_len: u8,
    pub assoc_info: [u8; ],
    pub __packed: },
// Disconnect Event
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_disconnect_reason {
    NO_NETWORK_AVAIL = 0x01,

// bmiss
    LOST_LINK = 0x02,

    DISCONNECT_CMD = 0x03,
    BSS_DISCONNECTED = 0x04,
    AUTH_FAILED = 0x05,
    ASSOC_FAILED = 0x06,
    NO_RESOURCES_AVAIL = 0x07,
    CSERV_DISCONNECT = 0x08,
    INVALID_PROFILE = 0x0a,
    DOT11H_CHANNEL_SWITCH = 0x0b,
    PROFILE_MISMATCH = 0x0c,
    CONNECTION_EVICTED = 0x0d,
    IBSS_MERGE = 0xe,
}

// AP mode disconnect proto_reasons
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ap_disconnect_reason {
    WMI_AP_REASON_STA_LEFT		= 101,
    WMI_AP_REASON_FROM_HOST		= 102,
    WMI_AP_REASON_COMM_TIMEOUT	= 103,
    WMI_AP_REASON_MAX_STA		= 104,
    WMI_AP_REASON_ACL		= 105,
    WMI_AP_REASON_STA_ROAM		= 106,
    WMI_AP_REASON_DFS_CHANNEL	= 107,
}

pub const ATH6KL_COUNTRY_RD_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_wmi_regdomain {
    pub reg_code: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_disconnect_event {
// reason code, see 802.11 spec.
    pub proto_reason_status: __le16,
// set if known
    pub bssid: [u8; ETH_ALEN],
// see WMI_DISCONNECT_REASON
    pub disconn_reason: u8,
    pub assoc_resp_len: u8,
    pub assoc_info: [u8; ],
    pub __packed: },
//
// BSS Info Event.
// Mechanism used to inform host of the presence and characteristic of
// wireless networks present.  Consists of bss info header followed by
// the beacon or probe-response frame body.  The 802.11 header is no included.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_bi_ftype {
    BEACON_FTYPE = 0x1,
    PROBERESP_FTYPE,
    ACTION_MGMT_FTYPE,
    PROBEREQ_FTYPE,
}

pub const DEF_LRSSI_SCAN_PERIOD: c_int = 5;
pub const DEF_LRSSI_ROAM_THRESHOLD: c_int = 20;
pub const DEF_LRSSI_ROAM_FLOOR: c_int = 60;
pub const DEF_SCAN_FOR_ROAM_INTVL: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_roam_ctrl {
    WMI_FORCE_ROAM = 1,
    WMI_SET_ROAM_MODE,
    WMI_SET_HOST_BIAS,
    WMI_SET_LRSSI_SCAN_PARAMS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_roam_mode {
    WMI_DEFAULT_ROAM_MODE = 1, /* RSSI based roam */
    WMI_HOST_BIAS_ROAM_MODE = 2, /* Host bias based roam */
    WMI_LOCK_BSS_MODE = 3, /* Lock to the current BSS */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct low_rssi_scan_params {
    pub lrssi_scan_period: __le16,
    pub lrssi_scan_threshold: a_sle16,
    pub lrssi_roam_threshold: a_sle16,
    pub roam_rssi_floor: u8,
    pub reserved: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roam_ctrl_cmd {
    pub /: *mut *mut u8 bssid[ETH_ALEN]; / WMI_FORCE_ROAM,
    pub /: *mut *mut u8 roam_mode; / WMI_SET_ROAM_MODE,
    pub WMI_SET_LRSSI_SCAN_PARAMS: *mut *mut low_rssi_scan_params params; /,
//
    pub info: } __packed,
    pub roam_ctrl: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_beacon_int_cmd {
    pub beacon_intvl: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_dtim_cmd {
    pub dtim_period: __le32,
    pub __packed: },
// BSS INFO HDR version 2.0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bss_info_hdr2 {
    pub /: *mut *mut __le16 ch; / frequency in MHz,
// see, enum wmi_bi_ftype
    pub frame_type: u8,
    pub /: *mut *mut u8 snr; / note: rssi = snr - 95 dBm,
    pub bssid: [u8; ETH_ALEN],
    pub ie_mask: __le16,
    pub __packed: },
// Command Error Event
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_error_code {
    INVALID_PARAM = 0x01,
    ILLEGAL_STATE = 0x02,
    INTERNAL_ERROR = 0x03,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cmd_error_event {
    pub cmd_id: __le16,
    pub err_code: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pstream_timeout_event {
    pub tx_queue_num: u8,
    pub rx_queue_num: u8,
    pub traffic_direc: u8,
    pub traffic_class: u8,
    pub __packed: },
//
// The WMI_NEIGHBOR_REPORT Event is generated by the target to inform
// the host of BSS's it has found that matches the current profile.
// It can be used by the host to cache PMKs and/to initiate pre-authentication
// if the BSS supports it.  The first bssid is always the current associated
// BSS.
// The bssid and bssFlags information repeats according to the number
// or APs reported.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_bss_flags {
    WMI_DEFAULT_BSS_FLAGS = 0x00,
    WMI_PREAUTH_CAPABLE_BSS = 0x01,
    WMI_PMKID_VALID_BSS = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_neighbor_info {
    pub bssid: [u8; ETH_ALEN],
    pub /: *mut *mut u8 bss_flags; / enum wmi_bss_flags,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_neighbor_report_event {
    pub num_neighbors: u8,
    pub neighbor: [wmi_neighbor_info; ],
    pub __packed: },
// TKIP MIC Error Event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tkip_micerr_event {
    pub key_id: u8,
    pub is_mcast: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_scan_status {
    WMI_SCAN_STATUS_SUCCESS = 0,
}

// WMI_SCAN_COMPLETE_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_scan_complete_event {
    pub status: a_sle32,
    pub __packed: },
pub const MAX_OPT_DATA_LEN: c_int = 1400;
//
// Special frame receive Event.
// Mechanism used to inform host of the reception of the special frames.
// Consists of special frame info header followed by special frame body.
// The 802.11 header is not included.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_opt_rx_info_hdr {
    pub ch: __le16,
    pub frame_type: u8,
    pub snr: i8,
    pub src_addr: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub __packed: },
// Reporting statistic
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_stats {
    pub pkt: __le32,
    pub byte: __le32,
    pub ucast_pkt: __le32,
    pub ucast_byte: __le32,
    pub mcast_pkt: __le32,
    pub mcast_byte: __le32,
    pub bcast_pkt: __le32,
    pub bcast_byte: __le32,
    pub rts_success_cnt: __le32,
    pub pkt_per_ac: [__le32; 4],
    pub err_per_ac: [__le32; 4],
    pub err: __le32,
    pub fail_cnt: __le32,
    pub retry_cnt: __le32,
    pub mult_retry_cnt: __le32,
    pub rts_fail_cnt: __le32,
    pub ucast_rate: a_sle32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_stats {
    pub pkt: __le32,
    pub byte: __le32,
    pub ucast_pkt: __le32,
    pub ucast_byte: __le32,
    pub mcast_pkt: __le32,
    pub mcast_byte: __le32,
    pub bcast_pkt: __le32,
    pub bcast_byte: __le32,
    pub frgment_pkt: __le32,
    pub err: __le32,
    pub crc_err: __le32,
    pub key_cache_miss: __le32,
    pub decrypt_err: __le32,
    pub dupl_frame: __le32,
    pub ucast_rate: a_sle32,
    pub __packed: },
pub const RATE_INDEX_WITHOUT_SGI_MASK: c_uint = 0x7f;
pub const RATE_INDEX_MSB: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tkip_ccmp_stats {
    pub tkip_local_mic_fail: __le32,
    pub tkip_cnter_measures_invoked: __le32,
    pub tkip_replays: __le32,
    pub tkip_fmt_err: __le32,
    pub ccmp_fmt_err: __le32,
    pub ccmp_replays: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_stats {
    pub pwr_save_failure_cnt: __le32,
    pub stop_tx_failure_cnt: __le16,
    pub atim_tx_failure_cnt: __le16,
    pub atim_rx_failure_cnt: __le16,
    pub bcn_rx_failure_cnt: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cserv_stats {
    pub cs_bmiss_cnt: __le32,
    pub cs_low_rssi_cnt: __le32,
    pub cs_connect_cnt: __le16,
    pub cs_discon_cnt: __le16,
    pub cs_ave_beacon_rssi: a_sle16,
    pub cs_roam_count: __le16,
    pub cs_rssi: a_sle16,
    pub cs_snr: u8,
    pub cs_ave_beacon_snr: u8,
    pub cs_last_roam_msec: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlan_net_stats {
    pub tx: tx_stats,
    pub rx: rx_stats,
    pub tkip_ccmp_stats: tkip_ccmp_stats,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arp_stats {
    pub arp_received: __le32,
    pub arp_matched: __le32,
    pub arp_replied: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wlan_wow_stats {
    pub wow_pkt_dropped: __le32,
    pub wow_evt_discarded: __le16,
    pub wow_host_pkt_wakeups: u8,
    pub wow_host_evt_wakeups: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_target_stats {
    pub lq_val: __le32,
    pub noise_floor_calib: a_sle32,
    pub pm_stats: pm_stats,
    pub stats: wlan_net_stats,
    pub wow_stats: wlan_wow_stats,
    pub arp_stats: arp_stats,
    pub cserv_stats: cserv_stats,
    pub __packed: },
//
// WMI_RSSI_THRESHOLD_EVENTID.
// Indicate the RSSI events to host. Events are indicated when we breach a
// threshold value.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_rssi_threshold_val {
    WMI_RSSI_THRESHOLD1_ABOVE = 0,
    WMI_RSSI_THRESHOLD2_ABOVE,
    WMI_RSSI_THRESHOLD3_ABOVE,
    WMI_RSSI_THRESHOLD4_ABOVE,
    WMI_RSSI_THRESHOLD5_ABOVE,
    WMI_RSSI_THRESHOLD6_ABOVE,
    WMI_RSSI_THRESHOLD1_BELOW,
    WMI_RSSI_THRESHOLD2_BELOW,
    WMI_RSSI_THRESHOLD3_BELOW,
    WMI_RSSI_THRESHOLD4_BELOW,
    WMI_RSSI_THRESHOLD5_BELOW,
    WMI_RSSI_THRESHOLD6_BELOW
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rssi_threshold_event {
    pub rssi: a_sle16,
    pub range: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_snr_threshold_val {
    WMI_SNR_THRESHOLD1_ABOVE = 1,
    WMI_SNR_THRESHOLD1_BELOW,
    WMI_SNR_THRESHOLD2_ABOVE,
    WMI_SNR_THRESHOLD2_BELOW,
    WMI_SNR_THRESHOLD3_ABOVE,
    WMI_SNR_THRESHOLD3_BELOW,
    WMI_SNR_THRESHOLD4_ABOVE,
    WMI_SNR_THRESHOLD4_BELOW
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_snr_threshold_event {
// see, enum wmi_snr_threshold_val
    pub range: u8,
    pub snr: u8,
    pub __packed: },
// WMI_REPORT_ROAM_TBL_EVENTID
pub const MAX_ROAM_TBL_CAND: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bss_roam_info {
    pub roam_util: a_sle32,
    pub bssid: [u8; ETH_ALEN],
    pub rssi: i8,
    pub rssidt: i8,
    pub last_rssi: i8,
    pub util: i8,
    pub bias: i8,
// for alignment
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_target_roam_tbl {
    pub roam_mode: __le16,
    pub num_entries: __le16,
    pub info: [wmi_bss_roam_info; ],
    pub __packed: },
// WMI_CAC_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cac_indication {
    CAC_INDICATION_ADMISSION = 0x00,
    CAC_INDICATION_ADMISSION_RESP = 0x01,
    CAC_INDICATION_DELETE = 0x02,
    CAC_INDICATION_NO_RESP = 0x03,
}

pub const WMM_TSPEC_IE_LEN: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cac_event {
    pub ac: u8,
    pub cac_indication: u8,
    pub status_code: u8,
    pub tspec_suggestion: [u8; WMM_TSPEC_IE_LEN],
    pub __packed: },
// WMI_APLIST_EVENTID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aplist_ver {
    APLIST_VER1 = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ap_info_v1 {
    pub bssid: [u8; ETH_ALEN],
    pub channel: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union wmi_ap_info {
    pub ap_info_v1: wmi_ap_info_v1,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_aplist_event {
    pub ap_list_ver: u8,
    pub num_ap: u8,
    pub ap_list: [wmi_ap_info; ],
    pub __packed: },
// Developer Commands
//
// WMI_SET_BITRATE_CMDID
//
// Get bit rate cmd uses same definition as set bit rate cmd
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_bit_rate {
    RATE_AUTO = -1,
    RATE_1Mb = 0,
    RATE_2Mb = 1,
    RATE_5_5Mb = 2,
    RATE_11Mb = 3,
    RATE_6Mb = 4,
    RATE_9Mb = 5,
    RATE_12Mb = 6,
    RATE_18Mb = 7,
    RATE_24Mb = 8,
    RATE_36Mb = 9,
    RATE_48Mb = 10,
    RATE_54Mb = 11,
    RATE_MCS_0_20 = 12,
    RATE_MCS_1_20 = 13,
    RATE_MCS_2_20 = 14,
    RATE_MCS_3_20 = 15,
    RATE_MCS_4_20 = 16,
    RATE_MCS_5_20 = 17,
    RATE_MCS_6_20 = 18,
    RATE_MCS_7_20 = 19,
    RATE_MCS_0_40 = 20,
    RATE_MCS_1_40 = 21,
    RATE_MCS_2_40 = 22,
    RATE_MCS_3_40 = 23,
    RATE_MCS_4_40 = 24,
    RATE_MCS_5_40 = 25,
    RATE_MCS_6_40 = 26,
    RATE_MCS_7_40 = 27,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_bit_rate_reply {
// see, enum wmi_bit_rate
    pub rate_index: i8,
    pub __packed: },
//
// WMI_SET_FIXRATES_CMDID
//
// Get fix rates cmd uses same definition as set fix rates cmd
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_fix_rates_reply {
// see wmi_bit_rate
    pub fix_rate_mask: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum roam_data_type {
// get the roam time data
    ROAM_DATA_TIME = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_target_roam_time {
    pub disassoc_time: __le32,
    pub no_txrx_time: __le32,
    pub assoc_time: __le32,
    pub allow_txrx_time: __le32,
    pub disassoc_bssid: [u8; ETH_ALEN],
    pub disassoc_bss_rssi: i8,
    pub assoc_bssid: [u8; ETH_ALEN],
    pub assoc_bss_rssi: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_txop_cfg {
    WMI_TXOP_DISABLED = 0,
    WMI_TXOP_ENABLED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_wmm_txop_cmd {
    pub txop_enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_keepalive_cmd {
    pub keep_alive_intvl: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_keepalive_cmd {
    pub configured: __le32,
    pub keep_alive_intvl: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_appie_cmd {
    pub /: *mut *mut u8 mgmt_frm_type; / enum wmi_mgmt_frame_type,
    pub ie_len: u8,
    pub ie_info: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_ie_cmd {
    pub ie_id: u8,
    pub /: *mut *mut u8 ie_field; / enum wmi_ie_field_type,
    pub ie_len: u8,
    pub reserved: u8,
    pub ie_info: [u8; ],
    pub __packed: },
// Notify the WSC registration status to the target
pub const WSC_REG_ACTIVE: c_int = 1;
pub const WSC_REG_INACTIVE: c_int = 0;
pub const WOW_MAX_FILTERS_PER_LIST: c_int = 4;
pub const WOW_PATTERN_SIZE: c_int = 64;
pub const MAC_MAX_FILTERS_PER_LIST: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wow_filter {
    pub wow_valid_filter: u8,
    pub wow_filter_id: u8,
    pub wow_filter_size: u8,
    pub wow_filter_offset: u8,
    pub wow_filter_mask: [u8; WOW_PATTERN_SIZE],
    pub wow_filter_pattern: [u8; WOW_PATTERN_SIZE],
    pub __packed: },
pub const MAX_IP_ADDRS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_ip_cmd {
// IP in network byte order
    pub ips: [__be32; MAX_IP_ADDRS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_wow_filters {
    WOW_FILTER_SSID			= BIT(1),
    WOW_FILTER_OPTION_MAGIC_PACKET  = BIT(2),
    WOW_FILTER_OPTION_EAP_REQ	= BIT(3),
    WOW_FILTER_OPTION_PATTERNS	= BIT(4),
    WOW_FILTER_OPTION_OFFLOAD_ARP	= BIT(5),
    WOW_FILTER_OPTION_OFFLOAD_NS	= BIT(6),
    WOW_FILTER_OPTION_OFFLOAD_GTK	= BIT(7),
    WOW_FILTER_OPTION_8021X_4WAYHS	= BIT(8),
    WOW_FILTER_OPTION_NLO_DISCVRY	= BIT(9),
    WOW_FILTER_OPTION_NWK_DISASSOC	= BIT(10),
    WOW_FILTER_OPTION_GTK_ERROR	= BIT(11),
    WOW_FILTER_OPTION_TEST_MODE	= BIT(15),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_host_mode {
    ATH6KL_HOST_MODE_AWAKE,
    ATH6KL_HOST_MODE_ASLEEP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_host_sleep_mode_cmd {
    pub awake: __le32,
    pub asleep: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath6kl_wow_mode {
    ATH6KL_WOW_MODE_DISABLE,
    ATH6KL_WOW_MODE_ENABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_wow_mode_cmd {
    pub enable_wow: __le32,
    pub filter: __le32,
    pub host_req_delay: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_add_wow_pattern_cmd {
    pub filter_list_id: u8,
    pub filter_size: u8,
    pub filter_offset: u8,
    pub filter: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_del_wow_pattern_cmd {
    pub filter_list_id: __le16,
    pub filter_id: __le16,
    pub __packed: },
// WMI_SET_TXE_NOTIFY_CMDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_txe_notify_cmd {
    pub rate: __le32,
    pub pkts: __le32,
    pub intvl: __le32,
    pub __packed: },
// WMI_TXE_NOTIFY_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_txe_notify_event {
    pub rate: __le32,
    pub pkts: __le32,
    pub __packed: },
// WMI_SET_AKMP_PARAMS_CMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pmkid {
    pub pmkid: [u8; WMI_PMKID_LEN],
    pub __packed: },
// WMI_GET_PMKID_LIST_CMD  Reply
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pmkid_list_reply {
    pub num_pmkid: __le32,
    pub bssid_list: [u8; ETH_ALEN][1],
    pub pmkid_list: [wmi_pmkid; 1],
    pub __packed: },
// WMI_ADDBA_REQ_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_addba_req_event {
    pub tid: u8,
    pub win_sz: u8,
    pub st_seq_no: __le16,
// f/w response for ADDBA Req; OK (0) or failure (!=0)
    pub status: u8,
    pub __packed: },
// WMI_ADDBA_RESP_EVENTID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_addba_resp_event {
    pub tid: u8,
// OK (0), failure (!=0)
    pub status: u8,
// three values: not supported(0), 3839, 8k
    pub amsdu_sz: __le16,
    pub __packed: },
// WMI_DELBA_EVENTID
// f/w received a DELBA for peer and processed it.
// Host is notified of this
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_delba_event {
    pub tid: u8,
    pub is_peer_initiator: u8,
    pub reason_code: __le16,
    pub __packed: },
pub const PEER_NODE_JOIN_EVENT: c_uint = 0x00;
pub const PEER_NODE_LEAVE_EVENT: c_uint = 0x01;
pub const PEER_FIRST_NODE_JOIN_EVENT: c_uint = 0x10;
pub const PEER_LAST_NODE_LEAVE_EVENT: c_uint = 0x11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_peer_node_event {
    pub event_code: u8,
    pub peer_mac_addr: [u8; ETH_ALEN],
    pub __packed: },
// Transmit complete event data structure(s)
// version 1 of tx complete msg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_complete_msg_v1 {
pub const TX_COMPLETE_STATUS_SUCCESS: c_int = 0;
pub const TX_COMPLETE_STATUS_RETRIES: c_int = 1;
pub const TX_COMPLETE_STATUS_NOLINK: c_int = 2;
pub const TX_COMPLETE_STATUS_TIMEOUT: c_int = 3;
pub const TX_COMPLETE_STATUS_OTHER: c_int = 4;
    pub status: u8,
// packet ID to identify parent packet
    pub pkt_id: u8,
// rate index on successful transmission
    pub rate_idx: u8,
// number of ACK failures in tx attempt
    pub ack_failures: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tx_complete_event {
// no of tx comp msgs following this struct
    pub num_msg: u8,
// length in bytes for each individual msg following this struct
    pub msg_len: u8,
// version of tx complete msg data following this struct
    pub msg_type: u8,
// individual messages follow this header
    pub reserved: u8,
    pub __packed: },
//
// ------- AP Mode definitions --------------
//
// !!! Warning !!!
// -Changing the following values needs compilation of both driver and firmware
//
pub const AP_MAX_NUM_STA: c_int = 10;
// Spl. AID used to set DTIM flag in the beacons
pub const MCAST_AID: c_uint = 0xFF;

// Used with WMI_AP_SET_NUM_STA_CMDID
//
// Used with WMI_AP_SET_MLME_CMDID
//
// MLME Commands

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ap_set_mlme_cmd {
    pub mac: [u8; ETH_ALEN],
    pub /: *mut *mut __le16 reason; / 802.11 reason code,
    pub /: *mut *mut *mut u8 cmd; / operation to perform (WMI_AP_),
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ap_set_pvb_cmd {
    pub flag: __le32,
    pub rsvd: __le16,
    pub aid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rx_frame_format_cmd {
// version of meta data for rx packets <0 = default> (0-7 = valid)
    pub meta_ver: u8,
//
// 1 == leave .11 header intact,
// 0 == replace .11 header with .3 <default>
//
    pub dot11_hdr: u8,
//
// 1 == defragmentation is performed by host,
// 0 == performed by target <default>
//
    pub defrag_on_host: u8,
// for alignment
    pub reserved: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ap_hidden_ssid_cmd {
    pub hidden_ssid: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_inact_period_cmd {
    pub inact_period: __le32,
    pub num_null_func: u8,
    pub __packed: },
// AP mode events
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ap_set_apsd_cmd {
    pub enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_ap_apsd_buffered_traffic_flags {
    WMI_AP_APSD_NO_DELIVERY_FRAMES =  0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ap_apsd_buffered_traffic_cmd {
    pub aid: __le16,
    pub bitmap: __le16,
    pub flags: __le32,
    pub __packed: },
// WMI_PS_POLL_EVENT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_pspoll_event {
    pub aid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_per_sta_stat {
    pub tx_bytes: __le32,
    pub tx_pkts: __le32,
    pub tx_error: __le32,
    pub tx_discard: __le32,
    pub rx_bytes: __le32,
    pub rx_pkts: __le32,
    pub rx_error: __le32,
    pub rx_discard: __le32,
    pub aid: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_ap_mode_stat {
    pub action: __le32,
    pub 1]: wmi_per_sta_stat sta[AP_MAX_NUM_STA +,
    pub __packed: },
// End of AP mode definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_remain_on_chnl_cmd {
    pub freq: __le32,
    pub duration: __le32,
    pub __packed: },
// wmi_send_action_cmd is to be deprecated. Use
// wmi_send_mgmt_cmd instead. The new structure supports P2P mgmt
// operations using station interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_send_action_cmd {
    pub id: __le32,
    pub freq: __le32,
    pub wait: __le32,
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_send_mgmt_cmd {
    pub id: __le32,
    pub freq: __le32,
    pub wait: __le32,
    pub no_cck: __le32,
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_tx_status_event {
    pub id: __le32,
    pub ack_status: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_probe_req_report_cmd {
    pub enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_disable_11b_rates_cmd {
    pub disable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_set_appie_extended_cmd {
    pub role_id: u8,
    pub mgmt_frm_type: u8,
    pub ie_len: u8,
    pub ie_info: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_remain_on_chnl_event {
    pub freq: __le32,
    pub duration: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_cancel_remain_on_chnl_event {
    pub freq: __le32,
    pub duration: __le32,
    pub status: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_rx_action_event {
    pub freq: __le32,
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_capabilities_event {
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_rx_probe_req_event {
    pub freq: __le32,
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_get_p2p_info {
    pub info_req_flags: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_info_event {
    pub info_req_flags: __le32,
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_capabilities {
    pub go_power_save: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_macaddr {
    pub mac_addr: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_hmodel {
    pub p2p_model: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_p2p_probe_response_cmd {
    pub freq: __le32,
    pub destination_addr: [u8; ETH_ALEN],
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },
// Extended WMI (WMIX)
//
// Extended WMIX commands are encapsulated in a WMI message with
// cmd=WMI_EXTENSION_CMD.
//
// Extended WMI commands are those that are needed during wireless
// operation, but which are not really wireless commands.  This allows,
// for instance, platform-specific commands.  Extended WMI commands are
// embedded in a WMI command message with WMI_COMMAND_ID=WMI_EXTENSION_CMDID.
// Extended WMI events are similarly embedded in a WMI event message with
// WMI_EVENT_ID=WMI_EXTENSION_EVENTID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmix_cmd_hdr {
    pub cmd_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmix_command_id {
    WMIX_DSETOPEN_REPLY_CMDID = 0x2001,
    WMIX_DSETDATA_REPLY_CMDID,
    WMIX_GPIO_OUTPUT_SET_CMDID,
    WMIX_GPIO_INPUT_GET_CMDID,
    WMIX_GPIO_REGISTER_SET_CMDID,
    WMIX_GPIO_REGISTER_GET_CMDID,
    WMIX_GPIO_INTR_ACK_CMDID,
    WMIX_HB_CHALLENGE_RESP_CMDID,
    WMIX_DBGLOG_CFG_MODULE_CMDID,
    WMIX_PROF_CFG_CMDID,	/* 0x200a */
    WMIX_PROF_ADDR_SET_CMDID,
    WMIX_PROF_START_CMDID,
    WMIX_PROF_STOP_CMDID,
    WMIX_PROF_COUNT_GET_CMDID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmix_event_id {
    WMIX_DSETOPENREQ_EVENTID = 0x3001,
    WMIX_DSETCLOSE_EVENTID,
    WMIX_DSETDATAREQ_EVENTID,
    WMIX_GPIO_INTR_EVENTID,
    WMIX_GPIO_DATA_EVENTID,
    WMIX_GPIO_ACK_EVENTID,
    WMIX_HB_CHALLENGE_RESP_EVENTID,
    WMIX_DBGLOG_EVENTID,
    WMIX_PROF_COUNT_EVENTID,
}

//
// ------Error Detection support-------
//
// WMIX_HB_CHALLENGE_RESP_CMDID
// Heartbeat Challenge Response command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmix_hb_challenge_resp_cmd {
    pub cookie: __le32,
    pub source: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_wmix_dbglog_cfg_module_cmd {
    pub valid: __le32,
    pub config: __le32,
    pub __packed: },
// End of Extended WMI (WMIX)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wmi_sync_flag {
    NO_SYNC_WMIFLAG = 0,

// transmit all queued data before cmd
    SYNC_BEFORE_WMIFLAG,

// any new data waits until cmd execs
    SYNC_AFTER_WMIFLAG,

    SYNC_BOTH_WMIFLAG,

// end marker
    END_WMIFLAG
}

    pub wmi): *mut htc_endpoint_id ath6kl_wmi_get_control_ep(struct wmi,
    pub ep_id): *mut *mut void ath6kl_wmi_set_control_ep(struct wmi wmi, enum htc_endpoint_id,
    pub skb): *mut *mut int ath6kl_wmi_dix_2_dot3(struct wmi wmi, struct sk_buff,
    pub if_idx): *mut *mut u8 meta_ver, void tx_meta_info, u8,
    pub skb): *mut *mut int ath6kl_wmi_dot11_hdr_remove(struct wmi wmi, struct sk_buff,
    pub skb): *mut int ath6kl_wmi_dot3_2_dix(struct sk_buff,
    pub ac): *mut bool wmm_enabled, u8,
    pub skb): *mut *mut int ath6kl_wmi_control_rx(struct wmi wmi, struct sk_buff,
    pub sync_flag): wmi_cmd_id cmd_id, wmi_sync_flag,
    pub nw_subtype): u8,
    pub channel): u16,
    pub if_idx): *mut *mut int ath6kl_wmi_disconnect_cmd(struct wmi wmi, u8,
    pub rates): *mut u32,
    pub enable): *mut *mut int ath6kl_wmi_enable_sched_scan_cmd(struct wmi wmi, u8 if_idx, bool,
    pub maxact_scan_per_ssid): u16,
    pub ie_mask): u32,
    pub ssid): *mut u8 ssid_len, u8,
    pub listen_beacons): u16,
    pub num_beacons): u16 bmiss_time, u16,
    pub pwr_mode): *mut *mut int ath6kl_wmi_powermode_cmd(struct wmi wmi, u8 if_idx, u8,
    pub ps_fail_event_policy): u16,
    pub pstream): *mut wmi_create_pstream_cmd,
    pub tsid): u8,
    pub timeout): *mut *mut int ath6kl_wmi_disctimeout_cmd(struct wmi wmi, u8 if_idx, u8,
    pub threshold): *mut *mut int ath6kl_wmi_set_rts_cmd(struct wmi wmi, u16,
    pub preamble_policy): u8,
    pub source): *mut *mut int ath6kl_wmi_get_challenge_resp_cmd(struct wmi wmi, u32 cookie, u32,
    pub config): *mut *mut int ath6kl_wmi_config_debug_module_cmd(struct wmi wmi, u32 valid, u32,
    pub if_idx): *mut *mut int ath6kl_wmi_get_stats_cmd(struct wmi wmi, u8,
    pub sync_flag): wmi_sync_flag,
    pub krk): *const *const int ath6kl_wmi_add_krk_cmd(struct wmi wmi, u8 if_idx, u8,
    pub key_index): *mut *mut int ath6kl_wmi_deletekey_cmd(struct wmi wmi, u8 if_idx, u8,
    pub set): *const *const u8 pmkid, bool,
    pub dbM): *mut *mut int ath6kl_wmi_set_tx_pwr_cmd(struct wmi wmi, u8 if_idx, u8,
    pub if_idx): *mut *mut int ath6kl_wmi_get_tx_pwr_cmd(struct wmi wmi, u8,
    pub wmi): *mut int ath6kl_wmi_get_roam_tbl_cmd(struct wmi,
    pub cfg): *mut *mut int ath6kl_wmi_set_wmm_txop(struct wmi wmi, u8 if_idx, enum wmi_txop_cfg,
    pub keep_alive_intvl): u8,
    pub htcap): *mut ath6kl_htcap,
    pub len): *mut *mut *mut int ath6kl_wmi_test_cmd(struct wmi wmi, void buf, size_t,
    pub rate_index): *mut *mut s32 ath6kl_wmi_get_rate(struct wmi wmi, s8,
    pub ips1): __be32 ips0, __be32,
    pub host_mode): ath6kl_host_mode,
    pub mask): *const cfg80211_bitrate_mask,
    pub host_req_delay): u32 filter, u16,
    pub mask): *const u8,
    pub filter_id): u16 list_id, u16,
    pub rssi): *mut *mut int ath6kl_wmi_set_rssi_filter_cmd(struct wmi wmi, u8 if_idx, s8,
    pub lrssi): *mut *mut int ath6kl_wmi_set_roam_lrssi_cmd(struct wmi wmi, u8,
    pub dtim_period): *mut *mut int ath6kl_wmi_ap_set_dtim_cmd(struct wmi wmi, u8 if_idx, u32,
    pub beacon_interval): u32,
    pub bssid): *const *const int ath6kl_wmi_force_roam_cmd(struct wmi wmi, u8,
    pub mode): *mut *mut int ath6kl_wmi_set_roam_mode_cmd(struct wmi wmi, enum wmi_roam_mode,
    pub mc_all_on): *mut *mut int ath6kl_wmi_mcast_filter_cmd(struct wmi wmi, u8 if_idx, bool,
    pub add_filter): *mut *mut u8 filter, bool,
    pub enable): *mut *mut int ath6kl_wmi_sta_bmiss_enhance_cmd(struct wmi wmi, u8 if_idx, bool,
    pub intvl): u32 rate, u32 pkts, u32,
    pub alpha2): *const *const int ath6kl_wmi_set_regdomain_cmd(struct wmi wmi, char,
// AP mode uAPSD
    pub enable): *mut *mut int ath6kl_wmi_ap_set_apsd(struct wmi wmi, u8 if_idx, u8,
    pub flags): u16 bitmap, u32,
    pub user_priority): u8 ath6kl_wmi_get_traffic_class(u8,
    pub layer2_pri): *mut *mut u8 ath6kl_wmi_determine_user_priority(u8 pkt, u32,
// AP mode
    pub enable): *mut *mut int ath6kl_wmi_ap_hidden_ssid(struct wmi wmi, u8 if_idx, bool,
    pub p): *mut wmi_connect_cmd,
    pub reason): *const *const u8 mac, u16,
    pub flag): *mut *mut int ath6kl_wmi_set_pvb_cmd(struct wmi wmi, u8 if_idx, u16 aid, bool,
    pub defrag_on_host): bool rx_dot11_hdr, bool,
    pub ie_len): *const *const u8 ie, u8,
    pub ie_len): *const *const u8 ie_info, u8,
// P2P
    pub disable): *mut *mut int ath6kl_wmi_disable_11b_rates_cmd(struct wmi wmi, bool,
    pub dur): u32,
    pub no_cck): u32,
    pub data_len): u16,
    pub enable): *mut *mut int ath6kl_wmi_probe_report_req_cmd(struct wmi wmi, u8 if_idx, bool,
    pub info_req_flags): *mut *mut int ath6kl_wmi_info_req_cmd(struct wmi wmi, u8 if_idx, u32,
    pub if_idx): *mut *mut int ath6kl_wmi_cancel_remain_on_chnl_cmd(struct wmi wmi, u8,
    pub ie_len): *const *const u8 ie, u8,
    pub inact_timeout): *mut *mut int ath6kl_wmi_set_inact_period(struct wmi wmi, u8 if_idx, int,
    pub t): *mut void ath6kl_wmi_sscan_timer(struct timer_list,
    pub source): *mut *mut int ath6kl_wmi_get_challenge_resp_cmd(struct wmi wmi, u32 cookie, u32,
    pub if_idx): *mut *mut *mut ath6kl_vif ath6kl_get_vif_by_index(ath6kl ar, u8,
    pub devt): *mut *mut void ath6kl_wmi_init(struct ath6kl,
    pub wmi): *mut void ath6kl_wmi_shutdown(struct wmi,
    pub wmi): *mut void ath6kl_wmi_reset(struct wmi,
