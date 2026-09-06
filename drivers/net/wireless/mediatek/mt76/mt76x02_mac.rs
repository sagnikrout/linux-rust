//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x02_mac.h
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
//
// Copyright (C) 2016 Felix Fietkau <nbd@nbd.name>
// Copyright (C) 2018 Stanislaw Gruszka <stf_xl@wp.pl>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_tx_status {
    pub valid:1: u8,
    pub success:1: u8,
    pub aggr:1: u8,
    pub ack_req:1: u8,
    pub wcid: u8,
    pub pktid: u8,
    pub retry: u8,
    pub rate: u16,
    pub __aligned(2): } __packed,

pub const MT_MAX_VIFS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_vif {
    pub /: *mut *mut mt76_wcid group_wcid; / must be first,
    pub idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_sta {
    pub /: *mut *mut mt76_wcid wcid; / must be first,
    pub vif: *mut mt76x02_vif,
    pub status: mt76x02_tx_status,
    pub n_frames: c_int,
    pub pktlen: ewma_pktlen,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_rxwi {
    pub rxinfo: __le32,
    pub ctl: __le32,
    pub tid_sn: __le16,
    pub rate: __le16,
    pub rssi: [u8; 4],
    pub bbp_rxinfo: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76x2_phy_bandwidth {
    MT_PHY_BW_20,
    MT_PHY_BW_40,
    MT_PHY_BW_80,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_txwi {
    pub flags: __le16,
    pub rate: __le16,
    pub ack_ctl: u8,
    pub wcid: u8,
    pub len_ctl: __le16,
    pub iv: __le32,
    pub eiv: __le32,
    pub aid: u8,
    pub txstream: u8,
    pub ctl2: u8,
    pub pktid: u8,
    pub __aligned(4): } __packed,
    pub 0x1000: u32 MAC_CSR0 =,
    pub i: c_int,
    pub {: for (i = 0; i < 500; i++),
    pub false: return,
    pub true: return,
    pub 10000): usleep_range(5000,,
    pub false: return,
    pub dev): *mut void mt76x02_mac_reset_counters(struct mt76x02_dev,
    pub enable): *mut *mut void mt76x02_mac_set_short_preamble(struct mt76x02_dev dev, bool,
    pub key): *mut u8 key_idx, struct ieee80211_key_conf,
    pub key): *mut ieee80211_key_conf,
    pub key): *mut ieee80211_key_conf,
    pub mac): *mut u8,
    pub drop): *mut *mut void mt76x02_mac_wcid_set_drop(struct mt76x02_dev dev, u8 idx, bool,
    pub rate): *const ieee80211_tx_rate,
    pub stat): *mut mt76x02_tx_status,
    pub update): *mut *mut mt76x02_tx_status stat, u8,
    pub rxi): *mut c_void,
    pub ht_mode): c_int,
    pub val): *mut *mut void mt76x02_mac_set_rts_thresh(struct mt76x02_dev dev, u32,
    pub addr): *const *const void mt76x02_mac_setaddr(struct mt76x02_dev dev, u8,
    pub len): *mut *mut ieee80211_sta sta, int,
    pub irq): *mut *mut void mt76x02_mac_poll_tx_status(struct mt76x02_dev dev, bool,
    pub e): *mut *mut void mt76x02_tx_complete_skb(struct mt76_dev mdev, struct mt76_queue_entry,
    pub mphy): *mut void mt76x02_update_channel(struct mt76_phy,
    pub work): *mut void mt76x02_mac_work(struct work_struct,
    pub dev): *mut void mt76x02_mac_cc_reset(struct mt76x02_dev,
    pub addr): *const *const void mt76x02_mac_set_bssid(struct mt76x02_dev dev, u8 idx, u8,
    pub skb): *mut *mut void mt76x02_mac_set_beacon(struct mt76x02_dev dev, struct sk_buff,
    pub enable): *mut *mut ieee80211_vif vif, bool,
    pub dev): *mut void mt76x02_edcca_init(struct mt76x02_dev,
