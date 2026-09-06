//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x02.h
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

// Macro flag: #define __MT76x02_H

pub const MT76x02_TX_RING_SIZE: c_int = 512;
pub const MT76x02_PSD_RING_SIZE: c_int = 128;
pub const MT76x02_N_WCIDS: c_int = 128;

pub const MT_TX_HANG_TH: c_int = 10;
pub const MT_MAX_CHAINS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_rx_freq_cal {
    pub high_gain: [i8; MT_MAX_CHAINS],
    pub rssi_offset: [i8; MT_MAX_CHAINS],
    pub lna_gain: i8,
    pub mcu_gain: u32,
    pub temp_offset: i16,
    pub freq_offset: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_calibration {
    pub rx: mt76x02_rx_freq_cal,
    pub agc_gain_init: [u8; MT_MAX_CHAINS],
    pub agc_gain_cur: [u8; MT_MAX_CHAINS],
    pub false_cca: u16,
    pub avg_rssi_all: i8,
    pub agc_gain_adjust: i8,
    pub agc_lowest_gain: i8,
    pub low_gain: i8,
    pub temp_vco: i8,
    pub temp: i8,
    pub init_cal_done: bool,
    pub tssi_cal_done: bool,
    pub tssi_comp_pending: bool,
    pub dpd_cal_done: bool,
    pub channel_cal_done: bool,
    pub gain_init_done: bool,
    pub tssi_target: c_int,
    pub tssi_dc: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_beacon_ops {
    pub nslots: c_uint,
    pub slot_size: c_uint,
    pub en): *mut *mut *mut void (pre_tbtt_enable)(struct mt76x02_dev dev, bool,
    pub en): *mut *mut *mut void (beacon_enable)(struct mt76x02_dev dev, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_rate_power {
    pub cck: [i8; 4],
    pub ofdm: [i8; 8],
    pub ht: [i8; 16],
    pub vht: [i8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x02_dev {
    pub mt76: mt76_dev,
    pub mphy: mt76_phy,
}

// edcca monitor
extern "C" {
    pub fn mt76x02_init_device(dev: *mut mt76x02_dev) -> c_int;
}
extern "C" {
    pub fn mt76x02_config_mac_addr_list(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02_wdt_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt76x02_tx_set_txpwr_auto(dev: *mut mt76x02_dev, txpwr: i8);
}
extern "C" {
    pub fn mt76x02_set_tx_ackto(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02_set_rts_threshold(hw: *mut ieee80211_hw, radio_idx: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn mt76x02_remove_hdr_pad(skb: *mut sk_buff, len: c_int);
}
extern "C" {
    pub fn mt76x02_tx_status_data(mdev: *mut mt76_dev, update: *mut u8) -> bool;
}
extern "C" {
    pub fn mt76x02_rx_poll_complete(mdev: *mut mt76_dev, q: mt76_rxq_id);
}
extern "C" {
    pub fn mt76x02_irq_handler(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn mt76x02_sta_ps(dev: *mut mt76_dev, sta: *mut ieee80211_sta, ps: bool);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct beacon_bc_data {
    pub dev: *mut mt76x02_dev,
    pub q: sk_buff_head,
    pub tail: [*mut sk_buff; 8],
}

extern "C" {
    pub fn mt76x02_init_beacon_config(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02e_init_beacon_config(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02_resync_beacon_timer(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02_update_beacon_iter(priv: *mut c_void, mac: *mut u8, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn mt76x02_mac_start(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn mt76x02_init_debugfs(dev: *mut mt76x02_dev);
}
extern "C" {
    pub fn container_of(_arg: wcid, mt76x02_sta: struct, _arg: wcid) -> return;
}
