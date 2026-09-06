//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intersil/p54/p54.h
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
// Shared defines for all mac80211 Prism54 code
//
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
//
// Based on the islsm (softmac prism54) driver, which is:
// Copyright 2004-2006 Jean-Baptiste Note <jbnote@gmail.com>, et al.
//

pub const ISL38XX_DEV_FIRMWARE_ADDR: c_uint = 0x20000;
pub const BR_CODE_MIN: c_uint = 0x80000000;
pub const BR_CODE_COMPONENT_ID: c_uint = 0x80000001;
pub const BR_CODE_COMPONENT_VERSION: c_uint = 0x80000002;
pub const BR_CODE_DEPENDENT_IF: c_uint = 0x80000003;
pub const BR_CODE_EXPOSED_IF: c_uint = 0x80000004;
pub const BR_CODE_DESCR: c_uint = 0x80000101;
pub const BR_CODE_MAX: c_uint = 0x8FFFFFFF;
pub const BR_CODE_END_OF_BRA: c_uint = 0xFF0000FF;
pub const LEGACY_BR_CODE_END_OF_BRA: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootrec {
    pub code: __le32,
    pub len: __le32,
    pub data: [u32; 10],
    pub __packed: },
// Interface role definitions
pub const BR_INTERFACE_ROLE_SERVER: c_uint = 0x0000;
pub const BR_INTERFACE_ROLE_CLIENT: c_uint = 0x8000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootrec_desc {
    pub modes: __le16,
    pub flags: __le16,
    pub rx_start: __le32,
    pub rx_end: __le32,
    pub headroom: u8,
    pub tailroom: u8,
    pub tx_queues: u8,
    pub tx_depth: u8,
    pub privacy_caps: u8,
    pub rx_keycache_size: u8,
    pub time_size: u8,
    pub padding: u8,
    pub rates: [u8; 16],
    pub padding2: [u8; 4],
    pub rx_mtu: __le16,
    pub __packed: },
pub const FW_FMAC: c_uint = 0x464d4143;
pub const FW_LM86: c_uint = 0x4c4d3836;
pub const FW_LM87: c_uint = 0x4c4d3837;
pub const FW_LM20: c_uint = 0x4c4d3230;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootrec_comp_id {
    pub fw_variant: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootrec_comp_ver {
    pub fw_version: [c_char; 24],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootrec_end {
    pub crc: __le16,
    pub padding: [u8; 2],
    pub md5: [u8; 16],
    pub __packed: },
// provide 16 bytes for the transport back-end
pub const P54_TX_INFO_DATA_SIZE: c_int = 16;
// stored in ieee80211_tx_info's rate_driver_data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_tx_info {
    pub start_addr: u32,
    pub end_addr: u32,
    pub )]: *mut *mut void data[P54_TX_INFO_DATA_SIZE / sizeof(void,
    pub extra_len: u32,
}

pub const P54_MAX_CTRL_FRAME_LEN: c_uint = 0x1000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_edcf_queue_param {
    pub aifs: __le16,
    pub cwmin: __le16,
    pub cwmax: __le16,
    pub txop: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_rssi_db_entry {
    pub freq: u16,
    pub mul: i16,
    pub add: i16,
    pub longbow_unkn: i16,
    pub longbow_unk2: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_cal_database {
    pub entries: usize,
    pub entry_size: usize,
    pub offset: usize,
    pub len: usize,
    pub __counted_by(len): u8 data[],
}

pub const EEPROM_READBACK_LEN: c_uint = 0x3fc;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_state {
    FW_STATE_OFF,
    FW_STATE_BOOTING,
    FW_STATE_READY,
    FW_STATE_RESET,
    FW_STATE_RESETTING,
}

pub const P54_LED_MAX_NAME_LEN: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_led_dev {
    pub hw_dev: *mut ieee80211_hw,
    pub led_dev: led_classdev,
    pub 1]: char name[P54_LED_MAX_NAME_LEN +,
    pub toggled: c_uint,
    pub index: c_uint,
    pub registered: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_tx_queue_stats {
    pub len: c_uint,
    pub limit: c_uint,
    pub count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_common {
    pub hw: *mut ieee80211_hw,
    pub vif: *mut ieee80211_vif,
    pub skb): *mut *mut *mut void (tx)(struct ieee80211_hw dev, struct sk_buff,
    pub dev): *mut *mut int (open)(struct ieee80211_hw,
    pub dev): *mut *mut void (stop)(struct ieee80211_hw,
    pub tx_pending: sk_buff_head,
    pub tx_queue: sk_buff_head,
    pub conf_mutex: mutex,
    pub registered: bool,
// memory management (as seen by the firmware)
    pub rx_start: u32,
    pub rx_end: u32,
    pub rx_mtu: u16,
    pub headroom: u8,
    pub tailroom: u8,
// firmware/hardware info
    pub tx_hdr_len: c_uint,
    pub fw_var: c_uint,
    pub fw_interface: c_uint,
    pub version: u8,
// (e)DCF / QOS state
    pub use_short_slot: bool,
    pub tx_stats_lock: spinlock_t,
    pub tx_stats: [p54_tx_queue_stats; 8],
    pub qos_params: [p54_edcf_queue_param; 8],
// Radio data
    pub rxhw: u16,
    pub rx_diversity_mask: u8,
    pub tx_diversity_mask: u8,
    pub output_power: c_uint,
    pub cur_rssi: *mut p54_rssi_db_entry,
    pub curchan: *mut ieee80211_channel,
    pub survey: *mut survey_info,
    pub chan_num: c_uint,
    pub stat_comp: completion,
    pub update_stats: bool,
    pub timestamp: c_uint,
    pub cached_cca: c_uint,
    pub cached_tx: c_uint,
    pub cached_rssi: c_uint,
    pub active: u64,
    pub cca: u64,
    pub tx: u64,
    pub rssi: u64,
    pub survey_raw: },
    pub noise: c_int,
// calibration, output power limit and rssi<->dBm conversation data
    pub iq_autocal: *mut pda_iq_autocal_entry,
    pub iq_autocal_len: c_uint,
    pub curve_data: *mut p54_cal_database,
    pub output_limit: *mut p54_cal_database,
    pub rssi_db: *mut p54_cal_database,
    pub band_table: [*mut ieee80211_supported_band; NUM_NL80211_BANDS],
// BBP/MAC state
    pub mac_addr: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub mc_maclist: [u8; 4][ETH_ALEN],
    pub wakeup_timer: u16,
    pub filter_flags: c_uint,
    pub mc_maclist_num: c_int,
    pub mode: c_int,
    pub tsf_high32: u32 tsf_low32,,
    pub basic_rate_mask: u32,
    pub aid: u16,
    pub coverage_class: u8,
    pub phy_idle: bool,
    pub phy_ps: bool,
    pub powersave_override: bool,
    pub beacon_req_id: __le32,
    pub beacon_comp: completion,
// cryptographic engine information
    pub privacy_caps: u8,
    pub rx_keycache_size: u8,
    pub used_rxkeys: *mut c_ulong,
// LED management
    pub leds: [p54_led_dev; 4],
    pub led_work: delayed_work,

    pub /: *mut *mut u16 softled_state; / bit field of glowing LEDs,
// statistics
    pub stats: ieee80211_low_level_stats,
    pub work: delayed_work,
// eeprom handling
    pub eeprom: *mut c_void,
    pub eeprom_slice_size: usize,
    pub eeprom_comp: completion,
    pub eeprom_mutex: mutex,
}

// interfaces for the drivers
extern "C" {
    pub fn p54_rx(dev: *mut ieee80211_hw, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn p54_free_skb(dev: *mut ieee80211_hw, skb: *mut sk_buff);
}
extern "C" {
    pub fn p54_parse_firmware(dev: *mut ieee80211_hw, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn p54_parse_eeprom(dev: *mut ieee80211_hw, eeprom: *mut c_void, len: c_int) -> c_int;
}
extern "C" {
    pub fn p54_read_eeprom(dev: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn p54_register_common(dev: *mut ieee80211_hw, pdev: *mut device) -> c_int;
}
extern "C" {
    pub fn p54_free_common(dev: *mut ieee80211_hw);
}
extern "C" {
    pub fn p54_unregister_common(dev: *mut ieee80211_hw);
}
