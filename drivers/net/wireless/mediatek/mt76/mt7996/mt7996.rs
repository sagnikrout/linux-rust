//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7996/mt7996.h
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
// Copyright (C) 2022 MediaTek Inc.
//

pub const MT7996_MAX_RADIOS: c_int = 3;

pub const MT7996_MAX_WMM_SETS: c_int = 4;

pub const MT7996_TX_RING_SIZE: c_int = 2048;
pub const MT7996_TX_MCU_RING_SIZE: c_int = 256;
pub const MT7996_TX_FWDL_RING_SIZE: c_int = 128;
pub const MT7996_RX_RING_SIZE: c_int = 1536;
pub const MT7996_RX_MCU_RING_SIZE: c_int = 512;
pub const MT7996_RX_MCU_RING_SIZE_WA: c_int = 1024;
pub const MT7996_NPU_TX_RING_SIZE: c_int = 1024;
pub const MT7996_NPU_RX_RING_SIZE: c_int = 1024;
pub const MT7996_NPU_TXD_SIZE: c_int = 3;
// scatter-gather of mcu event is not supported in connac3

pub const MT7996_DEVICE_ID: c_uint = 0x7990;
pub const MT7996_DEVICE_ID_2: c_uint = 0x7991;
pub const MT7992_DEVICE_ID: c_uint = 0x7992;
pub const MT7992_DEVICE_ID_2: c_uint = 0x799a;
pub const MT7990_DEVICE_ID: c_uint = 0x7993;
pub const MT7990_DEVICE_ID_2: c_uint = 0x799b;

pub const MT7996_EEPROM_SIZE: c_int = 7680;
pub const MT7996_EEPROM_BLOCK_SIZE: c_int = 16;
pub const MT7996_EXT_EEPROM_BLOCK_SIZE: c_int = 1024;
pub const MT7996_TOKEN_SIZE: c_int = 16384;
pub const MT7996_HW_TOKEN_SIZE: c_int = 8192;
pub const MT7996_CFEND_RATE_DEFAULT: c_uint = 0x49	/* OFDM 24M */;
pub const MT7996_CFEND_RATE_11B: c_uint = 0x03	/* 11B LP, 11M */;
pub const MT7996_IBF_MAX_NC: c_int = 2;
pub const MT7996_IBF_TIMEOUT: c_uint = 0x18;
pub const MT7996_IBF_TIMEOUT_LEGACY: c_uint = 0x48;
pub const MT7992_CFEND_RATE_DEFAULT: c_uint = 0x4b	/* OFDM 6M */;
pub const MT7992_IBF_TIMEOUT: c_uint = 0xff;
pub const MT7996_SKU_RATE_NUM: c_int = 417;
pub const MT7996_SKU_PATH_NUM: c_int = 494;
pub const MT7996_MAX_TWT_AGRT: c_int = 16;
pub const MT7996_MAX_STA_TWT_AGRT: c_int = 8;
pub const MT7996_MIN_TWT_DUR: c_int = 64;

// NOTE: used to map mt76_rates. idx may change if firmware expands table
pub const MT7996_BASIC_RATES_TBL: c_int = 31;
pub const MT7996_BEACON_RATES_TBL: c_int = 25;
pub const MT7996_THERMAL_THROTTLE_MAX: c_int = 100;
pub const MT7996_CDEV_THROTTLE_MAX: c_int = 99;
pub const MT7996_CRIT_TEMP_IDX: c_int = 0;
pub const MT7996_MAX_TEMP_IDX: c_int = 1;
pub const MT7996_CRIT_TEMP: c_int = 110;
pub const MT7996_MAX_TEMP: c_int = 120;
pub const MT7996_MAX_HIF_RXD_IN_PG: c_int = 5;
pub const MT7996_RRO_MSDU_PG_HASH_SIZE: c_int = 127;
pub const MT7996_RRO_MAX_SESSION: c_int = 1024;
pub const MT7996_RRO_WINDOW_MAX_LEN: c_int = 1024;
pub const MT7996_RRO_ADDR_ELEM_LEN: c_int = 128;
pub const MT7996_RRO_BA_BITMAP_LEN: c_int = 2;

// RRO 3.1
pub const MT7996_RRO_MSDU_PG_CR_CNT: c_int = 8;
pub const MT7996_RRO_MSDU_PG_SIZE_PER_CR: c_uint = 0x10000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7996_ram_type {
    MT7996_RAM_TYPE_WM,
    MT7996_RAM_TYPE_WA,
    MT7996_RAM_TYPE_DSP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7996_var_type {
    MT7996_VAR_TYPE_444,
    MT7996_VAR_TYPE_233,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7992_var_type {
    MT7992_VAR_TYPE_44,
    MT7992_VAR_TYPE_23,
    MT7992_VAR_TYPE_24,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7990_var_type {
    MT7990_VAR_TYPE_23,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7996_fem_type {
    MT7996_FEM_EXT,
    MT7996_FEM_INT,
    MT7996_FEM_MIX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7996_eeprom_mode {
    EEPROM_MODE_DEFAULT_BIN,
    EEPROM_MODE_EFUSE,
    EEPROM_MODE_FLASH,
    EEPROM_MODE_EXT,
}

pub const MT7996_EFUSE_BASE_OFFS_ADIE0: c_uint = 0x400;
pub const MT7996_EFUSE_BASE_OFFS_ADIE1: c_uint = 0x1e00;
pub const MT7996_EFUSE_BASE_OFFS_ADIE2: c_uint = 0x1200;
pub const MT7992_EFUSE_BASE_OFFS_ADIE1: c_uint = 0x1200;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7996_txq_id {
    MT7996_TXQ_FWDL = 16,
    MT7996_TXQ_MCU_WM,
    MT7996_TXQ_BAND0,
    MT7996_TXQ_BAND1,
    MT7996_TXQ_MCU_WA,
    MT7996_TXQ_BAND2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7996_rxq_id {
    MT7996_RXQ_MCU_WM = 0,
    MT7996_RXQ_MCU_WA,
    MT7996_RXQ_MCU_WA_MAIN = 2,
    MT7996_RXQ_MCU_WA_EXT = 3, /* for mt7992 */
    MT7996_RXQ_MCU_WA_TRI = 3,
    MT7996_RXQ_BAND0 = 4,
    MT7996_RXQ_BAND1 = 5, /* for mt7992 */
    MT7996_RXQ_BAND2 = 5,
    MT7996_RXQ_RRO_BAND0 = 8,
    MT7996_RXQ_RRO_BAND1 = 9,
    MT7996_RXQ_RRO_BAND2 = 6,
    MT7996_RXQ_MSDU_PG_BAND0 = 10,
    MT7996_RXQ_MSDU_PG_BAND1 = 11,
    MT7996_RXQ_MSDU_PG_BAND2 = 12,
    MT7996_RXQ_TXFREE0 = 9,
    MT7996_RXQ_TXFREE1 = 9,
    MT7996_RXQ_TXFREE2 = 7,
    MT7996_RXQ_RRO_IND = 0,
    MT7996_RXQ_RRO_RXDMAD_C = 0,
    MT7990_RXQ_TXFREE0 = 6,
    MT7990_RXQ_TXFREE1 = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_twt_flow {
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
pub struct mt7996_sta_link {
    pub /: *mut *mut mt76_wcid wcid; / must be first,
    pub sta: *mut mt7996_sta,
    pub rc_list: list_head,
    pub airtime_ac: [u32; 8],
    pub ack_signal: c_int,
    pub avg_ack_signal: ewma_avg_signal,
    pub changed: c_ulong,
    pub bip: mt76_connac_sta_key_conf,
    pub flowid_mask: u8,
    pub flow: [mt7996_twt_flow; MT7996_MAX_STA_TWT_AGRT],
    pub twt: },
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_sta {
    pub /: *mut *mut mt7996_sta_link deflink; / must be first,
    pub link: [*mut mt7996_sta_link __rcu; IEEE80211_MLD_MAX_NUM_LINKS],
    pub deflink_id: u8,
    pub seclink_id: u8,
    pub vif: *mut mt7996_vif,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_vif_link {
    pub /: *mut *mut mt76_vif_link mt76; / must be first,
    pub msta_link: mt7996_sta_link,
    pub bitrate_mask: cfg80211_bitrate_mask,
    pub mld_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_vif_link_info {
    pub queue_params: [ieee80211_tx_queue_params; IEEE80211_NUM_ACS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_vif {
    pub /: *mut *mut mt7996_vif_link deflink; / must be first,
    pub mt76: mt76_vif_data,
    pub link_info: [mt7996_vif_link_info; IEEE80211_MLD_MAX_NUM_LINKS],
    pub mld_group_idx: u8,
    pub mld_remap_idx: u8,
}

// crash-dump
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_crash_data {
    pub guid: guid_t,
    pub timestamp: timespec64,
    pub memdump_buf: *mut u8,
    pub memdump_buf_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_hif {
    pub list: list_head,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub speed: pci_bus_speed,
    pub width: pcie_link_width,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_wed_rro_addr {
    pub head_low: __le32,
    pub data: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_wed_rro_session_id {
    pub list: list_head,
    pub id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_msdu_page {
    pub list: list_head,
    pub q: *mut mt76_queue,
    pub dma_addr: dma_addr_t,
    pub buf: *mut c_void,
}

// data1

// data4

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_rro_hif {
    pub data0: __le32,
    pub data1: __le32,
    pub data2: __le32,
    pub data3: __le32,
    pub data4: __le32,
    pub data5: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_msdu_page_info {
    pub rxd: [mt7996_rro_hif; MT7996_MAX_HIF_RXD_IN_PG],
    pub pg_low: __le32,
    pub data: __le32,
}

pub const MT7996_MAX_RRO_RRS_RING: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_rro_queue_regs_emi {
    pub idx: __le16,
    pub rsv: __le16,
    pub ring: [}; MT7996_MAX_RRO_RRS_RING],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_phy {
    pub mt76: *mut mt76_phy,
    pub dev: *mut mt7996_dev,
    pub iftype: [ieee80211_sband_iftype_data; NUM_NL80211_BANDS][NUM_NL80211_IFTYPES],
    pub cdev: *mut thermal_cooling_device,
    pub cdev_state: u8,
    pub throttle_state: u8,
    pub /: *mut *mut u32 throttle_temp[2]; / 0: critical high, 1: maximum,
    pub rxfilter: u32,
    pub omac_mask: u64,
    pub noise: u16,
    pub coverage_class: i16,
    pub slottime: u8,
    pub beacon_rate: u16,
    pub rx_ampdu_ts: u32,
    pub ampdu_ref: u32,
    pub txpower: c_int,
    pub mib: mt76_mib_stats,
    pub state_ts: mt76_channel_state,
    pub orig_chainmask: u16,
    pub orig_antenna_mask: u16,
    pub has_aux_rx: bool,
    pub counter_reset: bool,
    pub rdd_tx_paused: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_dev {
    pub mt76: mt76_dev,
    pub mphy: mt76_phy,
}

// monitor rx chain configured channel
// protects coredump data

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdd_idx {
    MT_RDD_IDX_BAND2,	/* RDD idx for band idx 2 */
    MT_RDD_IDX_BAND1,	/* RDD idx for band idx 1 */
    MT_RDD_IDX_BACKGROUND,	/* RDD idx for background chain */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7996_rdd_cmd {
    RDD_STOP,
    RDD_START,
    RDD_DET_MODE,
    RDD_RADAR_EMULATE,
    RDD_START_TXQ = 20,
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
    pub fn container_of(_arg: phy->dev, mt7996_dev: struct, _arg: mt76) -> return;
}
extern "C" {
    pub fn __mt7996_phy(_arg: dev, _arg: MT_BAND1) -> return;
}
extern "C" {
    pub fn __mt7996_phy(_arg: dev, _arg: MT_BAND2) -> return;
}
extern "C" {
    pub fn rcu_dereference(_arg: msta->link[link_id]) -> return;
}
extern "C" {
    pub fn mt76_dereference(_arg: msta->link[link_id], _arg: &dev->mt76) -> return;
}

extern "C" {
    pub fn mt7996_rro_hw_init(dev: *mut mt7996_dev);
}
extern "C" {
    pub fn mt7996_wfsys_reset(dev: *mut mt7996_dev);
}
extern "C" {
    pub fn mt7996_irq_handler(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn __mt7996_get_tsf(hw: *mut ieee80211_hw, link: *mut mt7996_vif_link) -> u64;
}
extern "C" {
    pub fn mt7996_register_device(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_unregister_device(dev: *mut mt7996_dev);
}
extern "C" {
    pub fn mt7996_eeprom_init(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_eeprom_parse_hw_cap(dev: *mut mt7996_dev, phy: *mut mt7996_phy) -> c_int;
}
extern "C" {
    pub fn mt7996_eeprom_get_power_delta(dev: *mut mt7996_dev, band: c_int) -> i8;
}
extern "C" {
    pub fn mt7996_eeprom_has_background_radar(dev: *mut mt7996_dev) -> bool;
}
extern "C" {
    pub fn mt7996_dma_init(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_dma_reset(dev: *mut mt7996_dev, force: bool);
}
extern "C" {
    pub fn mt7996_dma_prefetch(dev: *mut mt7996_dev);
}
extern "C" {
    pub fn mt7996_dma_cleanup(dev: *mut mt7996_dev);
}
extern "C" {
    pub fn mt7996_dma_start(dev: *mut mt7996_dev, reset: bool, wed_reset: bool);
}
extern "C" {
    pub fn mt7996_init_txpower(phy: *mut mt7996_phy);
}
extern "C" {
    pub fn mt7996_txbf_init(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_reset(dev: *mut mt7996_dev);
}
extern "C" {
    pub fn mt7996_run(phy: *mut mt7996_phy) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_init(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_init_firmware(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_update_sta_rec_bw(data: *mut c_void, sta: *mut ieee80211_sta);
}
extern "C" {
    pub fn mt7996_set_channel(mphy: *mut mt76_phy) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_chan_info(phy: *mut mt7996_phy, tag: u16) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_eeprom(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_get_efuse_free_block(dev: *mut mt7996_dev, block_num: *mut u8) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_get_chip_config(dev: *mut mt7996_dev, cap: *mut u32) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_ser(dev: *mut mt7996_dev, action: u8, set: u8, band: u8) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_txbf(dev: *mut mt7996_dev, action: u8) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_fcc5_lpn(dev: *mut mt7996_dev, val: c_int) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_radio_en(phy: *mut mt7996_phy, enable: bool) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_rts_thresh(phy: *mut mt7996_phy, val: u32) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_bssid_mapping_addr(dev: *mut mt76_dev, band_idx: u8) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_get_chan_mib_info(phy: *mut mt7996_phy, chan_switch: bool) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_get_temperature(phy: *mut mt7996_phy) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_thermal_throttling(phy: *mut mt7996_phy, state: u8) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_thermal_protect(phy: *mut mt7996_phy, enable: bool) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_txpower_sku(phy: *mut mt7996_phy) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_rdd_resume_tx(phy: *mut mt7996_phy) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_rdd_cmd(dev: *mut mt7996_dev, cmd: c_int, rdd_idx: u8, val: u8) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_rf_regval(dev: *mut mt7996_dev, regidx: u32, val: *mut u32, set: bool) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_hdr_trans(dev: *mut mt7996_dev, hdr_trans: bool) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_rro(dev: *mut mt7996_dev, tag: u16, val: u16) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_wa_cmd(dev: *mut mt7996_dev, cmd: c_int, a1: u32, a2: u32, a3: u32) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_fw_log_2_host(dev: *mut mt7996_dev, type: u8, ctrl: u8) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_fw_dbg_ctrl(dev: *mut mt7996_dev, module: u32, level: u8) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_trigger_assert(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_rx_event(dev: *mut mt7996_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn mt7996_mcu_exit(dev: *mut mt7996_dev);
}
extern "C" {
    pub fn mt7996_mcu_get_all_sta_info(phy: *mut mt7996_phy, tag: u16) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_wed_rro_reset_sessions(dev: *mut mt7996_dev, id: u16) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_sniffer_mode(phy: *mut mt7996_phy, enabled: bool) -> c_int;
}
extern "C" {
    pub fn mt7996_mcu_set_dup_wtbl(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_mac_init(dev: *mut mt7996_dev);
}
extern "C" {
    pub fn mt7996_mac_wtbl_lmac_addr(dev: *mut mt7996_dev, wcid: u16, dw: u8) -> u32;
}
extern "C" {
    pub fn mt7996_mac_wtbl_update(dev: *mut mt7996_dev, idx: c_int, mask: u32) -> bool;
}
extern "C" {
    pub fn mt7996_mac_reset_counters(phy: *mut mt7996_phy);
}
extern "C" {
    pub fn mt7996_mac_cca_stats_reset(phy: *mut mt7996_phy);
}
extern "C" {
    pub fn mt7996_mac_enable_nf(dev: *mut mt7996_dev, band: u8);
}
extern "C" {
    pub fn mt7996_mac_update_beacons(phy: *mut mt7996_phy);
}
extern "C" {
    pub fn mt7996_mac_set_coverage_class(phy: *mut mt7996_phy);
}
extern "C" {
    pub fn mt7996_mac_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7996_mac_reset_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7996_mac_dump_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7996_mac_sta_rc_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7996_mac_update_stats(phy: *mut mt7996_phy);
}
extern "C" {
    pub fn mt7996_tx_token_put(dev: *mut mt7996_dev);
}
extern "C" {
    pub fn mt7996_rro_msdu_page_map_free(dev: *mut mt7996_dev);
}
extern "C" {
    pub fn mt7996_rro_rx_process(mdev: *mut mt76_dev, data: *mut c_void);
}
extern "C" {
    pub fn mt7996_rx_check(mdev: *mut mt76_dev, data: *mut c_void, len: c_int) -> bool;
}
extern "C" {
    pub fn mt7996_stats_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt76_dfs_start_rdd(dev: *mut mt7996_dev, force: bool) -> c_int;
}
extern "C" {
    pub fn mt7996_dfs_init_radar_detector(phy: *mut mt7996_phy) -> c_int;
}
extern "C" {
    pub fn mt7996_set_stream_he_eht_caps(phy: *mut mt7996_phy);
}
extern "C" {
    pub fn mt7996_set_stream_vht_txbf_caps(phy: *mut mt7996_phy);
}
extern "C" {
    pub fn mt7996_update_channel(mphy: *mut mt76_phy);
}
extern "C" {
    pub fn mt7996_init_debugfs(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_debugfs_rx_fw_monitor(dev: *mut mt7996_dev, data: *const c_void, len: c_int);
}
extern "C" {
    pub fn mt7996_debugfs_rx_log(dev: *mut mt7996_dev, data: *const c_void, len: c_int) -> bool;
}
extern "C" {
    pub fn mt7996_mcu_cp_support(dev: *mut mt7996_dev, mode: u8) -> c_int;
}

extern "C" {
    pub fn mt7996_wed_init_buf(ptr: *mut c_void, phys: dma_addr_t, token_id: c_int) -> u32;
}
extern "C" {
    pub fn mt7996_dma_rro_init(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_dma_rro_start(dev: *mut mt7996_dev);
}

extern "C" {
    pub fn __mt7996_npu_hw_init(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_npu_hw_init(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_npu_hw_stop(dev: *mut mt7996_dev) -> c_int;
}
extern "C" {
    pub fn mt7996_npu_rx_queues_init(dev: *mut mt7996_dev) -> c_int;
}

