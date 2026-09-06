//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/toshiba/ps3_gelic_wireless.h
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
// PS3 gelic network driver.
//
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// Copyright 2007 Sony Corporation
//

// return value from  GELIC_LV1_GET_WLAN_EVENT netcontrol
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_lv1_wl_event {
    GELIC_LV1_WL_EVENT_DEVICE_READY   = 0x01, /* Eurus ready */
    GELIC_LV1_WL_EVENT_SCAN_COMPLETED = 0x02, /* Scan has completed */
    GELIC_LV1_WL_EVENT_DEAUTH         = 0x04, /* Deauthed by the AP */
    GELIC_LV1_WL_EVENT_BEACON_LOST    = 0x08, /* Beacon lost detected */
    GELIC_LV1_WL_EVENT_CONNECTED      = 0x10, /* Connected to AP */
    GELIC_LV1_WL_EVENT_WPA_CONNECTED  = 0x20, /* WPA connection */
    GELIC_LV1_WL_EVENT_WPA_ERROR      = 0x40, /* MIC error */
}

// arguments for GELIC_LV1_POST_WLAN_COMMAND netcontrol
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_command {
    GELIC_EURUS_CMD_ASSOC		=  1, /* association start */
    GELIC_EURUS_CMD_DISASSOC	=  2, /* disassociate      */
    GELIC_EURUS_CMD_START_SCAN	=  3, /* scan start        */
    GELIC_EURUS_CMD_GET_SCAN	=  4, /* get scan result   */
    GELIC_EURUS_CMD_SET_COMMON_CFG	=  5, /* set common config */
    GELIC_EURUS_CMD_GET_COMMON_CFG	=  6, /* set common config */
    GELIC_EURUS_CMD_SET_WEP_CFG	=  7, /* set WEP config    */
    GELIC_EURUS_CMD_GET_WEP_CFG	=  8, /* get WEP config    */
    GELIC_EURUS_CMD_SET_WPA_CFG	=  9, /* set WPA config    */
    GELIC_EURUS_CMD_GET_WPA_CFG	= 10, /* get WPA config    */
    GELIC_EURUS_CMD_GET_RSSI_CFG	= 11, /* get RSSI info.    */
    GELIC_EURUS_CMD_MAX_INDEX
}

// for GELIC_EURUS_CMD_COMMON_CFG
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_bss_type {
    GELIC_EURUS_BSS_INFRA = 0,
    GELIC_EURUS_BSS_ADHOC = 1, /* not supported */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_auth_method {
    GELIC_EURUS_AUTH_OPEN = 0, /* FIXME: WLAN_AUTH_OPEN */
    GELIC_EURUS_AUTH_SHARED = 1, /* not supported */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_opmode {
    GELIC_EURUS_OPMODE_11BG = 0, /* 802.11b/g */
    GELIC_EURUS_OPMODE_11B = 1, /* 802.11b only */
    GELIC_EURUS_OPMODE_11G = 2, /* 802.11g only */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_eurus_common_cfg {
// all fields are big endian
    pub scan_index: u16,
    pub /: *mut *mut u16 bss_type; / infra or adhoc,
    pub /: *mut *mut u16 auth_method; / shared key or open,
    pub /: *mut *mut u16 op_mode; / B/G,
    pub __packed: },
// for GELIC_EURUS_CMD_WEP_CFG
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_wep_security {
    GELIC_EURUS_WEP_SEC_NONE	= 0,
    GELIC_EURUS_WEP_SEC_40BIT	= 1,
    GELIC_EURUS_WEP_SEC_104BIT	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_eurus_wep_cfg {
// all fields are big endian
    pub security: u16,
    pub key: [u8; 4][16],
    pub __packed: },
// for GELIC_EURUS_CMD_WPA_CFG
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_wpa_security {
    GELIC_EURUS_WPA_SEC_NONE		= 0x0000,
// group=TKIP, pairwise=TKIP
    GELIC_EURUS_WPA_SEC_WPA_TKIP_TKIP	= 0x0001,
// group=AES, pairwise=AES
    GELIC_EURUS_WPA_SEC_WPA_AES_AES		= 0x0002,
// group=TKIP, pairwise=TKIP
    GELIC_EURUS_WPA_SEC_WPA2_TKIP_TKIP	= 0x0004,
// group=AES, pairwise=AES
    GELIC_EURUS_WPA_SEC_WPA2_AES_AES	= 0x0008,
// group=TKIP, pairwise=AES
    GELIC_EURUS_WPA_SEC_WPA_TKIP_AES	= 0x0010,
// group=TKIP, pairwise=AES
    GELIC_EURUS_WPA_SEC_WPA2_TKIP_AES	= 0x0020,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_wpa_psk_type {
    GELIC_EURUS_WPA_PSK_PASSPHRASE	= 0, /* passphrase string   */
    GELIC_EURUS_WPA_PSK_BIN		= 1, /* 32 bytes binary key */
}

pub const GELIC_WL_EURUS_PSK_MAX_LEN: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_eurus_wpa_cfg {
// all fields are big endian
    pub security: u16,
    pub /: *mut *mut u16 psk_type; / psk key encoding type,
    pub /: *mut *mut u8 psk[GELIC_WL_EURUS_PSK_MAX_LEN]; / psk key; hex or passphrase,
    pub __packed: },
// for GELIC_EURUS_CMD_{START,GET}_SCAN
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_scan_capability {
    GELIC_EURUS_SCAN_CAP_ADHOC	= 0x0000,
    GELIC_EURUS_SCAN_CAP_INFRA	= 0x0001,
    GELIC_EURUS_SCAN_CAP_MASK	= 0x0001,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_scan_sec_type {
    GELIC_EURUS_SCAN_SEC_NONE	= 0x0000,
    GELIC_EURUS_SCAN_SEC_WEP	= 0x0100,
    GELIC_EURUS_SCAN_SEC_WPA	= 0x0200,
    GELIC_EURUS_SCAN_SEC_WPA2	= 0x0400,
    GELIC_EURUS_SCAN_SEC_MASK	= 0x0f00,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_scan_sec_wep_type {
    GELIC_EURUS_SCAN_SEC_WEP_UNKNOWN	= 0x0000,
    GELIC_EURUS_SCAN_SEC_WEP_40		= 0x0001,
    GELIC_EURUS_SCAN_SEC_WEP_104		= 0x0002,
    GELIC_EURUS_SCAN_SEC_WEP_MASK		= 0x0003,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_eurus_scan_sec_wpa_type {
    GELIC_EURUS_SCAN_SEC_WPA_UNKNOWN	= 0x0000,
    GELIC_EURUS_SCAN_SEC_WPA_TKIP		= 0x0001,
    GELIC_EURUS_SCAN_SEC_WPA_AES		= 0x0002,
    GELIC_EURUS_SCAN_SEC_WPA_MASK		= 0x0003,
}

//
// hw BSS information structure returned from GELIC_EURUS_CMD_GET_SCAN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_eurus_scan_info {
// all fields are big endian
    pub size: __be16,
    pub /: *mut *mut __be16 rssi; / percentage,
    pub /: *mut *mut __be16 channel; / channel number,
    pub /: *mut *mut __be16 beacon_period; / FIXME: in msec unit,
    pub capability: __be16,
    pub security: __be16,
    pub /: *mut *mut u8 bssid[8]; / last ETH_ALEN are valid. bssid[0],[1] are unused,
    pub /: *mut *mut u8 essid[32]; / IW_ESSID_MAX_SIZE,
    pub /: *mut *mut u8 rate[16]; / first 12 are valid,
    pub /: *mut *mut u8 ext_rate[16]; / first 16 are valid,
    pub reserved1: __be32,
    pub reserved2: __be32,
    pub reserved3: __be32,
    pub reserved4: __be32,
    pub /: *mut *mut u8 elements[]; / ie,
    pub __packed: },
// the hypervisor returns bbs up to 16

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_wl_scan_info {
    pub list: list_head,
    pub hwinfo: *mut gelic_eurus_scan_info,
    pub list: *mut *mut int valid; / set 1 if this entry was in latest scanned,
// from Eurus
    pub /: *mut *mut unsigned int eurus_index; / index in the Eurus list,
    pub /: *mut *mut unsigned long last_scanned; / acquired time,
    pub rate_len: c_uint,
    pub rate_ext_len: c_uint,
    pub essid_len: c_uint,
}

// for GELIC_EURUS_CMD_GET_RSSI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_eurus_rssi_info {
// big endian
    pub rssi: __be16,
    pub __packed: },
// for 'stat' member of gelic_wl_info
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_wl_info_status_bit {
    GELIC_WL_STAT_CONFIGURED,
    GELIC_WL_STAT_CH_INFO,   /* ch info acquired */
    GELIC_WL_STAT_ESSID_SET, /* ESSID specified by userspace */
    GELIC_WL_STAT_BSSID_SET, /* BSSID specified by userspace */
    GELIC_WL_STAT_WPA_PSK_SET, /* PMK specified by userspace */
    GELIC_WL_STAT_WPA_LEVEL_SET, /* WEP or WPA[2] selected */
}

// for 'scan_stat' member of gelic_wl_info
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_wl_scan_state {
// just initialized or get last scan result failed
    GELIC_WL_SCAN_STAT_INIT,
// scan request issued, accepted or chip is scanning
    GELIC_WL_SCAN_STAT_SCANNING,
// scan results retrieved
    GELIC_WL_SCAN_STAT_GOT_LIST,
}

// for 'cipher_method'
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_wl_cipher_method {
    GELIC_WL_CIPHER_NONE,
    GELIC_WL_CIPHER_WEP,
    GELIC_WL_CIPHER_TKIP,
    GELIC_WL_CIPHER_AES,
}

// for 'wpa_level'
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_wl_wpa_level {
    GELIC_WL_WPA_LEVEL_NONE,
    GELIC_WL_WPA_LEVEL_WPA,
    GELIC_WL_WPA_LEVEL_WPA2,
}

// for 'assoc_stat'
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gelic_wl_assoc_state {
    GELIC_WL_ASSOC_STAT_DISCONN,
    GELIC_WL_ASSOC_STAT_ASSOCIATING,
    GELIC_WL_ASSOC_STAT_ASSOCIATED,
}

// part of private data alloc_etherdev() allocated
pub const GELIC_WEP_KEYS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_wl_info {
// bss list
    pub scan_lock: mutex,
    pub network_list: list_head,
    pub network_free_list: list_head,
    pub networks: *mut gelic_wl_scan_info,
    pub /: *mut *mut unsigned long scan_age; / last scanned time,
    pub scan_stat: gelic_wl_scan_state,
    pub scan_done: completion,
// eurus command queue
    pub eurus_cmd_queue: *mut workqueue_struct,
    pub cmd_done_intr: completion,
// eurus event handling
    pub event_queue: *mut workqueue_struct,
    pub event_work: delayed_work,
// wl status bits
    pub stat: c_ulong,
    pub /: *mut *mut gelic_eurus_auth_method auth_method; / open/shared,
    pub group_cipher_method: gelic_wl_cipher_method,
    pub pairwise_cipher_method: gelic_wl_cipher_method,
    pub /: *mut *mut gelic_wl_wpa_level wpa_level; / wpa/wpa2,
// association handling
    pub assoc_stat_lock: mutex,
    pub assoc_work: delayed_work,
    pub assoc_stat: gelic_wl_assoc_state,
    pub assoc_done: completion,
    pub lock: spinlock_t,
    pub /: *mut *mut u16 ch_info; / available channels. bit0 = ch1,
// WEP keys
    pub key: [u8; GELIC_WEP_KEYS][IW_ENCODING_TOKEN_MAX],
    pub key_enabled: c_ulong,
    pub key_len: [c_uint; GELIC_WEP_KEYS],
    pub current_key: c_uint,
// WWPA PSK
    pub psk: [u8; GELIC_WL_EURUS_PSK_MAX_LEN],
    pub psk_type: gelic_eurus_wpa_psk_type,
    pub psk_len: c_uint,
    pub essid: [u8; IW_ESSID_MAX_SIZE],
    pub /: *mut *mut u8 bssid[ETH_ALEN]; / userland requested,
    pub /: *mut *mut u8 active_bssid[ETH_ALEN]; / associated bssid,
    pub essid_len: c_uint,
    pub iwstat: iw_statistics,
}

pub const GELIC_WL_BSS_MAX_ENT: c_int = 32;
pub const GELIC_WL_ASSOC_RETRY: c_int = 50;
extern "C" {
    pub fn container_of()wl: *mut (void, gelic_port: struct, _arg: priv) -> return;
}
extern "C" {
    pub fn port_priv(_arg: port) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gelic_eurus_cmd {
    pub work: work_struct,
    pub wl: *mut gelic_wl_info,
    pub /: *mut *mut unsigned int cmd; / command code,
    pub tag: u64,
    pub size: u64,
    pub buffer: *mut c_void,
    pub buf_size: c_uint,
    pub done: completion,
    pub status: c_int,
    pub cmd_status: u64,
}

// private ioctls to pass PSK

extern "C" {
    pub fn gelic_wl_driver_probe(card: *mut gelic_card) -> c_int;
}
extern "C" {
    pub fn gelic_wl_driver_remove(card: *mut gelic_card) -> c_int;
}
extern "C" {
    pub fn gelic_wl_interrupt(netdev: *mut net_device, status: u64);
}
