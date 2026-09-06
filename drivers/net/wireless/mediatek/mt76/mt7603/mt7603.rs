//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7603/mt7603.h
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

pub const MT7603_MAX_INTERFACES: c_int = 4;
pub const MT7603_WTBL_SIZE: c_int = 128;

pub const MT7603_RATE_RETRY: c_int = 2;
pub const MT7603_MCU_RX_RING_SIZE: c_int = 64;
pub const MT7603_RX_RING_SIZE: c_int = 128;
pub const MT7603_TX_RING_SIZE: c_int = 256;
pub const MT7603_PSD_RING_SIZE: c_int = 128;

pub const MT7603_EEPROM_SIZE: c_int = 1024;

pub const MT7603_EDCCA_BLOCK_TH: c_int = 10;
pub const MT7603_CFEND_RATE_DEFAULT: c_uint = 0x69 /* chip default (24M) */;
pub const MT7603_CFEND_RATE_11B: c_uint = 0x03 /* 11B LP, 11M */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7603_bw {
    MT_BW_20,
    MT_BW_40,
    MT_BW_80,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7603_rate_set {
    pub probe_rate: ieee80211_tx_rate,
    pub rates: [ieee80211_tx_rate; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7603_sta {
    pub /: *mut *mut mt76_wcid wcid; / must be first,
    pub vif: *mut mt7603_vif,
    pub tx_airtime_ac: [u32; 4],
    pub psq: sk_buff_head,
    pub rates: [ieee80211_tx_rate; 4],
    pub rateset: [mt7603_rate_set; 2],
    pub rate_set_tsf: u32,
    pub rate_count: u8,
    pub n_rates: u8,
    pub rate_probe: u8,
    pub smps: u8,
    pub ps: u8,
    pub ps_sleeping: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7603_vif {
    pub /: *mut *mut mt7603_sta sta; / must be first,
    pub idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7603_reset_cause {
    RESET_CAUSE_TX_HANG,
    RESET_CAUSE_TX_BUSY,
    RESET_CAUSE_RX_BUSY,
    RESET_CAUSE_BEACON_STUCK,
    RESET_CAUSE_RX_PSE_BUSY,
    RESET_CAUSE_MCU_HANG,
    RESET_CAUSE_RESET_FAILED,
    __RESET_CAUSE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7603_dev {
    pub mt76: mt76_dev,
    pub mphy: mt76_phy,
}

// need offset to prevent conflict with ampdu_ack_len
pub const MT_RATE_DRIVER_DATA_OFFSET: c_int = 4;
extern "C" {
    pub fn mt7603_reg_map(dev: *mut mt7603_dev, addr: u32) -> u32;
}
extern "C" {
    pub fn mt7603_irq_handler(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn mt7603_register_device(dev: *mut mt7603_dev) -> c_int;
}
extern "C" {
    pub fn mt7603_unregister_device(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_eeprom_init(dev: *mut mt7603_dev) -> c_int;
}
extern "C" {
    pub fn mt7603_dma_init(dev: *mut mt7603_dev) -> c_int;
}
extern "C" {
    pub fn mt7603_dma_cleanup(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_mcu_init(dev: *mut mt7603_dev) -> c_int;
}
extern "C" {
    pub fn mt7603_init_debugfs(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_mac_reset_counters(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_mac_dma_start(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_mac_start(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_mac_stop(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_mac_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7603_mac_set_timing(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_beacon_set_timer(dev: *mut mt7603_dev, idx: c_int, intval: c_int);
}
extern "C" {
    pub fn mt7603_mac_fill_rx(dev: *mut mt7603_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn mt7603_mac_add_txs(dev: *mut mt7603_dev, data: *mut c_void);
}
extern "C" {
    pub fn mt7603_mac_rx_ba_reset(dev: *mut mt7603_dev, addr: *mut c_void, tid: u8);
}
extern "C" {
    pub fn mt7603_mac_sta_poll(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_pse_client_reset(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_set_channel(mphy: *mut mt76_phy) -> c_int;
}
extern "C" {
    pub fn mt7603_mcu_set_channel(dev: *mut mt7603_dev) -> c_int;
}
extern "C" {
    pub fn mt7603_mcu_set_eeprom(dev: *mut mt7603_dev) -> c_int;
}
extern "C" {
    pub fn mt7603_mcu_exit(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_wtbl_clear(dev: *mut mt7603_dev, idx: c_int);
}
extern "C" {
    pub fn mt7603_wtbl_update_cap(dev: *mut mt7603_dev, sta: *mut ieee80211_sta);
}
extern "C" {
    pub fn mt7603_wtbl_restore_ps(dev: *mut mt7603_dev, sta: *mut mt7603_sta);
}
extern "C" {
    pub fn mt7603_filter_tx(dev: *mut mt7603_dev, mac_idx: c_int, idx: c_int, abort: bool);
}
extern "C" {
    pub fn mt7603_tx_complete_skb(mdev: *mut mt76_dev, e: *mut mt76_queue_entry);
}
extern "C" {
    pub fn mt7603_rx_poll_complete(mdev: *mut mt76_dev, q: mt76_rxq_id);
}
extern "C" {
    pub fn mt7603_sta_ps(mdev: *mut mt76_dev, sta: *mut ieee80211_sta, ps: bool);
}
extern "C" {
    pub fn mt7603_pre_tbtt_tasklet(t: *mut tasklet_struct);
}
extern "C" {
    pub fn mt7603_update_channel(mphy: *mut mt76_phy);
}
extern "C" {
    pub fn mt7603_edcca_set_strict(dev: *mut mt7603_dev, val: bool);
}
extern "C" {
    pub fn mt7603_cca_stats_reset(dev: *mut mt7603_dev);
}
extern "C" {
    pub fn mt7603_init_edcca(dev: *mut mt7603_dev);
}
