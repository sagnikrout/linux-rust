//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7615/mt7615.h
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
// Copyright (C) 2019 MediaTek Inc.

pub const MT7615_MAX_INTERFACES: c_int = 16;
pub const MT7615_MAX_WMM_SETS: c_int = 4;
pub const MT7663_WTBL_SIZE: c_int = 32;
pub const MT7615_WTBL_SIZE: c_int = 128;

pub const MT7615_RATE_RETRY: c_int = 2;
pub const MT7615_TX_RING_SIZE: c_int = 1024;
pub const MT7615_TX_MGMT_RING_SIZE: c_int = 128;
pub const MT7615_TX_MCU_RING_SIZE: c_int = 128;
pub const MT7615_TX_FWDL_RING_SIZE: c_int = 128;
pub const MT7615_RX_RING_SIZE: c_int = 1024;
pub const MT7615_RX_MCU_RING_SIZE: c_int = 512;
pub const MT7615_DRV_OWN_RETRY_COUNT: c_int = 10;

pub const MT7615_FIRMWARE_V1: c_int = 1;
pub const MT7615_FIRMWARE_V2: c_int = 2;
pub const MT7615_FIRMWARE_V3: c_int = 3;

pub const MT7615_EEPROM_SIZE: c_int = 1024;
pub const MT7663_EEPROM_SIZE: c_int = 1536;
pub const MT7615_TOKEN_SIZE: c_int = 4096;
pub const MT_FRAC_SCALE: c_int = 12;

pub const MT7615_BAR_RATE_DEFAULT: c_uint = 0x4b /* OFDM 6M */;
pub const MT7615_CFEND_RATE_DEFAULT: c_uint = 0x49 /* OFDM 24M */;
pub const MT7615_CFEND_RATE_11B: c_uint = 0x03 /* 11B LP, 11M */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7615_hw_txq_id {
    MT7615_TXQ_MAIN,
    MT7615_TXQ_EXT,
    MT7615_TXQ_MCU,
    MT7615_TXQ_FWDL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7622_hw_txq_id {
    MT7622_TXQ_AC0,
    MT7622_TXQ_AC1,
    MT7622_TXQ_AC2,
    MT7622_TXQ_FWDL = MT7615_TXQ_FWDL,
    MT7622_TXQ_AC3,
    MT7622_TXQ_MGMT,
    MT7622_TXQ_MCU = 15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_rate_set {
    pub probe_rate: ieee80211_tx_rate,
    pub rates: [ieee80211_tx_rate; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_rate_desc {
    pub rateset: bool,
    pub probe_val: u16,
    pub val: [u16; 4],
    pub bw_idx: u8,
    pub bw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_wtbl_rate_desc {
    pub node: list_head,
    pub rate: mt7615_rate_desc,
    pub sta: *mut mt7615_sta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7663s_intr {
    pub isr: u32,
    pub wtqcr: [u32; 8],
    pub tx: },
    pub num: [u16; 2],
    pub len: [u16; 2][16],
    pub rx: },
    pub rec_mb: [u32; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_sta {
    pub /: *mut *mut mt76_wcid wcid; / must be first,
    pub vif: *mut mt7615_vif,
    pub airtime_ac: [u32; 8],
    pub rates: [ieee80211_tx_rate; 4],
    pub rateset: [mt7615_rate_set; 2],
    pub rate_set_tsf: u32,
    pub rate_count: u8,
    pub n_rates: u8,
    pub rate_probe: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_vif {
    pub /: *mut *mut mt76_vif_link mt76; / must be first,
    pub sta: mt7615_sta,
    pub sta_added: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mib_stats {
    pub ack_fail_cnt: u32,
    pub fcs_err_cnt: u32,
    pub rts_cnt: u32,
    pub rts_retries_cnt: u32,
    pub ba_miss_cnt: u32,
    pub aggr_per: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_phy {
    pub mt76: *mut mt76_phy,
    pub dev: *mut mt7615_dev,
    pub monitor_vif: *mut ieee80211_vif,
    pub n_beacon_vif: u8,
    pub rxfilter: u32,
    pub omac_mask: u64,
    pub noise: u16,
    pub scs_en: bool,
    pub last_cca_adj: c_ulong,
    pub false_cca_cck: int false_cca_ofdm,,
    pub ofdm_sensitivity: i8,
    pub cck_sensitivity: i8,
    pub coverage_class: i16,
    pub slottime: u8,
    pub chfreq: u8,
    pub rdd_state: u8,
    pub rx_ampdu_ts: u32,
    pub ampdu_ref: u32,
    pub mib: mib_stats,
    pub scan_event_list: sk_buff_head,
    pub scan_work: delayed_work,
    pub roc_work: work_struct,
    pub roc_timer: timer_list,
    pub roc_wait: wait_queue_head_t,
    pub roc_grant: bool,

    pub reg_backup: *mut u32,
    pub last_freq_offset: i16,
    pub last_rcpi: [u8; 4],
    pub last_ib_rssi: [i8; 4],
    pub last_wb_rssi: [i8; 4],
    pub test: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_mcu_ops {
    pub enable): bool,
    pub enable): bool,
    pub enable): *mut *mut ieee80211_sta sta, bool,
    pub enable): bool,
    pub enable): *mut *mut ieee80211_sta sta, bool,
    pub enable): *mut *mut ieee80211_vif vif, bool,
    pub state): *mut *mut *mut int (set_pm_state)(struct mt7615_dev dev, int band, int,
    pub dev): *mut *mut int (set_drv_ctrl)(struct mt7615_dev,
    pub dev): *mut *mut int (set_fw_ctrl)(struct mt7615_dev,
    pub sta): *mut ieee80211_sta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_dev {
    pub mt76: mt76_dev,
    pub mphy: mt76_phy,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_pkt_queue_idx {
    MT_LMAC_AC00,
    MT_LMAC_AC01,
    MT_LMAC_AC02,
    MT_LMAC_AC03,
    MT_LMAC_ALTX0 = 0x10,
    MT_LMAC_BMC0,
    MT_LMAC_BCN0,
    MT_LMAC_PSMP0,
    MT_LMAC_ALTX1,
    MT_LMAC_BMC1,
    MT_LMAC_BCN1,
    MT_LMAC_PSMP1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7615_rdd_cmd {
    RDD_STOP,
    RDD_START,
    RDD_DET_MODE,
    RDD_DET_STOP,
    RDD_CAC_START,
    RDD_CAC_END,
    RDD_NORMAL_START,
    RDD_DISABLE_DFS_CAL,
    RDD_PULSE_DBG,
    RDD_READ_PULSE,
    RDD_RESUME_BF,
}

extern "C" {
    pub fn container_of(_arg: phy->dev, mt7615_dev: struct, _arg: mt76) -> return;
}

extern "C" {
    pub fn mt7622_wmac_init(dev: *mut mt7615_dev) -> c_int;
}

extern "C" {
    pub fn mt7615_thermal_init(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_reg_map(dev: *mut mt7615_dev, addr: u32) -> u32;
}
extern "C" {
    pub fn mt7615_reg_map(dev: *mut mt7615_dev, addr: u32) -> u32;
}
extern "C" {
    pub fn mt7615_init_device(dev: *mut mt7615_dev);
}
extern "C" {
    pub fn mt7615_register_device(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_unregister_device(dev: *mut mt7615_dev);
}
extern "C" {
    pub fn mt7615_register_ext_phy(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_unregister_ext_phy(dev: *mut mt7615_dev);
}
extern "C" {
    pub fn mt7615_eeprom_init(dev: *mut mt7615_dev, addr: u32) -> c_int;
}
extern "C" {
    pub fn mt7615_wait_pdma_busy(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_dma_init(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_dma_start(dev: *mut mt7615_dev);
}
extern "C" {
    pub fn mt7615_dma_cleanup(dev: *mut mt7615_dev);
}
extern "C" {
    pub fn mt7615_mcu_init(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_wait_for_mcu_init(dev: *mut mt7615_dev) -> bool;
}
extern "C" {
    pub fn mt7615_pm_wake_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7615_pm_power_save_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7615_mcu_set_chan_info(phy: *mut mt7615_phy, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_rx_event(dev: *mut mt7615_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn mt7615_mcu_rdd_send_pattern(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_fw_log_2_host(dev: *mut mt7615_dev, ctrl: u8) -> c_int;
}

extern "C" {
    pub fn MT_INT_TX_DONE(_arg: dev->mt76.q_mcu[MT_MCUQ_WM]->hw_idx) -> return;
}
extern "C" {
    pub fn mt7615_dma_reset(dev: *mut mt7615_dev);
}
extern "C" {
    pub fn mt7615_scan_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7615_roc_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7615_roc_timer(timer: *mut timer_list);
}
extern "C" {
    pub fn mt7615_set_channel(mphy: *mut mt76_phy) -> c_int;
}
extern "C" {
    pub fn mt7615_init_work(dev: *mut mt7615_dev);
}
extern "C" {
    pub fn mt7615_mcu_restart(dev: *mut mt76_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_update_channel(mphy: *mut mt76_phy);
}
extern "C" {
    pub fn mt7615_mac_wtbl_update(dev: *mut mt7615_dev, idx: c_int, mask: u32) -> bool;
}
extern "C" {
    pub fn mt7615_mac_reset_counters(phy: *mut mt7615_phy);
}
extern "C" {
    pub fn mt7615_mac_cca_stats_reset(phy: *mut mt7615_phy);
}
extern "C" {
    pub fn mt7615_mac_set_scs(phy: *mut mt7615_phy, enable: bool);
}
extern "C" {
    pub fn mt7615_mac_enable_nf(dev: *mut mt7615_dev, ext_phy: bool);
}
extern "C" {
    pub fn mt7615_mac_sta_poll(dev: *mut mt7615_dev);
}
extern "C" {
    pub fn mt7615_mac_set_timing(phy: *mut mt7615_phy);
}
extern "C" {
    pub fn mt7615_mac_reset_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7615_mac_get_sta_tid_sn(dev: *mut mt7615_dev, wcid: c_int, tid: u8) -> u32;
}
extern "C" {
    pub fn mt7615_rf_rr(dev: *mut mt7615_dev, wf: u32, reg: u32) -> u32;
}
extern "C" {
    pub fn mt7615_rf_wr(dev: *mut mt7615_dev, wf: u32, reg: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_set_dbdc(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_set_eeprom(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_get_temperature(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_set_tx_power(phy: *mut mt7615_phy) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_exit(dev: *mut mt7615_dev);
}
extern "C" {
    pub fn mt7615_tx_worker(w: *mut mt76_worker);
}
extern "C" {
    pub fn mt7615_tx_token_put(dev: *mut mt7615_dev);
}
extern "C" {
    pub fn mt7615_rx_check(mdev: *mut mt76_dev, data: *mut c_void, len: c_int) -> bool;
}
extern "C" {
    pub fn mt7615_mac_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7615_mcu_set_rx_hdr_trans_blacklist(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_set_fcc5_lpn(dev: *mut mt7615_dev, val: c_int) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_set_sku_en(phy: *mut mt7615_phy, enable: bool) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_apply_rx_dcoc(phy: *mut mt7615_phy) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_apply_tx_dpd(phy: *mut mt7615_phy) -> c_int;
}
extern "C" {
    pub fn mt7615_dfs_init_radar_detector(phy: *mut mt7615_phy) -> c_int;
}
extern "C" {
    pub fn mt7615_init_debugfs(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_mcu_wait_response(dev: *mut mt7615_dev, cmd: c_int, seq: c_int) -> c_int;
}
extern "C" {
    pub fn __mt7663_load_firmware(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7615_coredump_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7622_trigger_hif_int(dev: *mut mt7615_dev, en: bool);
}
// usb
extern "C" {
    pub fn mt7663_usb_sdio_tx_status_data(mdev: *mut mt76_dev, update: *mut u8) -> bool;
}
extern "C" {
    pub fn mt7663_usb_sdio_register_device(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7663u_mcu_init(dev: *mut mt7615_dev) -> c_int;
}
extern "C" {
    pub fn mt7663u_mcu_power_on(dev: *mut mt7615_dev) -> c_int;
}
// sdio
extern "C" {
    pub fn mt7663s_mcu_init(dev: *mut mt7615_dev) -> c_int;
}
