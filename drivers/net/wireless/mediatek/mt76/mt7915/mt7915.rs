//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7915/mt7915.h
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

pub const MT7915_MAX_INTERFACES: c_int = 19;
pub const MT7915_WTBL_SIZE: c_int = 288;
pub const MT7916_WTBL_SIZE: c_int = 544;

pub const MT7915_TX_RING_SIZE: c_int = 2048;
pub const MT7915_TX_MCU_RING_SIZE: c_int = 256;
pub const MT7915_TX_FWDL_RING_SIZE: c_int = 128;
pub const MT7915_RX_RING_SIZE: c_int = 1536;
pub const MT7915_RX_MCU_RING_SIZE: c_int = 512;

pub const MT7915_EEPROM_SIZE: c_int = 3584;
pub const MT7916_EEPROM_SIZE: c_int = 4096;
pub const MT7915_EEPROM_BLOCK_SIZE: c_int = 16;
pub const MT7915_HW_TOKEN_SIZE: c_int = 4096;
pub const MT7915_TOKEN_SIZE: c_int = 8192;
pub const MT7915_CFEND_RATE_DEFAULT: c_uint = 0x49	/* OFDM 24M */;
pub const MT7915_CFEND_RATE_11B: c_uint = 0x03	/* 11B LP, 11M */;
pub const MT7915_THERMAL_THROTTLE_MAX: c_int = 100;
pub const MT7915_CDEV_THROTTLE_MAX: c_int = 99;
pub const MT7915_SKU_RATE_NUM: c_int = 161;
pub const MT7915_SKU_PATH_NUM: c_int = 185;
pub const MT7915_MAX_TWT_AGRT: c_int = 16;
pub const MT7915_MAX_STA_TWT_AGRT: c_int = 8;
pub const MT7915_MIN_TWT_DUR: c_int = 64;

pub const MT7915_WED_RX_TOKEN_SIZE: c_int = 12288;
pub const MT7915_CRIT_TEMP_IDX: c_int = 0;
pub const MT7915_MAX_TEMP_IDX: c_int = 1;
pub const MT7915_CRIT_TEMP: c_int = 110;
pub const MT7915_MAX_TEMP: c_int = 120;
pub const MT7915_RTS_LEN_THRES: c_uint = 0x92b;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7915_txq_id {
    MT7915_TXQ_FWDL = 16,
    MT7915_TXQ_MCU_WM,
    MT7915_TXQ_BAND0,
    MT7915_TXQ_BAND1,
    MT7915_TXQ_MCU_WA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7915_rxq_id {
    MT7915_RXQ_BAND0 = 0,
    MT7915_RXQ_BAND1,
    MT7915_RXQ_MCU_WM = 0,
    MT7915_RXQ_MCU_WA,
    MT7915_RXQ_MCU_WA_EXT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7916_rxq_id {
    MT7916_RXQ_MCU_WM = 0,
    MT7916_RXQ_MCU_WA,
    MT7916_RXQ_MCU_WA_MAIN,
    MT7916_RXQ_MCU_WA_EXT,
    MT7916_RXQ_BAND0,
    MT7916_RXQ_BAND1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_twt_flow {
    pub list: list_head,
    pub start_tsf: u64,
    pub tsf: u64,
    pub duration: u32,
    pub wcid: u16,
    pub mantissa: __le16,
    pub exp: u8,
    pub table_id: u8,
    pub id: u8,
    pub protection:1: u8,
    pub flowtype:1: u8,
    pub trigger:1: u8,
    pub sched:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_sta {
    pub /: *mut *mut mt76_wcid wcid; / must be first,
    pub vif: *mut mt7915_vif,
    pub rc_list: list_head,
    pub airtime_ac: [u32; 8],
    pub ack_signal: c_int,
    pub avg_ack_signal: ewma_avg_signal,
    pub changed: c_ulong,
    pub jiffies: c_ulong,
    pub bip: mt76_connac_sta_key_conf,
    pub flowid_mask: u8,
    pub flow: [mt7915_twt_flow; MT7915_MAX_STA_TWT_AGRT],
    pub twt: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_vif_cap {
    pub ht_ldpc:1: bool,
    pub vht_ldpc:1: bool,
    pub he_ldpc:1: bool,
    pub vht_su_ebfer:1: bool,
    pub vht_su_ebfee:1: bool,
    pub vht_mu_ebfer:1: bool,
    pub vht_mu_ebfee:1: bool,
    pub he_su_ebfer:1: bool,
    pub he_su_ebfee:1: bool,
    pub he_mu_ebfer:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_vif {
    pub /: *mut *mut mt76_vif_link mt76; / must be first,
    pub cap: mt7915_vif_cap,
    pub sta: mt7915_sta,
    pub phy: *mut mt7915_phy,
    pub queue_params: [ieee80211_tx_queue_params; IEEE80211_NUM_ACS],
    pub bitrate_mask: cfg80211_bitrate_mask,
}

// crash-dump
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_crash_data {
    pub guid: guid_t,
    pub timestamp: timespec64,
    pub memdump_buf: *mut u8,
    pub memdump_buf_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_hif {
    pub list: list_head,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_phy {
    pub mt76: *mut mt76_phy,
    pub dev: *mut mt7915_dev,
    pub iftype: [ieee80211_sband_iftype_data; NUM_NL80211_BANDS][NUM_NL80211_IFTYPES],
    pub monitor_vif: *mut ieee80211_vif,
    pub tzone: *mut thermal_zone_device,
    pub cdev: *mut thermal_cooling_device,
    pub cdev_state: u8,
    pub throttle_state: u8,
    pub /: *mut *mut u32 throttle_temp[2]; / 0: critical high, 1: maximum,
    pub rxfilter: u32,
    pub omac_mask: u64,
    pub noise: u16,
    pub coverage_class: i16,
    pub slottime: u8,
    pub trb_ts: u32,
    pub rx_ampdu_ts: u32,
    pub ampdu_ref: u32,
    pub mib: mt76_mib_stats,
    pub state_ts: mt76_channel_state,
    pub sku_limit_en:1: bool,
    pub sku_path_en:1: bool,

    pub reg_backup: *mut u32,
    pub last_freq_offset: i32,
    pub last_rcpi: [u8; 4],
    pub last_ib_rssi: [i8; 4],
    pub last_wb_rssi: [i8; 4],
    pub last_snr: u8,
    pub spe_idx: u8,
    pub test: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_dev {
    pub mt76: mt76_dev,
    pub mphy: mt76_phy,
}

// monitor rx chain configured channel
// protects coredump data

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdd_idx {
    MT_RDD_IDX_BAND0,	/* RDD idx for band idx 0 (single-band) */
    MT_RDD_IDX_BAND1,	/* RDD idx for band idx 1 */
    MT_RDD_IDX_BACKGROUND,	/* RDD idx for background chain */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7915_rdd_cmd {
    RDD_STOP,
    RDD_START,
    RDD_DET_MODE,
    RDD_RADAR_EMULATE,
    RDD_START_TXQ = 20,
    RDD_SET_WF_ANT = 30,
    RDD_CAC_START = 50,
    RDD_CAC_END,
    RDD_NORMAL_START,
    RDD_DISABLE_DFS_CAL,
    RDD_PULSE_DBG,
    RDD_READ_PULSE,
    RDD_RESUME_BF,
    RDD_IRQ_OFF,
}

extern "C" {
    pub fn container_of(_arg: phy->dev, mt7915_dev: struct, _arg: mt76) -> return;
}
// without dbdc, the chainmask is stored unshifted, even if the phy is
// bound to band 1
//

extern "C" {
    pub fn mt7986_wmac_enable(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7986_wmac_disable(dev: *mut mt7915_dev);
}

extern "C" {
    pub fn mt7915_wfsys_reset(dev: *mut mt7915_dev);
}
extern "C" {
    pub fn mt7915_irq_handler(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn __mt7915_get_tsf(hw: *mut ieee80211_hw, mvif: *mut mt7915_vif) -> u64;
}
extern "C" {
    pub fn mt7915_wed_init_buf(ptr: *mut c_void, phys: dma_addr_t, token_id: c_int) -> u32;
}
extern "C" {
    pub fn mt7915_register_device(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7915_unregister_device(dev: *mut mt7915_dev);
}
extern "C" {
    pub fn mt7915_eeprom_init(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7915_eeprom_get_power_delta(dev: *mut mt7915_dev, band: c_int) -> i8;
}
extern "C" {
    pub fn mt7915_eeprom_has_background_radar(dev: *mut mt7915_dev) -> bool;
}
extern "C" {
    pub fn mt7915_dma_init(dev: *mut mt7915_dev, phy2: *mut mt7915_phy) -> c_int;
}
extern "C" {
    pub fn mt7915_dma_prefetch(dev: *mut mt7915_dev);
}
extern "C" {
    pub fn mt7915_dma_cleanup(dev: *mut mt7915_dev);
}
extern "C" {
    pub fn mt7915_dma_reset(dev: *mut mt7915_dev, force: bool) -> c_int;
}
extern "C" {
    pub fn mt7915_dma_start(dev: *mut mt7915_dev, reset: bool, wed_reset: bool) -> c_int;
}
extern "C" {
    pub fn mt7915_txbf_init(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7915_init_txpower(phy: *mut mt7915_phy);
}
extern "C" {
    pub fn mt7915_reset(dev: *mut mt7915_dev);
}
extern "C" {
    pub fn mt7915_run(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_init(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_init_firmware(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7915_set_channel(mphy: *mut mt76_phy) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_chan_info(phy: *mut mt7915_phy, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_tx(dev: *mut mt7915_dev, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_update_edca(dev: *mut mt7915_dev, req: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_eeprom(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_get_eeprom(dev: *mut mt7915_dev, offset: u32, read_buf: *mut u8) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_get_eeprom_free_block(dev: *mut mt7915_dev, block_num: *mut u8) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_ser(dev: *mut mt7915_dev, action: u8, set: u8, band: u8) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_sku_en(phy: *mut mt7915_phy) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_txpower_sku(phy: *mut mt7915_phy) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_txpower_frame_min(phy: *mut mt7915_phy, txpower: i8) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_txbf(dev: *mut mt7915_dev, action: u8) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_fcc5_lpn(dev: *mut mt7915_dev, val: c_int) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_muru_ctrl(dev: *mut mt7915_dev, cmd: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_apply_group_cal(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_apply_tx_dpd(phy: *mut mt7915_phy) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_get_chan_mib_info(phy: *mut mt7915_phy, chan_switch: bool) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_get_temperature(phy: *mut mt7915_phy) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_thermal_throttling(phy: *mut mt7915_phy, state: u8) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_set_thermal_protect(phy: *mut mt7915_phy) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_wed_wa_tx_stats(dev: *mut mt7915_dev, wcid: u16) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_rf_regval(dev: *mut mt7915_dev, regidx: u32, val: *mut u32, set: bool) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_wa_cmd(dev: *mut mt7915_dev, cmd: c_int, a1: u32, a2: u32, a3: u32) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_fw_log_2_host(dev: *mut mt7915_dev, type: u8, ctrl: u8) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_fw_dbg_ctrl(dev: *mut mt7915_dev, module: u32, level: u8) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_rx_event(dev: *mut mt7915_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn mt7915_mcu_exit(dev: *mut mt7915_dev);
}
extern "C" {
    pub fn mt7915_mac_init(dev: *mut mt7915_dev);
}
extern "C" {
    pub fn mt7915_mac_wtbl_lmac_addr(dev: *mut mt7915_dev, wcid: u16, dw: u8) -> u32;
}
extern "C" {
    pub fn mt7915_mac_wtbl_update(dev: *mut mt7915_dev, idx: c_int, mask: u32) -> bool;
}
extern "C" {
    pub fn mt7915_mac_reset_counters(phy: *mut mt7915_phy);
}
extern "C" {
    pub fn mt7915_mac_cca_stats_reset(phy: *mut mt7915_phy);
}
extern "C" {
    pub fn mt7915_mac_enable_nf(dev: *mut mt7915_dev, ext_phy: bool);
}
extern "C" {
    pub fn mt7915_mac_set_timing(phy: *mut mt7915_phy);
}
extern "C" {
    pub fn mt7915_mac_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7915_mac_reset_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7915_mac_dump_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7915_mac_sta_rc_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7915_mac_update_stats(phy: *mut mt7915_phy);
}
extern "C" {
    pub fn mt7915_rx_check(mdev: *mut mt76_dev, data: *mut c_void, len: c_int) -> bool;
}
extern "C" {
    pub fn mt7915_stats_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt76_dfs_start_rdd(dev: *mut mt7915_dev, force: bool) -> c_int;
}
extern "C" {
    pub fn mt7915_dfs_init_radar_detector(phy: *mut mt7915_phy) -> c_int;
}
extern "C" {
    pub fn mt7915_set_stream_he_caps(phy: *mut mt7915_phy);
}
extern "C" {
    pub fn mt7915_set_stream_vht_txbf_caps(phy: *mut mt7915_phy);
}
extern "C" {
    pub fn mt7915_update_channel(mphy: *mut mt76_phy);
}
extern "C" {
    pub fn mt7915_mcu_muru_debug_set(dev: *mut mt7915_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_muru_debug_get(phy: *mut mt7915_phy) -> c_int;
}
extern "C" {
    pub fn mt7915_mcu_wed_enable_rx_stats(dev: *mut mt7915_dev) -> c_int;
}
extern "C" {
    pub fn mt7915_init_debugfs(phy: *mut mt7915_phy) -> c_int;
}
extern "C" {
    pub fn mt7915_debugfs_rx_fw_monitor(dev: *mut mt7915_dev, data: *const c_void, len: c_int);
}
extern "C" {
    pub fn mt7915_debugfs_rx_log(dev: *mut mt7915_dev, data: *const c_void, len: c_int) -> bool;
}

