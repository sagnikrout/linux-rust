//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7925/mt7925.h
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
// Copyright (C) 2023 MediaTek Inc.

pub const MT7925_BEACON_RATES_TBL: c_int = 25;
pub const MT7925_TX_RING_SIZE: c_int = 2048;
pub const MT7925_TX_MCU_RING_SIZE: c_int = 256;
pub const MT7925_TX_FWDL_RING_SIZE: c_int = 128;
pub const MT7925_RX_RING_SIZE: c_int = 1536;
pub const MT7925_RX_MCU_RING_SIZE: c_int = 512;
pub const MT7928_RX_MCU_WA_RING_SIZE: c_int = 512;
pub const MT7925_EEPROM_SIZE: c_int = 3584;
pub const MT7925_TOKEN_SIZE: c_int = 8192;
pub const MT7925_EEPROM_BLOCK_SIZE: c_int = 16;
pub const MT7925_SKU_RATE_NUM: c_int = 161;

pub const MCU_UNI_EVENT_ROC: c_uint = 0x27;
pub const HIF_TRAFFIC_IDLE: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_mcu_hif_ctrl_basic_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub cid: u8,
    pub pad: [u8; 3],
    pub status: u32,
    pub hif_type: u8,
    pub hif_tx_traffic_status: u8,
    pub hif_rx_traffic_status: u8,
    pub hifsuspend: u8,
    pub rsv: [u8; 4],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7925_roc_req {
    MT7925_ROC_REQ_JOIN,
    MT7925_ROC_REQ_ROC,
    MT7925_ROC_REQ_SUB_LINK,
    MT7925_ROC_REQ_MLSR_AG = 10,
    MT7925_ROC_REQ_MLSR_AA,
    MT7925_ROC_REQ_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_roc_grant_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub bss_idx: u8,
    pub tokenid: u8,
    pub status: u8,
    pub primarychannel: u8,
    pub rfsco: u8,
    pub rfband: u8,
    pub channelwidth: u8,
    pub centerfreqseg1: u8,
    pub centerfreqseg2: u8,
    pub reqtype: u8,
    pub dbdcband: u8,
    pub rsv: [u8; 1],
    pub max_interval: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_beacon_loss_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub reason: u8,
    pub nr_btolink: u8,
    pub pad: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_uni_beacon_loss_event {
    pub bss_idx: u8,
    pub pad: [u8; 3],
    pub hdr: } __packed,
    pub beacon_loss: mt7925_beacon_loss_tlv,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_uni_rssi_monitor_event {
    pub tag: __le16,
    pub len: __le16,
    pub rssi: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7925_txq_id {
    MT7925_TXQ_BAND0,
    MT7925_TXQ_BAND1,
    MT7925_TXQ_MCU_WM = 15,
    MT7925_TXQ_FWDL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7925_rxq_id {
    MT7925_RXQ_BAND0 = 2,
    MT7925_RXQ_BAND1,
    MT7925_RXQ_MCU_WM = 0,
    MT7925_RXQ_MCU_WM2, /* for tx done */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7927_txq_id {
    MT7927_TXQ_BAND0 = MT7925_TXQ_BAND0,
    MT7927_TXQ_BAND1 = MT7925_TXQ_BAND1,
    MT7927_TXQ_MCU_WM = MT7925_TXQ_MCU_WM,
    MT7927_TXQ_FWDL = MT7925_TXQ_FWDL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7927_rxq_id {
    MT7927_RXQ_BAND0 = 4,
    MT7927_RXQ_MCU_WM = 6,
    MT7927_RXQ_DATA2 = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7928_rxq_id {
    MT7928_RXQ_BAND0,
    MT7928_RXQ_BAND1 = 2,
    MT7928_RXQ_MCU_WM = 3,
    MT7928_RXQ_MCU_WM2 = 1, /* for tx done */
}

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_clc_rule_v2 {
    pub flag: u32,
    pub alpha2: [u8; 2],
    pub rsv: [u8; 10],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_clc_rule {
    pub alpha2: [u8; 2],
    pub type: [u8; 2],
    pub seg_idx: u8,
    pub /: *mut *mut u8 flag; / UNII4~8 ctrl flag,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_clc_segment {
    pub idx: u8,
    pub rsv1: [u8; 3],
    pub offset: u32,
    pub len: u32,
    pub rsv2: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_clc_type0 {
    pub nr_country: u8,
    pub type: u8,
    pub nr_seg: u8,
    pub rsv: [u8; 7],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_clc_type2 {
    pub type: u8,
    pub rsv: [u8; 9],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_clc {
    pub len: __le32,
    pub idx: u8,
    pub ver: u8,
    pub t0: mt7925_clc_type0,
    pub t2: mt7925_clc_type2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7925_eeprom_field {
    MT_EE_CHIP_ID =		0x000,
    MT_EE_VERSION =		0x002,
    MT_EE_MAC_ADDR =	0x004,
    MT_EE_HW_TYPE =		0xa71,
    __MT_EE_MAX =		0x9ff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7925_txpwr {
    pub cck: [i8; 4][2],
    pub ofdm: [i8; 8][2],
    pub ht20: [i8; 8][2],
    pub ht40: [i8; 9][2],
    pub vht20: [i8; 12][2],
    pub vht40: [i8; 12][2],
    pub vht80: [i8; 12][2],
    pub vht160: [i8; 12][2],
    pub he26: [i8; 12][2],
    pub he52: [i8; 12][2],
    pub he106: [i8; 12][2],
    pub he242: [i8; 12][2],
    pub he484: [i8; 12][2],
    pub he996: [i8; 12][2],
    pub he996x2: [i8; 12][2],
    pub eht26: [i8; 16][2],
    pub eht52: [i8; 16][2],
    pub eht106: [i8; 16][2],
    pub eht242: [i8; 16][2],
    pub eht484: [i8; 16][2],
    pub eht996: [i8; 16][2],
    pub eht996x2: [i8; 16][2],
    pub eht996x4: [i8; 16][2],
    pub eht26_52: [i8; 16][2],
    pub eht26_106: [i8; 16][2],
    pub eht484_242: [i8; 16][2],
    pub eht996_484: [i8; 16][2],
    pub eht996_484_242: [i8; 16][2],
    pub eht996x2_484: [i8; 16][2],
    pub eht996x3: [i8; 16][2],
    pub eht996x3_484: [i8; 16][2],
}

extern "C" {
    pub fn __mt7925_start(phy: *mut mt792x_phy) -> c_int;
}
extern "C" {
    pub fn mt7925_register_device(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7925_unregister_device(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt7925_run_firmware(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7925_mcu_set_chan_info(phy: *mut mt792x_phy, tag: u16) -> c_int;
}
extern "C" {
    pub fn mt7925_mcu_set_tx(dev: *mut mt792x_dev, bss_conf: *mut ieee80211_bss_conf) -> c_int;
}
extern "C" {
    pub fn mt7925_mcu_set_eeprom(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7925_mcu_fw_log_2_host(dev: *mut mt792x_dev, ctrl: u8) -> c_int;
}
extern "C" {
    pub fn mt7925_mcu_rx_event(dev: *mut mt792x_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn mt7925_mcu_chip_config(dev: *mut mt792x_dev, cmd: *const c_char) -> c_int;
}
extern "C" {
    pub fn mt7925_mac_init(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7925_mac_wtbl_update(dev: *mut mt792x_dev, idx: c_int, mask: u32) -> bool;
}
extern "C" {
    pub fn mt7925_mac_reset_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7925_tx_token_put(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt7925_rx_check(mdev: *mut mt76_dev, data: *mut c_void, len: c_int) -> bool;
}
extern "C" {
    pub fn mt7925_stats_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7925_set_stream_he_eht_caps(phy: *mut mt792x_phy);
}
extern "C" {
    pub fn mt7925_init_mlo_caps(phy: *mut mt792x_phy) -> c_int;
}
extern "C" {
    pub fn mt7925_init_debugfs(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7925_mlo_pm_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7925_scan_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7925_roc_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7925_csa_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7925_coredump_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7925e_mac_reset(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7925e_mcu_init(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7925_mac_add_txs(dev: *mut mt792x_dev, data: *mut c_void);
}
extern "C" {
    pub fn mt7928_mac_add_txs_msg(dev: *mut mt792x_dev, evt: *mut c_void);
}
extern "C" {
    pub fn mt7925_set_runtime_pm(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt7925_set_ipv6_ns_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7925_mcu_get_temperature(phy: *mut mt792x_phy) -> c_int;
}
extern "C" {
    pub fn mt7925_usb_sdio_tx_status_data(mdev: *mut mt76_dev, update: *mut u8) -> bool;
}
extern "C" {
    pub fn mt7925_mcu_regval(dev: *mut mt792x_dev, regidx: u32, val: *mut u32, set: bool) -> c_int;
}
extern "C" {
    pub fn mt7925_roc_abort_sync(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt7925_mcu_set_rts_thresh(phy: *mut mt792x_phy, val: u32) -> c_int;
}
extern "C" {
    pub fn mt7925_mcu_wf_rf_pin_ctrl(phy: *mut mt792x_phy) -> c_int;
}
extern "C" {
    pub fn mt7925_mcu_set_rssimonitor(dev: *mut mt792x_dev, vif: *mut ieee80211_vif) -> c_int;
}
