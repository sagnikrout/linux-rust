//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76_connac.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2020 MediaTek Inc.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_pkt_type {
    PKT_TYPE_TXS,
    PKT_TYPE_TXRXV,
    PKT_TYPE_NORMAL,
    PKT_TYPE_RX_DUP_RFB,
    PKT_TYPE_RX_TMR,
    PKT_TYPE_RETRIEVE,
    PKT_TYPE_TXRX_NOTIFY,
    PKT_TYPE_RX_EVENT,
    PKT_TYPE_NORMAL_MCU,
    PKT_TYPE_RX_FW_MONITOR	= 0x0c,
    PKT_TYPE_TXRX_NOTIFY_V0	= 0x18,
}

pub const MT76_CONNAC_SCAN_IE_LEN: c_int = 600;
pub const MT76_CONNAC_MAX_NUM_SCHED_SCAN_INTERVAL: c_int = 10;

pub const MT76_CONNAC_MAX_SCHED_SCAN_SSID: c_int = 10;
pub const MT76_CONNAC_MAX_SCAN_MATCH: c_int = 16;
pub const MT76_CONNAC_MAX_WMM_SETS: c_int = 4;

pub const MT_USB_HDR_SIZE: c_int = 4;
pub const MT_USB_TAIL_SIZE: c_int = 4;

pub const MT_SDIO_TAIL_SIZE: c_int = 8;
pub const MT_SDIO_HDR_SIZE: c_int = 4;

// PCIE part
pub const PCIE_AER_UNC_STATUS_OFFSET: c_uint = 0x204;
pub const PCIE_AER_UNC_MASK_OFFSET: c_uint = 0x208;
pub const PCIE_AER_CO_STATUS_OFFSET: c_uint = 0x210;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_reg_map {
    pub phys: u32,
    pub maps: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_pm {
    pub enable:1: bool,
    pub enable_user:1: bool,
    pub ds_enable:1: bool,
    pub ds_enable_user:1: bool,
    pub suspended:1: bool,
    pub txq_lock: spinlock_t,
    pub wcid: *mut mt76_wcid,
    pub skb: *mut sk_buff,
    pub tx_q: [}; IEEE80211_NUM_ACS],
    pub wake_work: work_struct,
    pub wait: wait_queue_head_t,
    pub lock: spinlock_t,
    pub count: u32,
    pub wake: },
    pub mutex: mutex,
    pub ps_work: delayed_work,
    pub last_activity: c_ulong,
    pub idle_timeout: c_ulong,
    pub last_wake_event: c_ulong,
    pub awake_time: c_ulong,
    pub last_doze_event: c_ulong,
    pub doze_time: c_ulong,
    pub lp_wake: c_uint,
    pub stats: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_coredump {
    pub msg_list: sk_buff_head,
    pub work: delayed_work,
    pub last_activity: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_sta_key_conf {
    pub keyidx: i8,
    pub key: [u8; 16],
}

pub const MT_TXP_MAX_BUF_NUM: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_fw_txp {
    pub flags: __le16,
    pub token: __le16,
    pub bss_idx: u8,
    pub rept_wds_wcid: __le16,
    pub nbuf: u8,
    pub buf: [__le32; MT_TXP_MAX_BUF_NUM],
    pub len: [__le16; MT_TXP_MAX_BUF_NUM],
    pub __aligned(4): } __packed,
pub const MT_HW_TXP_MAX_MSDU_NUM: c_int = 4;
pub const MT_HW_TXP_MAX_BUF_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_txp_ptr {
    pub buf0: __le32,
    pub len0: __le16,
    pub len1: __le16,
    pub buf1: __le32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_hw_txp {
    pub msdu_id: [__le16; MT_HW_TXP_MAX_MSDU_NUM],
    pub 2]: mt76_connac_txp_ptr ptr[MT_HW_TXP_MAX_BUF_NUM /,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_txp_common {
    pub fw: mt76_connac_fw_txp,
    pub hw: mt76_connac_hw_txp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_tx_free {
    pub rx_byte_cnt: __le16,
    pub ctrl: __le16,
    pub txd: __le32,
    pub __aligned(4): } __packed,
    pub mt76_connac_wowlan_support: extern struct wiphy_wowlan_support,
    pub 0x7928: return mt76_chip(dev) == 0x7925 || mt76_chip(dev) == 0x7927 || mt76_chip(dev) ==,
    pub 0x7925: return mt76_chip(dev) ==,
    pub 0x7927: return mt76_chip(dev) ==,
    pub 0x7928: return mt76_chip(dev) ==,
    pub 0x7927: return mt76_chip(dev) ==,
    pub 0x7920: return mt76_chip(dev) ==,
    pub 0x7902: return mt76_chip(dev) ==,
    pub 0x7922: return mt76_chip(dev) ==,
    pub 0x7663: return mt76_chip(dev) ==,
    pub 0x7915: return mt76_chip(dev) ==,
    pub 0x7906: return mt76_chip(dev) ==,
    pub 0x7981: return mt76_chip(dev) ==,
    pub 0x7986: return mt76_chip(dev) ==,
    pub is_mt7986(dev): return is_mt7981(dev) ||,
    pub mt76_chip(dev): u16 chip =,
    pub 0x7991: return chip == 0x7990 || chip ==,
    pub 0x7992: return mt76_chip(dev) ==,
    pub 0x7993: return mt76_chip(dev) ==,
    pub is_mt7990(dev): return is_mt7996(dev) || is_mt7992(dev) ||,
    pub false: return,
    pub 0x7622: return mt76_chip(dev) ==,
    pub 0x7611: return mt76_chip(dev) == 0x7615 || mt76_chip(dev) ==,
    pub 0x7611: return mt76_chip(dev) ==,
    pub is_mt7622(dev): return is_mt7615(dev) || is_mt7663(dev) ||,
    pub false: return,
    pub true: return,
}

// LMAC uses the reverse order of mac80211 AC indexes
extern "C" {
    pub fn mt76_connac_pm_wake(phy: *mut mt76_phy, pm: *mut mt76_connac_pm) -> c_int;
}
extern "C" {
    pub fn mt76_connac_gen_ppe_thresh(he_ppet: *mut u8, nss: c_int, band: nl80211_band);
}
extern "C" {
    pub fn mt76_connac_set_txpower_cur(phy: *mut mt76_phy, max_power: i8);
}
extern "C" {
    pub fn mt76_connac2_tx_check_aggr(sta: *mut ieee80211_sta, txwi: *mut __le32);
}
extern "C" {
    pub fn mt76_connac2_tx_token_put(dev: *mut mt76_dev);
}
// connac3
