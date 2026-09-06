//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/mwifiex/fw.h
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
// NXP Wireless LAN device driver: Firmware specific macros & structures
//
// Copyright 2011-2020 NXP
//

pub const INTF_HEADER_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfc_1042_hdr {
    pub llc_dsap: u8,
    pub llc_ssap: u8,
    pub llc_ctrl: u8,
    pub snap_oui: [u8; 3],
    pub snap_type: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_packet_hdr {
    pub eth803_hdr: ethhdr,
    pub rfc1042_hdr: rfc_1042_hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_packet_hdr {
    pub eth803_hdr: ethhdr,
    pub rfc1042_hdr: rfc_1042_hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_fw_header {
    pub dnld_cmd: __le32,
    pub base_addr: __le32,
    pub data_length: __le32,
    pub crc: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_fw_data {
    pub header: mwifiex_fw_header,
    pub seq_num: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_fw_dump_header {
    pub seq_num: __le16,
    pub reserved: __le16,
    pub type: __le16,
    pub len: __le16,
    pub __packed: },
pub const FW_DUMP_INFO_ENDED: c_uint = 0x0002;
pub const MWIFIEX_FW_DNLD_CMD_1: c_uint = 0x1;
pub const MWIFIEX_FW_DNLD_CMD_5: c_uint = 0x5;
pub const MWIFIEX_FW_DNLD_CMD_6: c_uint = 0x6;
pub const MWIFIEX_FW_DNLD_CMD_7: c_uint = 0x7;
pub const B_SUPPORTED_RATES: c_int = 5;
pub const G_SUPPORTED_RATES: c_int = 9;
pub const BG_SUPPORTED_RATES: c_int = 13;
pub const A_SUPPORTED_RATES: c_int = 9;
pub const HOSTCMD_SUPPORTED_RATES: c_int = 14;
pub const N_SUPPORTED_RATES: c_int = 3;

// bit 13: 11ac BAND_AAC
// bit 12: reserved for lab testing, will be reused for BAND_AN
// bit 11: 11n  BAND_GN
// bit 10: 11a  BAND_A
// bit 9: 11g   BAND_G
// bit 8: 11b   BAND_B
// Map these bits to band capability by right shifting 8 bits.
//

pub const HostCmd_WEP_KEY_INDEX_MASK: c_uint = 0x3fff;
pub const KEY_INFO_ENABLED: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KEY_TYPE_ID {
    KEY_TYPE_ID_WEP = 0,
    KEY_TYPE_ID_TKIP,
    KEY_TYPE_ID_AES,
    KEY_TYPE_ID_WAPI,
    KEY_TYPE_ID_AES_CMAC,
    KEY_TYPE_ID_AES_CMAC_DEF,
}

pub const WPA_PN_SIZE: c_int = 8;
pub const KEY_PARAMS_FIXED_LEN: c_int = 10;
pub const KEY_INDEX_MASK: c_uint = 0xf;
pub const KEY_API_VER_MAJOR_V2: c_int = 2;

pub const MAX_POLL_TRIES: c_int = 100;
pub const MAX_FIRMWARE_POLL_TRIES: c_int = 150;
pub const FIRMWARE_READY_SDIO: c_uint = 0xfedc;
pub const FIRMWARE_READY_PCIE: c_uint = 0xfedcba00;
pub const MWIFIEX_COEX_MODE_TIMESHARE: c_uint = 0x01;
pub const MWIFIEX_COEX_MODE_SPATIAL: c_uint = 0x82;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_usb_ep {
    MWIFIEX_USB_EP_CMD_EVENT = 1,
    MWIFIEX_USB_EP_DATA = 2,
    MWIFIEX_USB_EP_DATA_CH2 = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MWIFIEX_802_11_PRIVACY_FILTER {
    MWIFIEX_802_11_PRIV_FILTER_ACCEPT_ALL,
    MWIFIEX_802_11_PRIV_FILTER_8021X_WEP
}

pub const UAP_BSS_PARAMS_I: c_int = 0;
pub const UAP_CUSTOM_IE_I: c_int = 1;
pub const MWIFIEX_AUTO_IDX_MASK: c_uint = 0xffff;
pub const MWIFIEX_DELETE_MASK: c_uint = 0x0000;
pub const MGMT_MASK_ASSOC_REQ: c_uint = 0x01;
pub const MGMT_MASK_REASSOC_REQ: c_uint = 0x04;
pub const MGMT_MASK_ASSOC_RESP: c_uint = 0x02;
pub const MGMT_MASK_REASSOC_RESP: c_uint = 0x08;
pub const MGMT_MASK_PROBE_REQ: c_uint = 0x10;
pub const MGMT_MASK_PROBE_RESP: c_uint = 0x20;
pub const MGMT_MASK_BEACON: c_uint = 0x100;
pub const TLV_TYPE_UAP_SSID: c_uint = 0x0000;
pub const TLV_TYPE_UAP_RATES: c_uint = 0x0001;
pub const TLV_TYPE_PWR_CONSTRAINT: c_uint = 0x0020;
pub const PROPRIETARY_TLV_BASE_ID: c_uint = 0x0100;

pub const MWIFIEX_TX_DATA_BUF_SIZE_2K: c_int = 2048;
pub const SSN_MASK: c_uint = 0xfff0;
pub const BA_RESULT_SUCCESS: c_uint = 0x0;
pub const BA_RESULT_TIMEOUT: c_uint = 0x2;

pub const BA_STREAM_NOT_ALLOWED: c_uint = 0xff;

pub const MWIFIEX_TX_DATA_BUF_SIZE_4K: c_int = 4096;
pub const MWIFIEX_TX_DATA_BUF_SIZE_8K: c_int = 8192;
pub const MWIFIEX_TX_DATA_BUF_SIZE_12K: c_int = 12288;

pub const MWIFIEX_DEF_11N_TX_BF_CAP: c_uint = 0x09E1E008;

// dev_cap bitmap
// BIT
// 0-16		reserved
// 17		IEEE80211_HT_CAP_SUP_WIDTH_20_40
// 18-22	reserved
// 23		IEEE80211_HT_CAP_SGI_20
// 24		IEEE80211_HT_CAP_SGI_40
// 25		IEEE80211_HT_CAP_TX_STBC
// 26		IEEE80211_HT_CAP_RX_STBC
// 27-28	reserved
// 29		IEEE80211_HT_CAP_GRN_FLD
// 30-31	reserved
//

// httxcfg bitmap
// 0		reserved
// 1		20/40 Mhz enable(1)/disable(0)
// 2-3		reserved
// 4		green field enable(1)/disable(0)
// 5		short GI in 20 Mhz enable(1)/disable(0)
// 6		short GI in 40 Mhz enable(1)/disable(0)
// 7-15		reserved
//

// 11AC Tx and Rx MCS map for 1x1 mode:
// IEEE80211_VHT_MCS_SUPPORT_0_9 for stream 1
// IEEE80211_VHT_MCS_NOT_SUPPORTED for remaining 7 streams
//
pub const MWIFIEX_11AC_MCS_MAP_1X1: c_uint = 0xfffefffe;
// 11AC Tx and Rx MCS map for 2x2 mode:
// IEEE80211_VHT_MCS_SUPPORT_0_9 for stream 1 and 2
// IEEE80211_VHT_MCS_NOT_SUPPORTED for remaining 6 streams
//
pub const MWIFIEX_11AC_MCS_MAP_2X2: c_uint = 0xfffafffa;

pub const HT_STREAM_1X1: c_uint = 0x11;
pub const HT_STREAM_2X2: c_uint = 0x22;

pub const LLC_SNAP_LEN: c_int = 8;
// HW_SPEC fw_cap_info

// Clear SU Beanformer, MU beanformer, MU beanformee and
// sounding dimensions bits
//

pub const MOD_CLASS_HR_DSSS: c_uint = 0x03;
pub const MOD_CLASS_OFDM: c_uint = 0x07;
pub const MOD_CLASS_HT: c_uint = 0x08;
pub const HT_BW_20: c_int = 0;
pub const HT_BW_40: c_int = 1;
pub const DFS_CHAN_MOVE_TIME: c_int = 10000;
pub const HostCmd_CMD_GET_HW_SPEC: c_uint = 0x0003;
pub const HostCmd_CMD_802_11_SCAN: c_uint = 0x0006;
pub const HostCmd_CMD_802_11_GET_LOG: c_uint = 0x000b;
pub const HostCmd_CMD_MAC_MULTICAST_ADR: c_uint = 0x0010;
pub const HostCmd_CMD_802_11_EEPROM_ACCESS: c_uint = 0x0059;
pub const HostCmd_CMD_802_11_ASSOCIATE: c_uint = 0x0012;
pub const HostCmd_CMD_802_11_SNMP_MIB: c_uint = 0x0016;
pub const HostCmd_CMD_MAC_REG_ACCESS: c_uint = 0x0019;
pub const HostCmd_CMD_BBP_REG_ACCESS: c_uint = 0x001a;
pub const HostCmd_CMD_RF_REG_ACCESS: c_uint = 0x001b;
pub const HostCmd_CMD_PMIC_REG_ACCESS: c_uint = 0x00ad;
pub const HostCmd_CMD_RF_TX_PWR: c_uint = 0x001e;
pub const HostCmd_CMD_RF_ANTENNA: c_uint = 0x0020;
pub const HostCmd_CMD_802_11_DEAUTHENTICATE: c_uint = 0x0024;
pub const HostCmd_CMD_MAC_CONTROL: c_uint = 0x0028;
pub const HostCmd_CMD_802_11_AD_HOC_START: c_uint = 0x002b;
pub const HostCmd_CMD_802_11_AD_HOC_JOIN: c_uint = 0x002c;
pub const HostCmd_CMD_802_11_AD_HOC_STOP: c_uint = 0x0040;
pub const HostCmd_CMD_802_11_MAC_ADDRESS: c_uint = 0x004D;
pub const HostCmd_CMD_802_11D_DOMAIN_INFO: c_uint = 0x005b;
pub const HostCmd_CMD_802_11_KEY_MATERIAL: c_uint = 0x005e;
pub const HostCmd_CMD_802_11_BG_SCAN_CONFIG: c_uint = 0x006b;
pub const HostCmd_CMD_802_11_BG_SCAN_QUERY: c_uint = 0x006c;
pub const HostCmd_CMD_WMM_GET_STATUS: c_uint = 0x0071;
pub const HostCmd_CMD_802_11_SUBSCRIBE_EVENT: c_uint = 0x0075;
pub const HostCmd_CMD_802_11_TX_RATE_QUERY: c_uint = 0x007f;
pub const HostCmd_CMD_802_11_IBSS_COALESCING_STATUS: c_uint = 0x0083;
pub const HostCmd_CMD_MEM_ACCESS: c_uint = 0x0086;
pub const HostCmd_CMD_CFG_DATA: c_uint = 0x008f;
pub const HostCmd_CMD_VERSION_EXT: c_uint = 0x0097;
pub const HostCmd_CMD_MEF_CFG: c_uint = 0x009a;
pub const HostCmd_CMD_RSSI_INFO: c_uint = 0x00a4;
pub const HostCmd_CMD_FUNC_INIT: c_uint = 0x00a9;
pub const HostCmd_CMD_FUNC_SHUTDOWN: c_uint = 0x00aa;
pub const HOST_CMD_APCMD_SYS_RESET: c_uint = 0x00af;
pub const HostCmd_CMD_UAP_SYS_CONFIG: c_uint = 0x00b0;
pub const HostCmd_CMD_UAP_BSS_START: c_uint = 0x00b1;
pub const HostCmd_CMD_UAP_BSS_STOP: c_uint = 0x00b2;
pub const HOST_CMD_APCMD_STA_LIST: c_uint = 0x00b3;
pub const HostCmd_CMD_UAP_STA_DEAUTH: c_uint = 0x00b5;
pub const HostCmd_CMD_11N_CFG: c_uint = 0x00cd;
pub const HostCmd_CMD_11N_ADDBA_REQ: c_uint = 0x00ce;
pub const HostCmd_CMD_11N_ADDBA_RSP: c_uint = 0x00cf;
pub const HostCmd_CMD_11N_DELBA: c_uint = 0x00d0;
pub const HostCmd_CMD_RECONFIGURE_TX_BUFF: c_uint = 0x00d9;
pub const HostCmd_CMD_CHAN_REPORT_REQUEST: c_uint = 0x00dd;
pub const HostCmd_CMD_AMSDU_AGGR_CTRL: c_uint = 0x00df;
pub const HostCmd_CMD_TXPWR_CFG: c_uint = 0x00d1;
pub const HostCmd_CMD_TX_RATE_CFG: c_uint = 0x00d6;
pub const HostCmd_CMD_ROBUST_COEX: c_uint = 0x00e0;
pub const HostCmd_CMD_802_11_PS_MODE_ENH: c_uint = 0x00e4;
pub const HostCmd_CMD_802_11_HS_CFG_ENH: c_uint = 0x00e5;
pub const HostCmd_CMD_P2P_MODE_CFG: c_uint = 0x00eb;
pub const HostCmd_CMD_CAU_REG_ACCESS: c_uint = 0x00ed;
pub const HostCmd_CMD_SET_BSS_MODE: c_uint = 0x00f7;
pub const HostCmd_CMD_PCIE_DESC_DETAILS: c_uint = 0x00fa;
pub const HostCmd_CMD_802_11_SCAN_EXT: c_uint = 0x0107;
pub const HostCmd_CMD_COALESCE_CFG: c_uint = 0x010a;
pub const HostCmd_CMD_MGMT_FRAME_REG: c_uint = 0x010c;
pub const HostCmd_CMD_REMAIN_ON_CHAN: c_uint = 0x010d;
pub const HostCmd_CMD_GTK_REKEY_OFFLOAD_CFG: c_uint = 0x010f;
pub const HostCmd_CMD_11AC_CFG: c_uint = 0x0112;
pub const HostCmd_CMD_HS_WAKEUP_REASON: c_uint = 0x0116;
pub const HostCmd_CMD_TDLS_CONFIG: c_uint = 0x0100;
pub const HostCmd_CMD_MC_POLICY: c_uint = 0x0121;
pub const HostCmd_CMD_TDLS_OPER: c_uint = 0x0122;
pub const HostCmd_CMD_FW_DUMP_EVENT: c_uint = 0x0125;
pub const HostCmd_CMD_SDIO_SP_RX_AGGR_CFG: c_uint = 0x0223;
pub const HostCmd_CMD_STA_CONFIGURE: c_uint = 0x023f;
pub const HostCmd_CMD_CHAN_REGION_CFG: c_uint = 0x0242;
pub const HostCmd_CMD_PACKET_AGGR_CTRL: c_uint = 0x0251;
pub const HostCmd_CMD_ADD_NEW_STATION: c_uint = 0x025f;
pub const PROTOCOL_NO_SECURITY: c_uint = 0x01;
pub const PROTOCOL_STATIC_WEP: c_uint = 0x02;
pub const PROTOCOL_WPA: c_uint = 0x08;
pub const PROTOCOL_WPA2: c_uint = 0x20;
pub const PROTOCOL_WPA2_MIXED: c_uint = 0x28;
pub const PROTOCOL_EAP: c_uint = 0x40;
pub const KEY_MGMT_NONE: c_uint = 0x04;
pub const KEY_MGMT_PSK: c_uint = 0x02;
pub const KEY_MGMT_EAP: c_uint = 0x01;
pub const KEY_MGMT_PSK_SHA256: c_uint = 0x100;
pub const KEY_MGMT_SAE: c_uint = 0x400;
pub const CIPHER_TKIP: c_uint = 0x04;
pub const CIPHER_AES_CCMP: c_uint = 0x08;
pub const VALID_CIPHER_BITMAP: c_uint = 0x0c;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ENH_PS_MODES {
    EN_PS = 1,
    DIS_PS = 2,
    EN_AUTO_DS = 3,
    DIS_AUTO_DS = 4,
    SLEEP_CONFIRM = 5,
    GET_PS = 0,
    EN_AUTO_PS = 0xff,
    DIS_AUTO_PS = 0xfe,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum P2P_MODES {
    P2P_MODE_DISABLE = 0,
    P2P_MODE_DEVICE = 1,
    P2P_MODE_GO = 2,
    P2P_MODE_CLIENT = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_channel_flags {
    MWIFIEX_CHANNEL_PASSIVE = BIT(0),
    MWIFIEX_CHANNEL_DFS = BIT(1),
    MWIFIEX_CHANNEL_NOHT40 = BIT(2),
    MWIFIEX_CHANNEL_NOHT80 = BIT(3),
    MWIFIEX_CHANNEL_DISABLED = BIT(7),
}

pub const HostCmd_RET_BIT: c_uint = 0x8000;
pub const HostCmd_ACT_GEN_GET: c_uint = 0x0000;
pub const HostCmd_ACT_GEN_SET: c_uint = 0x0001;
pub const HOST_CMD_ACT_GEN_SET: c_uint = 0x0001;
// Add this non-CamelCase-style macro to comply with checkpatch requirements.
// This macro will eventually replace all existing CamelCase-style macros in
// the future for consistency.
//
pub const HostCmd_ACT_GEN_REMOVE: c_uint = 0x0004;
pub const HostCmd_ACT_BITWISE_SET: c_uint = 0x0002;
pub const HostCmd_ACT_BITWISE_CLR: c_uint = 0x0003;
pub const HostCmd_RESULT_OK: c_uint = 0x0000;

pub const HostCmd_BSS_MODE_IBSS: c_uint = 0x0002;
pub const HostCmd_BSS_MODE_ANY: c_uint = 0x0003;
pub const HostCmd_SCAN_RADIO_TYPE_BG: c_int = 0;
pub const HostCmd_SCAN_RADIO_TYPE_A: c_int = 1;
pub const HS_CFG_CANCEL: c_uint = 0xffffffff;
pub const HS_CFG_COND_DEF: c_uint = 0x00000000;
pub const HS_CFG_GPIO_DEF: c_uint = 0xff;
pub const HS_CFG_GAP_DEF: c_uint = 0xff;
pub const HS_CFG_COND_BROADCAST_DATA: c_uint = 0x00000001;
pub const HS_CFG_COND_UNICAST_DATA: c_uint = 0x00000002;
pub const HS_CFG_COND_MAC_EVENT: c_uint = 0x00000004;
pub const HS_CFG_COND_MULTICAST_DATA: c_uint = 0x00000008;
pub const CONNECT_ERR_AUTH_ERR_STA_FAILURE: c_uint = 0xFFFB;
pub const CONNECT_ERR_ASSOC_ERR_TIMEOUT: c_uint = 0xFFFC;
pub const CONNECT_ERR_ASSOC_ERR_AUTH_REFUSED: c_uint = 0xFFFD;
pub const CONNECT_ERR_AUTH_MSG_UNHANDLED: c_uint = 0xFFFE;
pub const CONNECT_ERR_STA_FAILURE: c_uint = 0xFFFF;

pub const HostCmd_CMD_ID_MASK: c_uint = 0x0fff;
pub const HostCmd_SEQ_NUM_MASK: c_uint = 0x00ff;
pub const HostCmd_BSS_NUM_MASK: c_uint = 0x0f00;
pub const HostCmd_BSS_TYPE_MASK: c_uint = 0xf000;
pub const HostCmd_ACT_SET_RX: c_uint = 0x0001;
pub const HostCmd_ACT_SET_TX: c_uint = 0x0002;
pub const HostCmd_ACT_SET_BOTH: c_uint = 0x0003;
pub const HostCmd_ACT_GET_RX: c_uint = 0x0004;
pub const HostCmd_ACT_GET_TX: c_uint = 0x0008;
pub const HostCmd_ACT_GET_BOTH: c_uint = 0x000c;
pub const HostCmd_ACT_REMOVE_STA: c_uint = 0x0;
pub const HostCmd_ACT_ADD_STA: c_uint = 0x1;
pub const RF_ANTENNA_AUTO: c_uint = 0xFFFF;

pub const EVENT_DUMMY_HOST_WAKEUP_SIGNAL: c_uint = 0x00000001;
pub const EVENT_LINK_LOST: c_uint = 0x00000003;
pub const EVENT_LINK_SENSED: c_uint = 0x00000004;
pub const EVENT_MIB_CHANGED: c_uint = 0x00000006;
pub const EVENT_INIT_DONE: c_uint = 0x00000007;
pub const EVENT_DEAUTHENTICATED: c_uint = 0x00000008;
pub const EVENT_DISASSOCIATED: c_uint = 0x00000009;
pub const EVENT_PS_AWAKE: c_uint = 0x0000000a;
pub const EVENT_PS_SLEEP: c_uint = 0x0000000b;
pub const EVENT_MIC_ERR_MULTICAST: c_uint = 0x0000000d;
pub const EVENT_MIC_ERR_UNICAST: c_uint = 0x0000000e;
pub const EVENT_DEEP_SLEEP_AWAKE: c_uint = 0x00000010;
pub const EVENT_ADHOC_BCN_LOST: c_uint = 0x00000011;
pub const EVENT_WMM_STATUS_CHANGE: c_uint = 0x00000017;
pub const EVENT_BG_SCAN_REPORT: c_uint = 0x00000018;
pub const EVENT_RSSI_LOW: c_uint = 0x00000019;
pub const EVENT_SNR_LOW: c_uint = 0x0000001a;
pub const EVENT_MAX_FAIL: c_uint = 0x0000001b;
pub const EVENT_RSSI_HIGH: c_uint = 0x0000001c;
pub const EVENT_SNR_HIGH: c_uint = 0x0000001d;
pub const EVENT_IBSS_COALESCED: c_uint = 0x0000001e;
pub const EVENT_IBSS_STA_CONNECT: c_uint = 0x00000020;
pub const EVENT_IBSS_STA_DISCONNECT: c_uint = 0x00000021;
pub const EVENT_DATA_RSSI_LOW: c_uint = 0x00000024;
pub const EVENT_DATA_SNR_LOW: c_uint = 0x00000025;
pub const EVENT_DATA_RSSI_HIGH: c_uint = 0x00000026;
pub const EVENT_DATA_SNR_HIGH: c_uint = 0x00000027;
pub const EVENT_LINK_QUALITY: c_uint = 0x00000028;
pub const EVENT_PORT_RELEASE: c_uint = 0x0000002b;
pub const EVENT_UAP_STA_DEAUTH: c_uint = 0x0000002c;
pub const EVENT_UAP_STA_ASSOC: c_uint = 0x0000002d;
pub const EVENT_UAP_BSS_START: c_uint = 0x0000002e;
pub const EVENT_PRE_BEACON_LOST: c_uint = 0x00000031;
pub const EVENT_ADDBA: c_uint = 0x00000033;
pub const EVENT_DELBA: c_uint = 0x00000034;
pub const EVENT_BA_STREAM_TIEMOUT: c_uint = 0x00000037;
pub const EVENT_AMSDU_AGGR_CTRL: c_uint = 0x00000042;
pub const EVENT_UAP_BSS_IDLE: c_uint = 0x00000043;
pub const EVENT_UAP_BSS_ACTIVE: c_uint = 0x00000044;
pub const EVENT_WEP_ICV_ERR: c_uint = 0x00000046;
pub const EVENT_HS_ACT_REQ: c_uint = 0x00000047;
pub const EVENT_BW_CHANGE: c_uint = 0x00000048;
pub const EVENT_UAP_MIC_COUNTERMEASURES: c_uint = 0x0000004c;
pub const EVENT_HOSTWAKE_STAIE: c_uint = 0x0000004d;
pub const EVENT_CHANNEL_SWITCH_ANN: c_uint = 0x00000050;
pub const EVENT_TDLS_GENERIC_EVENT: c_uint = 0x00000052;
pub const EVENT_RADAR_DETECTED: c_uint = 0x00000053;
pub const EVENT_CHANNEL_REPORT_RDY: c_uint = 0x00000054;
pub const EVENT_TX_DATA_PAUSE: c_uint = 0x00000055;
pub const EVENT_EXT_SCAN_REPORT: c_uint = 0x00000058;
pub const EVENT_RXBA_SYNC: c_uint = 0x00000059;
pub const EVENT_UNKNOWN_DEBUG: c_uint = 0x00000063;
pub const EVENT_BG_SCAN_STOPPED: c_uint = 0x00000065;
pub const EVENT_REMAIN_ON_CHAN_EXPIRED: c_uint = 0x0000005f;
pub const EVENT_MULTI_CHAN_INFO: c_uint = 0x0000006a;
pub const EVENT_FW_DUMP_INFO: c_uint = 0x00000073;
pub const EVENT_TX_STATUS_REPORT: c_uint = 0x00000074;

pub const EVENT_ID_MASK: c_uint = 0xffff;
pub const BSS_NUM_MASK: c_uint = 0xf;

pub const MWIFIEX_MAX_PATTERN_LEN: c_int = 40;
pub const MWIFIEX_MAX_OFFSET_LEN: c_int = 100;
pub const MWIFIEX_MAX_ND_MATCH_SETS: c_int = 10;
pub const STACK_NBYTES: c_int = 100;
pub const TYPE_DNUM: c_int = 1;
pub const TYPE_BYTESEQ: c_int = 2;
pub const MAX_OPERAND: c_uint = 0x40;

pub const MEF_MODE_HOST_SLEEP: c_int = 1;
pub const MEF_ACTION_ALLOW_AND_WAKEUP_HOST: c_int = 3;
pub const MEF_ACTION_AUTO_ARP: c_uint = 0x10;

pub const MWIFIEX_MAX_SUPPORTED_IPADDR: c_int = 4;
pub const ACT_TDLS_DELETE: c_uint = 0x00;
pub const ACT_TDLS_CREATE: c_uint = 0x01;
pub const ACT_TDLS_CONFIG: c_uint = 0x02;
pub const TDLS_EVENT_LINK_TEAR_DOWN: c_int = 3;
pub const TDLS_EVENT_CHAN_SWITCH_RESULT: c_int = 7;
pub const TDLS_EVENT_START_CHAN_SWITCH: c_int = 8;
pub const TDLS_EVENT_CHAN_SWITCH_STOPPED: c_int = 9;
pub const TDLS_BASE_CHANNEL: c_int = 0;
pub const TDLS_OFF_CHANNEL: c_int = 1;
pub const ACT_TDLS_CS_ENABLE_CONFIG: c_uint = 0x00;
pub const ACT_TDLS_CS_INIT: c_uint = 0x06;
pub const ACT_TDLS_CS_STOP: c_uint = 0x07;
pub const ACT_TDLS_CS_PARAMS: c_uint = 0x08;
pub const MWIFIEX_DEF_CS_UNIT_TIME: c_int = 2;
pub const MWIFIEX_DEF_CS_THR_OTHERLINK: c_int = 10;
pub const MWIFIEX_DEF_THR_DIRECTLINK: c_int = 0;
pub const MWIFIEX_DEF_CS_TIME: c_int = 10;
pub const MWIFIEX_DEF_CS_TIMEOUT: c_int = 16;
pub const MWIFIEX_DEF_CS_REG_CLASS: c_int = 12;
pub const MWIFIEX_DEF_CS_PERIODICITY: c_int = 1;
pub const MWIFIEX_FW_V15: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_header {
    pub type: __le16,
    pub len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_data {
    pub header: mwifiex_ie_types_header,
    pub data: [u8; ],
    pub __packed: },
pub const MWIFIEX_TxPD_POWER_MGMT_NULL_PACKET: c_uint = 0x01;
pub const MWIFIEX_TxPD_POWER_MGMT_LAST_PACKET: c_uint = 0x08;
pub const MWIFIEX_TXPD_FLAGS_TDLS_PACKET: c_uint = 0x10;
pub const MWIFIEX_RXPD_FLAGS_TDLS_PACKET: c_uint = 0x01;
pub const MWIFIEX_TXPD_FLAGS_REQ_TX_STATUS: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HS_WAKEUP_REASON {
    NO_HSWAKEUP_REASON = 0,
    BCAST_DATA_MATCHED,
    MCAST_DATA_MATCHED,
    UCAST_DATA_MATCHED,
    MASKTABLE_EVENT_MATCHED,
    NON_MASKABLE_EVENT_MATCHED,
    NON_MASKABLE_CONDITION_MATCHED,
    MAGIC_PATTERN_MATCHED,
    CONTROL_FRAME_MATCHED,
    MANAGEMENT_FRAME_MATCHED,
    GTK_REKEY_FAILURE,
    RESERVED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct txpd {
    pub bss_type: u8,
    pub bss_num: u8,
    pub tx_pkt_length: __le16,
    pub tx_pkt_offset: __le16,
    pub tx_pkt_type: __le16,
    pub tx_control: __le32,
    pub priority: u8,
    pub flags: u8,
    pub pkt_delay_2ms: u8,
    pub reserved1: [u8; 2],
    pub tx_token_id: u8,
    pub reserved: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxpd {
    pub bss_type: u8,
    pub bss_num: u8,
    pub rx_pkt_length: __le16,
    pub rx_pkt_offset: __le16,
    pub rx_pkt_type: __le16,
    pub seq_num: __le16,
    pub priority: u8,
    pub rx_rate: u8,
    pub snr: i8,
    pub nf: i8,
// For: Non-802.11 AC cards
//
// Ht Info [Bit 0] RxRate format: LG=0, HT=1
// [Bit 1]  HT Bandwidth: BW20 = 0, BW40 = 1
// [Bit 2]  HT Guard Interval: LGI = 0, SGI = 1
//
// For: 802.11 AC cards
// [Bit 1] [Bit 0] RxRate format: legacy rate = 00 HT = 01 VHT = 10
// [Bit 3] [Bit 2] HT/VHT Bandwidth BW20 = 00 BW40 = 01
// BW80 = 10  BW160 = 11
// [Bit 4] HT/VHT Guard interval LGI = 0 SGI = 1
// [Bit 5] STBC support Enabled = 1
// [Bit 6] LDPC support Enabled = 1
// [Bit 7] Reserved
//
    pub ht_info: u8,
    pub reserved: [u8; 3],
    pub flags: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uap_txpd {
    pub bss_type: u8,
    pub bss_num: u8,
    pub tx_pkt_length: __le16,
    pub tx_pkt_offset: __le16,
    pub tx_pkt_type: __le16,
    pub tx_control: __le32,
    pub priority: u8,
    pub flags: u8,
    pub pkt_delay_2ms: u8,
    pub reserved1: [u8; 2],
    pub tx_token_id: u8,
    pub reserved: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uap_rxpd {
    pub bss_type: u8,
    pub bss_num: u8,
    pub rx_pkt_length: __le16,
    pub rx_pkt_offset: __le16,
    pub rx_pkt_type: __le16,
    pub seq_num: __le16,
    pub priority: u8,
    pub rx_rate: u8,
    pub snr: i8,
    pub nf: i8,
    pub ht_info: u8,
    pub reserved: [u8; 3],
    pub flags: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_auth {
    pub auth_alg: __le16,
    pub auth_transaction: __le16,
    pub status_code: __le16,
// possibly followed by Challenge text
    pub variable: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ieee80211_mgmt {
    pub frame_control: __le16,
    pub duration: __le16,
    pub da: [u8; ETH_ALEN],
    pub sa: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub seq_ctrl: __le16,
    pub addr4: [u8; ETH_ALEN],
    pub auth: mwifiex_auth,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_fw_chan_stats {
    pub chan_num: u8,
    pub bandcfg: u8,
    pub flags: u8,
    pub noise: i8,
    pub total_bss: __le16,
    pub cca_scan_dur: __le16,
    pub cca_busy_dur: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_chan_scan_mode_bitmasks {
    MWIFIEX_PASSIVE_SCAN = BIT(0),
    MWIFIEX_DISABLE_CHAN_FILT = BIT(1),
    MWIFIEX_HIDDEN_SSID_REPORT = BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_chan_scan_param_set {
    pub radio_type: u8,
    pub chan_number: u8,
    pub chan_scan_mode_bitmap: u8,
    pub min_scan_time: __le16,
    pub max_scan_time: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_chan_list_param_set {
    pub header: mwifiex_ie_types_header,
    pub chan_scan_param: [mwifiex_chan_scan_param_set; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_rxba_sync {
    pub header: mwifiex_ie_types_header,
    pub mac: [u8; ETH_ALEN],
    pub tid: u8,
    pub reserved: u8,
    pub seq_num: __le16,
    pub bitmap_len: __le16,
    pub bitmap: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chan_band_param_set {
    pub radio_type: u8,
    pub chan_number: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_chan_band_list_param_set {
    pub header: mwifiex_ie_types_header,
    pub chan_band_param: [chan_band_param_set; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_rates_param_set {
    pub header: mwifiex_ie_types_header,
    pub rates: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_ssid_param_set {
    pub header: mwifiex_ie_types_header,
    pub ssid: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_host_mlme {
    pub header: mwifiex_ie_types_header,
    pub host_mlme: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_num_probes {
    pub header: mwifiex_ie_types_header,
    pub num_probes: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_repeat_count {
    pub header: mwifiex_ie_types_header,
    pub repeat_count: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_min_rssi_threshold {
    pub header: mwifiex_ie_types_header,
    pub rssi_threshold: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_bgscan_start_later {
    pub header: mwifiex_ie_types_header,
    pub start_later: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_scan_chan_gap {
    pub header: mwifiex_ie_types_header,
// time gap in TUs to be used between two consecutive channels scan
    pub chan_gap: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_random_mac {
    pub header: mwifiex_ie_types_header,
    pub mac: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ietypes_chanstats {
    pub header: mwifiex_ie_types_header,
    pub chanstats: [mwifiex_fw_chan_stats; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_wildcard_ssid_params {
    pub header: mwifiex_ie_types_header,
    pub max_ssid_length: u8,
    pub ssid: [u8; ],
    pub __packed: },
pub const TSF_DATA_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_tsf_timestamp {
    pub header: mwifiex_ie_types_header,
    pub tsf_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_cf_param_set {
    pub cfp_cnt: u8,
    pub cfp_period: u8,
    pub cfp_max_duration: __le16,
    pub cfp_duration_remaining: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ibss_param_set {
    pub atim_window: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_ss_param_set {
    pub header: mwifiex_ie_types_header,
    pub cf_param_set): DECLARE_FLEX_ARRAY(struct mwifiex_cf_param_set,,
    pub ibss_param_set): DECLARE_FLEX_ARRAY(struct mwifiex_ibss_param_set,,
    pub cf_ibss: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_fh_param_set {
    pub dwell_time: __le16,
    pub hop_set: u8,
    pub hop_pattern: u8,
    pub hop_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ds_param_set {
    pub current_chan: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_phy_param_set {
    pub header: mwifiex_ie_types_header,
    pub fh_param_set): DECLARE_FLEX_ARRAY(struct mwifiex_fh_param_set,,
    pub ds_param_set): DECLARE_FLEX_ARRAY(struct mwifiex_ds_param_set,,
    pub fh_ds: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_auth_type {
    pub header: mwifiex_ie_types_header,
    pub auth_type: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_vendor_param_set {
    pub header: mwifiex_ie_types_header,
    pub ie: [u8; MWIFIEX_MAX_VSIE_LEN],
}

pub const MWIFIEX_TDLS_IDLE_TIMEOUT_IN_SEC: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_tdls_idle_timeout {
    pub header: mwifiex_ie_types_header,
    pub value: __le16,
    pub __packed: },
pub const MWIFIEX_AUTHTYPE_SAE: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_sae_pwe_mode {
    pub header: mwifiex_ie_types_header,
    pub pwe: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_rsn_param_set {
    pub header: mwifiex_ie_types_header,
    pub rsn_ie: [u8; ],
    pub __packed: },
pub const KEYPARAMSET_FIXED_LEN: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_type_key_param_set {
    pub type: __le16,
    pub length: __le16,
    pub key_type_id: __le16,
    pub key_info: __le16,
    pub key_len: __le16,
    pub key: [u8; 50],
    pub __packed: },
pub const IGTK_PN_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_cmac_param {
    pub ipn: [u8; IGTK_PN_LEN],
    pub key: [u8; WLAN_KEY_LEN_AES_CMAC],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_wep_param {
    pub key_len: __le16,
    pub key: [u8; WLAN_KEY_LEN_WEP104],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tkip_param {
    pub pn: [u8; WPA_PN_SIZE],
    pub key_len: __le16,
    pub key: [u8; WLAN_KEY_LEN_TKIP],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_aes_param {
    pub pn: [u8; WPA_PN_SIZE],
    pub key_len: __le16,
    pub key: [u8; WLAN_KEY_LEN_CCMP_256],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_wapi_param {
    pub pn: [u8; PN_LEN],
    pub key_len: __le16,
    pub key: [u8; WLAN_KEY_LEN_SMS4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_cmac_aes_param {
    pub ipn: [u8; IGTK_PN_LEN],
    pub key_len: __le16,
    pub key: [u8; WLAN_KEY_LEN_AES_CMAC],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_type_key_param_set_v2 {
    pub type: __le16,
    pub len: __le16,
    pub mac_addr: [u8; ETH_ALEN],
    pub key_idx: u8,
    pub key_type: u8,
    pub key_info: __le16,
    pub wep: mwifiex_wep_param,
    pub tkip: mwifiex_tkip_param,
    pub aes: mwifiex_aes_param,
    pub wapi: mwifiex_wapi_param,
    pub cmac_aes: mwifiex_cmac_aes_param,
    pub key_params: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_key_material_v2 {
    pub action: __le16,
    pub key_param_set: mwifiex_ie_type_key_param_set_v2,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_key_material {
    pub action: __le16,
    pub key_param_set: mwifiex_ie_type_key_param_set,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_key_material_wep {
    pub action: __le16,
    pub key_param_set: [mwifiex_ie_type_key_param_set; NUM_WEP_KEYS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_gen {
    pub command: __le16,
    pub size: __le16,
    pub seq_num: __le16,
    pub result: __le16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sleep_resp_ctrl {
    RESP_NOT_NEEDED = 0,
    RESP_NEEDED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ps_param {
    pub null_pkt_interval: __le16,
    pub multiple_dtims: __le16,
    pub bcn_miss_timeout: __le16,
    pub local_listen_interval: __le16,
    pub adhoc_wake_period: __le16,
    pub mode: __le16,
    pub delay_to_ps: __le16,
    pub __packed: },
pub const HS_DEF_WAKE_INTERVAL: c_int = 100;
pub const HS_DEF_INACTIVITY_TIMEOUT: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ps_param_in_hs {
    pub header: mwifiex_ie_types_header,
    pub hs_wake_int: __le32,
    pub hs_inact_timeout: __le32,
    pub __packed: },
pub const BITMAP_AUTO_DS: c_uint = 0x01;
pub const BITMAP_STA_PS: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_auto_ds_param {
    pub header: mwifiex_ie_types_header,
    pub deep_sleep_timeout: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_ps_param {
    pub header: mwifiex_ie_types_header,
    pub param: mwifiex_ps_param,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_ps_mode_enh {
    pub action: __le16,
    pub opt_ps: mwifiex_ps_param,
    pub ps_bitmap: __le16,
    pub params: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum API_VER_ID {
    KEY_API_VER_ID = 1,
    FW_API_VER_ID = 2,
    UAP_FW_API_VER_ID = 3,
    CHANRPT_API_VER_ID = 4,
    FW_HOTFIX_VER_ID = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_spec_api_rev {
    pub header: mwifiex_ie_types_header,
    pub api_id: __le16,
    pub major_ver: u8,
    pub minor_ver: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_get_hw_spec {
    pub hw_if_version: __le16,
    pub version: __le16,
    pub reserved: __le16,
    pub num_of_mcast_adr: __le16,
    pub permanent_addr: [u8; ETH_ALEN],
    pub region_code: __le16,
    pub number_of_antenna: __le16,
    pub fw_release_number: __le32,
    pub reserved_1: __le32,
    pub reserved_2: __le32,
    pub reserved_3: __le32,
    pub fw_cap_info: __le32,
    pub dot_11n_dev_cap: __le32,
    pub dev_mcs_support: u8,
    pub /: *mut *mut __le16 mp_end_port; / SDIO only, reserved for other interfaces,
    pub /: *mut *mut __le16 mgmt_buf_count; / mgmt IE buffer count,
    pub reserved_5: __le32,
    pub reserved_6: __le32,
    pub dot_11ac_dev_cap: __le32,
    pub dot_11ac_mcs_support: __le32,
    pub tlvs: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_rssi_info {
    pub action: __le16,
    pub ndata: __le16,
    pub nbcn: __le16,
    pub reserved: [__le16; 9],
    pub reserved_1: c_longlong,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_rssi_info_rsp {
    pub action: __le16,
    pub ndata: __le16,
    pub nbcn: __le16,
    pub data_rssi_last: __le16,
    pub data_nf_last: __le16,
    pub data_rssi_avg: __le16,
    pub data_nf_avg: __le16,
    pub bcn_rssi_last: __le16,
    pub bcn_nf_last: __le16,
    pub bcn_rssi_avg: __le16,
    pub bcn_nf_avg: __le16,
    pub tsf_bcn: c_longlong,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_mac_address {
    pub action: __le16,
    pub mac_addr: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_mac_control {
    pub action: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_mac_multicast_adr {
    pub action: __le16,
    pub num_of_adrs: __le16,
    pub mac_list: [u8; MWIFIEX_MAX_MULTICAST_LIST_SIZE][ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_deauthenticate {
    pub mac_addr: [u8; ETH_ALEN],
    pub reason_code: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_associate {
    pub peer_sta_addr: [u8; ETH_ALEN],
    pub cap_info_bitmap: __le16,
    pub listen_interval: __le16,
    pub beacon_period: __le16,
    pub dtim_period: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_assoc_rsp {
    pub cap_info_bitmap: __le16,
    pub status_code: __le16,
    pub a_id: __le16,
    pub ie_buffer: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_associate_rsp {
    pub assoc_rsp: ieee_types_assoc_rsp,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_cf_param_set {
    pub element_id: u8,
    pub len: u8,
    pub cfp_cnt: u8,
    pub cfp_period: u8,
    pub cfp_max_duration: __le16,
    pub cfp_duration_remaining: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_ibss_param_set {
    pub element_id: u8,
    pub len: u8,
    pub atim_window: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ieee_types_ss_param_set {
    pub cf_param_set: ieee_types_cf_param_set,
    pub ibss_param_set: ieee_types_ibss_param_set,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_fh_param_set {
    pub element_id: u8,
    pub len: u8,
    pub dwell_time: __le16,
    pub hop_set: u8,
    pub hop_pattern: u8,
    pub hop_index: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_ds_param_set {
    pub element_id: u8,
    pub len: u8,
    pub current_chan: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ieee_types_phy_param_set {
    pub fh_param_set: ieee_types_fh_param_set,
    pub ds_param_set: ieee_types_ds_param_set,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_oper_mode_ntf {
    pub element_id: u8,
    pub len: u8,
    pub oper_mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_ad_hoc_start {
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub bss_mode: u8,
    pub beacon_period: __le16,
    pub dtim_period: u8,
    pub ss_param_set: ieee_types_ss_param_set,
    pub phy_param_set: ieee_types_phy_param_set,
    pub reserved1: u16,
    pub cap_info_bitmap: __le16,
    pub data_rate: [u8; HOSTCMD_SUPPORTED_RATES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_ad_hoc_start_result {
    pub pad: [u8; 3],
    pub bssid: [u8; ETH_ALEN],
    pub pad2: [u8; 2],
    pub result: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_ad_hoc_join_result {
    pub result: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adhoc_bss_desc {
    pub bssid: [u8; ETH_ALEN],
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub bss_mode: u8,
    pub beacon_period: __le16,
    pub dtim_period: u8,
    pub time_stamp: [u8; 8],
    pub local_time: [u8; 8],
    pub phy_param_set: ieee_types_phy_param_set,
    pub ss_param_set: ieee_types_ss_param_set,
    pub cap_info_bitmap: __le16,
    pub data_rates: [u8; HOSTCMD_SUPPORTED_RATES],
//
// DO NOT ADD ANY FIELDS TO THIS STRUCTURE.
// It is used in the Adhoc join command and will cause a
// binary layout mismatch with the firmware
//
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_ad_hoc_join {
    pub bss_descriptor: adhoc_bss_desc,
    pub reserved1: u16,
    pub reserved2: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_get_log {
    pub mcast_tx_frame: __le32,
    pub failed: __le32,
    pub retry: __le32,
    pub multi_retry: __le32,
    pub frame_dup: __le32,
    pub rts_success: __le32,
    pub rts_failure: __le32,
    pub ack_failure: __le32,
    pub rx_frag: __le32,
    pub mcast_rx_frame: __le32,
    pub fcs_error: __le32,
    pub tx_frame: __le32,
    pub reserved: __le32,
    pub wep_icv_err_cnt: [__le32; 4],
    pub bcn_rcv_cnt: __le32,
    pub bcn_miss_cnt: __le32,
    pub __packed: },
// Enumeration for rate format
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _mwifiex_rate_format {
    MWIFIEX_RATE_FORMAT_LG = 0,
    MWIFIEX_RATE_FORMAT_HT,
    MWIFIEX_RATE_FORMAT_VHT,
    MWIFIEX_RATE_FORMAT_AUTO = 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_tx_rate_query {
    pub tx_rate: u8,
// Tx Rate Info: For 802.11 AC cards
//
// [Bit 0-1] tx rate formate: LG = 0, HT = 1, VHT = 2
// [Bit 2-3] HT/VHT Bandwidth: BW20 = 0, BW40 = 1, BW80 = 2, BW160 = 3
// [Bit 4]   HT/VHT Guard Interval: LGI = 0, SGI = 1
//
// For non-802.11 AC cards
// Ht Info [Bit 0] RxRate format: LG=0, HT=1
// [Bit 1]  HT Bandwidth: BW20 = 0, BW40 = 1
// [Bit 2]  HT Guard Interval: LGI = 0, SGI = 1
//
    pub ht_info: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tx_pause_tlv {
    pub header: mwifiex_ie_types_header,
    pub peermac: [u8; ETH_ALEN],
    pub tx_pause: u8,
    pub pkt_cnt: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Host_Sleep_Action {
    HS_CONFIGURE = 0x0001,
    HS_ACTIVATE  = 0x0002,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_hs_config_param {
    pub conditions: __le32,
    pub gpio: u8,
    pub gap: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hs_activate_param {
    pub resp_ctrl: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_hs_cfg_enh {
    pub action: __le16,
    pub hs_config: mwifiex_hs_config_param,
    pub hs_activate: hs_activate_param,
    pub params: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SNMP_MIB_INDEX {
    OP_RATE_SET_I = 1,
    DTIM_PERIOD_I = 3,
    RTS_THRESH_I = 5,
    SHORT_RETRY_LIM_I = 6,
    LONG_RETRY_LIM_I = 7,
    FRAG_THRESH_I = 8,
    DOT11D_I = 9,
    DOT11H_I = 10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mwifiex_assocmd_failurepoint {
    MWIFIEX_ASSOC_CMD_SUCCESS = 0,
    MWIFIEX_ASSOC_CMD_FAILURE_ASSOC,
    MWIFIEX_ASSOC_CMD_FAILURE_AUTH,
    MWIFIEX_ASSOC_CMD_FAILURE_JOIN
}

pub const MAX_SNMP_BUF_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_snmp_mib {
    pub query_type: __le16,
    pub oid: __le16,
    pub buf_size: __le16,
    pub value: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_rate_scope {
    pub type: __le16,
    pub length: __le16,
    pub hr_dsss_rate_bitmap: __le16,
    pub ofdm_rate_bitmap: __le16,
    pub ht_mcs_rate_bitmap: [__le16; 8],
    pub vht_mcs_rate_bitmap: [__le16; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_rate_drop_pattern {
    pub type: __le16,
    pub length: __le16,
    pub rate_drop_mode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_tx_rate_cfg {
    pub action: __le16,
    pub cfg_index: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_power_group {
    pub modulation_class: u8,
    pub first_rate_code: u8,
    pub last_rate_code: u8,
    pub power_step: i8,
    pub power_min: i8,
    pub power_max: i8,
    pub ht_bandwidth: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_types_power_group {
    pub type: __le16,
    pub length: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_txpwr_cfg {
    pub action: __le16,
    pub cfg_index: __le16,
    pub mode: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_rf_tx_pwr {
    pub action: __le16,
    pub cur_level: __le16,
    pub max_power: u8,
    pub min_power: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_rf_ant_mimo {
    pub action_tx: __le16,
    pub tx_ant_mode: __le16,
    pub action_rx: __le16,
    pub rx_ant_mode: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_rf_ant_siso {
    pub action: __le16,
    pub ant_mode: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_tdls_oper {
    pub tdls_action: __le16,
    pub reason: __le16,
    pub peer_mac: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tdls_config {
    pub enable: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tdls_config_cs_params {
    pub unit_time: u8,
    pub thr_otherlink: u8,
    pub thr_directlink: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tdls_init_cs_params {
    pub peer_mac: [u8; ETH_ALEN],
    pub primary_chan: u8,
    pub second_chan_offset: u8,
    pub band: u8,
    pub switch_time: __le16,
    pub switch_timeout: __le16,
    pub reg_class: u8,
    pub periodicity: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tdls_stop_cs_params {
    pub peer_mac: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_tdls_config {
    pub tdls_action: __le16,
    pub tdls_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_chan_desc {
    pub start_freq: __le16,
    pub chan_width: u8,
    pub chan_num: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_chan_rpt_req {
    pub chan_desc: mwifiex_chan_desc,
    pub msec_dwell_time: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_chan_rpt_event {
    pub result: __le32,
    pub start_tsf: __le64,
    pub duration: __le32,
    pub tlvbuf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_sdio_sp_rx_aggr_cfg {
    pub action: u8,
    pub enable: u8,
    pub block_size: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_fixed_bcn_param {
    pub timestamp: __le64,
    pub beacon_period: __le16,
    pub cap_info_bitmap: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_event_scan_result {
    pub event_id: __le16,
    pub bss_index: u8,
    pub bss_type: u8,
    pub more_event: u8,
    pub reserved: [u8; 3],
    pub buf_size: __le16,
    pub num_of_set: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_status_event {
    pub packet_type: u8,
    pub tx_token_id: u8,
    pub status: u8,
    pub __packed: },
pub const MWIFIEX_USER_SCAN_CHAN_MAX: c_int = 50;
pub const MWIFIEX_MAX_SSID_LIST_LENGTH: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_scan_cmd_config {
//
// BSS mode to be sent in the firmware command
//
    pub bss_mode: u8,
// Specific BSSID used to filter scan results in the firmware
    pub specific_bssid: [u8; ETH_ALEN],
// Length of TLVs sent in command starting at tlvBuffer
    pub tlv_buf_len: u32,
//
// SSID TLV(s) and ChanList TLVs to be sent in the firmware command
//
// TLV_TYPE_CHANLIST, mwifiex_ie_types_chan_list_param_set
// WLAN_EID_SSID, mwifiex_ie_types_ssid_param_set
//
    pub stored: *mut *mut u8 tlv_buf[]; / SSID TLV(s) and ChanList TLVs are,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_user_scan_chan {
    pub chan_number: u8,
    pub radio_type: u8,
    pub scan_type: u8,
    pub reserved: u8,
    pub scan_time: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_user_scan_cfg {
//
// BSS mode to be sent in the firmware command
//
    pub bss_mode: u8,
// Configure the number of probe requests for active chan scans
    pub num_probes: u8,
    pub reserved: u8,
// BSSID filter sent in the firmware command to limit the results
    pub specific_bssid: [u8; ETH_ALEN],
// SSID filter list used in the firmware to limit the scan results
    pub ssid_list: *mut cfg80211_ssid,
    pub num_ssids: u8,
// Variable number (fixed maximum) of channels to scan up
    pub chan_list: [mwifiex_user_scan_chan; MWIFIEX_USER_SCAN_CHAN_MAX],
    pub scan_chan_gap: u16,
    pub random_mac: [u8; ETH_ALEN],
    pub __packed: },
pub const MWIFIEX_BG_SCAN_CHAN_MAX: c_int = 38;
pub const MWIFIEX_BSS_MODE_INFRA: c_int = 1;
pub const MWIFIEX_BGSCAN_ACT_GET: c_uint = 0x0000;
pub const MWIFIEX_BGSCAN_ACT_SET: c_uint = 0x0001;
pub const MWIFIEX_BGSCAN_ACT_SET_ALL: c_uint = 0xff01;
// ssid match
pub const MWIFIEX_BGSCAN_SSID_MATCH: c_uint = 0x0001;
// ssid match and RSSI exceeded
pub const MWIFIEX_BGSCAN_SSID_RSSI_MATCH: c_uint = 0x0004;
// wait for all channel scan to complete to report scan result
pub const MWIFIEX_BGSCAN_WAIT_ALL_CHAN_DONE: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_bg_scan_cfg {
    pub action: u16,
    pub enable: u8,
    pub bss_type: u8,
    pub chan_per_scan: u8,
    pub scan_interval: u32,
    pub report_condition: u32,
    pub num_probes: u8,
    pub rssi_threshold: u8,
    pub snr_threshold: u8,
    pub repeat_count: u16,
    pub start_later: u16,
    pub ssid_list: *mut cfg80211_match_set,
    pub num_ssids: u8,
    pub chan_list: [mwifiex_user_scan_chan; MWIFIEX_BG_SCAN_CHAN_MAX],
    pub scan_chan_gap: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ie_body {
    pub grp_key_oui: [u8; 4],
    pub ptk_cnt: [u8; 2],
    pub ptk_body: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_scan {
    pub bss_mode: u8,
    pub bssid: [u8; ETH_ALEN],
    pub tlv_buffer: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_scan_rsp {
    pub bss_descript_size: __le16,
    pub number_of_sets: u8,
    pub bss_desc_and_tlv_buffer: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_scan_ext {
    pub reserved: u32,
    pub tlv_buffer: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_bss_mode {
    pub header: mwifiex_ie_types_header,
    pub bss_mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_bss_scan_rsp {
    pub header: mwifiex_ie_types_header,
    pub bssid: [u8; ETH_ALEN],
    pub frame_body: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_bss_scan_info {
    pub header: mwifiex_ie_types_header,
    pub rssi: __le16,
    pub anpi: __le16,
    pub cca_busy_fraction: u8,
    pub radio_type: u8,
    pub channel: u8,
    pub reserved: u8,
    pub tsf: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_bg_scan_config {
    pub action: __le16,
    pub enable: u8,
    pub bss_type: u8,
    pub chan_per_scan: u8,
    pub reserved: u8,
    pub reserved1: __le16,
    pub scan_interval: __le32,
    pub reserved2: __le32,
    pub report_condition: __le32,
    pub reserved3: __le16,
    pub tlv: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_bg_scan_query {
    pub flush: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_bg_scan_query_rsp {
    pub report_condition: __le32,
    pub scan_resp: host_cmd_ds_802_11_scan_rsp,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ietypes_domain_param_set {
    pub header: mwifiex_ie_types_header,
    pub country_code: [u8; IEEE80211_COUNTRY_STRING_LEN],
    pub triplet: [ieee80211_country_ie_triplet; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11d_domain_info {
    pub action: __le16,
    pub domain: mwifiex_ietypes_domain_param_set,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11d_domain_info_rsp {
    pub action: __le16,
    pub domain: mwifiex_ietypes_domain_param_set,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_11n_addba_req {
    pub add_req_result: u8,
    pub peer_mac_addr: [u8; ETH_ALEN],
    pub dialog_token: u8,
    pub block_ack_param_set: __le16,
    pub block_ack_tmo: __le16,
    pub ssn: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_11n_addba_rsp {
    pub add_rsp_result: u8,
    pub peer_mac_addr: [u8; ETH_ALEN],
    pub dialog_token: u8,
    pub status_code: __le16,
    pub block_ack_param_set: __le16,
    pub block_ack_tmo: __le16,
    pub ssn: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_11n_delba {
    pub del_result: u8,
    pub peer_mac_addr: [u8; ETH_ALEN],
    pub del_ba_param_set: __le16,
    pub reason_code: __le16,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_11n_batimeout {
    pub tid: u8,
    pub peer_mac_addr: [u8; ETH_ALEN],
    pub origninator: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_11n_cfg {
    pub action: __le16,
    pub ht_tx_cap: __le16,
    pub ht_tx_info: __le16,
    pub /: *mut *mut __le16 misc_config; / Needed for 802.11AC cards only,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_txbuf_cfg {
    pub action: __le16,
    pub buff_size: __le16,
    pub /: *mut *mut __le16 mp_end_port; / SDIO only, reserved for other interfaces,
    pub reserved3: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_amsdu_aggr_ctrl {
    pub action: __le16,
    pub enable: __le16,
    pub curr_buf_size: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_sta_deauth {
    pub mac: [u8; ETH_ALEN],
    pub reason: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_sta_info {
    pub header: mwifiex_ie_types_header,
    pub mac: [u8; ETH_ALEN],
    pub power_mfg_status: u8,
    pub rssi: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_sta_list {
    pub sta_count: __le16,
    pub tlv: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_pwr_capability {
    pub header: mwifiex_ie_types_header,
    pub min_pwr: i8,
    pub max_pwr: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_local_pwr_constraint {
    pub header: mwifiex_ie_types_header,
    pub chan: u8,
    pub constraint: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_wmm_param_set {
    pub header: mwifiex_ie_types_header,
    pub wmm_ie: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_mgmt_frame {
    pub header: mwifiex_ie_types_header,
    pub frame_control: __le16,
    pub frame_contents: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_wmm_queue_status {
    pub header: mwifiex_ie_types_header,
    pub queue_index: u8,
    pub disabled: u8,
    pub medium_time: __le16,
    pub flow_required: u8,
    pub flow_created: u8,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_vendor_header {
    pub element_id: u8,
    pub len: u8,
    pub oui: [u8; 3],
    pub oui_type: u8,
    pub oui: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_wmm_parameter {
//
// WMM Parameter IE - Vendor Specific Header:
// element_id  [221/0xdd]
// Len         [24]
// Oui         [00:50:f2]
// OuiType     [2]
// OuiSubType  [1]
// Version     [1]
//
    pub vend_hdr: ieee_types_vendor_header,
    pub oui_subtype: u8,
    pub version: u8,
    pub qos_info_bitmap: u8,
    pub reserved: u8,
    pub ac_params: [ieee_types_wmm_ac_parameters; IEEE80211_NUM_ACS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee_types_wmm_info {
//
// WMM Info IE - Vendor Specific Header:
// element_id  [221/0xdd]
// Len         [7]
// Oui         [00:50:f2]
// OuiType     [2]
// OuiSubType  [0]
// Version     [1]
//
    pub vend_hdr: ieee_types_vendor_header,
    pub oui_subtype: u8,
    pub version: u8,
    pub qos_info_bitmap: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_wmm_get_status {
    pub 2]: u8 wmm_param_tlv[sizeof(struct ieee_types_wmm_parameter) +,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_wmm_ac_status {
    pub disabled: u8,
    pub flow_required: u8,
    pub flow_created: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_htcap {
    pub header: mwifiex_ie_types_header,
    pub ht_cap: ieee80211_ht_cap,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_vhtcap {
    pub header: mwifiex_ie_types_header,
    pub vht_cap: ieee80211_vht_cap,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_aid {
    pub header: mwifiex_ie_types_header,
    pub aid: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_oper_mode_ntf {
    pub header: mwifiex_ie_types_header,
    pub oper_mode: u8,
    pub __packed: },
// VHT Operations IE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_vht_oper {
    pub header: mwifiex_ie_types_header,
    pub chan_width: u8,
    pub chan_center_freq_1: u8,
    pub chan_center_freq_2: u8,
// Basic MCS set map, each 2 bits stands for a NSS
    pub basic_mcs_map: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_wmmcap {
    pub header: mwifiex_ie_types_header,
    pub wmm_info: mwifiex_types_wmm_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_htinfo {
    pub header: mwifiex_ie_types_header,
    pub ht_oper: ieee80211_ht_operation,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_2040bssco {
    pub header: mwifiex_ie_types_header,
    pub bss_co_2040: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_extcap {
    pub header: mwifiex_ie_types_header,
    pub ext_capab: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_mem_access {
    pub action: __le16,
    pub reserved: __le16,
    pub addr: __le32,
    pub value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_qos_info {
    pub header: mwifiex_ie_types_header,
    pub qos_info: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_mac_reg_access {
    pub action: __le16,
    pub offset: __le16,
    pub value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_bbp_reg_access {
    pub action: __le16,
    pub offset: __le16,
    pub value: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_rf_reg_access {
    pub action: __le16,
    pub offset: __le16,
    pub value: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_pmic_reg_access {
    pub action: __le16,
    pub offset: __le16,
    pub value: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_eeprom_access {
    pub action: __le16,
    pub offset: __le16,
    pub byte_count: __le16,
    pub value: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_assoc_event {
    pub sta_addr: [u8; ETH_ALEN],
    pub type: __le16,
    pub len: __le16,
    pub frame_control: __le16,
    pub cap_info: __le16,
    pub listen_interval: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_sys_config {
    pub action: __le16,
    pub tlv: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_11ac_vht_cfg {
    pub action: __le16,
    pub band_config: u8,
    pub misc_config: u8,
    pub cap_info: __le32,
    pub mcs_tx_set: __le32,
    pub mcs_rx_set: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_akmp {
    pub header: mwifiex_ie_types_header,
    pub key_mgmt: __le16,
    pub key_mgmt_operation: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_pwk_cipher {
    pub header: mwifiex_ie_types_header,
    pub proto: __le16,
    pub cipher: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_gwk_cipher {
    pub header: mwifiex_ie_types_header,
    pub cipher: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_passphrase {
    pub header: mwifiex_ie_types_header,
    pub passphrase: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_wep_key {
    pub header: mwifiex_ie_types_header,
    pub key_index: u8,
    pub is_default: u8,
    pub key: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_auth_type {
    pub header: mwifiex_ie_types_header,
    pub auth_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_encrypt_protocol {
    pub header: mwifiex_ie_types_header,
    pub proto: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_ssid {
    pub header: mwifiex_ie_types_header,
    pub ssid: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_rates {
    pub header: mwifiex_ie_types_header,
    pub rates: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_bssid_list {
    pub header: mwifiex_ie_types_header,
    pub bssid: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_bcast_ssid {
    pub header: mwifiex_ie_types_header,
    pub bcast_ctl: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_beacon_period {
    pub header: mwifiex_ie_types_header,
    pub period: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_dtim_period {
    pub header: mwifiex_ie_types_header,
    pub period: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_frag_threshold {
    pub header: mwifiex_ie_types_header,
    pub frag_thr: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_rts_threshold {
    pub header: mwifiex_ie_types_header,
    pub rts_thr: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_retry_limit {
    pub header: mwifiex_ie_types_header,
    pub limit: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_mac_addr {
    pub header: mwifiex_ie_types_header,
    pub mac_addr: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_channel_band {
    pub header: mwifiex_ie_types_header,
    pub band_config: u8,
    pub channel: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_ageout_timer {
    pub header: mwifiex_ie_types_header,
    pub sta_ao_timer: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_tlv_power_constraint {
    pub header: mwifiex_ie_types_header,
    pub constraint: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_btcoex_scan_time {
    pub header: mwifiex_ie_types_header,
    pub coex_scan: u8,
    pub reserved: u8,
    pub min_scan_time: __le16,
    pub max_scan_time: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_btcoex_aggr_win_size {
    pub header: mwifiex_ie_types_header,
    pub coex_win_size: u8,
    pub tx_win_size: u8,
    pub rx_win_size: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_robust_coex {
    pub header: mwifiex_ie_types_header,
    pub mode: __le32,
    pub __packed: },
pub const MWIFIEX_VERSION_STR_LENGTH: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_version_ext {
    pub version_str_sel: u8,
    pub version_str: [c_char; MWIFIEX_VERSION_STR_LENGTH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_mgmt_frame_reg {
    pub action: __le16,
    pub mask: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_p2p_mode_cfg {
    pub action: __le16,
    pub mode: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_remain_on_chan {
    pub action: __le16,
    pub status: u8,
    pub reserved: u8,
    pub band_cfg: u8,
    pub channel: u8,
    pub duration: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_ibss_status {
    pub action: __le16,
    pub enable: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub beacon_interval: __le16,
    pub atim_window: __le16,
    pub use_g_rate_protect: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_fw_mef_entry {
    pub mode: u8,
    pub action: u8,
    pub exprsize: __le16,
    pub expr: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_mef_cfg {
    pub criteria: __le32,
    pub num_entries: __le16,
    pub mef_entry_data: [u8; ],
    pub __packed: },
pub const CONNECTION_TYPE_INFRA: c_int = 0;
pub const CONNECTION_TYPE_ADHOC: c_int = 1;
pub const CONNECTION_TYPE_AP: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_set_bss_mode {
    pub con_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_pcie_details {
// TX buffer descriptor ring address
    pub txbd_addr_lo: __le32,
    pub txbd_addr_hi: __le32,
// TX buffer descriptor ring count
    pub txbd_count: __le32,
// RX buffer descriptor ring address
    pub rxbd_addr_lo: __le32,
    pub rxbd_addr_hi: __le32,
// RX buffer descriptor ring count
    pub rxbd_count: __le32,
// Event buffer descriptor ring address
    pub evtbd_addr_lo: __le32,
    pub evtbd_addr_hi: __le32,
// Event buffer descriptor ring count
    pub evtbd_count: __le32,
// Sleep cookie buffer physical address
    pub sleep_cookie_addr_lo: __le32,
    pub sleep_cookie_addr_hi: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_rssi_threshold {
    pub header: mwifiex_ie_types_header,
    pub abs_value: u8,
    pub evt_freq: u8,
    pub __packed: },
pub const MWIFIEX_DFS_REC_HDR_LEN: c_int = 8;
pub const MWIFIEX_DFS_REC_HDR_NUM: c_int = 10;
pub const MWIFIEX_BIN_COUNTER_LEN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_radar_det_event {
    pub detect_count: __le32,
    pub 3=mic*/: *mut *mut u8 reg_domain; /1=fcc, 2=etsi,,
    pub 2=pri(radar)*/: *mut *mut u8 det_type; /0=none, 1=pw(chirp),,
    pub pw_chirp_type: __le16,
    pub pw_chirp_idx: u8,
    pub pw_value: u8,
    pub pri_radar_type: u8,
    pub pri_bincnt: u8,
    pub bin_counter: [u8; MWIFIEX_BIN_COUNTER_LEN],
    pub num_dfs_records: u8,
    pub dfs_record_hdr: [u8; MWIFIEX_DFS_REC_HDR_NUM][MWIFIEX_DFS_REC_HDR_LEN],
    pub passed: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_multi_chan_info {
    pub header: mwifiex_ie_types_header,
    pub status: __le16,
    pub tlv_buffer: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_mc_group_info {
    pub header: mwifiex_ie_types_header,
    pub chan_group_id: u8,
    pub chan_buf_weight: u8,
    pub band_config: u8,
    pub chan_num: u8,
    pub chan_time: __le32,
    pub reserved: __le32,
    pub sdio_func_num: u8,
    pub usb_ep_num: u8,
    pub hid_num: },
    pub intf_num: u8,
    pub bss_type_numlist: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meas_rpt_map {
    pub rssi:3: u8,
    pub unmeasured:1: u8,
    pub radar:1: u8,
    pub unidentified_sig:1: u8,
    pub ofdm_preamble:1: u8,
    pub bss:1: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_chan_rpt_data {
    pub header: mwifiex_ie_types_header,
    pub map: meas_rpt_map,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_subsc_evt {
    pub action: __le16,
    pub events: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chan_switch_result {
    pub cur_chan: u8,
    pub status: u8,
    pub reason: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_tdls_generic_event {
    pub type: __le16,
    pub peer_mac: [u8; ETH_ALEN],
    pub switch_result: chan_switch_result,
    pub cs_stop_reason: u8,
    pub reason_code: __le16,
    pub reserved: __le16,
    pub u: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie {
    pub ie_index: __le16,
    pub mgmt_subtype_mask: __le16,
    pub ie_length: __le16,
    pub ie_buffer: [u8; IEEE_MAX_IE_SIZE],
    pub __packed: },
pub const MAX_MGMT_IE_INDEX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_list {
    pub type: __le16,
    pub len: __le16,
    pub ie_list: [mwifiex_ie; MAX_MGMT_IE_INDEX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coalesce_filt_field_param {
    pub operation: u8,
    pub operand_len: u8,
    pub offset: __le16,
    pub operand_byte_stream: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coalesce_receive_filt_rule {
    pub header: mwifiex_ie_types_header,
    pub num_of_fields: u8,
    pub pkt_type: u8,
    pub max_coalescing_delay: __le16,
    pub params: [coalesce_filt_field_param; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_coalesce_cfg {
    pub action: __le16,
    pub num_of_rules: __le16,
    pub rule_data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_multi_chan_policy {
    pub action: __le16,
    pub policy: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_robust_coex {
    pub action: __le16,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_wakeup_reason {
    pub wakeup_reason: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_gtk_rekey_params {
    pub action: __le16,
    pub kck: [u8; NL80211_KCK_LEN],
    pub kek: [u8; NL80211_KEK_LEN],
    pub replay_ctr_low: __le32,
    pub replay_ctr_high: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_chan_region_cfg {
    pub action: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_pkt_aggr_ctrl {
    pub action: __le16,
    pub enable: __le16,
    pub tx_aggr_max_size: __le16,
    pub tx_aggr_max_num: __le16,
    pub tx_aggr_align: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_sta_configure {
    pub action: __le16,
    pub tlv_buffer: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_ie_types_sta_flag {
    pub header: mwifiex_ie_types_header,
    pub sta_flags: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_add_station {
    pub action: __le16,
    pub aid: __le16,
    pub peer_mac: [u8; ETH_ALEN],
    pub listen_interval: __le32,
    pub cap_info: __le16,
    pub tlv: [u8; ],
    pub __packed: },
pub const MWIFIEX_CFG_TYPE_CAL: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_802_11_cfg_data {
    pub action: __le16,
    pub type: __le16,
    pub data_len: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd_ds_command {
    pub command: __le16,
    pub size: __le16,
    pub seq_num: __le16,
    pub result: __le16,
    pub hw_spec: host_cmd_ds_get_hw_spec,
    pub mac_ctrl: host_cmd_ds_mac_control,
    pub mac_addr: host_cmd_ds_802_11_mac_address,
    pub mc_addr: host_cmd_ds_mac_multicast_adr,
    pub get_log: host_cmd_ds_802_11_get_log,
    pub rssi_info: host_cmd_ds_802_11_rssi_info,
    pub rssi_info_rsp: host_cmd_ds_802_11_rssi_info_rsp,
    pub smib: host_cmd_ds_802_11_snmp_mib,
    pub tx_rate: host_cmd_ds_tx_rate_query,
    pub tx_rate_cfg: host_cmd_ds_tx_rate_cfg,
    pub txp_cfg: host_cmd_ds_txpwr_cfg,
    pub txp: host_cmd_ds_rf_tx_pwr,
    pub ant_mimo: host_cmd_ds_rf_ant_mimo,
    pub ant_siso: host_cmd_ds_rf_ant_siso,
    pub psmode_enh: host_cmd_ds_802_11_ps_mode_enh,
    pub opt_hs_cfg: host_cmd_ds_802_11_hs_cfg_enh,
    pub scan: host_cmd_ds_802_11_scan,
    pub ext_scan: host_cmd_ds_802_11_scan_ext,
    pub scan_resp: host_cmd_ds_802_11_scan_rsp,
    pub bg_scan_config: host_cmd_ds_802_11_bg_scan_config,
    pub bg_scan_query: host_cmd_ds_802_11_bg_scan_query,
    pub bg_scan_query_resp: host_cmd_ds_802_11_bg_scan_query_rsp,
    pub associate: host_cmd_ds_802_11_associate,
    pub associate_rsp: host_cmd_ds_802_11_associate_rsp,
    pub deauth: host_cmd_ds_802_11_deauthenticate,
    pub adhoc_start: host_cmd_ds_802_11_ad_hoc_start,
    pub start_result: host_cmd_ds_802_11_ad_hoc_start_result,
    pub join_result: host_cmd_ds_802_11_ad_hoc_join_result,
    pub adhoc_join: host_cmd_ds_802_11_ad_hoc_join,
    pub domain_info: host_cmd_ds_802_11d_domain_info,
    pub domain_info_resp: host_cmd_ds_802_11d_domain_info_rsp,
    pub add_ba_req: host_cmd_ds_11n_addba_req,
    pub add_ba_rsp: host_cmd_ds_11n_addba_rsp,
    pub del_ba: host_cmd_ds_11n_delba,
    pub tx_buf: host_cmd_ds_txbuf_cfg,
    pub amsdu_aggr_ctrl: host_cmd_ds_amsdu_aggr_ctrl,
    pub htcfg: host_cmd_ds_11n_cfg,
    pub get_wmm_status: host_cmd_ds_wmm_get_status,
    pub key_material: host_cmd_ds_802_11_key_material,
    pub key_material_v2: host_cmd_ds_802_11_key_material_v2,
    pub key_material_wep: host_cmd_ds_802_11_key_material_wep,
    pub verext: host_cmd_ds_version_ext,
    pub reg_mask: host_cmd_ds_mgmt_frame_reg,
    pub roc_cfg: host_cmd_ds_remain_on_chan,
    pub mode_cfg: host_cmd_ds_p2p_mode_cfg,
    pub ibss_coalescing: host_cmd_ds_802_11_ibss_status,
    pub mef_cfg: host_cmd_ds_mef_cfg,
    pub mem: host_cmd_ds_mem_access,
    pub mac_reg: host_cmd_ds_mac_reg_access,
    pub bbp_reg: host_cmd_ds_bbp_reg_access,
    pub rf_reg: host_cmd_ds_rf_reg_access,
    pub pmic_reg: host_cmd_ds_pmic_reg_access,
    pub bss_mode: host_cmd_ds_set_bss_mode,
    pub pcie_host_spec: host_cmd_ds_pcie_details,
    pub eeprom: host_cmd_ds_802_11_eeprom_access,
    pub subsc_evt: host_cmd_ds_802_11_subsc_evt,
    pub uap_sys_config: host_cmd_ds_sys_config,
    pub sta_deauth: host_cmd_ds_sta_deauth,
    pub sta_list: host_cmd_ds_sta_list,
    pub vht_cfg: host_cmd_11ac_vht_cfg,
    pub coalesce_cfg: host_cmd_ds_coalesce_cfg,
    pub tdls_config: host_cmd_ds_tdls_config,
    pub tdls_oper: host_cmd_ds_tdls_oper,
    pub chan_rpt_req: host_cmd_ds_chan_rpt_req,
    pub sdio_rx_aggr_cfg: host_cmd_sdio_sp_rx_aggr_cfg,
    pub mc_policy: host_cmd_ds_multi_chan_policy,
    pub coex: host_cmd_ds_robust_coex,
    pub hs_wakeup_reason: host_cmd_ds_wakeup_reason,
    pub rekey: host_cmd_ds_gtk_rekey_params,
    pub reg_cfg: host_cmd_ds_chan_region_cfg,
    pub pkt_aggr_ctrl: host_cmd_ds_pkt_aggr_ctrl,
    pub sta_cfg: host_cmd_ds_sta_configure,
    pub sta_info: host_cmd_ds_add_station,
    pub cfg_data: host_cmd_ds_802_11_cfg_data,
    pub params: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwifiex_opt_sleep_confirm {
    pub command: __le16,
    pub size: __le16,
    pub seq_num: __le16,
    pub result: __le16,
    pub action: __le16,
    pub resp_ctrl: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_spec_max_conn {
    pub header: mwifiex_ie_types_header,
    pub max_p2p_conn: u8,
    pub max_sta_conn: u8,
    pub __packed: },
