//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7921/mt7921.h
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

pub const MT7921_MAX_AID: c_int = 20;
pub const MT7921_TX_RING_SIZE: c_int = 2048;
pub const MT7921_TX_MCU_RING_SIZE: c_int = 256;
pub const MT7921_TX_FWDL_RING_SIZE: c_int = 128;
pub const MT7921_RX_RING_SIZE: c_int = 1536;
pub const MT7921_RX_MCU_RING_SIZE: c_int = 8;
pub const MT7921_RX_MCU_WA_RING_SIZE: c_int = 512;
// MT7902 Rx Ring0 is for both Rx Event and Tx Done Event
pub const MT7902_RX_MCU_RING_SIZE: c_int = 512;
pub const MT7921_EEPROM_SIZE: c_int = 3584;
pub const MT7921_TOKEN_SIZE: c_int = 8192;
pub const MT7921_EEPROM_BLOCK_SIZE: c_int = 16;
pub const MT7921_SKU_RATE_NUM: c_int = 161;

pub const MCU_UNI_EVENT_ROC: c_uint = 0x27;
pub const MCU_UNI_EVENT_CLC: c_uint = 0x80;
pub const EXT_CMD_RADIO_LED_CTRL_ENABLE: c_uint = 0x1;
pub const EXT_CMD_RADIO_ON_LED: c_uint = 0x2;
pub const EXT_CMD_RADIO_OFF_LED: c_uint = 0x3;
pub const WF_RF_PIN_INIT: c_uint = 0x0;
pub const WF_RF_PIN_POLL: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7921_roc_req {
    MT7921_ROC_REQ_JOIN,
    MT7921_ROC_REQ_ROC,
    MT7921_ROC_REQ_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_realease_info {
    pub len: __le16,
    pub pad_len: u8,
    pub tag: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_fw_features {
    pub segment: u8,
    pub data: u8,
    pub rsv: [u8; 14],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_roc_grant_tlv {
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7921_sdio_pkt_type {
    MT7921_SDIO_TXD,
    MT7921_SDIO_DATA,
    MT7921_SDIO_CMD,
    MT7921_SDIO_FWDL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_sdio_intr {
    pub isr: u32,
    pub wtqcr: [u32; 16],
    pub tx: },
    pub num: [u16; 2],
    pub len0: [u16; 16],
    pub len1: [u16; 128],
    pub rx: },
    pub rec_mb: [u32; 2],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7921_txq_id {
    MT7921_TXQ_BAND0,
    MT7921_TXQ_BAND1,
    MT7921_TXQ_FWDL = 16,
    MT7921_TXQ_MCU_WM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7921_rxq_id {
    MT7921_RXQ_BAND0 = 0,
    MT7921_RXQ_BAND1,
    MT7921_RXQ_MCU_WM = 0,
}

// MT7902 assigns its MCU-WM TXQ at index 15
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7902_txq_id {
    MT7902_TXQ_MCU_WM = 15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_dma_layout {
    pub mcu_wm_txq: u8,
    pub mcu_rxdone_ring_size: u16,
    pub has_mcu_wa: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_clc_rule {
    pub alpha2: [u8; 2],
    pub type: [u8; 2],
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_clc {
    pub len: __le32,
    pub idx: u8,
    pub ver: u8,
    pub nr_country: u8,
    pub type: u8,
    pub rsv: [u8; 8],
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt7921_eeprom_field {
    MT_EE_CHIP_ID =		0x000,
    MT_EE_VERSION =		0x002,
    MT_EE_MAC_ADDR =	0x004,
    MT_EE_WIFI_CONF =	0x07c,
    MT_EE_HW_TYPE =		0x55b,
    __MT_EE_MAX =		0x9ff
}

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_txpwr {
    pub ch: u8,
    pub rsv: [u8; 3],
    pub ch: u8,
    pub cck: [u8; 4],
    pub ofdm: [u8; 8],
    pub ht20: [u8; 8],
    pub ht40: [u8; 9],
    pub vht20: [u8; 12],
    pub vht40: [u8; 12],
    pub vht80: [u8; 12],
    pub vht160: [u8; 12],
    pub he26: [u8; 12],
    pub he52: [u8; 12],
    pub he106: [u8; 12],
    pub he242: [u8; 12],
    pub he484: [u8; 12],
    pub he996: [u8; 12],
    pub he996x2: [u8; 12],
    pub data: [}; TXPWR_MAX_NUM],
}

extern "C" {
    pub fn mt7921_reg_map(dev: *mut mt792x_dev, addr: u32) -> u32;
}
extern "C" {
    pub fn __mt7921_start(phy: *mut mt792x_phy) -> c_int;
}
extern "C" {
    pub fn mt7921_register_device(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921_unregister_device(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt7921_run_firmware(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921_set_channel(mphy: *mut mt76_phy) -> c_int;
}
extern "C" {
    pub fn mt7921_mcu_set_chan_info(phy: *mut mt792x_phy, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn mt7921_mcu_set_tx(dev: *mut mt792x_dev, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn mt7921_mcu_set_eeprom(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921_mcu_fw_log_2_host(dev: *mut mt792x_dev, ctrl: u8) -> c_int;
}
extern "C" {
    pub fn mt7921_mcu_rx_event(dev: *mut mt792x_dev, skb: *mut sk_buff);
}
extern "C" {
    pub fn mt7921_mcu_radio_led_ctrl(dev: *mut mt792x_dev, value: u8) -> c_int;
}
extern "C" {
    pub fn mt7921_mcu_wf_rf_pin_ctrl(phy: *mut mt792x_phy, action: u8) -> c_int;
}
// use read to push write
extern "C" {
    pub fn mt76_rr(_arg: dev, _arg: mt7921_reg_map_l1(dev, _arg: addr)) -> return;
}

extern "C" {
    pub fn mt7921_mac_init(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921_mac_wtbl_update(dev: *mut mt792x_dev, idx: c_int, mask: u32) -> bool;
}
extern "C" {
    pub fn mt7921_mac_reset_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7921_rx_check(mdev: *mut mt76_dev, data: *mut c_void, len: c_int) -> bool;
}
extern "C" {
    pub fn mt7921_stats_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7921_set_stream_he_caps(phy: *mut mt792x_phy);
}
extern "C" {
    pub fn mt7921_init_debugfs(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921_scan_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7921_roc_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7921_csa_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7921_mcu_uni_bss_ps(dev: *mut mt792x_dev, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn mt7921_coredump_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7921_get_txpwr_info(dev: *mut mt792x_dev, txpwr: *mut mt7921_txpwr) -> c_int;
}
extern "C" {
    pub fn mt7921e_driver_own(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921e_mac_reset(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921e_mcu_init(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921s_wfsys_reset(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921s_mac_reset(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921s_init_reset(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921s_mcu_init(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921s_mcu_drv_pmctrl(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921s_mcu_fw_pmctrl(dev: *mut mt792x_dev) -> c_int;
}
extern "C" {
    pub fn mt7921_mac_add_txs(dev: *mut mt792x_dev, data: *mut c_void);
}
extern "C" {
    pub fn mt7921_set_runtime_pm(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt7921_set_ipv6_ns_work(work: *mut work_struct);
}
extern "C" {
    pub fn mt7921_mcu_get_temperature(phy: *mut mt792x_phy) -> c_int;
}
extern "C" {
    pub fn mt7921_usb_sdio_tx_status_data(mdev: *mut mt76_dev, update: *mut u8) -> bool;
}
// usb
extern "C" {
    pub fn mt7921_roc_abort_sync(dev: *mut mt792x_dev);
}
extern "C" {
    pub fn mt7921_mcu_set_rssimonitor(dev: *mut mt792x_dev, vif: *mut ieee80211_vif) -> c_int;
}
