//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intersil/p54/lmac.h
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
// LMAC Interface specific definitions for mac80211 Prism54 drivers
//
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
// Copyright (c) 2007 - 2009, Christian Lamparter <chunkeey@web.de>
//
// Based on:
// - the islsm (softmac prism54) driver, which is:
// Copyright 2004-2006 Jean-Baptiste Note <jbnote@gmail.com>, et al.
//
// - LMAC API interface header file for STLC4560 (lmac_longbow.h)
// Copyright (C) 2007 Conexant Systems, Inc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p54_control_frame_types {
    P54_CONTROL_TYPE_SETUP = 0,
    P54_CONTROL_TYPE_SCAN,
    P54_CONTROL_TYPE_TRAP,
    P54_CONTROL_TYPE_DCFINIT,
    P54_CONTROL_TYPE_RX_KEYCACHE,
    P54_CONTROL_TYPE_TIM,
    P54_CONTROL_TYPE_PSM,
    P54_CONTROL_TYPE_TXCANCEL,
    P54_CONTROL_TYPE_TXDONE,
    P54_CONTROL_TYPE_BURST,
    P54_CONTROL_TYPE_STAT_READBACK,
    P54_CONTROL_TYPE_BBP,
    P54_CONTROL_TYPE_EEPROM_READBACK,
    P54_CONTROL_TYPE_LED,
    P54_CONTROL_TYPE_GPIO,
    P54_CONTROL_TYPE_TIMER,
    P54_CONTROL_TYPE_MODULATION,
    P54_CONTROL_TYPE_SYNTH_CONFIG,
    P54_CONTROL_TYPE_DETECTOR_VALUE,
    P54_CONTROL_TYPE_XBOW_SYNTH_CFG,
    P54_CONTROL_TYPE_CCE_QUIET,
    P54_CONTROL_TYPE_PSM_STA_UNLOCK,
    P54_CONTROL_TYPE_PCS,
    P54_CONTROL_TYPE_BT_BALANCER = 28,
    P54_CONTROL_TYPE_GROUP_ADDRESS_TABLE = 30,
    P54_CONTROL_TYPE_ARPTABLE = 31,
    P54_CONTROL_TYPE_BT_OPTIONS = 35,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_hdr {
    pub flags: __le16,
    pub len: __le16,
    pub req_id: __le32,
    pub /: *mut *mut __le16 type; / enum p54_control_frame_types,
    pub rts_tries: u8,
    pub tries: u8,
    pub data: [u8; ],
    pub __packed: },

//
// shared interface ID definitions
// The interface ID is a unique identification of a specific interface.
// The following values are reserved: 0x0000, 0x0002, 0x0012, 0x0014, 0x0015
//
pub const IF_ID_ISL36356A: c_uint = 0x0001	/* ISL36356A <-> Firmware */;
pub const IF_ID_MVC: c_uint = 0x0003	/* MAC Virtual Coprocessor */;
pub const IF_ID_DEBUG: c_uint = 0x0008	/* PolDebug Interface */;
pub const IF_ID_PRODUCT: c_uint = 0x0009;
pub const IF_ID_OEM: c_uint = 0x000a;
pub const IF_ID_PCI3877: c_uint = 0x000b	/* 3877 <-> Host PCI */;
pub const IF_ID_ISL37704C: c_uint = 0x000c	/* ISL37704C <-> Fw */;
pub const IF_ID_ISL39000: c_uint = 0x000f	/* ISL39000 <-> Fw */;
pub const IF_ID_ISL39300A: c_uint = 0x0010	/* ISL39300A <-> Fw */;
pub const IF_ID_ISL37700_UAP: c_uint = 0x0016	/* ISL37700 uAP Fw <-> Fw */;
pub const IF_ID_ISL39000_UAP: c_uint = 0x0017	/* ISL39000 uAP Fw <-> Fw */;
pub const IF_ID_LMAC: c_uint = 0x001a	/* Interface exposed by LMAC */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exp_if {
    pub role: __le16,
    pub if_id: __le16,
    pub variant: __le16,
    pub btm_compat: __le16,
    pub top_compat: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dep_if {
    pub role: __le16,
    pub if_id: __le16,
    pub variant: __le16,
    pub __packed: },
// driver <-> lmac definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_eeprom_lm86 {
    pub offset: __le16,
    pub len: __le16,
    pub data: [u8; 0],
    pub v1: } __packed,
    pub offset: __le32,
    pub len: __le16,
    pub magic2: u8,
    pub pad: u8,
    pub magic: [u8; 4],
    pub data: [u8; 0],
    pub v2: } __packed,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p54_rx_decrypt_status {
    P54_DECRYPT_NONE = 0,
    P54_DECRYPT_OK,
    P54_DECRYPT_NOKEY,
    P54_DECRYPT_NOMICHAEL,
    P54_DECRYPT_NOCKIPMIC,
    P54_DECRYPT_FAIL_WEP,
    P54_DECRYPT_FAIL_TKIP,
    P54_DECRYPT_FAIL_MICHAEL,
    P54_DECRYPT_FAIL_CKIPKP,
    P54_DECRYPT_FAIL_CKIPMIC,
    P54_DECRYPT_FAIL_AESCCMP
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_rx_data {
    pub flags: __le16,
    pub len: __le16,
    pub freq: __le16,
    pub antenna: u8,
    pub rate: u8,
    pub rssi: u8,
    pub quality: u8,
    pub decrypt_status: u8,
    pub rssi_raw: u8,
    pub tsf32: __le32,
    pub unalloc0: __le32,
    pub align: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p54_trap_type {
    P54_TRAP_SCAN = 0,
    P54_TRAP_TIMER,
    P54_TRAP_BEACON_TX,
    P54_TRAP_FAA_RADIO_ON,
    P54_TRAP_FAA_RADIO_OFF,
    P54_TRAP_RADAR,
    P54_TRAP_NO_BEACON,
    P54_TRAP_TBTT,
    P54_TRAP_SCO_ENTER,
    P54_TRAP_SCO_EXIT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_trap {
    pub event: __le16,
    pub frequency: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p54_frame_sent_status {
    P54_TX_OK = 0,
    P54_TX_FAILED,
    P54_TX_PSM,
    P54_TX_PSM_CANCELLED = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_frame_sent {
    pub status: u8,
    pub tries: u8,
    pub ack_rssi: u8,
    pub quality: u8,
    pub seq: __le16,
    pub antenna: u8,
    pub padding: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p54_tx_data_crypt {
    P54_CRYPTO_NONE = 0,
    P54_CRYPTO_WEP,
    P54_CRYPTO_TKIP,
    P54_CRYPTO_TKIPMICHAEL,
    P54_CRYPTO_CCX_WEPMIC,
    P54_CRYPTO_CCX_KPMIC,
    P54_CRYPTO_CCX_KP,
    P54_CRYPTO_AESCCMP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum p54_tx_data_queue {
    P54_QUEUE_BEACON	= 0,
    P54_QUEUE_FWSCAN	= 1,
    P54_QUEUE_MGMT		= 2,
    P54_QUEUE_CAB		= 3,
    P54_QUEUE_DATA		= 4,

    P54_QUEUE_AC_NUM	= 4,
    P54_QUEUE_AC_VO		= 4,
    P54_QUEUE_AC_VI		= 5,
    P54_QUEUE_AC_BE		= 6,
    P54_QUEUE_AC_BK		= 7,

// keep last
    P54_QUEUE_NUM		= 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_tx_data {
    pub rateset: [u8; 8],
    pub rts_rate_idx: u8,
    pub crypt_offset: u8,
    pub key_type: u8,
    pub key_len: u8,
    pub key: [u8; 16],
    pub hw_queue: u8,
    pub backlog: u8,
    pub durations: [__le16; 4],
    pub tx_antenna: u8,
    pub cts_rate: u8,
    pub output_power: __le16,
    pub longbow: } __packed,
    pub output_power: u8,
    pub cts_rate: u8,
    pub unalloc: u8,
    pub normal: } __packed,
    pub __packed: },
    pub unalloc2: [u8; 2],
    pub align: [u8; ],
    pub __packed: },
// unit is ms
pub const P54_TX_FRAME_LIFETIME: c_int = 2000;
pub const P54_TX_TIMEOUT: c_int = 4000;
pub const P54_STATISTICS_UPDATE: c_int = 5000;
pub const P54_FILTER_TYPE_NONE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_setup_mac {
    pub mac_mode: __le16,
    pub mac_addr: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub rx_antenna: u8,
    pub rx_align: u8,
    pub basic_rate_mask: __le32,
    pub rts_rates: [u8; 8],
    pub rx_addr: __le32,
    pub max_rx: __le16,
    pub rxhw: __le16,
    pub wakeup_timer: __le16,
    pub unalloc0: __le16,
    pub v1: } __packed,
    pub rx_addr: __le32,
    pub max_rx: __le16,
    pub rxhw: __le16,
    pub timer: __le16,
    pub truncate: __le16,
    pub basic_rate_mask: __le32,
    pub sbss_offset: u8,
    pub mcast_window: u8,
    pub rx_rssi_threshold: u8,
    pub rx_ed_threshold: u8,
    pub ref_clock: __le32,
    pub lpf_bandwidth: __le16,
    pub osc_start_delay: __le16,
    pub v2: } __packed,
    pub __packed: },
    pub __packed: },
pub const P54_SETUP_V1_LEN: c_int = 40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_scan_head {
    pub mode: __le16,
    pub dwell: __le16,
    pub scan_params: [u8; 20],
    pub freq: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_pa_curve_data_sample {
    pub rf_power: u8,
    pub pa_detector: u8,
    pub data_barker: u8,
    pub data_bpsk: u8,
    pub data_qpsk: u8,
    pub data_16qam: u8,
    pub data_64qam: u8,
    pub padding: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_scan_body {
    pub pa_points_per_curve: u8,
    pub val_barker: u8,
    pub val_bpsk: u8,
    pub val_qpsk: u8,
    pub val_16qam: u8,
    pub val_64qam: u8,
    pub curve_data: [p54_pa_curve_data_sample; 8],
    pub dup_bpsk: u8,
    pub dup_qpsk: u8,
    pub dup_16qam: u8,
    pub dup_64qam: u8,
    pub __packed: },
//
// Warning: Longbow's structures are bogus.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_channel_output_limit_longbow {
    pub rf_power_points: [__le16; 12],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_pa_curve_data_sample_longbow {
    pub rf_power: __le16,
    pub pa_detector: __le16,
    pub data: [__le16; 4],
    pub __packed: } points[3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_scan_body_longbow {
    pub power_limits: p54_channel_output_limit_longbow,
    pub curve_data: [p54_pa_curve_data_sample_longbow; 8],
    pub /: *mut *mut __le16 unkn[6]; / maybe more power_limits or rate_mask,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union p54_scan_body_union {
    pub normal: p54_scan_body,
    pub longbow: p54_scan_body_longbow,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_scan_tail_rate {
    pub basic_rate_mask: __le32,
    pub rts_rates: [u8; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_led {
    pub flags: __le16,
    pub mask: [__le16; 2],
    pub delay: [__le16; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_edcf {
    pub flags: u8,
    pub slottime: u8,
    pub sifs: u8,
    pub eofpad: u8,
    pub queue: [p54_edcf_queue_param; 8],
    pub mapping: [u8; 4],
    pub frameburst: __le16,
    pub round_trip_delay: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_statistics {
    pub rx_success: __le32,
    pub rx_bad_fcs: __le32,
    pub rx_abort: __le32,
    pub rx_abort_phy: __le32,
    pub rts_success: __le32,
    pub rts_fail: __le32,
    pub tsf32: __le32,
    pub airtime: __le32,
    pub noise: __le32,
    pub sample_noise: [__le32; 8],
    pub sample_cca: __le32,
    pub sample_tx: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_xbow_synth {
    pub magic1: __le16,
    pub magic2: __le16,
    pub freq: __le16,
    pub padding: [u32; 5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_timer {
    pub interval: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_keycache {
    pub entry: u8,
    pub key_id: u8,
    pub mac: [u8; ETH_ALEN],
    pub padding: [u8; 2],
    pub key_type: u8,
    pub key_len: u8,
    pub key: [u8; 24],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_burst {
    pub flags: u8,
    pub queue: u8,
    pub backlog: u8,
    pub pad: u8,
    pub durations: [__le16; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_psm_interval {
    pub interval: __le16,
    pub periods: __le16,
    pub __packed: },
pub const P54_PSM_CAM: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_psm {
    pub mode: __le16,
    pub aid: __le16,
    pub intervals: [p54_psm_interval; 4],
    pub beacon_rssi_skip_max: u8,
    pub rssi_delta_threshold: u8,
    pub nr: u8,
    pub exclude: [u8; 1],
    pub __packed: },
pub const MC_FILTER_ADDRESS_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_group_address_table {
    pub filter_enable: __le16,
    pub num_address: __le16,
    pub mac_list: [u8; MC_FILTER_ADDRESS_NUM][ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_txcancel {
    pub req_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_sta_unlock {
    pub addr: [u8; ETH_ALEN],
    pub padding: u16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_tim {
    pub count: u8,
    pub padding: [u8; 3],
    pub entry: [__le16; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_cce_quiet {
    pub period: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_bt_balancer {
    pub prio_thresh: __le16,
    pub acl_thresh: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p54_arp_table {
    pub filter_enable: __le16,
    pub ipv4_addr: [u8; 4],
    pub __packed: },
// LED control
    pub priv): *mut int p54_set_leds(struct p54_common,
    pub priv): *mut int p54_init_leds(struct p54_common,
    pub priv): *mut void p54_unregister_leds(struct p54_common,
// xmit functions
    pub skb): *mut sk_buff,
    pub req_id): *mut *mut int p54_tx_cancel(struct p54_common priv, __le32,
    pub skb): *mut *mut void p54_tx(struct p54_common priv, struct sk_buff,
// synth/phy configuration
    pub priv): *mut int p54_init_xbow_synth(struct p54_common,
    pub dwell): *mut *mut int p54_scan(struct p54_common priv, u16 mode, u16,
// MAC
    pub addr): *mut *mut int p54_sta_unlock(struct p54_common priv, u8,
    pub set): *mut *mut int p54_update_beacon_tim(struct p54_common priv, u16 aid, bool,
    pub priv): *mut int p54_setup_mac(struct p54_common,
    pub priv): *mut int p54_set_ps(struct p54_common,
    pub priv): *mut int p54_fetch_statistics(struct p54_common,
    pub priv): *mut int p54_set_groupfilter(struct p54_common,
// e/v DCF setup
    pub priv): *mut int p54_set_edcf(struct p54_common,
// cryptographic engine
    pub key): *mut *mut *mut u8 idx, u8 len, u8 addr, u8,
// eeprom
    pub len): u16 offset, u16,
    pub freq): *const *const *const p54_rssi_db_entry p54_rssi_find(p54_common p, u16,
// utility
    pub ie): *mut *mut *mut u8 p54_find_ie(struct sk_buff skb, u8,
