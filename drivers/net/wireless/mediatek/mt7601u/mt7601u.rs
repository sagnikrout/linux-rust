//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt7601u/mt7601u.h
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

pub const MT_BBP_REG_VERSION: c_uint = 0x00;

pub const MT_USB_AGGR_TIMEOUT: c_uint = 0x80 /* * 33ns */;
pub const MT_RX_ORDER: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_dma_buf {
    pub urb: *mut urb,
    pub buf: *mut c_void,
    pub dma: dma_addr_t,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_mcu {
    pub mutex: mutex,
    pub msg_seq: u8,
    pub resp: mt7601u_dma_buf,
    pub resp_cmpl: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_freq_cal {
    pub work: delayed_work,
    pub freq: u8,
    pub enabled: bool,
    pub adjusting: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_stats {
    pub rx_stat: [u64; 6],
    pub tx_stat: [u64; 6],
    pub aggr_stat: [u64; 2],
    pub aggr_n: [u64; 32],
    pub zero_len_del: [u64; 2],
}

pub const N_RX_ENTRIES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_rx_queue {
    pub dev: *mut mt7601u_dev,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_dma_buf_rx {
    pub urb: *mut urb,
    pub p: *mut page,
    pub e: [}; N_RX_ENTRIES],
    pub start: c_uint,
    pub end: c_uint,
    pub entries: c_uint,
    pub pending: c_uint,
}

pub const N_TX_ENTRIES: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_tx_queue {
    pub dev: *mut mt7601u_dev,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_dma_buf_tx {
    pub urb: *mut urb,
    pub skb: *mut sk_buff,
    pub e: [}; N_TX_ENTRIES],
    pub start: c_uint,
    pub end: c_uint,
    pub entries: c_uint,
    pub used: c_uint,
    pub fifo_seq: c_uint,
}

// WCID allocation:
// 0: mcast wcid
// 1: bssid wcid
// 1...: STAs
// ...7e: group wcids
// 7f: reserved
//
pub const N_WCIDS: c_int = 128;

pub const MT_EE_TEMPERATURE_SLOPE: c_int = 39;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt_temp_mode {
    MT_TEMP_MODE_NORMAL,
    MT_TEMP_MODE_HIGH,
    MT_TEMP_MODE_LOW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt_bw {
    MT_BW_20,
    MT_BW_40,
}

//
// struct mt7601u_dev - adapter structure
// @lock:		protects @wcid->tx_rate.
// @mac_lock:		locks out mac80211's tx status and rx paths.
// @tx_lock:		protects @tx_q and changes of MT7601U_STATE_*_STATS
// flags in @state.
// @rx_lock:		protects @rx_q.
// @con_mon_lock:	protects @ap_bssid, @bcn_*, @avg_rssi.
// @mutex:		ensures exclusive access from mac80211 callbacks.
// @vendor_req_mutex:	protects @vend_buf, ensures atomicity of read/write
// accesses
// @reg_atomic_mutex:	ensures atomicity of indirect register accesses
// (accesses to RF and BBP).
// @hw_atomic_mutex:	ensures exclusive access to HW during critical
// operations (power management, channel switch).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_dev {
    pub hw: *mut ieee80211_hw,
    pub dev: *mut device,
    pub state: c_ulong,
    pub mutex: mutex,
    pub BITS_PER_LONG]: unsigned long wcid_mask[N_WCIDS /,
    pub chandef: cfg80211_chan_def,
    pub sband_2g: *mut ieee80211_supported_band,
    pub mcu: mt7601u_mcu,
    pub cal_work: delayed_work,
    pub mac_work: delayed_work,
    pub stat_wq: *mut workqueue_struct,
    pub stat_work: delayed_work,
    pub mon_wcid: *mut mt76_wcid,
    pub wcid: [*mut mt76_wcid __rcu; N_WCIDS],
    pub lock: spinlock_t,
    pub mac_lock: spinlock_t,
    pub beacon_offsets: *const u16,
    pub macaddr: [u8; ETH_ALEN],
    pub ee: *mut mt7601u_eeprom_params,
    pub vendor_req_mutex: mutex,
    pub vend_buf: *mut c_void,
    pub reg_atomic_mutex: mutex,
    pub hw_atomic_mutex: mutex,
    pub rxfilter: u32,
    pub debugfs_reg: u32,
    pub out_eps: [u8; 8],
    pub in_eps: [u8; 8],
    pub out_max_packet: u16,
    pub in_max_packet: u16,
// TX
    pub tx_lock: spinlock_t,
    pub tx_tasklet: tasklet_struct,
    pub tx_q: *mut mt7601u_tx_queue,
    pub tx_skb_done: sk_buff_head,
    pub avg_ampdu_len: core::sync::atomic::AtomicI32,
// RX
    pub rx_lock: spinlock_t,
    pub rx_tasklet: tasklet_struct,
    pub rx_q: mt7601u_rx_queue,
// Connection monitoring things
    pub con_mon_lock: spinlock_t,
    pub ap_bssid: [u8; ETH_ALEN],
    pub bcn_freq_off: i8,
    pub bcn_phy_mode: u8,
    pub avg_rssi: ewma_rssi,
    pub agc_save: u8,
    pub freq_cal: mt7601u_freq_cal,
    pub tssi_read_trig: bool,
    pub tssi_init: i8,
    pub tssi_init_hvga: i8,
    pub tssi_init_hvga_offset_db: i16,
    pub prev_pwr_diff: c_int,
    pub temp_mode: mt_temp_mode,
    pub curr_temp: c_int,
    pub dpd_temp: c_int,
    pub raw_temp: i8,
    pub pll_lock_protect: bool,
    pub bw: u8,
    pub chan_ext_below: bool,
// PA mode
    pub rf_pa_mode: [u32; 2],
    pub stats: mac_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7601u_tssi_params {
    pub tssi0: c_char,
    pub trgt_power: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_wcid {
    pub idx: u8,
    pub hw_key_idx: u8,
    pub tx_rate: u16,
    pub tx_rate_set: bool,
    pub tx_rate_nss: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_vif {
    pub idx: u8,
    pub group_wcid: mt76_wcid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_sta {
    pub wcid: mt76_wcid,
    pub agg_ssn: [u16; IEEE80211_NUM_TIDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_reg_pair {
    pub reg: u32,
    pub value: u32,
}

extern "C" {
    pub fn mt7601u_init_debugfs(dev: *mut mt7601u_dev);
}
extern "C" {
    pub fn mt7601u_rr(dev: *mut mt7601u_dev, offset: u32) -> u32;
}
extern "C" {
    pub fn mt7601u_wr(dev: *mut mt7601u_dev, offset: u32, val: u32);
}
extern "C" {
    pub fn mt7601u_rmw(dev: *mut mt7601u_dev, offset: u32, mask: u32, val: u32) -> u32;
}
extern "C" {
    pub fn mt7601u_rmc(dev: *mut mt7601u_dev, offset: u32, mask: u32, val: u32) -> u32;
}
extern "C" {
    pub fn mt7601u_wait_asic_ready(dev: *mut mt7601u_dev) -> c_int;
}
// Compatibility with mt76

extern "C" {
    pub fn mt7601u_rr(_arg: dev, _arg: offset) -> return;
}
extern "C" {
    pub fn mt7601u_wr(_arg: dev, _arg: offset, _arg: val) -> return;
}
extern "C" {
    pub fn mt7601u_rmw(_arg: dev, _arg: offset, _arg: mask, _arg: val) -> return;
}
extern "C" {
    pub fn mt76_rmw(_arg: dev, _arg: offset, _arg: 0, _arg: val) -> return;
}
extern "C" {
    pub fn mt76_rmw(_arg: dev, _arg: offset, _arg: val, _arg: 0) -> return;
}
extern "C" {
    pub fn mt7601u_addr_wr(dev: *mut mt7601u_dev, offset: u32, addr: *const u8);
}
// Init
extern "C" {
    pub fn mt7601u_init_hardware(dev: *mut mt7601u_dev) -> c_int;
}
extern "C" {
    pub fn mt7601u_register_device(dev: *mut mt7601u_dev) -> c_int;
}
extern "C" {
    pub fn mt7601u_cleanup(dev: *mut mt7601u_dev);
}
extern "C" {
    pub fn mt7601u_mac_start(dev: *mut mt7601u_dev) -> c_int;
}
extern "C" {
    pub fn mt7601u_mac_stop(dev: *mut mt7601u_dev);
}
// PHY
extern "C" {
    pub fn mt7601u_phy_init(dev: *mut mt7601u_dev) -> c_int;
}
extern "C" {
    pub fn mt7601u_wait_bbp_ready(dev: *mut mt7601u_dev) -> c_int;
}
extern "C" {
    pub fn mt7601u_set_rx_path(dev: *mut mt7601u_dev, path: u8);
}
extern "C" {
    pub fn mt7601u_set_tx_dac(dev: *mut mt7601u_dev, path: u8);
}
extern "C" {
    pub fn mt7601u_bbp_set_bw(dev: *mut mt7601u_dev, bw: c_int) -> c_int;
}
extern "C" {
    pub fn mt7601u_agc_save(dev: *mut mt7601u_dev);
}
extern "C" {
    pub fn mt7601u_agc_restore(dev: *mut mt7601u_dev);
}
extern "C" {
    pub fn mt7601u_phy_recalibrate_after_assoc(dev: *mut mt7601u_dev);
}
// MAC
extern "C" {
    pub fn mt7601u_mac_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7601u_mac_set_short_preamble(dev: *mut mt7601u_dev, short_preamb: bool);
}
extern "C" {
    pub fn mt7601u_mac_config_tsf(dev: *mut mt7601u_dev, enable: bool, interval: c_int);
}
extern "C" {
    pub fn mt7601u_mac_set_ampdu_factor(dev: *mut mt7601u_dev);
}
// TX
extern "C" {
    pub fn mt7601u_tx_status(dev: *mut mt7601u_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn mt7601u_tx_stat(work: *mut work_struct);
}
// util
extern "C" {
    pub fn mt76_remove_hdr_pad(skb: *mut sk_buff);
}
extern "C" {
    pub fn mt76_insert_hdr_pad(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn mt7601u_bbp_set_ctrlch(dev: *mut mt7601u_dev, below: bool) -> u32;
}
extern "C" {
    pub fn mt7601u_rmc(_arg: dev, _arg: MT_TX_BAND_CFG, _arg: 1, _arg: below) -> return;
}
extern "C" {
    pub fn mt7601u_dma_init(dev: *mut mt7601u_dev) -> c_int;
}
extern "C" {
    pub fn mt7601u_dma_cleanup(dev: *mut mt7601u_dev);
}
