//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ti/wl1251/cmd.h
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
// This file is part of wl1251
//
// Copyright (c) 1998-2007 Texas Instruments Incorporated
// Copyright (C) 2008 Nokia Corporation
//

extern "C" {
    pub fn wl1251_cmd_send(wl: *mut wl1251, type: u16, buf: *mut c_void, buf_len: usize) -> c_int;
}
extern "C" {
    pub fn wl1251_cmd_interrogate(wl: *mut wl1251, id: u16, buf: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn wl1251_cmd_configure(wl: *mut wl1251, id: u16, buf: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn wl1251_cmd_data_path_rx(wl: *mut wl1251, channel: u8, enable: bool) -> c_int;
}
extern "C" {
    pub fn wl1251_cmd_data_path_tx(wl: *mut wl1251, channel: u8, enable: bool) -> c_int;
}
extern "C" {
    pub fn wl1251_cmd_ps_mode(wl: *mut wl1251, ps_mode: u8) -> c_int;
}
extern "C" {
    pub fn wl1251_cmd_trigger_scan_to(wl: *mut wl1251, timeout: u32) -> c_int;
}
// unit ms
pub const WL1251_COMMAND_TIMEOUT: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_commands {
    CMD_RESET           = 0,
    CMD_INTERROGATE     = 1,    /*use this to read information elements*/
    CMD_CONFIGURE       = 2,    /*use this to write information elements*/
    CMD_ENABLE_RX       = 3,
    CMD_ENABLE_TX       = 4,
    CMD_DISABLE_RX      = 5,
    CMD_DISABLE_TX      = 6,
    CMD_SCAN            = 8,
    CMD_STOP_SCAN       = 9,
    CMD_VBM             = 10,
    CMD_START_JOIN      = 11,
    CMD_SET_KEYS        = 12,
    CMD_READ_MEMORY     = 13,
    CMD_WRITE_MEMORY    = 14,
    CMD_BEACON          = 19,
    CMD_PROBE_RESP      = 20,
    CMD_NULL_DATA       = 21,
    CMD_PROBE_REQ       = 22,
    CMD_TEST            = 23,
    CMD_RADIO_CALIBRATE     = 25,   /* OBSOLETE */
    CMD_ENABLE_RX_PATH      = 27,   /* OBSOLETE */
    CMD_NOISE_HIST      = 28,
    CMD_RX_RESET        = 29,
    CMD_PS_POLL         = 30,
    CMD_QOS_NULL_DATA   = 31,
    CMD_LNA_CONTROL     = 32,
    CMD_SET_BCN_MODE    = 33,
    CMD_MEASUREMENT      = 34,
    CMD_STOP_MEASUREMENT = 35,
    CMD_DISCONNECT       = 36,
    CMD_SET_PS_MODE      = 37,
    CMD_CHANNEL_SWITCH   = 38,
    CMD_STOP_CHANNEL_SWICTH = 39,
    CMD_AP_DISCOVERY     = 40,
    CMD_STOP_AP_DISCOVERY = 41,
    CMD_SPS_SCAN = 42,
    CMD_STOP_SPS_SCAN = 43,
    CMD_HEALTH_CHECK     = 45,
    CMD_DEBUG            = 46,
    CMD_TRIGGER_SCAN_TO  = 47,

    NUM_COMMANDS,
    MAX_COMMAND_ID = 0xFFFF,
}

pub const MAX_CMD_PARAMS: c_int = 572;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_cmd_header {
    pub id: u16,
    pub status: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_command {
    pub header: wl1251_cmd_header,
    pub parameters: [u8; MAX_CMD_PARAMS],
    pub __packed: },
}

//
// CMD_READ_MEMORY
//
// The host issues this command to read the WiLink device memory/registers.
//
// Note: The Base Band address has special handling (16 bits registers and
// addresses). For more information, see the hardware specification.
//
// CMD_WRITE_MEMORY
//
// The host issues this command to write the WiLink device memory/registers.
//
// The Base Band address has special handling (16 bits registers and
// addresses). For more information, see the hardware specification.
//
pub const MAX_READ_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_read_write_memory {
    pub header: wl1251_cmd_header,
// The address of the memory to read from or write to.
    pub addr: u32,
// The amount of data in bytes to read from or write to the WiLink
// device.
    pub size: u32,
// The actual value read from or written to the Wilink. The source
    pub value: [u8; MAX_READ_SIZE],
    pub __packed: },
pub const CMDMBOX_HEADER_LEN: c_int = 4;
pub const CMDMBOX_INFO_ELEM_HEADER_LEN: c_int = 4;
pub const WL1251_SCAN_OPT_PASSIVE: c_int = 1;
pub const WL1251_SCAN_OPT_5GHZ_BAND: c_int = 2;
pub const WL1251_SCAN_OPT_TRIGGERD_SCAN: c_int = 4;
pub const WL1251_SCAN_OPT_PRIORITY_HIGH: c_int = 8;
pub const WL1251_SCAN_MIN_DURATION: c_int = 30000;
pub const WL1251_SCAN_MAX_DURATION: c_int = 60000;
pub const WL1251_SCAN_NUM_PROBES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_scan_parameters {
    pub rx_config_options: __le32,
    pub rx_filter_options: __le32,
//
// Scan options:
// bit 0: When this bit is set, passive scan.
// bit 1: Band, when this bit is set we scan
// in the 5Ghz band.
// bit 2: voice mode, 0 for normal scan.
// bit 3: scan priority, 1 for high priority.
//
    pub scan_options: __le16,
// Number of channels to scan
    pub num_channels: u8,
// Number opf probe requests to send, per channel
    pub num_probe_requests: u8,
// Rate and modulation for probe requests
    pub tx_rate: __le16,
    pub tid_trigger: u8,
    pub ssid_len: u8,
    pub ssid: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_scan_ch_parameters {
    pub /: *mut *mut __le32 min_duration; / in TU,
    pub /: *mut *mut __le32 max_duration; / in TU,
    pub bssid_lsb: u32,
    pub bssid_msb: u16,
//
// bits 0-3: Early termination count.
// bits 4-5: Early termination condition.
//
    pub early_termination: u8,
    pub tx_power_att: u8,
    pub channel: u8,
    pub pad: [u8; 3],
    pub __packed: },
// SCAN parameters
pub const SCAN_MAX_NUM_OF_CHANNELS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_cmd_scan {
    pub header: wl1251_cmd_header,
    pub params: wl1251_scan_parameters,
    pub channels: [wl1251_scan_ch_parameters; SCAN_MAX_NUM_OF_CHANNELS],
    pub __packed: },
}

pub const JOIN_CMD_CTRL_TX_FLUSH: c_uint = 0x80 /* Firmware flushes all Tx */;
pub const JOIN_CMD_CTRL_EARLY_WAKEUP_ENABLE: c_uint = 0x01 /* Early wakeup time */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_join {
    pub header: wl1251_cmd_header,
    pub bssid_lsb: u32,
    pub bssid_msb: u16,
    pub /: *mut *mut u16 beacon_interval; / in TBTTs,
    pub rx_config_options: u32,
    pub rx_filter_options: u32,
//
// The target uses this field to determine the rate at
// which to transmit control frame responses (such as
// ACK or CTS frames).
//
    pub basic_rate_set: u16,
    pub dtim_interval: u8,
    pub /: *mut *mut u8 tx_ctrl_frame_rate; / OBSOLETE,
    pub /: *mut *mut u8 tx_ctrl_frame_mod; / OBSOLETE,
//
// bits 0-2: This bitwise field specifies the type
// of BSS to start or join (BSS_TYPE_*).
// bit 4: Band - The radio band in which to join
// or start.
// 0 - 2.4GHz band
// 1 - 5GHz band
// bits 3, 5-7: Reserved
//
    pub bss_type: u8,
    pub channel: u8,
    pub ssid_len: u8,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub /: *mut *mut *mut u8 ctrl; / JOIN_CMD_CTRL_,
    pub /: *mut *mut u8 tx_mgt_frame_rate; / OBSOLETE,
    pub /: *mut *mut u8 tx_mgt_frame_mod; / OBSOLETE,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_enabledisable_path {
    pub header: wl1251_cmd_header,
    pub channel: u8,
    pub padding: [u8; 3],
    pub __packed: },
pub const WL1251_MAX_TEMPLATE_SIZE: c_int = 300;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_cmd_packet_template {
    pub header: wl1251_cmd_header,
    pub size: __le16,
    pub data: [u8; ],
    pub __packed: },
pub const TIM_ELE_ID: c_int = 5;
pub const PARTIAL_VBM_MAX: c_int = 251;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_tim {
    pub identity: u8,
    pub length: u8,
    pub dtim_count: u8,
    pub dtim_period: u8,
    pub bitmap_ctrl: u8,
    pub /: *mut *mut u8 pvb_field[PARTIAL_VBM_MAX]; / Partial Virtual Bitmap,
    pub __packed: },
// Virtual Bit Map update
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_cmd_vbm_update {
    pub header: wl1251_cmd_header,
    pub len: __le16,
    pub padding: [u8; 2],
    pub tim: wl1251_tim,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_cmd_ps_mode {
    CHIP_ACTIVE_MODE,
    CHIP_POWER_SAVE_MODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_cmd_ps_params {
    pub header: wl1251_cmd_header,
    pub /: *mut *mut *mut u8 ps_mode; / STATION_,
    pub /: *mut *mut u8 send_null_data; / Do we have to send NULL data packet ?,
    pub /: *mut *mut u8 retries; / Number of retires for the initial NULL data packet,
//
// TUs during which the target stays awake after switching
// to power save mode.
//
    pub hang_over_period: u8,
    pub null_data_rate: u16,
    pub pad: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_cmd_trigger_scan_to {
    pub header: wl1251_cmd_header,
    pub timeout: u32,
    pub __packed: },
// HW encryption keys
pub const NUM_ACCESS_CATEGORIES_COPY: c_int = 4;
pub const MAX_KEY_SIZE: c_int = 32;
// When set, disable HW encryption
pub const DF_ENCRYPTION_DISABLE: c_uint = 0x01;
// When set, disable HW decryption
pub const DF_SNIFF_MODE_ENABLE: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_cmd_key_action {
    KEY_ADD_OR_REPLACE = 1,
    KEY_REMOVE         = 2,
    KEY_SET_ID         = 3,
    MAX_KEY_ACTION     = 0xffff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wl1251_cmd_key_type {
    KEY_WEP_DEFAULT       = 0,
    KEY_WEP_ADDR          = 1,
    KEY_AES_GROUP         = 4,
    KEY_AES_PAIRWISE      = 5,
    KEY_WEP_GROUP         = 6,
    KEY_TKIP_MIC_GROUP    = 10,
    KEY_TKIP_MIC_PAIRWISE = 11,
}

//
// key_type_e   key size    key format
// ----------   ---------   ----------
// 0x00         5, 13, 29   Key data
// 0x01         5, 13, 29   Key data
// 0x04         16          16 bytes of key data
// 0x05         16          16 bytes of key data
// 0x0a         32          16 bytes of TKIP key data
// 8 bytes of RX MIC key data
// 8 bytes of TX MIC key data
// 0x0b         32          16 bytes of TKIP key data
// 8 bytes of RX MIC key data
// 8 bytes of TX MIC key data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl1251_cmd_set_keys {
    pub header: wl1251_cmd_header,
// Ignored for default WEP key
    pub addr: [u8; ETH_ALEN],
// key_action_e
    pub key_action: u16,
    pub reserved_1: u16,
// key size in bytes
    pub key_size: u8,
// key_type_e
    pub key_type: u8,
    pub ssid_profile: u8,
//
// TKIP, AES: frame's key id field.
// For WEP default key: key id;
//
    pub id: u8,
    pub reserved_2: [u8; 6],
    pub key: [u8; MAX_KEY_SIZE],
    pub ac_seq_num16: [u16; NUM_ACCESS_CATEGORIES_COPY],
    pub ac_seq_num32: [u32; NUM_ACCESS_CATEGORIES_COPY],
    pub __packed: },
