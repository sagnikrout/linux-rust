//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt7601u/mac.h
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
// Copyright (C) 2014 Felix Fietkau <nbd@openwrt.org>
// Copyright (C) 2015 Jakub Kicinski <kubakici@wp.pl>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_tx_status {
    pub valid:1: u8,
    pub success:1: u8,
    pub aggr:1: u8,
    pub ack_req:1: u8,
    pub is_probe:1: u8,
    pub wcid: u8,
    pub pktid: u8,
    pub retry: u8,
    pub rate: u16,
    pub __aligned(2): } __packed,
// Note: values in original "RSSI" and "SNR" fields are not actually what they
// are called for MT7601U, names used by this driver are educated guesses
// (see vendor mac/ral_omac.c).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_rxwi {
    pub rxinfo: __le32,
    pub ctl: __le32,
    pub frag_sn: __le16,
    pub rate: __le16,
    pub unknown: u8,
    pub zero: [u8; 3],
    pub snr: u8,
    pub ant: u8,
    pub gain: u8,
    pub freq_off: u8,
    pub resv2: __le32,
    pub expert_ant: __le32,
    pub __aligned(4): } __packed,

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_phy_type {
    MT_PHY_TYPE_CCK,
    MT_PHY_TYPE_OFDM,
    MT_PHY_TYPE_HT,
    MT_PHY_TYPE_HT_GF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_phy_bandwidth {
    MT_PHY_BW_20,
    MT_PHY_BW_40,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_txwi {
    pub flags: __le16,
    pub rate_ctl: __le16,
    pub ack_ctl: u8,
    pub wcid: u8,
    pub len_ctl: __le16,
    pub iv: __le32,
    pub eiv: __le32,
    pub aid: u8,
    pub txstream: u8,
    pub ctl: __le16,
    pub __aligned(4): } __packed,

    pub rxi): *mut *mut u8 data, void,
    pub key): *mut ieee80211_key_conf,
    pub rate): *const ieee80211_tx_rate,
    pub key): *mut ieee80211_key_conf,
    pub nss_val): *const *const ieee80211_tx_rate rate, u8,
    pub dev): *mut mt7601u_mac_fetch_tx_status(struct mt7601u_dev,
    pub stat): *mut *mut void mt76_send_tx_status(struct mt7601u_dev dev, struct mt76_tx_status,
    pub addr): *const *const void mt7601u_set_macaddr(struct mt7601u_dev dev, u8,
