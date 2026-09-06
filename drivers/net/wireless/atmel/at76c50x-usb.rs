//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/atmel/at76c50x-usb.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2002,2003 Oliver Kurth
// (c) 2003,2004 Joerg Albert <joerg.albert@gmx.de>
// (c) 2007 Guido Guenther <agx@sigxcpu.org>
//
// This driver was based on information from the Sourceforge driver
// released and maintained by Atmel:
//
// http://sourceforge.net/projects/atmelwlandriver
//
// Although the code was completely re-written,
// it would have been impossible without Atmel's decision to
// release an Open Source driver (unfortunately the firmware was
// kept binary only). Thanks for that decision to Atmel!
//
// Board types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum board_type {
    BOARD_503_ISL3861 = 1,
    BOARD_503_ISL3863 = 2,
    BOARD_503 = 3,
    BOARD_503_ACC = 4,
    BOARD_505 = 5,
    BOARD_505_2958 = 6,
    BOARD_505A = 7,
    BOARD_505AMX = 8
}

pub const CMD_STATUS_IDLE: c_uint = 0x00;
pub const CMD_STATUS_COMPLETE: c_uint = 0x01;
pub const CMD_STATUS_UNKNOWN: c_uint = 0x02;
pub const CMD_STATUS_INVALID_PARAMETER: c_uint = 0x03;
pub const CMD_STATUS_FUNCTION_NOT_SUPPORTED: c_uint = 0x04;
pub const CMD_STATUS_TIME_OUT: c_uint = 0x07;
pub const CMD_STATUS_IN_PROGRESS: c_uint = 0x08;
pub const CMD_STATUS_HOST_FAILURE: c_uint = 0xff;
pub const CMD_STATUS_SCAN_FAILED: c_uint = 0xf0;
// answers to get op mode
pub const OPMODE_NONE: c_uint = 0x00;
pub const OPMODE_NORMAL_NIC_WITH_FLASH: c_uint = 0x01;
pub const OPMODE_HW_CONFIG_MODE: c_uint = 0x02;
pub const OPMODE_DFU_MODE_WITH_FLASH: c_uint = 0x03;
pub const OPMODE_NORMAL_NIC_WITHOUT_FLASH: c_uint = 0x04;
pub const CMD_SET_MIB: c_uint = 0x01;
pub const CMD_GET_MIB: c_uint = 0x02;
pub const CMD_SCAN: c_uint = 0x03;
pub const CMD_JOIN: c_uint = 0x04;
pub const CMD_START_IBSS: c_uint = 0x05;
pub const CMD_RADIO_ON: c_uint = 0x06;
pub const CMD_RADIO_OFF: c_uint = 0x07;
pub const CMD_STARTUP: c_uint = 0x0B;
pub const MIB_LOCAL: c_uint = 0x01;
pub const MIB_MAC_ADDR: c_uint = 0x02;
pub const MIB_MAC: c_uint = 0x03;
pub const MIB_MAC_MGMT: c_uint = 0x05;
pub const MIB_MAC_WEP: c_uint = 0x06;
pub const MIB_PHY: c_uint = 0x07;
pub const MIB_FW_VERSION: c_uint = 0x08;
pub const MIB_MDOMAIN: c_uint = 0x09;
pub const ADHOC_MODE: c_int = 1;
pub const INFRASTRUCTURE_MODE: c_int = 2;
// values for struct mib_local, field preamble_type
pub const PREAMBLE_TYPE_LONG: c_int = 0;
pub const PREAMBLE_TYPE_SHORT: c_int = 1;
pub const PREAMBLE_TYPE_AUTO: c_int = 2;
// values for tx_rate
pub const TX_RATE_1MBIT: c_int = 0;
pub const TX_RATE_2MBIT: c_int = 1;
pub const TX_RATE_5_5MBIT: c_int = 2;
pub const TX_RATE_11MBIT: c_int = 3;
pub const TX_RATE_AUTO: c_int = 4;
// power management modes
pub const AT76_PM_OFF: c_int = 1;
pub const AT76_PM_ON: c_int = 2;
pub const AT76_PM_SMART: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwcfg_r505 {
    pub cr39_values: [u8; 14],
    pub reserved1: [u8; 14],
    pub bb_cr: [u8; 14],
    pub pidvid: [u8; 4],
    pub mac_addr: [u8; ETH_ALEN],
    pub regulatory_domain: u8,
    pub reserved2: [u8; 14],
    pub cr15_values: [u8; 14],
    pub reserved3: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwcfg_rfmd {
    pub cr20_values: [u8; 14],
    pub cr21_values: [u8; 14],
    pub bb_cr: [u8; 14],
    pub pidvid: [u8; 4],
    pub mac_addr: [u8; ETH_ALEN],
    pub regulatory_domain: u8,
    pub low_power_values: [u8; 14],
    pub normal_power_values: [u8; 14],
    pub reserved1: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwcfg_intersil {
    pub mac_addr: [u8; ETH_ALEN],
    pub cr31_values: [u8; 14],
    pub cr58_values: [u8; 14],
    pub pidvid: [u8; 4],
    pub regulatory_domain: u8,
    pub reserved: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union at76_hwcfg {
    pub i: hwcfg_intersil,
    pub r3: hwcfg_rfmd,
    pub r5: hwcfg_r505,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at76_card_config {
    pub exclude_unencrypted: u8,
    pub promiscuous_mode: u8,
    pub short_retry_limit: u8,
    pub encryption_type: u8,
    pub rts_threshold: __le16,
    pub /: *mut *mut __le16 fragmentation_threshold; / 256..2346,
    pub basic_rate_set: [u8; 4],
    pub /: *mut *mut u8 auto_rate_fallback; / 0,1,
    pub channel: u8,
    pub privacy_invoked: u8,
    pub /: *mut *mut u8 wep_default_key_id; / 0..3,
    pub current_ssid: [u8; 32],
    pub wep_default_key_value: [u8; 4][WEP_LARGE_KEY_LEN],
    pub ssid_len: u8,
    pub short_preamble: u8,
    pub beacon_period: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at76_command {
    pub cmd: u8,
    pub reserved: u8,
    pub size: __le16,
    pub __counted_by_le(size): u8 data[],
    pub __packed: },
// Length of Atmel-specific Rx header before 802.11 frame

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at76_rx_buffer {
    pub wlength: __le16,
    pub rx_rate: u8,
    pub newbss: u8,
    pub fragmentation: u8,
    pub rssi: u8,
    pub link_quality: u8,
    pub noise_level: u8,
    pub rx_time: __le32,
    pub packet: [u8; IEEE80211_MAX_FRAG_THRESHOLD],
    pub __packed: },
// Length of Atmel-specific Tx header before 802.11 frame

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at76_tx_buffer {
    pub wlength: __le16,
    pub tx_rate: u8,
    pub padding: u8,
    pub reserved: [u8; 4],
    pub packet: [u8; IEEE80211_MAX_FRAG_THRESHOLD],
    pub __packed: },
// defines for scan_type below
pub const SCAN_TYPE_ACTIVE: c_int = 0;
pub const SCAN_TYPE_PASSIVE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at76_req_scan {
    pub bssid: [u8; ETH_ALEN],
    pub essid: [u8; 32],
    pub scan_type: u8,
    pub channel: u8,
    pub probe_delay: __le16,
    pub min_channel_time: __le16,
    pub max_channel_time: __le16,
    pub essid_size: u8,
    pub international_scan: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at76_req_ibss {
    pub bssid: [u8; ETH_ALEN],
    pub essid: [u8; 32],
    pub bss_type: u8,
    pub channel: u8,
    pub essid_size: u8,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at76_req_join {
    pub bssid: [u8; ETH_ALEN],
    pub essid: [u8; 32],
    pub bss_type: u8,
    pub channel: u8,
    pub timeout: __le16,
    pub essid_size: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mib_local {
    pub reserved0: u16,
    pub beacon_enable: u8,
    pub txautorate_fallback: u8,
    pub reserved1: u8,
    pub ssid_size: u8,
    pub promiscuous_mode: u8,
    pub reserved2: u16,
    pub preamble_type: u8,
    pub reserved3: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mib_mac_addr {
    pub mac_addr: [u8; ETH_ALEN],
    pub /: *mut *mut u8 res[2]; / ???,
    pub group_addr: [u8; 4][ETH_ALEN],
    pub group_addr_status: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mib_mac {
    pub max_tx_msdu_lifetime: __le32,
    pub max_rx_lifetime: __le32,
    pub frag_threshold: __le16,
    pub rts_threshold: __le16,
    pub cwmin: __le16,
    pub cwmax: __le16,
    pub short_retry_time: u8,
    pub long_retry_time: u8,
    pub /: *mut *mut u8 scan_type; / active or passive,
    pub scan_channel: u8,
    pub /: *mut *mut __le16 probe_delay; / delay before ProbeReq in active scan, RO,
    pub min_channel_time: __le16,
    pub max_channel_time: __le16,
    pub listen_interval: __le16,
    pub desired_ssid: [u8; 32],
    pub desired_bssid: [u8; ETH_ALEN],
    pub /: *mut *mut u8 desired_bsstype; / ad-hoc or infrastructure,
    pub reserved2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mib_mac_mgmt {
    pub beacon_period: __le16,
    pub CFP_max_duration: __le16,
    pub medium_occupancy_limit: __le16,
    pub /: *mut *mut __le16 station_id; / assoc id,
    pub ATIM_window: __le16,
    pub CFP_mode: u8,
    pub privacy_option_implemented: u8,
    pub DTIM_period: u8,
    pub CFP_period: u8,
    pub current_bssid: [u8; ETH_ALEN],
    pub current_essid: [u8; 32],
    pub current_bss_type: u8,
    pub power_mgmt_mode: u8,
// rfmd and 505
    pub ibss_change: u8,
    pub res: u8,
    pub multi_domain_capability_implemented: u8,
    pub multi_domain_capability_enabled: u8,
    pub country_string: [u8; IEEE80211_COUNTRY_STRING_LEN],
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mib_mac_wep {
    pub /: *mut *mut u8 privacy_invoked; / 0 disable encr., 1 enable encr,
    pub wep_default_key_id: u8,
    pub wep_key_mapping_len: u8,
    pub exclude_unencrypted: u8,
    pub wep_icv_error_count: __le32,
    pub wep_excluded_count: __le32,
    pub wep_default_keyvalue: [u8; WEP_KEYS][WEP_LARGE_KEY_LEN],
    pub /: *mut *mut u8 encryption_level; / 1 for 40bit, 2 for 104bit encryption,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mib_phy {
    pub ed_threshold: __le32,
    pub slot_time: __le16,
    pub sifs_time: __le16,
    pub preamble_length: __le16,
    pub plcp_header_length: __le16,
    pub mpdu_max_length: __le16,
    pub cca_mode_supported: __le16,
    pub operation_rate_set: [u8; 4],
    pub channel_id: u8,
    pub current_cca_mode: u8,
    pub phy_type: u8,
    pub current_reg_domain: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mib_fw_version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mib_mdomain {
    pub tx_powerlevel: [u8; 14],
    pub /: *mut *mut u8 channel_list[14]; / 0 for invalid channels,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_mib_buffer {
    pub type: u8,
    pub size: u8,
    pub index: u8,
    pub reserved: u8,
    pub byte: u8,
    pub word: __le16,
    pub addr: [u8; ETH_ALEN],
    pub wep_mib: mib_mac_wep,
    pub data: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at76_fw_header {
    pub /: *mut *mut __le32 crc; / CRC32 of the whole image,
    pub /: *mut *mut __le32 board_type; / firmware compatibility code,
    pub /: *mut *mut u8 build; / firmware build number,
    pub /: *mut *mut u8 patch; / firmware patch level,
    pub /: *mut *mut u8 minor; / firmware minor version,
    pub /: *mut *mut u8 major; / firmware major version,
    pub /: *mut *mut __le32 str_offset; / offset of the copyright string,
    pub /: *mut *mut __le32 int_fw_offset; / internal firmware image offset,
    pub /: *mut *mut __le32 int_fw_len; / internal firmware image length,
    pub /: *mut *mut __le32 ext_fw_offset; / external firmware image offset,
    pub /: *mut *mut __le32 ext_fw_len; / external firmware image length,
    pub __packed: },
// a description of a regulatory domain and the allowed channels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_domain {
    pub code: u16,
    pub name: *const c_char,
    pub /: *mut *mut u32 channel_map; / if bit N is set, channel (N+1) is allowed,
}

// Data for one loaded firmware file
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwentry {
    pub fwname: *const *const c_char,
    pub fw: *const firmware,
    pub extfw_size: c_int,
    pub intfw_size: c_int,
// pointer to loaded firmware, no need to free
    pub /: *mut *mut *mut u8 extfw; / external firmware, extfw_size bytes long,
    pub /: *mut *mut *mut u8 intfw; / internal firmware, intfw_size bytes long,
    pub /: *mut *mut board_type board_type; / board type,
    pub fw_version: mib_fw_version,
    pub /: *mut *mut int loaded; / Loaded and parsed successfully,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at76_priv {
    pub /: *mut *mut *mut usb_device udev; / USB device pointer,
    pub /: *mut *mut *mut sk_buff rx_skb; / skbuff for receiving data,
    pub /: *mut *mut *mut sk_buff tx_skb; / skbuff for transmitting data,
    pub /: *mut *mut *mut void bulk_out_buffer; / buffer for sending data,
    pub /: *mut *mut *mut urb tx_urb; / URB for sending data,
    pub /: *mut *mut *mut urb rx_urb; / URB for receiving data,
    pub /: *mut *mut unsigned int tx_pipe; / bulk out pipe,
    pub /: *mut *mut unsigned int rx_pipe; / bulk in pipe,
    pub /: *mut *mut mutex mtx; / locks this structure,
// work queues
    pub work_set_promisc: work_struct,
    pub work_submit_rx: work_struct,
    pub work_join_bssid: work_struct,
    pub dwork_hw_scan: delayed_work,
    pub rx_tasklet: tasklet_struct,
// the WEP stuff
    pub /: *mut *mut int wep_enabled; / 1 if WEP is enabled,
    pub /: *mut *mut int wep_key_id; / key id to be used,
    pub /: *mut *mut u8 wep_keys[WEP_KEYS][WEP_LARGE_KEY_LEN]; / WEP keys,
    pub /: *mut *mut u8 wep_keys_len[WEP_KEYS]; / length of WEP keys,
    pub channel: c_int,
    pub iw_mode: c_int,
    pub bssid: [u8; ETH_ALEN],
    pub essid: [u8; IW_ESSID_MAX_SIZE],
    pub essid_size: c_int,
    pub radio_on: c_int,
    pub promisc: c_int,
    pub /: *mut *mut int preamble_type; / 0 - long, 1 - short, 2 - auto,
    pub /: *mut *mut int auth_mode; / authentication type: 0 open, 1 shared key,
    pub /: *mut *mut int txrate; / 0,1,2,3 = 1,2,5.5,11 Mbps, 4 is auto,
    pub /: *mut *mut int frag_threshold; / threshold for fragmentation of tx packets,
    pub /: *mut *mut int rts_threshold; / threshold for RTS mechanism,
    pub short_retry_limit: c_int,
    pub /: *mut *mut int scan_min_time; / scan min channel time,
    pub /: *mut *mut int scan_max_time; / scan max channel time,
    pub /: *mut *mut int scan_mode; / SCAN_TYPE_ACTIVE, SCAN_TYPE_PASSIVE,
    pub /: *mut *mut int scan_need_any; / if set, need to scan for any ESSID,
    pub /: *mut *mut bool scanning; / if set, the scan is running,
    pub /: *mut *mut u16 assoc_id; / current association ID, if associated,
    pub /: *mut *mut u8 pm_mode; / power management mode,
    pub /: *mut *mut u32 pm_period; / power management period in microseconds,
    pub /: *const *const *const reg_domain domain; / reg domain description,
// These fields contain HW config provided by the device (not all of
// these fields are used by all board types)
    pub mac_addr: [u8; ETH_ALEN],
    pub regulatory_domain: u8,
    pub card_config: at76_card_config,
    pub board_type: board_type,
    pub fw_version: mib_fw_version,
    pub device_unplugged:1: c_uint,
    pub netdev_registered:1: c_uint,
    pub /: *mut *mut set_mib_buffer mib_buf; / global buffer for set_mib calls,
    pub /: *mut *mut int beacon_period; / period of mgmt beacons, Kus,
    pub hw: *mut ieee80211_hw,
    pub mac80211_registered: c_int,
}

pub const AT76_SUPPORTED_FILTERS: c_int = 0;

pub const DEF_RTS_THRESHOLD: c_int = 1536;
pub const DEF_FRAG_THRESHOLD: c_int = 1536;
pub const DEF_SHORT_RETRY_LIMIT: c_int = 8;
pub const DEF_CHANNEL: c_int = 10;
pub const DEF_SCAN_MIN_TIME: c_int = 10;
pub const DEF_SCAN_MAX_TIME: c_int = 120;
// the max padding size for tx in bytes (see calc_padding)
pub const MAX_PADDING_SIZE: c_int = 53;
