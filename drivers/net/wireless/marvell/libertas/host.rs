//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas/host.h
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


// SPDX-License-Identifier: GPL-2.0
//
// This file function prototypes, data structure
// and  definitions for all the host/station commands
//

pub const DEFAULT_AD_HOC_CHANNEL: c_int = 6;
pub const CMD_OPTION_WAITFORRSP: c_uint = 0x0002;
// Host command IDs
//
// Return command are almost always the same as the host command, but with
// bit 15 set high.  There are a few exceptions, though...
//

// Return command convention exceptions:
pub const CMD_RET_802_11_ASSOCIATE: c_uint = 0x8012;
// Command codes
pub const CMD_GET_HW_SPEC: c_uint = 0x0003;
pub const CMD_EEPROM_UPDATE: c_uint = 0x0004;
pub const CMD_802_11_RESET: c_uint = 0x0005;
pub const CMD_802_11_SCAN: c_uint = 0x0006;
pub const CMD_802_11_GET_LOG: c_uint = 0x000b;
pub const CMD_MAC_MULTICAST_ADR: c_uint = 0x0010;
pub const CMD_802_11_AUTHENTICATE: c_uint = 0x0011;
pub const CMD_802_11_EEPROM_ACCESS: c_uint = 0x0059;
pub const CMD_802_11_ASSOCIATE: c_uint = 0x0050;
pub const CMD_802_11_SET_WEP: c_uint = 0x0013;
pub const CMD_802_11_GET_STAT: c_uint = 0x0014;
pub const CMD_802_3_GET_STAT: c_uint = 0x0015;
pub const CMD_802_11_SNMP_MIB: c_uint = 0x0016;
pub const CMD_MAC_REG_MAP: c_uint = 0x0017;
pub const CMD_BBP_REG_MAP: c_uint = 0x0018;
pub const CMD_MAC_REG_ACCESS: c_uint = 0x0019;
pub const CMD_BBP_REG_ACCESS: c_uint = 0x001a;
pub const CMD_RF_REG_ACCESS: c_uint = 0x001b;
pub const CMD_802_11_RADIO_CONTROL: c_uint = 0x001c;
pub const CMD_802_11_RF_CHANNEL: c_uint = 0x001d;
pub const CMD_802_11_RF_TX_POWER: c_uint = 0x001e;
pub const CMD_802_11_RSSI: c_uint = 0x001f;
pub const CMD_802_11_RF_ANTENNA: c_uint = 0x0020;
pub const CMD_802_11_PS_MODE: c_uint = 0x0021;
pub const CMD_802_11_DATA_RATE: c_uint = 0x0022;
pub const CMD_RF_REG_MAP: c_uint = 0x0023;
pub const CMD_802_11_DEAUTHENTICATE: c_uint = 0x0024;
pub const CMD_802_11_REASSOCIATE: c_uint = 0x0025;
pub const CMD_MAC_CONTROL: c_uint = 0x0028;
pub const CMD_802_11_AD_HOC_START: c_uint = 0x002b;
pub const CMD_802_11_AD_HOC_JOIN: c_uint = 0x002c;
pub const CMD_802_11_QUERY_TKIP_REPLY_CNTRS: c_uint = 0x002e;
pub const CMD_802_11_ENABLE_RSN: c_uint = 0x002f;
pub const CMD_802_11_SET_AFC: c_uint = 0x003c;
pub const CMD_802_11_GET_AFC: c_uint = 0x003d;
pub const CMD_802_11_DEEP_SLEEP: c_uint = 0x003e;
pub const CMD_802_11_AD_HOC_STOP: c_uint = 0x0040;
pub const CMD_802_11_HOST_SLEEP_CFG: c_uint = 0x0043;
pub const CMD_802_11_WAKEUP_CONFIRM: c_uint = 0x0044;
pub const CMD_802_11_HOST_SLEEP_ACTIVATE: c_uint = 0x0045;
pub const CMD_802_11_BEACON_STOP: c_uint = 0x0049;
pub const CMD_802_11_MAC_ADDRESS: c_uint = 0x004d;
pub const CMD_802_11_LED_GPIO_CTRL: c_uint = 0x004e;
pub const CMD_802_11_BAND_CONFIG: c_uint = 0x0058;
pub const CMD_GSPI_BUS_CONFIG: c_uint = 0x005a;
pub const CMD_802_11D_DOMAIN_INFO: c_uint = 0x005b;
pub const CMD_802_11_KEY_MATERIAL: c_uint = 0x005e;
pub const CMD_802_11_SLEEP_PARAMS: c_uint = 0x0066;
pub const CMD_802_11_INACTIVITY_TIMEOUT: c_uint = 0x0067;
pub const CMD_802_11_SLEEP_PERIOD: c_uint = 0x0068;
pub const CMD_802_11_TPC_CFG: c_uint = 0x0072;
pub const CMD_802_11_PA_CFG: c_uint = 0x0073;
pub const CMD_802_11_FW_WAKE_METHOD: c_uint = 0x0074;
pub const CMD_802_11_SUBSCRIBE_EVENT: c_uint = 0x0075;
pub const CMD_802_11_RATE_ADAPT_RATESET: c_uint = 0x0076;
pub const CMD_802_11_TX_RATE_QUERY: c_uint = 0x007f;
pub const CMD_GET_TSF: c_uint = 0x0080;
pub const CMD_BT_ACCESS: c_uint = 0x0087;
pub const CMD_FWT_ACCESS: c_uint = 0x0095;
pub const CMD_802_11_MONITOR_MODE: c_uint = 0x0098;
pub const CMD_MESH_ACCESS: c_uint = 0x009b;
pub const CMD_MESH_CONFIG_OLD: c_uint = 0x00a3;
pub const CMD_MESH_CONFIG: c_uint = 0x00ac;
pub const CMD_SET_BOOT2_VER: c_uint = 0x00a5;
pub const CMD_FUNC_INIT: c_uint = 0x00a9;
pub const CMD_FUNC_SHUTDOWN: c_uint = 0x00aa;
pub const CMD_802_11_BEACON_CTRL: c_uint = 0x00b0;
// For the IEEE Power Save
pub const PS_MODE_ACTION_ENTER_PS: c_uint = 0x0030;
pub const PS_MODE_ACTION_EXIT_PS: c_uint = 0x0031;
pub const PS_MODE_ACTION_SLEEP_CONFIRMED: c_uint = 0x0034;
pub const CMD_ENABLE_RSN: c_uint = 0x0001;
pub const CMD_DISABLE_RSN: c_uint = 0x0000;
pub const CMD_ACT_GET: c_uint = 0x0000;
pub const CMD_ACT_SET: c_uint = 0x0001;
// Define action or option for CMD_802_11_SET_WEP
pub const CMD_ACT_ADD: c_uint = 0x0002;
pub const CMD_ACT_REMOVE: c_uint = 0x0004;
pub const CMD_TYPE_WEP_40_BIT: c_uint = 0x01;
pub const CMD_TYPE_WEP_104_BIT: c_uint = 0x02;
pub const CMD_NUM_OF_WEP_KEYS: c_int = 4;
pub const CMD_WEP_KEY_INDEX_MASK: c_uint = 0x3fff;
// Define action or option for CMD_802_11_SCAN
pub const CMD_BSS_TYPE_BSS: c_uint = 0x0001;
pub const CMD_BSS_TYPE_IBSS: c_uint = 0x0002;
pub const CMD_BSS_TYPE_ANY: c_uint = 0x0003;
// Define action or option for CMD_802_11_SCAN
pub const CMD_SCAN_TYPE_ACTIVE: c_uint = 0x0000;
pub const CMD_SCAN_TYPE_PASSIVE: c_uint = 0x0001;
pub const CMD_SCAN_RADIO_TYPE_BG: c_int = 0;
pub const CMD_SCAN_PROBE_DELAY_TIME: c_int = 0;
// Define action or option for CMD_MAC_CONTROL
pub const CMD_ACT_MAC_RX_ON: c_uint = 0x0001;
pub const CMD_ACT_MAC_TX_ON: c_uint = 0x0002;
pub const CMD_ACT_MAC_LOOPBACK_ON: c_uint = 0x0004;
pub const CMD_ACT_MAC_WEP_ENABLE: c_uint = 0x0008;
pub const CMD_ACT_MAC_INT_ENABLE: c_uint = 0x0010;
pub const CMD_ACT_MAC_MULTICAST_ENABLE: c_uint = 0x0020;
pub const CMD_ACT_MAC_BROADCAST_ENABLE: c_uint = 0x0040;
pub const CMD_ACT_MAC_PROMISCUOUS_ENABLE: c_uint = 0x0080;
pub const CMD_ACT_MAC_ALL_MULTICAST_ENABLE: c_uint = 0x0100;
pub const CMD_ACT_MAC_STRICT_PROTECTION_ENABLE: c_uint = 0x0400;
// Event flags for CMD_802_11_SUBSCRIBE_EVENT
pub const CMD_SUBSCRIBE_RSSI_LOW: c_uint = 0x0001;
pub const CMD_SUBSCRIBE_SNR_LOW: c_uint = 0x0002;
pub const CMD_SUBSCRIBE_FAILCOUNT: c_uint = 0x0004;
pub const CMD_SUBSCRIBE_BCNMISS: c_uint = 0x0008;
pub const CMD_SUBSCRIBE_RSSI_HIGH: c_uint = 0x0010;
pub const CMD_SUBSCRIBE_SNR_HIGH: c_uint = 0x0020;
pub const RADIO_PREAMBLE_LONG: c_uint = 0x00;
pub const RADIO_PREAMBLE_SHORT: c_uint = 0x02;
pub const RADIO_PREAMBLE_AUTO: c_uint = 0x04;
// Define action or option for CMD_802_11_RF_CHANNEL
pub const CMD_OPT_802_11_RF_CHANNEL_GET: c_uint = 0x00;
pub const CMD_OPT_802_11_RF_CHANNEL_SET: c_uint = 0x01;
// Define action or option for CMD_802_11_DATA_RATE
pub const CMD_ACT_SET_TX_AUTO: c_uint = 0x0000;
pub const CMD_ACT_SET_TX_FIX_RATE: c_uint = 0x0001;
pub const CMD_ACT_GET_TX_RATE: c_uint = 0x0002;
// Options for CMD_802_11_FW_WAKE_METHOD
pub const CMD_WAKE_METHOD_UNCHANGED: c_uint = 0x0000;
pub const CMD_WAKE_METHOD_COMMAND_INT: c_uint = 0x0001;
pub const CMD_WAKE_METHOD_GPIO: c_uint = 0x0002;
// Object IDs for CMD_802_11_SNMP_MIB
pub const SNMP_MIB_OID_BSS_TYPE: c_uint = 0x0000;
pub const SNMP_MIB_OID_OP_RATE_SET: c_uint = 0x0001;
pub const SNMP_MIB_OID_BEACON_PERIOD: c_uint = 0x0002  /* Reserved on v9+ */;
pub const SNMP_MIB_OID_DTIM_PERIOD: c_uint = 0x0003  /* Reserved on v9+ */;
pub const SNMP_MIB_OID_ASSOC_TIMEOUT: c_uint = 0x0004  /* Reserved on v9+ */;
pub const SNMP_MIB_OID_RTS_THRESHOLD: c_uint = 0x0005;
pub const SNMP_MIB_OID_SHORT_RETRY_LIMIT: c_uint = 0x0006;
pub const SNMP_MIB_OID_LONG_RETRY_LIMIT: c_uint = 0x0007;
pub const SNMP_MIB_OID_FRAG_THRESHOLD: c_uint = 0x0008;
pub const SNMP_MIB_OID_11D_ENABLE: c_uint = 0x0009;
pub const SNMP_MIB_OID_11H_ENABLE: c_uint = 0x000A;
// Define action or option for CMD_BT_ACCESS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_bt_access_opts {
// The bt commands start at 5 instead of 1 because the old dft commands
// are mapped to 1-4.  These old commands are no longer maintained and
// should not be called.
//
    CMD_ACT_BT_ACCESS_ADD = 5,
    CMD_ACT_BT_ACCESS_DEL,
    CMD_ACT_BT_ACCESS_LIST,
    CMD_ACT_BT_ACCESS_RESET,
    CMD_ACT_BT_ACCESS_SET_INVERT,
    CMD_ACT_BT_ACCESS_GET_INVERT
}

// Define action or option for CMD_FWT_ACCESS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_fwt_access_opts {
    CMD_ACT_FWT_ACCESS_ADD = 1,
    CMD_ACT_FWT_ACCESS_DEL,
    CMD_ACT_FWT_ACCESS_LOOKUP,
    CMD_ACT_FWT_ACCESS_LIST,
    CMD_ACT_FWT_ACCESS_LIST_ROUTE,
    CMD_ACT_FWT_ACCESS_LIST_NEIGHBOR,
    CMD_ACT_FWT_ACCESS_RESET,
    CMD_ACT_FWT_ACCESS_CLEANUP,
    CMD_ACT_FWT_ACCESS_TIME,
}

// Define action or option for CMD_802_11_HOST_SLEEP_CFG
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_wol_cfg_opts {
    CMD_ACT_ACTION_NONE = 0,
    CMD_ACT_SET_WOL_RULE,
    CMD_ACT_GET_WOL_RULE,
    CMD_ACT_RESET_WOL_RULE,
}

// Define action or option for CMD_MESH_ACCESS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_mesh_access_opts {
    CMD_ACT_MESH_GET_TTL = 1,
    CMD_ACT_MESH_SET_TTL,
    CMD_ACT_MESH_GET_STATS,
    CMD_ACT_MESH_GET_ANYCAST,
    CMD_ACT_MESH_SET_ANYCAST,
    CMD_ACT_MESH_SET_LINK_COSTS,
    CMD_ACT_MESH_GET_LINK_COSTS,
    CMD_ACT_MESH_SET_BCAST_RATE,
    CMD_ACT_MESH_GET_BCAST_RATE,
    CMD_ACT_MESH_SET_RREQ_DELAY,
    CMD_ACT_MESH_GET_RREQ_DELAY,
    CMD_ACT_MESH_SET_ROUTE_EXP,
    CMD_ACT_MESH_GET_ROUTE_EXP,
    CMD_ACT_MESH_SET_AUTOSTART_ENABLED,
    CMD_ACT_MESH_GET_AUTOSTART_ENABLED,
    CMD_ACT_MESH_SET_GET_PRB_RSP_LIMIT = 17,
}

// Define actions and types for CMD_MESH_CONFIG
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_mesh_config_actions {
    CMD_ACT_MESH_CONFIG_STOP = 0,
    CMD_ACT_MESH_CONFIG_START,
    CMD_ACT_MESH_CONFIG_SET,
    CMD_ACT_MESH_CONFIG_GET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_mesh_config_types {
    CMD_TYPE_MESH_SET_BOOTFLAG = 1,
    CMD_TYPE_MESH_SET_BOOTTIME,
    CMD_TYPE_MESH_SET_DEF_CHANNEL,
    CMD_TYPE_MESH_SET_MESH_IE,
    CMD_TYPE_MESH_GET_DEFAULTS,
    CMD_TYPE_MESH_GET_MESH_IE, /* GET_DEFAULTS is superset of GET_MESHIE */
}

// Card Event definition
pub const MACREG_INT_CODE_TX_PPA_FREE: c_int = 0;
pub const MACREG_INT_CODE_TX_DMA_DONE: c_int = 1;
pub const MACREG_INT_CODE_LINK_LOST_W_SCAN: c_int = 2;
pub const MACREG_INT_CODE_LINK_LOST_NO_SCAN: c_int = 3;
pub const MACREG_INT_CODE_LINK_SENSED: c_int = 4;
pub const MACREG_INT_CODE_CMD_FINISHED: c_int = 5;
pub const MACREG_INT_CODE_MIB_CHANGED: c_int = 6;
pub const MACREG_INT_CODE_INIT_DONE: c_int = 7;
pub const MACREG_INT_CODE_DEAUTHENTICATED: c_int = 8;
pub const MACREG_INT_CODE_DISASSOCIATED: c_int = 9;
pub const MACREG_INT_CODE_PS_AWAKE: c_int = 10;
pub const MACREG_INT_CODE_PS_SLEEP: c_int = 11;
pub const MACREG_INT_CODE_MIC_ERR_MULTICAST: c_int = 13;
pub const MACREG_INT_CODE_MIC_ERR_UNICAST: c_int = 14;
pub const MACREG_INT_CODE_WM_AWAKE: c_int = 15;
pub const MACREG_INT_CODE_DEEP_SLEEP_AWAKE: c_int = 16;
pub const MACREG_INT_CODE_ADHOC_BCN_LOST: c_int = 17;
pub const MACREG_INT_CODE_HOST_AWAKE: c_int = 18;
pub const MACREG_INT_CODE_STOP_TX: c_int = 19;
pub const MACREG_INT_CODE_START_TX: c_int = 20;
pub const MACREG_INT_CODE_CHANNEL_SWITCH: c_int = 21;
pub const MACREG_INT_CODE_MEASUREMENT_RDY: c_int = 22;
pub const MACREG_INT_CODE_WMM_CHANGE: c_int = 23;
pub const MACREG_INT_CODE_BG_SCAN_REPORT: c_int = 24;
pub const MACREG_INT_CODE_RSSI_LOW: c_int = 25;
pub const MACREG_INT_CODE_SNR_LOW: c_int = 26;
pub const MACREG_INT_CODE_MAX_FAIL: c_int = 27;
pub const MACREG_INT_CODE_RSSI_HIGH: c_int = 28;
pub const MACREG_INT_CODE_SNR_HIGH: c_int = 29;
pub const MACREG_INT_CODE_MESH_AUTO_STARTED: c_int = 35;
pub const MACREG_INT_CODE_FIRMWARE_READY: c_int = 48;
// 802.11-related definitions
// TxPD descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txpd {
// union to cope up with later FW revisions
// Current Tx packet status
    pub tx_status: __le32,
// BSS type: client, AP, etc.
    pub bss_type: u8,
// BSS number
    pub bss_num: u8,
// Reserved
    pub reserved: __le16,
    pub bss: },
    pub u: },
// Tx control
    pub tx_control: __le32,
    pub tx_packet_location: __le32,
// Tx packet length
    pub tx_packet_length: __le16,
// First 2 byte of destination MAC address
    pub tx_dest_addr_high: [u8; 2],
// Last 4 byte of destination MAC address
    pub tx_dest_addr_low: [u8; 4],
// Pkt Priority
    pub priority: u8,
// Pkt Trasnit Power control
    pub powermgmt: u8,
// Amount of time the packet has been queued (units = 2ms)
    pub pktdelay_2ms: u8,
// reserved
    pub reserved1: u8,
    pub __packed: },
// RxPD Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxpd {
// union to cope up with later FW revisions
// Current Rx packet status
    pub status: __le16,
// BSS type: client, AP, etc.
    pub bss_type: u8,
// BSS number
    pub bss_num: u8,
    pub bss: } __packed,
    pub u: } __packed,
// SNR
    pub snr: u8,
// Tx control
    pub rx_control: u8,
// Pkt length
    pub pkt_len: __le16,
// Noise Floor
    pub nf: u8,
// Rx Packet Rate
    pub rx_rate: u8,
// Pkt addr
    pub pkt_ptr: __le32,
// Next Rx RxPD addr
    pub next_rxpd_ptr: __le32,
// Pkt Priority
    pub priority: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_header {
    pub command: __le16,
    pub size: __le16,
    pub seqnum: __le16,
    pub result: __le16,
    pub __packed: },
// Generic structure to hold all key types.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_key {
    pub len: u16,
    pub /: *mut *mut *mut u16 flags; / KEY_INFO_ from defs.h,
    pub /: *mut *mut *mut u16 type; / KEY_TYPE_ from defs.h,
    pub key: [u8; 32],
}

// lbs_offset_value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbs_offset_value {
    pub offset: u32,
    pub value: u32,
    pub __packed: },
pub const MAX_11D_TRIPLETS: c_int = 83;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mrvl_ie_domain_param_set {
    pub header: mrvl_ie_header,
    pub country_code: [u8; IEEE80211_COUNTRY_STRING_LEN],
    pub triplet: [ieee80211_country_ie_triplet; MAX_11D_TRIPLETS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11d_domain_info {
    pub hdr: cmd_header,
    pub action: __le16,
    pub domain: mrvl_ie_domain_param_set,
    pub __packed: },
//
// Define data structure for CMD_GET_HW_SPEC
// This structure defines the response for the GET_HW_SPEC command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_get_hw_spec {
    pub hdr: cmd_header,
// HW Interface version number
    pub hwifversion: __le16,
// HW version number
    pub version: __le16,
// Max number of TxPD FW can handle
    pub nr_txpd: __le16,
// Max no of Multicast address
    pub nr_mcast_adr: __le16,
// MAC address
    pub permanentaddr: [u8; 6],
// region Code
    pub regioncode: __le16,
// Number of antenna used
    pub nr_antenna: __le16,
// FW release number, example 0x01030304 = 2.3.4p1
    pub fwrelease: __le32,
// Base Address of TxPD queue
    pub wcb_base: __le32,
// Read Pointer of RxPd queue
    pub rxpd_rdptr: __le32,
// Write Pointer of RxPd queue
    pub rxpd_wrptr: __le32,
// FW/HW capability
    pub fwcapinfo: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_subscribe_event {
    pub hdr: cmd_header,
    pub action: __le16,
    pub events: __le16,
// A TLV to the CMD_802_11_SUBSCRIBE_EVENT command can contain a
// number of TLVs. From the v5.1 manual, those TLVs would add up to
// 40 bytes. However, future firmware might add additional TLVs, so I
// bump this up a bit.
//
    pub tlv: [u8; 128],
    pub __packed: },
//
// This scan handle Country Information IE(802.11d compliant)
// Define data structure for CMD_802_11_SCAN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_scan {
    pub hdr: cmd_header,
    pub bsstype: u8,
    pub bssid: [u8; ETH_ALEN],
    pub tlvbuffer: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_scan_rsp {
    pub hdr: cmd_header,
    pub bssdescriptsize: __le16,
    pub nr_sets: u8,
    pub bssdesc_and_tlvbuffer: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_get_log {
    pub hdr: cmd_header,
    pub mcasttxframe: __le32,
    pub failed: __le32,
    pub retry: __le32,
    pub multiretry: __le32,
    pub framedup: __le32,
    pub rtssuccess: __le32,
    pub rtsfailure: __le32,
    pub ackfailure: __le32,
    pub rxfrag: __le32,
    pub mcastrxframe: __le32,
    pub fcserror: __le32,
    pub txframe: __le32,
    pub wepundecryptable: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_mac_control {
    pub hdr: cmd_header,
    pub action: __le16,
    pub reserved: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_mac_multicast_adr {
    pub hdr: cmd_header,
    pub action: __le16,
    pub nr_of_adrs: __le16,
    pub MRVDRV_MAX_MULTICAST_LIST_SIZE]: *mut *mut u8 maclist[ETH_ALEN,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_authenticate {
    pub hdr: cmd_header,
    pub bssid: [u8; ETH_ALEN],
    pub authtype: u8,
    pub reserved: [u8; 10],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_deauthenticate {
    pub hdr: cmd_header,
    pub macaddr: [u8; ETH_ALEN],
    pub reasoncode: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_associate {
    pub hdr: cmd_header,
    pub bssid: [u8; 6],
    pub capability: __le16,
    pub listeninterval: __le16,
    pub bcnperiod: __le16,
    pub dtimperiod: u8,
// 512 permitted - enough for required and most optional IEs
    pub iebuf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_associate_response {
    pub hdr: cmd_header,
    pub capability: __le16,
    pub statuscode: __le16,
    pub aid: __le16,
// max 512
    pub iebuf: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_set_wep {
    pub hdr: cmd_header,
// ACT_ADD, ACT_REMOVE or ACT_ENABLE
    pub action: __le16,
// key Index selected for Tx
    pub keyindex: __le16,
// 40, 128bit or TXWEP
    pub keytype: [u8; 4],
    pub keymaterial: [u8; 4][16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_snmp_mib {
    pub hdr: cmd_header,
    pub action: __le16,
    pub oid: __le16,
    pub bufsize: __le16,
    pub value: [u8; 128],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_reg_access {
    pub hdr: cmd_header,
    pub action: __le16,
    pub offset: __le16,
    pub /: *mut *mut u8 bbp_rf; / for BBP and RF registers,
    pub /: *mut *mut __le32 mac; / for MAC registers,
    pub value: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_radio_control {
    pub hdr: cmd_header,
    pub action: __le16,
    pub control: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_beacon_control {
    pub hdr: cmd_header,
    pub action: __le16,
    pub beacon_enable: __le16,
    pub beacon_period: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_sleep_params {
    pub hdr: cmd_header,
// ACT_GET/ACT_SET
    pub action: __le16,
// Sleep clock error in ppm
    pub error: __le16,
// Wakeup offset in usec
    pub offset: __le16,
// Clock stabilization time in usec
    pub stabletime: __le16,
// control periodic calibration
    pub calcontrol: u8,
// control the use of external sleep clock
    pub externalsleepclk: u8,
// reserved field, should be set to zero
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_rf_channel {
    pub hdr: cmd_header,
    pub action: __le16,
    pub channel: __le16,
    pub /: *mut *mut __le16 rftype; / unused,
    pub /: *mut *mut __le16 reserved; / unused,
    pub /: *mut *mut u8 channellist[32]; / unused,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_rssi {
    pub hdr: cmd_header,
//
// request:  number of beacons (N) to average the SNR and NF over
// response: SNR of most recent beacon
//
    pub n_or_snr: __le16,
//
// The following fields are only set in the response.
// In the request these are reserved and should be set to 0.
//
    pub /: *mut *mut __le16 nf; / most recent beacon noise floor,
    pub /: *mut *mut __le16 avg_snr; / average SNR weighted by N from request,
    pub /: *mut *mut __le16 avg_nf; / average noise floor weighted by N from request,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_mac_address {
    pub hdr: cmd_header,
    pub action: __le16,
    pub macadd: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_rf_tx_power {
    pub hdr: cmd_header,
    pub action: __le16,
    pub curlevel: __le16,
    pub maxlevel: i8,
    pub minlevel: i8,
    pub __packed: },
// MONITOR_MODE only exists in OLPC v5 firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_monitor_mode {
    pub hdr: cmd_header,
    pub action: __le16,
    pub mode: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_set_boot2_ver {
    pub hdr: cmd_header,
    pub action: __le16,
    pub version: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_fw_wake_method {
    pub hdr: cmd_header,
    pub action: __le16,
    pub method: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_ps_mode {
    pub hdr: cmd_header,
    pub action: __le16,
//
// Interval for keepalive in PS mode:
// 0x0000 = don't change
// 0x001E = firmware default
// 0xFFFF = disable
//
    pub nullpktinterval: __le16,
//
// Number of DTIM intervals to wake up for:
// 0 = don't change
// 1 = firmware default
// 5 = max
//
    pub multipledtim: __le16,
    pub reserved: __le16,
    pub locallisteninterval: __le16,
//
// AdHoc awake period (FW v9+ only):
// 0 = don't change
// 1 = always awake (IEEE standard behavior)
// 2 - 31 = sleep for (n - 1) periods and awake for 1 period
// 32 - 254 = invalid
// 255 = sleep at each ATIM
//
    pub adhoc_awake_period: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_confirm_sleep {
    pub hdr: cmd_header,
    pub action: __le16,
    pub nullpktinterval: __le16,
    pub multipledtim: __le16,
    pub reserved: __le16,
    pub locallisteninterval: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_data_rate {
    pub hdr: cmd_header,
    pub action: __le16,
    pub reserved: __le16,
    pub rates: [u8; MAX_RATES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_rate_adapt_rateset {
    pub hdr: cmd_header,
    pub action: __le16,
    pub enablehwauto: __le16,
    pub bitmap: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_ad_hoc_start {
    pub hdr: cmd_header,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub bsstype: u8,
    pub beaconperiod: __le16,
    pub /: *mut *mut u8 dtimperiod; / Reserved on v9 and later,
    pub ibss: ieee_ie_ibss_param_set,
    pub reserved1: [u8; 4],
    pub ds: ieee_ie_ds_param_set,
    pub reserved2: [u8; 4],
    pub /: *mut *mut __le16 probedelay; / Reserved on v9 and later,
    pub capability: __le16,
    pub rates: [u8; MAX_RATES],
    pub tlv_memory_size_pad: [u8; 100],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_ad_hoc_result {
    pub hdr: cmd_header,
    pub pad: [u8; 3],
    pub bssid: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adhoc_bssdesc {
    pub bssid: [u8; ETH_ALEN],
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub type: u8,
    pub beaconperiod: __le16,
    pub dtimperiod: u8,
    pub timestamp: __le64,
    pub localtime: __le64,
    pub ds: ieee_ie_ds_param_set,
    pub reserved1: [u8; 4],
    pub ibss: ieee_ie_ibss_param_set,
    pub reserved2: [u8; 4],
    pub capability: __le16,
    pub rates: [u8; MAX_RATES],
//
// DO NOT ADD ANY FIELDS TO THIS STRUCTURE. It is used below in the
// Adhoc join command and will cause a binary layout mismatch with
// the firmware
//
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_ad_hoc_join {
    pub hdr: cmd_header,
    pub bss: adhoc_bssdesc,
    pub /: *mut *mut __le16 failtimeout; / Reserved on v9 and later,
    pub /: *mut *mut __le16 probedelay; / Reserved on v9 and later,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_ad_hoc_stop {
    pub hdr: cmd_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_enable_rsn {
    pub hdr: cmd_header,
    pub action: __le16,
    pub enable: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MrvlIEtype_keyParamSet {
// type ID
    pub type: __le16,
// length of Payload
    pub length: __le16,
// type of key: WEP=0, TKIP=1, AES=2
    pub keytypeid: __le16,
// key control Info specific to a keytypeid
    pub keyinfo: __le16,
// length of key
    pub keylen: __le16,
// key material of size keylen
    pub key: [u8; 32],
    pub __packed: },
pub const MAX_WOL_RULES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_wol_rule {
    pub rule_no: u8,
    pub rule_ops: u8,
    pub sig_offset: __le16,
    pub sig_length: __le16,
    pub reserve: __le16,
    pub sig_mask: __be32,
    pub signature: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wol_config {
    pub action: u8,
    pub pattern: u8,
    pub no_rules_in_cmd: u8,
    pub result: u8,
    pub rule: [host_wol_rule; MAX_WOL_RULES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_host_sleep {
    pub hdr: cmd_header,
    pub criteria: __le32,
    pub gpio: u8,
    pub gap: u16,
    pub wol_conf: wol_config,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_key_material {
    pub hdr: cmd_header,
    pub action: __le16,
    pub keyParamSet: [MrvlIEtype_keyParamSet; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_eeprom_access {
    pub hdr: cmd_header,
    pub action: __le16,
    pub offset: __le16,
    pub len: __le16,
// firmware says it returns a maximum of 20 bytes
pub const LBS_EEPROM_READ_LEN: c_int = 20;
    pub value: [u8; LBS_EEPROM_READ_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_tpc_cfg {
    pub hdr: cmd_header,
    pub action: __le16,
    pub enable: u8,
    pub P0: i8,
    pub P1: i8,
    pub P2: i8,
    pub usesnr: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_pa_cfg {
    pub hdr: cmd_header,
    pub action: __le16,
    pub enable: u8,
    pub P0: i8,
    pub P1: i8,
    pub P2: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_led_ctrl {
    pub hdr: cmd_header,
    pub action: __le16,
    pub numled: __le16,
    pub data: [u8; 256],
    pub __packed: },
// Automatic Frequency Control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_802_11_afc {
    pub hdr: cmd_header,
    pub afc_auto: __le16,
    pub threshold: __le16,
    pub period: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_tx_rate_query {
    pub txrate: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_get_tsf {
    pub tsfvalue: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_bt_access {
    pub hdr: cmd_header,
    pub action: __le16,
    pub id: __le32,
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_fwt_access {
    pub hdr: cmd_header,
    pub action: __le16,
    pub id: __le32,
    pub valid: u8,
    pub da: [u8; ETH_ALEN],
    pub dir: u8,
    pub ra: [u8; ETH_ALEN],
    pub ssn: __le32,
    pub dsn: __le32,
    pub metric: __le32,
    pub rate: u8,
    pub hopcount: u8,
    pub ttl: u8,
    pub expiration: __le32,
    pub sleepmode: u8,
    pub snr: __le32,
    pub references: __le32,
    pub prec: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_mesh_config {
    pub hdr: cmd_header,
    pub action: __le16,
    pub channel: __le16,
    pub type: __le16,
    pub length: __le16,
    pub /: *mut *mut u8 data[128]; / last position reserved,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_ds_mesh_access {
    pub hdr: cmd_header,
    pub action: __le16,
    pub /: *mut *mut __le32 data[32]; / last position reserved,
    pub __packed: },
// Number of stats counters returned by the firmware
pub const MESH_STATS_NUM: c_int = 8;
